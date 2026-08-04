variable "aws_region" {
  description = "AWS region"
  type        = string
  default     = "us-east-1"
}

variable "cluster_name" {
  description = "EKS cluster name"
  type        = string
}

variable "cluster_version" {
  description = "EKS cluster version"
  type        = string
  default     = "1.28"
}

variable "vpc_id" {
  description = "VPC ID"
  type        = string
}

variable "subnet_ids" {
  description = "Subnet IDs"
  type        = list(string)
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
