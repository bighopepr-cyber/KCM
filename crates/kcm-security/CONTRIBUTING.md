# Contributing to kcm-security

> For project-wide contribution guidelines, see the root `CONTRIBUTING.md`.

## Overview

`kcm-security` is the security crate for KCM. It enforces access control, manages encryption, maintains audit logs, and handles secret storage. This crate has the **highest quality bar** in the entire project. A bug here can compromise the entire system.

## Before Contributing

1. Read `SECURITY.md` in this directory.
2. Read `AGENTS.md` at the project root.
3. Read the relevant SSOT documents (`docs/PRD3.md §30` for security).
4. Understand the dependency chain — `kcm-security` depends only on `kcm-core`.
5. Ensure you have a clear understanding of the change you are proposing.

## Coding Standards

### Cryptographic Rules

- **Use audited libraries only.** Use `aes-gcm` for AES-256-GCM, `blake3` for hashing/KDF, `getrandom` for CSPRNG.
- **No custom cryptography.** Do not implement your own encryption, hashing, or key derivation.
- **Constant-time comparisons.** All comparisons involving secret material must use constant-time operations.
- **No hardcoded keys.** Never embed keys, passwords, or secrets in source code.
- **AEAD only.** Use AES-256-GCM (or equivalent AEAD). Never use ECB, CBC without MAC, or other non-authenticated modes.
- **Random nonces.** Use `getrandom` for nonce generation. Never reuse nonces with the same key.

### General Rust Rules

- All public APIs return `Result<T, KcmError>`.
- No `unwrap()` or `panic!()` in production code paths.
- No `todo!()`, `unimplemented!()`, `FIXME`, or `TODO` markers.
- Follow existing code style — check neighboring files for conventions.
- Use `parking_lot` for synchronization primitives (project standard).

## Module Architecture Rules

| Rule | Description |
|------|-------------|
| Single dependency | `kcm-security` depends on `kcm-core` only. Do not add dependencies on other KCM crates. |
| No upward dependencies | No other crate should be imported by `kcm-security`. |
| Self-contained | All security logic lives in this crate. Do not delegate to `kcm-runtime` or `kcm-interface`. |
| One responsibility per module | `rbac.rs` = access control, `encryption.rs` = encryption, `audit.rs` = audit logging, `secrets.rs` = secret management. |

## Documentation Rules

- All cryptographic decisions must be documented with rationale.
- Security properties must be stated explicitly (e.g., "this provides confidentiality and authenticity").
- Threat mitigations must be documented.
- API documentation must include security considerations.
- Changes to security behavior require SSOT update in `docs/PRD3.md §30`.

## Testing Requirements

Every change must include or update tests covering:

| Test Type | Description | Example |
|-----------|-------------|---------|
| **Encryption Roundtrip** | Encrypt then decrypt, verify data integrity | `test_encrypt_decrypt_roundtrip` |
| **RBAC Permission Matrix** | All 5 permission levels × all operation types | `test_rbac_permission_matrix` |
| **Audit Log Integrity** | Verify hash chain is maintained across entries | `test_audit_log_chain_integrity` |
| **Secret Rotation** | Rotate keys, verify old data still accessible | `test_secret_key_rotation` |
| **Edge Cases** | Empty inputs, maximum sizes, boundary conditions | `test_encryption_empty_input` |
| **Error Paths** | Unauthorized access, invalid keys, corrupted data | `test_rbac_unauthorized_access` |

Run tests with:
```bash
cargo test -p kcm-security
```

## Performance Rules

- Encryption operations must not allocate excessively. Reuse buffers where possible.
- Audit log append must be O(1) amortized.
- RBAC checks must be O(1) — no unbounded iteration.
- Key derivation is expected to be slow (this is a security feature, not a bug).

## Review Checklist

Before submitting a PR, verify:

- [ ] All crypto uses audited libraries only
- [ ] No hardcoded keys or secrets
- [ ] AEAD encryption (AES-256-GCM) used correctly
- [ ] 96-bit random nonces generated via CSPRNG
- [ ] Hash chain maintained in audit log
- [ ] FIFO eviction at 100K entries implemented
- [ ] RBAC check on every public API
- [ ] Role hierarchy enforced
- [ ] Constant-time comparisons for secrets
- [ ] No `unwrap()` in production code
- [ ] All public APIs return `Result<T, KcmError>`
- [ ] No secrets in logs or error messages
- [ ] Tests cover happy path, error paths, and edge cases
- [ ] No clippy warnings
- [ ] `cargo fmt --check` passes
- [ ] SSOT updated if behavior changed

**Security engineer review is required for all changes to this crate.**

## Pull Request Requirements

1. **Title:** Clear, concise description of the change.
2. **Description:** Explain what changed, why, and security implications.
3. **Tests:** Include or update tests. No merge without passing tests.
4. **SSOT:** Update `docs/PRD3.md §30` if security behavior changed.
5. **Review:** At least one security engineer approval required.
6. **CI:** All CI checks must pass (format, clippy, build, tests).
7. **No force-push** to shared branches.

## References

- `AGENTS.md` — Engineering constitution
- `SECURITY.md` — Security policy for this crate
- `docs/PRD3.md §30` — Security specification (SSOT)
- `kcm-security/src/rbac.rs` — RBAC implementation
- `kcm-security/src/encryption.rs` — Encryption implementation
- `kcm-security/src/audit.rs` — Audit log implementation
- `kcm-security/src/secrets.rs` — Secret management implementation
