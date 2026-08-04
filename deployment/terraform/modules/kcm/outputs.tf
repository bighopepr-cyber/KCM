output "endpoint" {
  description = "KCM service endpoint"
  value       = "${kubernetes_service.kcm.status[0].load_balancer[0].ingress[0].hostname}:8080"
}

output "grpc_endpoint" {
  description = "KCM gRPC endpoint"
  value       = "${kubernetes_service.kcm.status[0].load_balancer[0].ingress[0].hostname}:50051"
}

output "namespace" {
  description = "KCM namespace"
  value       = kubernetes_namespace.kcm.metadata[0].name
}
