# Contributing to deployment/

> For core engine contribution rules, refer to the repository [CONTRIBUTING.md](../CONTRIBUTING.md).

## Overview

This document defines contribution standards for the KCM deployment infrastructure. All deployment configurations — Docker, Kubernetes, Helm, Terraform, Prometheus, and Grafana — must follow these guidelines to maintain consistency, security, and reliability.

## Before Contributing

1. **Read the repository root [CONTRIBUTING.md](../CONTRIBUTING.md)** for general contribution rules
2. **Read [deployment/SECURITY.md](SECURITY.md)** for security requirements applicable to deployment configurations
3. **Understand the deployment architecture** by reviewing `docs/PRD3.md §33` and `docs/specs/KCM_DEPLOYMENT_SPEC.md`
4. **Check open issues** for existing deployment work or known issues
5. **Discuss significant changes** in a GitHub issue before implementing

## Coding Standards

### YAML Formatting

- Use 2-space indentation for all YAML files
- Align keys logically for readability
- Use meaningful key names matching Helm template conventions
- Keep line length under 120 characters
- Include trailing newline at end of file

### Terraform Formatting

- Run `terraform fmt` before committing
- Use consistent variable naming: `snake_case` for all variables
- Group related resources in separate files by concern
- Include descriptions for all variables and outputs
- Pin provider versions in `required_providers` blocks

### Helm Template Conventions

- Use `{{-` and `-}}` trims to avoid extra whitespace
- Prefix template helper names with the chart name
- Use `values.yaml` as the single source of truth for defaults
- Document all values with comments in `values.yaml`
- Use `_helpers.tpl` for reusable template functions

## Module Architecture Rules

Each deployment target is independent. Changes to one target must not affect others.

| Target | Independence Rule |
|--------|------------------|
| Docker | Dockerfile changes must not depend on Kubernetes or Helm |
| Kubernetes | k8s/ manifests must be deployable without Helm |
| Helm | Helm chart must be self-contained; no external chart dependencies without explicit justification |
| Terraform | Each cloud module must be independent and state-isolated |
| Prometheus | Alert rules must not depend on specific Kubernetes resources |
| Grafana | Dashboard provisioning must not require manual intervention |

### File Organization

```
deployment/
├── Dockerfile              # Multi-stage Docker build
├── docker-compose.yml      # Single-node development
├── docker-compose.monitoring.yml  # Monitoring stack
├── grafana/                # Dashboard provisioning
├── helm/                   # Kubernetes Helm chart
├── k8s/                    # Raw Kubernetes manifests
├── prometheus/             # Prometheus configuration
├── terraform/              # Infrastructure-as-Code modules
├── CONTRIBUTING.md         # This file
├── SECURITY.md             # Security policy
└── CODE_OF_CONDUCT.md      # Community guidelines
```

## Documentation Rules

- Every deployment file must have a header comment describing its purpose
- Helm values must be documented with inline comments
- Terraform variables must have descriptions and example values
- All non-trivial configurations must reference the relevant SSOT document
- README files in each subdirectory should explain deployment procedures
- Changes to deployment behavior must update the relevant spec documents

## Testing Requirements

### Helm Lint

```bash
helm lint helm/ --strict
```

All Helm templates must pass linting with no warnings or errors. Validate template rendering with:

```bash
helm template test helm/ --values helm/values.yaml
```

### Terraform Validate

```bash
terraform validate terraform/
terraform fmt -check terraform/
```

All Terraform configurations must pass validation and format checks. Run `terraform plan` against a test environment before merging.

### Docker Build Test

```bash
docker build -t kcm:test .
docker run --rm kcm:test --help
```

All Dockerfiles must build successfully and produce a functional image. Verify the image runs as non-root:

```bash
docker run --rm --entrypoint id kcm:test
```

The output must show a non-root user (UID > 0).

### Kubernetes Manifest Validation

```bash
kubectl apply --dry-run=client -f k8s/
```

All raw Kubernetes manifests must pass dry-run validation.

### Integration Testing

Deployment configurations must be validated in a staging environment before promotion to production. Integration tests must verify:

- Application starts and passes health checks
- Prometheus scrapes metrics successfully
- Grafana dashboards render correctly
- Network policies allow required traffic
- Secrets are injected correctly

## Performance Rules

- Docker images must minimize layer count and total size
- Helm templates must not create unnecessary resources
- Terraform plans must not propose changes to unchanged resources
- Prometheus scrape intervals must be appropriate for the metric type
- Grafana dashboard queries must not cause excessive API load

## Review Checklist

All deployment PRs must satisfy:

- [ ] Helm lint passes with `--strict`
- [ ] Terraform fmt check passes
- [ ] Docker build succeeds
- [ ] Docker image runs as non-root
- [ ] No hardcoded credentials in any file
- [ ] All Kubernetes manifests pass dry-run validation
- [ ] Network policies are updated for new services
- [ ] Resource limits are defined for all workloads
- [ ] Documentation is updated for behavioral changes
- [ ] Security review completed (per SECURITY.md checklist)
- [ ] SSOT documents are updated if behavior changed
- [ ] Changes are limited to a single deployment target per PR

## Pull Request Requirements

### Title Format

```
[deployment/<target>]: <brief description>
```

Examples:
- `[deployment/helm]: Add resource limits to worker pods`
- `[deployment/terraform]: Add Azure module for AKS provisioning`
- `[deployment/prometheus]: Add alert rule for WAL replication lag`

### PR Description

Every deployment PR must include:

1. **What**: Description of the deployment change
2. **Why**: Justification linked to an issue or SSOT requirement
3. **How**: Implementation approach and affected files
4. **Testing**: Validation performed (lint, validate, build, deploy)
5. **Rollback**: How to revert if the change causes issues
6. **Security impact**: Whether the change affects the security posture

### Merge Requirements

- All CI checks pass
- At least one reviewer approves
- Security checklist completed
- No conflicts with the target branch
- All feedback addressed

## References

- [Helm Best Practices](https://helm.sh/docs/chart_best_practices/)
- [Terraform Style Guide](https://developer.hashicorp.com/terraform/language/style)
- [Dockerfile Best Practices](https://docs.docker.com/develop/develop-images/dockerfile_best-practices/)
- [Kubernetes Deployment Best Practices](https://kubernetes.io/docs/concepts/configuration/overview/)
- [SSOT: docs/PRD3.md §33](../PRD3.md) — Deployment architecture
- [SSOT: docs/specs/KCM_DEPLOYMENT_SPEC.md](specs/KCM_DEPLOYMENT_SPEC.md)
