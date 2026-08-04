terraform {
  required_version = ">= 1.0"
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 5.0"
    }
  }
}

provider "google" {
  project = var.gcp_project
  region  = var.gcp_region
}

module "kcm" {
  source = "../modules/kcm"

  cluster_name    = var.cluster_name
  cluster_version = var.cluster_version
  network         = var.network
  subnetwork      = var.subnetwork
  
  kcm_image     = var.kcm_image
  kcm_replicas  = var.kcm_replicas
  kcm_namespace = var.kcm_namespace
  
  environment = var.environment
  labels      = var.labels
}
