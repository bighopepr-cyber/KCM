# kcm-security Technical Specification

## Overview

`kcm-security` is the security and access control crate for KCM. It provides Role-Based Access Control (RBAC) with 5 permission levels, AES-256-GCM authenticated encryption, hash-chained audit logging, and secure secret management. This crate is the single authority for all security operations in the KCM system.

## Scope

This specification covers:
- RBAC implementation with 5-tier permission hierarchy
- AES-256-GCM encryption with BLAKE3 key derivation
- Hash-chained, tamper-evident audit logging
- Secure secret storage with key rotation
- Integration points with other KCM crates

## Responsibilities

| Responsibility | Description |
|---------------|-------------|
| **RBAC** | Enforce access control on every operation. Role hierarchy: Read → Write → Admin → SuperAdmin → Owner. |
| **Encryption** | Provide AES-256-GCM authenticated encryption. Key derivation via BLAKE3 KDF. 96-bit random nonces. |
| **Audit Logging** | Maintain hash-chained, tamper-evident audit trail. FIFO eviction at 100K entries. |
| **Secret Management** | Secure storage of encryption keys, API tokens, and other secrets. Key rotation support. |

## Technical Specification

### RBAC — Role-Based Access Control

#### Permission Levels

| Level | Name | Capabilities |
|-------|------|-------------|
| 0 | `Read` | Read-only access. Cannot modify, delete, or administer. |
| 1 | `Write` | Create and update data. Cannot delete or manage users. |
| 2 | `Admin` | Full data management including delete. Cannot manage users. |
| 3 | `SuperAdmin` | User management, role assignment, system configuration. |
| 4 | `Owner` | Full system control. Unrestricted access to all operations. |

#### Role Hierarchy

```
Owner (4) > SuperAdmin (3) > Admin (2) > Write (1) > Read (0)
```

- A role includes all capabilities of lower roles.
- `SuperAdmin` cannot grant `Owner` permissions.
- `Owner` is unique — only one Owner can exist at a time.
- Role assignment is logged to the audit log.

#### Permission Check Algorithm

```
fn has_permission(user_role: Role, required: Permission) -> bool {
    user_role.level >= required.level
}
```

### Encryption — AES-256-GCM

#### Algorithm

- **Cipher:** AES-256-GCM (Galois/Counter Mode)
- **Key Size:** 256 bits (32 bytes)
- **Nonce Size:** 96 bits (12 bytes)
- **Tag Size:** 128 bits (16 bytes)

#### Key Derivation

- **KDF:** BLAKE3 in keyed mode
- **Input:** Master secret + context-specific info
- **Output:** 256-bit derived key
- **Domain Separation:** Different context strings for different key purposes

#### Nonce Generation

- **Source:** `getrandom` (CSPRNG)
- **Size:** 96 bits (12 bytes)
- **Uniqueness:** Guaranteed per key by random generation (birthday bound: 2^48 encryptions)

#### Encrypted Payload Format

```
[nonce: 12 bytes][ciphertext: variable][tag: 16 bytes]
```

### Audit Log — Hash-Chained Design

#### Entry Structure

Each audit event contains:
- `timestamp` — When the event occurred (i64)
- `actor` — Who performed the action (String)
- `action` — What action was taken (String)
- `resource` — Which resource was affected (String)
- `result` — Outcome of the action (String)
- `prev_hash` — BLAKE3 hash of the previous entry (32 bytes)

#### Hash Chain

```
Entry[0].prev_hash = 0x000...000 (genesis block)
Entry[i].prev_hash = BLAKE3(Entry[i-1].timestamp || Entry[i-1].actor || Entry[i-1].action || Entry[i-1].resource || Entry[i-1].result || Entry[i-1].prev_hash)
```

#### Tamper Detection

- Tampering with any entry breaks the hash chain.
- Verification walks the chain from the most recent entry to the genesis block.
- Any mismatch indicates tampering.

#### Eviction Policy

