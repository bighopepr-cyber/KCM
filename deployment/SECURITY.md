# deployment/ Security Policy

> For project-wide security policies, refer to the [SECURITY.md](../SECURITY.md) located in the repository root.

## Overview

This document defines security requirements, threat models, and validation procedures for the KCM deployment infrastructure. All deployment configurations — Docker, Kubernetes, Helm, Terraform, Prometheus, and Grafana — must comply with these policies before any release artifact is produced.

## Security Scope

| Component | Risk Level | Rationale |
|-----------|------------|-----------|
| Docker | **High** | Container images run with elevated privileges; misconfiguration leads to container escape or privilege escalation |
| Kubernetes | **High** | Cluster-level access control, network policy enforcement, and workload isolation depend on correct configuration |
| Helm | **High** | Helm charts control Kubernetes resource creation; template injection or secret leakage can compromise the entire cluster |
| Terraform | **High** | Infrastructure-as-Code provisions cloud resources; state files contain secrets and resource addresses |
| Prometheus | **Medium** | Exposes internal metrics endpoints; misconfigured scraping can leak sensitive runtime data |
| Grafana | **Medium** | Dashboard access reveals system topology; default credentials or open endpoints expose operational intelligence |

## Threat Model

### Container Escape

Attackers may exploit kernel vulnerabilities, misconfigured container runtimes, or privileged containers to escape the container boundary and access the host system. Mitigations include running all containers as non-root, dropping Linux capabilities, using seccomp/AppArmor profiles, and disabling privileged mode.

### Privilege Escalation

Misconfigured RBAC roles, over-privileged service accounts, or hostPath volume mounts can allow workloads to escalate privileges within the cluster. All Helm templates must enforce least-privilege service accounts and restrict volume access.

### Secrets in Images

Embedding credentials, API keys, or TLS certificates in Docker images creates a persistent attack surface. All secrets must be injected at runtime via Kubernetes secrets, Helm secrets, or cloud provider secret managers — never baked into images.

### Exposed Ports

Unnecessary exposed ports expand the attack surface. Each container must expose only the ports required for its function. Network policies must restrict ingress and egress traffic to approved sources.

### RBAC Misconfiguration

Overly permissive ClusterRoleBindings or wildcard rules can grant cluster-admin access to untrusted workloads. Helm templates must use namespace-scoped Roles with explicit verb and resource lists.

### Resource Exhaustion

Unbounded resource requests allow a single compromised workload to starve others. All deployments must define CPU and memory limits and requests. Horizontal Pod Autoscalers must include maximum replica constraints.

## Security Risks

| Risk | Severity | Mitigation |
|------|----------|------------|
| Container escape via privileged mode | Critical | Drop ALL capabilities, run as non-root, use seccomp profiles |
| Secrets embedded in container layers | Critical | Use Kubernetes secrets, mount as volumes, rotate regularly |
| Overly permissive RBAC rules | High | Namespace-scoped roles, explicit verb/resource lists |
| Unrestricted network egress | High | Default-deny network policies, explicit egress allowlists |
| Terraform state containing plaintext secrets | Critical | Use remote state backends with encryption at rest |
| Grafana default credentials | High | Force credential change on first login, use SSO |
| Unscanned container images | High | Integrate image scanning in CI, block known CVEs |
| Missing resource limits | Medium | Enforce ResourceQuota and LimitRange in namespaces |
| Plaintext Prometheus scrape configs | Medium | Use Kubernetes service discovery with proper auth |
| Helm values containing hardcoded secrets | Critical | Use sealed-secrets or external secret operators |

## Access Control

### Deployment Infrastructure Access

| Resource | Access Level | Granting Mechanism |
|----------|-------------|-------------------|
| Docker registry | Push/pull | CI/CD service account |
| Kubernetes cluster | Deploy | Deployer RoleBinding |
| Kubernetes namespaces | Create/delete | Namespace admin RoleBinding |
| Terraform state | Read/write | CI/CD service account with S3/GCS access |
| Prometheus | Read | Network policy + basic auth |
| Grafana | Read/write | RBAC + SSO integration |

### Principle of Least Privilege

All service accounts must be granted only the minimum permissions required for their function. No default service accounts may be used for deployment workloads. Audit all role bindings quarterly.

## RBAC Integration

### Kubernetes RBAC

Kubernetes RBAC controls namespace-level access. Helm templates must define:

- **ServiceAccount** per workload with no default token automounting
- **Role** with explicit verbs (`get`, `list`, `watch`, `create`, `update`, `delete`) and resources
- **RoleBinding** scoped to the target namespace

### kcm-security Integration

The `kcm-security` crate provides application-level RBAC with 5 permission levels:

| kcm-security Level | Kubernetes Equivalent | Use Case |
|--------------------|-----------------------|----------|
| None (0) | ClusterRole: none | Unauthenticated health checks |
| Read (1) | Role: get, list, watch | Query operations |
| Write (2) | Role: get, list, watch, create, update | Insert/update operations |
| Admin (3) | Role: all verbs | Schema management |
| SuperAdmin (4) | ClusterRole: all verbs | Cluster-wide administration |

Deployment configurations must map kcm-security permission levels to Kubernetes RBAC roles to enforce consistent authorization across application and infrastructure layers.

## Sensitive Assets

