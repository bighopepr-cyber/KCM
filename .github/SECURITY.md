# .github/ Security Policy

> For project-wide security policies, refer to the [SECURITY.md](../SECURITY.md) located in the repository root.

## Overview

Security policies specific to the `.github/` configuration, covering CI/CD pipeline security, workflow permissions, and secret management.

## Security Scope

- CI/CD pipeline security
- Workflow permissions
- Secret management in GitHub Actions

## Threat Model

| Threat | Description | Mitigation |
|--------|-------------|------------|
| CI injection | Malicious code execution via CI | Pinned action versions, no PR checkout |
| Workflow compromise | Unauthorized workflow modification | CODEOWNERS enforcement, branch protection |
| Secret exposure | Secrets leaked in logs | No echo of secrets, secret isolation |
| Unauthorized merges | Bypassing code review | Branch protection rules, required reviews |

## Security Risks

| Risk | Severity | Impact |
|------|----------|--------|
| Unpinned GitHub Actions | High | Supply chain attack |
| Overprivileged tokens | Medium | Unauthorized repository access |
| Missing branch protection | High | Direct push to main |
| Exposed secrets | Critical | Credential compromise |

## Access Control

| Mechanism | Implementation |
|-----------|---------------|
| CODEOWNERS enforcement | Required reviews from code owners |
| Branch protection | Require PR reviews, status checks |
| Workflow permissions | Least-privilege per workflow |
| Secret isolation | Separate secrets per environment |

## RBAC Integration

| Role | Access Level | Scope |
|------|-------------|-------|
| Admin | Full access | All workflows and secrets |
| Maintainer | Write access | Workflow modification |
| Contributor | Read access | Workflow trigger only |

## Sensitive Assets

| Asset | Location | Protection |
|-------|----------|------------|
| CI secrets | GitHub repository secrets | Encrypted at rest |
| Workflow tokens | GitHub Actions runtime | Scoped to workflow |
| CODEOWNERS | .github/CODEOWNERS | Branch protection |

## Secret Management

| Practice | Description |
|----------|-------------|
| GitHub Actions secrets | Use repository secrets for sensitive values |
| No hardcoded tokens | Never commit tokens or keys |
| Secret rotation | Rotate secrets periodically |
| Audit access | Monitor secret usage in workflow logs |

## Secure Development Rules

| Rule | Description |
|------|-------------|
| Least-privilege permissions | Each workflow uses minimum required permissions |
| Pinned action versions | Pin actions to specific SHA or version |
| No PR checkout in CI | Avoid checking out untrusted PR code |
| Secret isolation | Secrets are not exposed to forked PRs |

## Audit Logging

| Log | Purpose |
|-----|---------|
| Workflow runs | Track all CI/CD executions |
| Secret access | Monitor secret usage |
| Failed builds | Identify potential issues |

## Validation Checklist

- [ ] All workflows use least-privilege permissions
- [ ] All GitHub Actions are pinned to specific versions
- [ ] No secrets are hardcoded in workflow files
- [ ] Branch protection rules are enabled
- [ ] CODEOWNERS file is properly configured
- [ ] Forked PRs cannot access secrets
- [ ] Workflow runs are logged and monitored

## References

- [GitHub Actions Security](https://docs.github.com/en/actions/security-for-github-actions)
- [Repository Security](../SECURITY.md)
- [CODEOWNERS](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners)