- **FIFO** — First In, First Out
- **Capacity:** 100,000 entries
- When capacity is reached, oldest entry is removed.
- Eviction is logged as a separate audit event.

### Secrets — Secure Storage

#### Key Hierarchy

```
Master Secret (highest trust)
├── Encryption Key (derived via BLAKE3 KDF)
├── Audit Key (derived via BLAKE3 KDF)
└── API Keys (derived via BLAKE3 KDF)
```

#### Key Rotation

- New keys derived from master secret with incremented salt.
- Old keys retained for decryption of existing data.
- Rotation is atomic — no window where data is inaccessible.
- Rotation events are logged to the audit log.

## Architecture

```
┌─────────────────────────────────────────────────┐
│                  kcm-security                    │
├─────────┬──────────┬──────────┬─────────────────┤
│  rbac.rs│encrypt.rs│ audit.rs │  secrets.rs      │
├─────────┴──────────┴──────────┴─────────────────┤
│              kcm-core (types, errors)            │
└─────────────────────────────────────────────────┘
```

## Internal Components

| Module | File | Responsibility |
|--------|------|---------------|
| RBAC | `rbac.rs` | Role definitions, permission checks, role hierarchy enforcement |
| Encryption | `encryption.rs` | AES-256-GCM encrypt/decrypt, BLAKE3 KDF, nonce generation |
| Audit Log | `audit.rs` | Hash-chained entry append, chain verification, FIFO eviction |
| Secrets | `secrets.rs` | Secret storage, key rotation, secure memory handling |

## Data Model

### Permission Enum

```rust
#[repr(u8)]
pub enum Permission {
    Read = 0,
    Write = 1,
    Admin = 2,
    SuperAdmin = 3,
    Owner = 4,
}
```

### Role Struct

```rust
pub struct Role {
    pub name: String,
    pub level: Permission,
    pub granted_by: String,
    pub granted_at: i64,
}
```

### AuditEvent Struct

```rust
pub struct AuditEvent {
    pub timestamp: i64,
    pub actor: String,
    pub action: String,
    pub resource: String,
    pub result: String,
    pub prev_hash: [u8; 32],
}
```

### EncryptedPayload

```rust
pub struct EncryptedPayload {
    pub nonce: [u8; 12],
    pub ciphertext: Vec<u8>,
    pub tag: [u8; 16],
}
```

### SecretEntry

```rust
pub struct SecretEntry {
    pub id: String,
    pub encrypted_value: EncryptedPayload,
    pub created_at: i64,
    pub rotated_at: Option<i64>,
    pub key_version: u32,
}
```

## Execution Flow

### RBAC Check Flow

```
1. Caller invokes operation
2. Extract user role from context
3. Determine required permission for operation
4. Call rbac::has_permission(user_role, required_permission)
5. If denied → return KcmError::Conflict("permission denied")
6. If allowed → proceed with operation
7. Log operation to audit log
```

### Encryption Flow

```
1. Caller provides plaintext + context
2. Generate 96-bit random nonce via getrandom
3. Derive encryption key via BLAKE3 KDF(master_secret, context)
4. Encrypt plaintext via AES-256-GCM(key, nonce, plaintext)
5. Return EncryptedPayload { nonce, ciphertext, tag }
```

### Audit Log Append Flow

```
1. Construct AuditEvent with timestamp, actor, action, resource, result
2. Retrieve previous entry's hash (or genesis hash if empty)
3. Compute prev_hash = BLAKE3(previous_entry)
4. Append event to VecDeque<AuditEvent>
5. If size > 100K → remove oldest entry (FIFO)
6. Return Ok(())
```

## Public API

