# kcm-security Security Policy

> For the project-wide security policy, see the root `SECURITY.md`.

## Overview

`kcm-security` **is** the security crate for KCM. It implements all cryptographic operations, access control, audit logging, and secret management. This crate is held to the highest security standards in the entire project. Any vulnerability here has a direct blast radius across every other crate.

## Security Scope

| Component | Severity | Description |
|-----------|----------|-------------|
| RBAC | **Critical** | Role-Based Access Control with 5 permission levels. Every operation flows through this gate. |
| Encryption | **Critical** | AES-256-GCM authenticated encryption. Compromise means all stored data is exposed. |
| Audit Log | **High** | Hash-chained, tamper-evident audit trail. Integrity loss destroys forensic capability. |
| Secrets | **Critical** | Key storage, derivation, and rotation. Compromise means all encrypted data is exposed. |

## Threat Model

| Threat | Impact | Mitigation |
|--------|--------|------------|
| **Privilege Escalation** | Attacker gains permissions beyond their role. | RBAC enforced on every operation; no bypass paths. Role hierarchy is enforced — lower roles cannot grant higher roles. |
| **Key Compromise** | All encrypted data exposed. | AES-256-GCM with 96-bit random nonces. Keys derived via BLAKE3 KDF, never stored in plaintext. Key rotation supported. |
| **Audit Log Tampering** | Attacker removes evidence of malicious actions. | Hash-chained entries — each entry includes hash of previous entry. Tampering breaks the chain and is detectable. FIFO eviction at 100K entries. |
| **Secret Exposure** | Secrets leaked to unauthorized parties. | Secrets encrypted at rest. Access gated by RBAC. No plaintext secrets in logs or error messages. |
| **Timing Attacks** | Attacker infers sensitive data from response timing. | All comparisons use constant-time operations. No early returns on secret comparisons. |

## Security Risks

### Privilege Escalation

The RBAC system is the single gateway for all access control decisions. If an attacker can bypass RBAC, they gain unrestricted access to all KCM data and operations.

**Mitigation:**
- RBAC check is mandatory on every public API call.
- Role hierarchy is enforced at the type level — `SuperAdmin` cannot grant `Owner` permissions.
- No backdoor or admin bypass exists in the codebase.
- All role mutations are logged to the audit log.

### Key Compromise

AES-256-GCM provides confidentiality and authenticity only as long as keys remain secret. Key compromise exposes all data encrypted with that key.

**Mitigation:**
- Keys are derived from master secrets via BLAKE3 KDF, never stored in plaintext.
- 96-bit random nonces prevent nonce reuse attacks.
- Key rotation is supported — old keys can be retired without data loss.
- Memory containing keys is zeroed after use where possible.

### Audit Log Tampering

If an attacker can modify or delete audit log entries, they can cover their tracks.

**Mitigation:**
- Each audit entry includes a BLAKE3 hash of the previous entry, forming a hash chain.
- Tampering with any entry breaks the chain and is detectable.
- Audit log append is serialized via `Mutex`.
- FIFO eviction at 100K entries prevents unbounded memory growth.

### Secret Exposure

Secrets (API keys, tokens, master keys) are the most sensitive data in the system.

**Mitigation:**
- Secrets are encrypted at rest using AES-256-GCM.
- Access to secrets requires `Admin` or higher permission level.
- Secrets are never logged, included in error messages, or exposed through public APIs.
- Key rotation allows retiring compromised secrets.

### Timing Attacks

Response timing can leak information about secret values or comparison results.

**Mitigation:**
- All secret comparisons use constant-time operations.
- No early returns on comparisons involving sensitive data.
- Error responses are uniform regardless of input validity.

## Access Control

RBAC with 5 permission levels (defined in `rbac.rs`):

| Level | Name | Capabilities |
|-------|------|-------------|
| 0 | `Read` | Read-only access to data. Cannot modify, delete, or manage. |
| 1 | `Write` | Create and update data. Cannot delete, manage users, or administer. |
| 2 | `Admin` | Full data management including delete. Cannot manage users or system config. |
| 3 | `SuperAdmin` | User management, role assignment, system configuration. Cannot claim Owner. |
| 4 | `Owner` | Full system control. Can perform any operation including system destruction. |

**Hierarchy:** `Owner > SuperAdmin > Admin > Write > Read`

## RBAC Integration

This crate is the **sole authority** for all access control in KCM. All other crates reference this implementation:

