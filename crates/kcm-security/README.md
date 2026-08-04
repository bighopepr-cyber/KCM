# kcm-security

Security and access control for KCM: RBAC, AES-256-GCM encryption, and audit logging.

## Purpose

Provides authentication, authorization, encryption, and audit capabilities for KCM deployments requiring security compliance.

## Modules

| Module | Purpose |
|--------|---------|
| `rbac` | Role-Based Access Control (5 permission levels) |
| `encryption` | AES-256-GCM authenticated encryption |
| `audit` | Hash-chained audit log |

## Dependencies

| Dependency | Purpose |
|------------|---------|
| `kcm-core` | Core types |
| `parking_lot` | Thread-safe state |
| `blake3` | Key derivation and audit chain hashing |
| `aes-gcm` | Authenticated encryption |
| `getrandom` | Cryptographic nonce generation |

## RBAC Permission Levels

| Level | Role | Permissions |
|-------|------|-------------|
| 0 | Guest | Read-only (public facts) |
| 1 | Reader | Read (all accessible facts) |
| 2 | Writer | Read + Insert + Update |
| 3 | Admin | Read + Write + Delete + Schema changes |
| 4 | SuperAdmin | Full access + security + compliance |

## Encryption

- Algorithm: AES-256-GCM
- Key derivation: BLAKE3 from master key + context
- Nonce: 96-bit random (getrandom)
- Authentication: GCM tag verification on every decrypt

```rust
use kcm_security::encryption::EncryptionEngine;

let engine = EncryptionEngine::new(master_key)?;
let ciphertext = engine.encrypt(plaintext)?;
let plaintext = engine.decrypt(&ciphertext)?;
```

## Audit Log

Hash-chained log entries:
```
entry[i].hash = BLAKE3(entry[i].data || entry[i-1].hash)
```

Any tampering breaks the chain and is detectable.

| Event Type | Logged |
|------------|--------|
| Authentication | User ID, timestamp, success/failure |
| Authorization | User ID, resource, operation, allowed/denied |
| Data Access | User ID, fact IDs, operation |
| Schema Change | User ID, change description |
| Security Event | Event type, details |