```rust
// RBAC
pub fn has_permission(user_role: &Role, required: Permission) -> bool
pub fn assign_role(user: &str, role: Role, assigner: &Role) -> Result<(), KcmError>
pub fn revoke_role(user: &str, revoker: &Role) -> Result<(), KcmError>

// Encryption
pub fn encrypt(plaintext: &[u8], key: &[u8; 32], context: &[u8]) -> Result<EncryptedPayload, KcmError>
pub fn decrypt(payload: &EncryptedPayload, key: &[u8; 32], context: &[u8]) -> Result<Vec<u8>, KcmError>
pub fn derive_key(master: &[u8], context: &[u8]) -> [u8; 32]

// Audit Log
pub fn append_audit_event(event: AuditEvent) -> Result<(), KcmError>
pub fn verify_chain() -> Result<bool, KcmError>
pub fn get_audit_events(limit: usize) -> Vec<AuditEvent>

// Secrets
pub fn store_secret(id: &str, value: &[u8], key: &[u8; 32]) -> Result<(), KcmError>
pub fn retrieve_secret(id: &str, key: &[u8; 32]) -> Result<Vec<u8>, KcmError>
pub fn rotate_key(old_key: &[u8; 32], new_key: &[u8; 32]) -> Result<(), KcmError>
```

## Configuration

| Parameter | Default | Description |
|-----------|---------|-------------|
| `audit_log_capacity` | 100,000 | Maximum audit log entries before FIFO eviction |
| `nonce_size` | 12 bytes (96 bits) | AES-GCM nonce size |
| `tag_size` | 16 bytes (128 bits) | AES-GCM authentication tag size |
| `key_size` | 32 bytes (256 bits) | AES-256 key size |

## Dependencies

| Dependency | Version | Purpose | Justification |
|------------|---------|---------|---------------|
| `kcm-core` | — | Types, errors (`KcmError`, `Fact`) | Core type definitions |
| `parking_lot` | — | `Mutex`, `RwLock` | 3-5x faster than std sync primitives |
| `blake3` | — | Hashing, KDF | Fastest cryptographic hash, used for key derivation and audit chain |
| `aes-gcm` | — | AES-256-GCM encryption | Audited AEAD cipher |
| `getrandom` | — | CSPRNG | Cryptographically secure random number generation |
| `serde_json` | — | Serialization | Secret entry serialization |
| `reqwest` | — | HTTP client | External key management service integration |

## Error Handling

All public APIs return `Result<T, KcmError>`:

```rust
pub enum KcmError {
    NotFound(String),
    OutOfMemory,
    InvalidArgument(String),
    Io(String),
    Corrupted(String),
    Conflict(String),
    TransactionAborted,
}
```

| Operation | Error on Failure |
|-----------|-----------------|
| RBAC denied | `KcmError::Conflict("permission denied")` |
| Invalid key | `KcmError::InvalidArgument("invalid key")` |
| Decrypt failure | `KcmError::Corrupted("decryption failed")` |
| Chain broken | `KcmError::Corrupted("audit chain integrity failure")` |
| Secret not found | `KcmError::NotFound("secret not found")` |

## Performance Characteristics

| Operation | Complexity | Notes |
|-----------|-----------|-------|
| Permission check | O(1) | Single integer comparison |
| Encryption | O(n) | Linear in plaintext size |
| Decryption | O(n) | Linear in ciphertext size |
| Audit log append | O(1) amortized | VecDeque push_back |
| Chain verification | O(n) | Linear in chain length |
| Secret store | O(n) | Linear in secret size |

## Security Considerations

### Cryptographic Analysis

- **AES-256-GCM** provides IND-CCA2 security (confidentiality) and INT-CTXT authenticity.
- **96-bit nonce** with random generation provides birthday bound of 2^48 encryptions per key.
- **BLAKE3 KDF** provides domain separation and key stretching.
- **Hash chain** provides tamper-evidence — any modification breaks the chain.

### Threat Mitigations

