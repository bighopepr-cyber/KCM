output "cluster_endpoint" {
  description = "EKS cluster endpoint"
  value       = module.kcm.endpoint
}

output "cluster_name" {
  description = "EKS cluster name"
  value       = var.cluster_name
}
