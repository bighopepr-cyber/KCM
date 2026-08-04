resource "kubernetes_namespace" "kcm" {
  metadata {
    name = var.kcm_namespace
    labels = {
      app = "kcm"
    }
  }
}

resource "kubernetes_config_map" "kcm" {
  metadata {
    name      = "kcm-config"
    namespace = kubernetes_namespace.kcm.metadata[0].name
  }

  data = {
    RUST_LOG       = "info"
    KCM_DATA_PATH  = "/data/kcm.db"
    KCM_BIND_ADDR  = "0.0.0.0:8080"
  }
}

resource "kubernetes_stateful_set" "kcm" {
  metadata {
    name      = "kcm-server"
    namespace = kubernetes_namespace.kcm.metadata[0].name
    labels = {
      app = "kcm-server"
    }
  }

  spec {
    service_name = "kcm-service"
    replicas     = var.kcm_replicas

    selector {
      match_labels = {
        app = "kcm-server"
      }
    }

    template {
      metadata {
        labels = {
          app = "kcm-server"
        }
      }

      spec {
        container {
          name  = "kcm-server"
          image = var.kcm_image

          port {
            container_port = 8080
            name           = "http"
          }

          port {
            container_port = 50051
            name           = "grpc"
          }

          env_from {
            config_map_ref {
              name = kubernetes_config_map.kcm.metadata[0].name
            }
          }

          resources {
            requests = {
              memory = "512Mi"
              cpu    = "500m"
            }
            limits = {
              memory = "2Gi"
              cpu    = "2000m"
            }
          }

          liveness_probe {
            http_get {
              path = "/health"
              port = 8080
            }
            initial_delay_seconds = 10
            period_seconds        = 10
          }

          readiness_probe {
            http_get {
              path = "/health"
              port = 8080
            }
            initial_delay_seconds = 5
            period_seconds        = 5
          }

          volume_mount {
            name       = "data"
            mount_path = "/data"
          }
        }
      }
    }

    volume_claim_template {
      metadata {
        name = "data"
      }

      spec {
        access_modes = ["ReadWriteOnce"]
        resources {
          requests = {
            storage = "100Gi"
          }
        }
      }
    }
  }
}

resource "kubernetes_service" "kcm" {
  metadata {
    name      = "kcm-service"
    namespace = kubernetes_namespace.kcm.metadata[0].name
  }

  spec {
    selector = {
      app = "kcm-server"
    }

    port {
      name        = "http"
      port        = 8080
      target_port = 8080
    }

    port {
      name        = "grpc"
      port        = 50051
      target_port = 50051
    }

    type = "LoadBalancer"
  }
}
