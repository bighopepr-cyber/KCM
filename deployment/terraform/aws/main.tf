terraform {
  required_version = ">= 1.0"
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

provider "aws" {
  region = var.aws_region
}

module "kcm" {
  source = "../modules/kcm"

  cluster_name    = var.cluster_name
  cluster_version = var.cluster_version
  vpc_id          = var.vpc_id
  subnet_ids      = var.subnet_ids
  
  kcm_image     = var.kcm_image
  kcm_replicas  = var.kcm_replicas
  kcm_namespace = var.kcm_namespace
  
  environment = var.environment
  tags        = var.tags
}

output "kcm_endpoint" {
  value = module.kcm.endpoint
}

output "kcm_grpc_endpoint" {
  value = module.kcm.grpc_endpoint
}