- `kcm-interface` — REST and gRPC handlers enforce RBAC before processing requests.
- `kcm-distributed` — Shard operations check permissions before data access.
- `kcm-runtime` — Database operations enforce RBAC on mutations.
- `kcm-storage` — Storage layer trusts the runtime's RBAC checks.

No crate implements its own access control. All authorization flows through `kcm-security`.

## Sensitive Assets

| Asset | Classification | Protection |
|-------|---------------|------------|
| Encryption Keys | Top Secret | Derived via BLAKE3 KDF, encrypted at rest, rotation supported |
| Audit Logs | Confidential | Hash-chained, append-only, tamper-evident |
| RBAC Policies | Confidential | Stored with integrity checks, role hierarchy enforced |
| Secrets | Top Secret | AES-256-GCM encrypted, access-gated, never logged |

## Secret Management

- **Key Derivation:** All keys derived from master secrets using BLAKE3 KDF.
- **Key Rotation:** Supported — old keys can be retired, new keys derived without data loss.
- **Secure Storage:** Secrets encrypted at rest. No plaintext secrets in memory longer than necessary.
- **Access Control:** Secret access requires `Admin` or higher permission level.
- **Zeroization:** Memory containing secrets is cleared after use where the Rust memory model allows.

## Secure Development Rules

1. **Use audited crypto libraries only.** Never implement custom cryptography. Use `aes-gcm` for encryption, `blake3` for hashing.
2. **Never hardcode keys or secrets.** All keys must be derived or loaded from secure storage.
3. **Use AEAD encryption (AES-256-GCM).** Provides both confidentiality and authenticity. Never use ECB or other non-AEAD modes.
4. **Hash-chained audit log.** Each entry includes the hash of the previous entry, making tampering detectable.
5. **FIFO eviction at 100K entries.** Prevents unbounded memory growth while maintaining a sufficient audit trail.
6. **RBAC check on every operation.** No operation proceeds without authorization verification.
7. **Key derivation via BLAKE3.** Never use raw hashes for key derivation. Use BLAKE3's keyed mode or KDF.
8. **CSPRNG for nonce generation.** Use `getrandom` for all cryptographic randomness. Never use `rand::thread_rng()`.
9. **No `unwrap()` in production code.** All operations return `Result<T, KcmError>`.
10. **Result return on all public APIs.** Every public function returns `Result<T, KcmError>`. No panics, no exceptions.

## Audit Logging

The audit log is hash-chained and tamper-evident:

```
Entry[i].hash = BLAKE3(Entry[i-1].hash || Entry[i].data)
```

- Each entry contains: timestamp, actor, action, resource, result, and the hash of the previous entry.
- Tampering with any entry breaks the hash chain.
- FIFO eviction at 100K entries — oldest entries removed first.
- Audit log append is serialized via `Mutex<VecDeque<AuditEvent>>`.

## Validation Checklist

- [ ] All cryptographic operations use audited libraries (`aes-gcm`, `blake3`)
- [ ] No hardcoded keys or secrets anywhere in the codebase
- [ ] AES-256-GCM with 96-bit random nonces for all encryption
- [ ] BLAKE3 KDF for all key derivation
- [ ] Hash chain integrity on audit log
- [ ] FIFO eviction at 100K entries implemented
- [ ] RBAC check present on every public API
- [ ] Role hierarchy enforced — no privilege escalation paths
- [ ] Constant-time comparisons for all secret values
- [ ] No `unwrap()` or `panic!()` in production code
- [ ] All public APIs return `Result<T, KcmError>`
- [ ] No secrets in logs or error messages
- [ ] Memory zeroization for secrets where possible
- [ ] Nonce generation uses CSPRNG (`getrandom`)
- [ ] Encryption roundtrip tests pass
- [ ] RBAC permission matrix tests pass
- [ ] Audit log integrity tests pass
- [ ] Secret rotation tests pass
- [ ] No clippy warnings
- [ ] All tests pass (`cargo test -p kcm-security`)

## References

- `kcm-core/src/lib.rs` — Core types (`Fact`, `KcmError`, `SubjectID`)
- `kcm-security/src/rbac.rs` — RBAC implementation
- `kcm-security/src/encryption.rs` — AES-256-GCM encryption
- `kcm-security/src/audit.rs` — Hash-chained audit log
- `kcm-security/src/secrets.rs` — Secret management
- `docs/PRD3.md §30` — Security specification (SSOT)
- `AGENTS.md` — Engineering constitution