| Threat | Mitigation |
|--------|-----------|
| Privilege escalation | RBAC enforced on every operation; hierarchy enforced at type level |
| Key compromise | Key rotation; BLAKE3 KDF; no plaintext key storage |
| Audit tampering | Hash chain; FIFO eviction; serialized append |
| Secret exposure | AES-256-GCM at rest; access-gated; no logging of secrets |
| Timing attacks | Constant-time comparisons; uniform error responses |
| Nonce reuse | Random 96-bit nonces via CSPRNG; birthday bound at 2^48 |

## Integration

`kcm-security` is consumed by:

| Consumer | Usage |
|----------|-------|
| `kcm-interface` | REST/gRPC handlers enforce RBAC before processing |
| `kcm-distributed` | Shard operations check permissions before data access |
| `kcm-runtime` | Database operations enforce RBAC on mutations |
| `kcm-storage` | Trusts runtime's RBAC checks (no direct dependency) |

## Sequence Diagram — Encryption + Audit Log Flow

```
┌──────────┐     ┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│  Caller  │     │ kcm-security │     │  audit.rs    │     │ encryption.rs│
└────┬─────┘     └──────┬───────┘     └──────┬───────┘     └──────┬───────┘
     │                  │                    │                    │
     │  encrypt(data)   │                    │                    │
     │─────────────────>│                    │                    │
     │                  │  derive_key()      │                    │
     │                  │───────────────────────────────────────>│
     │                  │  key               │                    │
     │                  │<───────────────────────────────────────│
     │                  │                    │                    │
     │                  │  encrypt(key,data) │                    │
     │                  │───────────────────────────────────────>│
     │                  │  EncryptedPayload  │                    │
     │                  │<───────────────────────────────────────│
     │                  │                    │                    │
     │                  │  append_audit()    │                    │
     │                  │───────────────────>│                    │
     │                  │  Ok(())            │                    │
     │                  │<───────────────────│                    │
     │  Result          │                    │                    │
     │<─────────────────│                    │                    │
```

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                        KCM System                               │
├───────────┬──────────┬──────────────┬──────────────────────────┤
│kcm-       │kcm-      │kcm-          │kcm-                      │
│interface  │distributed│runtime       │storage                   │
├───────────┴──────────┴──────────────┴──────────────────────────┤
│                    kcm-security                                 │
├─────────┬──────────┬──────────┬────────────────────────────────┤
│  rbac.rs│encrypt.rs│ audit.rs │  secrets.rs                    │
├─────────┴──────────┴──────────┴────────────────────────────────┤
│              kcm-core (types, errors)                           │
├────────────────────────────────────────────────────────────────┤
│         parking_lot · blake3 · aes-gcm · getrandom              │
└────────────────────────────────────────────────────────────────┘
```

## References

- `kcm-core/src/lib.rs` — Core types and error definitions
- `kcm-security/src/rbac.rs` — RBAC implementation
- `kcm-security/src/encryption.rs` — Encryption implementation
- `kcm-security/src/audit.rs` — Audit log implementation
- `kcm-security/src/secrets.rs` — Secret management implementation
- `docs/PRD3.md §30` — Security specification (SSOT authority)
- `AGENTS.md` — Engineering constitution

## SSOT Alignment

| Specification | SSOT Source | Status |
|--------------|-------------|--------|
| RBAC 5 permission levels | `docs/PRD3.md §30` | ✅ Aligned |
| AES-256-GCM encryption | `docs/PRD3.md §30` | ✅ Aligned |
| BLAKE3 KDF | `docs/PRD3.md §30` | ✅ Aligned |
| Hash-chained audit log | `docs/PRD3.md §30` | ✅ Aligned |
| FIFO eviction at 100K | `AGENTS.md` | ✅ Aligned |
| Key rotation | `docs/PRD3.md §30` | ✅ Aligned |
| `Result<T, KcmError>` return | `AGENTS.md` | ✅ Aligned |
| No `unwrap()` in production | `AGENTS.md` | ✅ Aligned |
| Single dependency on kcm-core | `AGENTS.md` | ✅ Aligned |
| parking_lot for synchronization | `AGENTS.md` | ✅ Aligned |
