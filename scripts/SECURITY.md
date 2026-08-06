# scripts/ Security Policy

> For project-wide security policies, refer to the [SECURITY.md](../SECURITY.md) located in the repository root.

## Overview

This document defines the security policy for all scripts and CLI tools within the `scripts/` directory. It covers threat modeling, access control, secret management, and secure development practices specific to build automation, validation utilities, and CLI tooling.

## Security Scope

| Script / Tool | Risk Level | Description |
|---|---|---|
| `validate-ssot.sh` | Medium | SSOT compliance checks — reads files, writes reports |
| `validate-sdk-api.sh` | Medium | SDK API validation — parses source files, checks signatures |
| `bench-regression.py` | Low | Benchmark regression detection — reads benchmark output, produces reports |
| `kcm-cli/` tools | High | CLI tools for database management — execute database operations, handle file I/O, manage data |

## Threat Model

| Threat | Vector | Impact |
|---|---|---|
| Script injection | User-controlled input passed to shell commands | Arbitrary command execution |
| Path traversal in CLI tools | Unsanitized file paths in CLI arguments | Read/write arbitrary files |
| Privilege escalation via CLI | CLI tools run with elevated permissions | Unauthorized database access or modification |
| Credential exposure in scripts | Hardcoded secrets or leaked environment variables | Credential compromise |

## Security Risks

- **Shell injection**: Scripts that interpolate user input into shell commands without proper quoting or escaping.
- **Symlink attacks**: Scripts that follow symlinks without validation may read or write unintended files.
- **Temporary file races**: Scripts using predictable temporary file names are vulnerable to TOCTOU races.
- **Unvalidated paths**: CLI tools accepting file paths without canonicalization may be exploited for directory traversal.

## Access Control

All scripts and CLI tools must follow the principle of least privilege:

- Scripts execute with the permissions of the invoking user.
- CLI tools must not require root or elevated privileges for normal operations.
- File operations must be restricted to the working directory and explicitly permitted paths.

## RBAC Integration

- CLI tools that perform database operations must integrate with the KCM RBAC system (`kcm-security` crate).
- Permission checks occur before any write or destructive operation.
- Audit events are logged for all privileged operations.

## Sensitive Assets

| Asset | Classification | Handling |
|---|---|---|
| CLI tool binaries | Internal | Distributed only through official build channels |
| Script outputs | Internal | May contain file system paths and system information; do not commit to public repositories |
| Benchmark data | Low | Performance data; no confidentiality requirement |

## Secret Management

- **No secrets in scripts**: Scripts must not contain hardcoded credentials, API keys, tokens, or encryption keys.
- Secrets required by scripts must be supplied via environment variables or a secrets manager.
- Scripts must not log or print secret values.
- `.env` files must not be committed to the repository.

## Secure Development Rules

| Rule | Description |
|---|---|
| Input validation in scripts | All script arguments must be validated before use. Reject unexpected input. |
| No eval | Shell scripts must not use `eval` on user-controlled input. Python scripts must not use `eval()` or `exec()` on untrusted data. |
| Safe path handling | Use canonicalized, absolute paths. Validate paths against an allowlist before file operations. |
| No hardcoded paths | Scripts must not hardcode absolute filesystem paths. Use relative paths or configurable base directories. |
| Permission checks | CLI tools must verify file/directory permissions before read/write operations. |
| Quoting | All shell variable expansions must be double-quoted to prevent word splitting and glob expansion. |
| Temporary files | Use `mktemp` for temporary files. Clean up on exit. |

## Audit Logging

- CLI tools performing database operations must emit audit events through the `kcm-security` audit log.
- Audit events include: operation type, timestamp, user identity (if available), target resource, and outcome.
- Audit events are hash-chained and tamper-evident per the security specification.

## Validation Checklist

- [ ] No hardcoded secrets or credentials in any script or CLI source file.
- [ ] All user input is validated before use.
- [ ] Shell scripts pass `shellcheck` with no warnings.
- [ ] Python scripts pass `pylint` with no errors.
- [ ] No use of `eval` or `exec` on untrusted input.
- [ ] File paths are canonicalized and validated before operations.
- [ ] CLI tools integrate with KCM RBAC for privileged operations.
- [ ] Audit events are logged for database operations.
- [ ] Temporary files are created securely and cleaned up.
- [ ] Scripts do not require elevated privileges for normal operation.

## References

- [Repository Security Policy](../SECURITY.md)
- [KCM Security Crate](../crates/kcm-security/)
- [AGENTS.md — Non-Negotiable Rules](../AGENTS.md)
- [SSOT Technical Specification](../docs/scripts/spesifikasi.md)