| Asset | Sensitivity | Storage | Rotation |
|-------|-------------|---------|----------|
| Docker images | High | Container registry (private) | Rebuild on dependency update |
| Helm values (production) | Critical | Sealed secrets / Vault | 90 days |
| Terraform state | Critical | Remote backend (encrypted) | N/A (contains resource addresses) |
| Terraform variables (secrets) | Critical | Vault / environment variables | 90 days |
| Prometheus configs | Medium | ConfigMap / GitOps | On change |
| Grafana dashboards | Medium | ConfigMap / provisioning files | On change |
| TLS certificates | Critical | Kubernetes TLS secrets | 30 days |
| Service account tokens | High | Kubernetes (auto-generated) | 365 days |

## Secret Management

### Kubernetes Secrets

All application secrets must be stored as Kubernetes Secret resources, never in ConfigMaps or environment variables in plaintext. Use one of:

- **Sealed Secrets**: Encrypt secrets for safe storage in Git
- **External Secrets Operator**: Sync from cloud provider secret managers (AWS Secrets Manager, Azure Key Vault, GCP Secret Manager)
- **HashiCorp Vault**: Centralized secret management with dynamic credentials

### Helm Secrets

Helm values files must never contain plaintext secrets. Use:

- `helm-secrets` plugin with SOPS encryption
- External secret references in values
- Sealed secret templates

### Terraform Sensitive Variables

All Terraform variables containing secrets must be marked as `sensitive = true`. Remote state backends must use encryption at rest. Use Vault providers or cloud-native secret managers for dynamic credentials.

### No Hardcoded Credentials

**Prohibition**: No credential, API key, password, or token may appear in any file within the `deployment/` directory in plaintext. All credentials must be injected at deployment time via secret management systems.

## Secure Development Rules

### Non-Root Containers

All Dockerfiles must include `USER nonroot:nonroot` or equivalent. The KCM process must not run as root inside the container.

### Read-Only Filesystems

Containers must use read-only root filesystems where possible. Writable paths must be limited to explicitly defined volume mounts (e.g., `/data`, `/tmp`).

### Resource Limits

All Kubernetes deployments must define:
- `resources.requests.cpu` and `resources.requests.memory`
- `resources.limits.cpu` and `resources.limits.memory`
- Horizontal Pod Autoscaler with `maxReplicas` and `minReplicas`

### Network Policies

Every namespace must have a default-deny ingress policy. Egress must be restricted to known destinations. Inter-service communication must be explicitly allowed via label selectors.

### Image Scanning

All container images must be scanned for known vulnerabilities before deployment. Images with critical CVEs must be blocked from promotion to production. Scan results must be retained for audit.

### No Latest Tags

Production images must use immutable tags (SHA256 digest or semantic version). The `latest` tag must never be used in production Helm values or Terraform configurations.

## Audit Logging

Deployment operations must be logged for security review:

| Event | Log Source | Retention |
|-------|-----------|-----------|
| Image push to registry | Registry audit logs | 1 year |
| Kubernetes API operations | Kubernetes audit logs | 1 year |
| Helm install/upgrade/rollback | Helm release history | 1 year |
| Terraform apply/destroy | Terraform Cloud / CI logs | 1 year |
| Prometheus config changes | Git commit history | Indefinite |
| Grafana dashboard changes | Git commit history | Indefinite |
| Secret access | Vault audit logs | 1 year |

## Validation Checklist

- [ ] All Dockerfiles use non-root user
- [ ] All Dockerfiles use multi-stage builds
- [ ] All container images use immutable tags (no `latest`)
- [ ] All container images are scanned for CVEs
- [ ] All Kubernetes deployments define resource limits and requests
- [ ] All Kubernetes namespaces have default-deny network policies
- [ ] All RBAC roles are namespace-scoped with explicit verbs
- [ ] No service account tokens are automounted unnecessarily
- [ ] All Helm values files contain no plaintext secrets
- [ ] All Terraform variables marked as sensitive use `sensitive = true`
- [ ] All Terraform state backends use encryption at rest
- [ ] All Prometheus endpoints require authentication
- [ ] All Grafana instances have default credentials disabled
- [ ] All secrets are managed via a secrets management system
- [ ] No hardcoded credentials exist in any deployment file
- [ ] All deployment files pass linting and validation
- [ ] All network policies enforce default-deny ingress
- [ ] All containers use read-only root filesystems
- [ ] All seccomp profiles are applied
- [ ] All audit logging is enabled and retained per policy
- [ ] All deployment changes are reviewed by at least one security engineer
- [ ] All production deployments use GitOps with signed commits

## References

- [Docker Security Best Practices](https://docs.docker.com/engine/security/)
- [Kubernetes Security Context](https://kubernetes.io/docs/tasks/configure-pod-container/security-context/)
- [Helm Security](https://helm.sh/docs/security/)
- [Terraform Security](https://developer.hashicorp.com/terraform/docs/security)
- [Prometheus Security](https://prometheus.io/docs/security/)
- [NIST Container Security](https://csrc.nist.gov/publications/detail/sp/800-190/final)
- [SSOT: docs/PRD3.md §33](../PRD3.md) — Deployment architecture
- [SSOT: docs/specs/KCM_DEPLOYMENT_SPEC.md](specs/KCM_DEPLOYMENT_SPEC.md)
