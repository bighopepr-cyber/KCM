variable "cluster_name" {
  description = "AKS cluster name"
  type        = string
}

variable "cluster_version" {
  description = "AKS cluster version"
  type        = string
  default     = "1.28"
}

variable "resource_group" {
  description = "Resource group name"
  type        = string
}

variable "vnet_subnet_id" {
  description = "VNet subnet ID"
  type        = string
}

variable "kcm_image" {
  description = "KCM Docker image"
  type        = string
  default     = "kcm/kcm-server:latest"
}

variable "kcm_replicas" {
  description = "Number of KCM replicas"
  type        = number
  default     = 1
}

variable "kcm_namespace" {
  description = "Kubernetes namespace"
  type        = string
  default     = "kcm"
}

variable "environment" {
  description = "Environment name"
  type        = string
  default     = "production"
}

variable "tags" {
  description = "Resource tags"
  type        = map(string)
  default     = {}
}
