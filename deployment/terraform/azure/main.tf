terraform {
  required_version = ">= 1.0"
  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "~> 3.0"
    }
  }
}

provider "azurerm" {
  features {}
}

module "kcm" {
  source = "../modules/kcm"

  cluster_name    = var.cluster_name
  cluster_version = var.cluster_version
  resource_group  = var.resource_group
  vnet_subnet_id  = var.vnet_subnet_id
  
  kcm_image     = var.kcm_image
  kcm_replicas  = var.kcm_replicas
  kcm_namespace = var.kcm_namespace
  
  environment = var.environment
  tags        = var.tags
}
