# .cargo/ Security Policy

> For project-wide security policies, refer to the [SECURITY.md](../SECURITY.md) located in the repository root.

## Overview

The `.cargo/` directory contains build configuration that directly affects how the KCM codebase is compiled. Compromised or misconfigured Cargo settings can introduce vulnerabilities, supply chain attacks, or build reproducibility issues.

## Security Scope

| Asset | Risk Level | Justification |
|-------|------------|---------------|
| `config.toml` | High | Controls compilation flags, linker behavior, and registry access |
| Registry settings | Medium | Affects dependency source verification |
| Network settings | Low | Controls git fetch behavior and retry policies |

## Threat Model

| Threat | Likelihood | Impact | Mitigation |
|--------|-----------|--------|------------|
| Build config manipulation | Low | Critical | Code review for all config changes |
| Supply chain attack via cargo | Medium | High | Use sparse protocol, audit dependencies |
| Malicious linker flags | Low | High | Review all rustflags before merge |
| Dependency substitution | Low | Critical | Lock file verification, no custom registries |

## Security Risks

| Risk | Description | Mitigation |
|------|-------------|------------|
| Disabled compiler protections | Unintentionally removing security-related compiler flags | Review all rustflags changes |
| Custom registry injection | Adding unverified registries | Only crates.io allowed without security review |
| Network bypass | Configuring insecure network behavior | `git-fetch-with-cli = true` ensures proper TLS |
| Reproducibility loss | Non-deterministic build settings | Pin target-cpu, avoid host-dependent defaults |

## Access Control

| Role | Read | Write | Approve |
|------|------|-------|---------|
| All Contributors | Yes | No | No |
| Core Maintainer | Yes | Yes | Config changes |
| Security Engineer | Yes | Yes | Security-relevant changes |

## RBAC Integration

Build configuration changes follow KCM RBAC:

| Permission Level | Access |
|-----------------|--------|
| Public (0) | Read-only |
| Read (1) | Read-only |
| Write (2) | Propose changes via PR |
| Admin (3) | Approve config changes |
| SuperAdmin (4) | Direct config modification |

## Sensitive Assets

| Asset | Sensitivity | Protection |
|-------|-------------|------------|
| `config.toml` | High | All changes require code review |

## Secret Management

**No secrets are permitted in `.cargo/config.toml` or any other Cargo configuration file.**

| Rule | Enforcement |
|------|-------------|
| No registry tokens | Code review |
| No API keys in build scripts | Code review |
| No credential-bearing URLs | Code review |

## Secure Development Rules

| Rule | Description |
|------|-------------|
| Pin toolchain | Use `rust-toolchain.toml` to pin Rust version |
| Audit dependencies | Run `cargo audit` regularly |
| No custom registries | Only crates.io without security review |
| Review all rustflags | Security-relevant flags must be reviewed |
| No network overrides | Do not disable TLS or certificate verification |
| Validate build reproducibility | Ensure identical builds across environments |

## Audit Logging

- All changes to `.cargo/config.toml` are tracked via git history
- PR reviews require maintainer approval
- CI validates configuration on every push

## Validation Checklist

Before merging any `.cargo/` configuration change:

- [ ] No hardcoded secrets, tokens, or API keys
- [ ] No custom registries added without security review
- [ ] All rustflags reviewed for security implications
- [ ] No TLS/certificate verification disabled
- [ ] Build reproducibility verified
- [ ] `cargo audit` passes
- [ ] `cargo clippy --workspace -- -D warnings` passes

## References

- [SECURITY.md](../SECURITY.md) — Repository root security policy
- [Cargo Configuration Security](https://doc.rust-lang.org/cargo/reference/config.html)
- [Cargo Audit](https://docs.rs/cargo-audit/)
- `AGENTS.md` — Engineering constitution (Security section)
- `kcm-security` crate — RBAC implementation
