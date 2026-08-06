# examples/ Security Policy

> For project-wide security policies, refer to the [SECURITY.md](../SECURITY.md) located in the repository root.

## Overview

This document defines the security policy specific to the `examples/` directory of the KCM project. All examples are educational in nature and designed to demonstrate KCM functionality using in-memory databases. They are **not** intended for production use.

## Security Scope

- All examples operate on **in-memory databases** only.
- No examples persist data to disk or connect to external services.
- No production data is used in any example.
- Examples are compiled and run locally during development and CI.

## Threat Model

The primary threat is **example code being copied into production without security review**.

| Threat | Likelihood | Impact | Mitigation |
|--------|-----------|--------|------------|
| Example copied to production | Medium | Medium | Clear documentation warnings |
| Hardcoded paths in examples | Low | Low | Linting rules enforce no hardcoded paths |
| Secrets in example code | Low | High | CI scans for secrets; policy禁止 hardcoded secrets |
| Privilege escalation via example | Low | Low | Examples use no authentication/authorization |

## Security Risks

**Overall risk: Low** — examples are educational, non-production, and run in isolated environments.

| Risk Category | Level | Notes |
|---------------|-------|-------|
| Data exposure | None | No real data used |
| Code injection | None | No user input processing |
| Privilege escalation | None | No access control in examples |
| Dependency vulnerabilities | Low | Examples use same deps as core |
| Supply chain | Low | Dependencies inherited from Cargo.toml |

## Access Control

- Examples have no access control mechanisms.
- Examples run as the invoking user with no elevated privileges.
- No authentication or authorization is demonstrated in examples.
- RBAC integration examples (when added) will use mock/in-memory data only.

## RBAC Integration

When RBAC examples are added:

- Must use in-memory permission stores.
- Must not connect to external LDAP/AD/OAuth providers.
- Must demonstrate concepts only, not serve as production RBAC implementations.
- Permission data must be synthetic and non-sensitive.

## Sensitive Assets

The following assets must **never** appear in example code:

- API keys or tokens
- Database credentials
- Private keys or certificates
- Personally identifiable information (PII)
- Internal hostnames or IPs
- Production configuration values

## Secret Management

- **No secrets are permitted in example code.**
- Examples must not read secrets from environment variables or files.
- If a secret is needed for demonstration, use a clearly labeled placeholder: `YOUR_API_KEY_HERE`.
- CI pipelines scan for secret patterns in all example files.

## Secure Development Rules

| Rule | Requirement |
|------|-------------|
| No hardcoded paths | Examples must use relative or temp directories |
| Temp directories | All file operations use `tempfile` or platform temp dirs |
| Error handling | All examples demonstrate proper error handling with `Result<T, KcmError>` |
| No unwrap | Examples follow the same no-`unwrap()` policy as production code |
| Minimal dependencies | Examples should not introduce new external dependencies |
| No network access | Examples must not make external network calls |

## Audit Logging

- Example code does not implement audit logging.
- Audit logging examples (when added) will use in-memory log buffers.
- No audit data is written to disk.

## Validation Checklist

Before merging example code:

- [ ] No hardcoded secrets, keys, or credentials
- [ ] No hardcoded absolute paths
- [ ] No network calls to external services
- [ ] Uses temp directories for any file operations
- [ ] Demonstrates proper error handling
- [ ] No `unwrap()` in non-test code
- [ ] No PII or real data
- [ ] README includes security warning
- [ ] Compiles and runs without elevated privileges
- [ ] Passes CI secret scanning

## References

- [Repository SECURITY.md](../SECURITY.md)
- [KCM Error Model](../AGENTS.md#error-model)
- [Examples README](README.md)
