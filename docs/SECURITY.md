# docs/ Security Policy

> For project-wide security policies, refer to the [SECURITY.md](../SECURITY.md) located in the repository root.

## Overview

Documentation must not expose secrets, credentials, or sensitive architecture details that could compromise the security of the KCM system. All documentation follows the principle of least disclosure — only include information necessary for the reader's role.

## Security Scope

| Document Category | Risk Level | Justification |
|-------------------|------------|---------------|
| `specs/` | Medium | Contains system architecture details, API contracts, format specifications |
| `adr/` | Low | Decision records with rationale, no operational secrets |
| `runbook/` | High | Operational procedures may reference credentials, access paths, and incident response |
| `sdk/` | Medium | API usage guides may expose internal implementation details |
| `handbook/` | Low | Contributor guides with no sensitive content |

## Threat Model

| Threat | Likelihood | Impact | Mitigation |
|--------|-----------|--------|------------|
| Information disclosure via specs | Medium | High | Review all specs before publication; redact internal IPs, ports, credentials |
| Outdated security guidance | Medium | Medium | Quarterly review of security-related documentation |
| Credential leakage in runbooks | Low | Critical | Use placeholder references, never embed real credentials |
| Architecture exposure to attackers | Low | High | Limit internal architecture details in public-facing docs |

## Security Risks

| Risk | Description | Mitigation |
|------|-------------|------------|
| Exposed credentials | Runbooks referencing actual passwords or tokens | Use vault references, never hardcode |
| Outdated encryption guidance | Specs describing deprecated algorithms | Review against current standards quarterly |
| Internal network details | Runbooks exposing internal hostnames or IPs | Use placeholder values |
| Unintended API exposure | SDK docs revealing undocumented endpoints | Cross-reference with API spec |

## Access Control

| Role | Read | Write | Approve |
|------|------|-------|---------|
| All Contributors | All public docs | Own contributions | No |
| Security Engineer | All docs | Security specs | Security changes |
| Documentation Guardian | All docs | All docs | All documentation |
| Specification Lock | All specs | Frozen contracts | Contract changes |

## RBAC Integration

Documentation access follows the KCM RBAC model defined in `kcm-security`:

| Permission Level | Documentation Access |
|-----------------|---------------------|
| Public (0) | Public-facing SDK docs only |
| Read (1) | All documentation |
| Write (2) | All documentation, own contributions |
| Admin (3) | All documentation, approve changes |
| SuperAdmin (4) | Full access, security policy changes |

## Sensitive Assets

| Asset | Location | Sensitivity |
|-------|----------|-------------|
| Security trust specifications | `specs/KCM_SECURITY_TRUST_SPEC.md` | High — contains security architecture details |
| Disaster recovery runbook | `runbook/DISASTER_RECOVERY.md` | Critical — operational recovery procedures |
| Operational runbook | `runbook/OPERATIONAL_RUNBOOK.md` | High — system access procedures |
| Deployment specifications | `specs/KCM_DEPLOYMENT_SPEC.md` | Medium — infrastructure details |

## Secret Management

**No secrets are permitted in any documentation file.**

| Rule | Enforcement |
|------|-------------|
| No hardcoded passwords | Grep CI check |
| No API keys | Grep CI check |
| No private keys | Grep CI check |
| No connection strings with credentials | Grep CI check |
| Use vault references only | Code review |

Placeholders must follow this format:

```
# Use: ${VAULT:secret/path/credential_name}
# Never: actual_password_here
```

## Secure Development Rules

| Rule | Description |
|------|-------------|
| Review security specs | All security-related documentation changes require security engineer review |
| Keep docs current | Outdated security guidance is a vulnerability; update with implementation changes |
| No credentials in runbooks | Reference vault paths or environment variables only |
| Validate before merge | Run `scripts/validate-ssot.sh` and check for secret patterns |
| Redact internal details | Internal IPs, hostnames, and network topology must use placeholders |

## Audit Logging

All changes to security-sensitive documentation are tracked:

- Git history provides full change audit trail
- PR reviews require at least one security engineer approval for `specs/KCM_SECURITY_TRUST_SPEC.md`
- Runbook changes require operational team review

## Validation Checklist

Before merging any documentation change:

- [ ] No hardcoded secrets, credentials, or API keys
- [ ] No internal IP addresses or hostnames (use placeholders)
- [ ] Security specs reviewed by security engineer
- [ ] Runbook credentials reference vault paths
- [ ] All links are valid and point to correct locations
- [ ] SSOT hierarchy is maintained
- [ ] No outdated security guidance
- [ ] `scripts/validate-ssot.sh` passes

## References

- [SECURITY.md](../SECURITY.md) — Repository root security policy
- `specs/KCM_SECURITY_TRUST_SPEC.md` — Security architecture specification
- `runbook/DISASTER_RECOVERY.md` — Disaster recovery procedures
- `runbook/OPERATIONAL_RUNBOOK.md` — Operational procedures
- `AGENTS.md` — Engineering constitution (Security section)
- `kcm-security` crate — RBAC and encryption implementation
