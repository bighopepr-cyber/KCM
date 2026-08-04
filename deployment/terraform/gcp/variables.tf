variable "gcp_project" {
  description = "GCP project ID"
  type        = string
}

variable "gcp_region" {
  description = "GCP region"
  type        = string
  default     = "us-central1"
}

variable "cluster_name" {
  description = "GKE cluster name"
  type        = string
}

variable "cluster_version" {
  description = "GKE cluster version"
  type        = string
  default     = "1.28"
}

variable "network" {
  description = "VPC network"
  type        = string
}

variable "subnetwork" {
  description = "Subnetwork"
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

variable "labels" {
  description = "Resource labels"
  type        = map(string)
  default     = {}
}
