# {{CRATE_NAME}} Security Policy

Security considerations specific to the `{{CRATE_NAME}}` crate.

> For project-wide security policies, refer to the [SECURITY.md](../../SECURITY.md) located in the repository root.

## Overview

{{SECURITY_OVERVIEW}}

## Security Scope

| Asset | Risk Level | Description |
|-------|-----------|-------------|
| {{ASSET_1}} | {{RISK_1}} | {{ASSET_1_DESC}} |

## Threat Model

| Threat | Vector | Mitigation |
|--------|--------|------------|
| {{THREAT_1}} | {{VECTOR_1}} | {{MITIGATION_1}} |

## Security Risks

{{SECURITY_RISKS}}

## Access Control

{{ACCESS_CONTROL}}

## RBAC Integration

{{RBAC_INTEGRATION}}

## Sensitive Assets

{{SENSITIVE_ASSETS}}

## Secret Management

{{SECRET_MANAGEMENT}}

## Secure Development Rules

1. {{RULE_1}}
2. {{RULE_2}}
3. All public APIs return `Result<T, KcmError>`
4. No `unwrap()` in production code paths
5. No `panic!()` in production code paths

## Audit Logging

{{AUDIT_LOGGING}}

## Validation Checklist

- [ ] {{CHECK_1}}
- [ ] {{CHECK_2}}
- [ ] All public APIs return `Result<T, KcmError>`
- [ ] No `unwrap()` in production code
- [ ] No `panic!()` in production code

## References

- [SECURITY.md](../../SECURITY.md) — Project-wide security policy
- [AGENTS.md](../../AGENTS.md) — Engineering constitution
- [SSOT.md](../../SSOT.md) — Single Source of Truth
