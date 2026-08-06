# KCM Security & Trust Specification

**Document ID:** KCM-SEC-001
**Version:** 1.0.0
**Status:** Active
**Owner:** Specification Lock (P4)
**Authority:** P2 (PRD3.md §4)

---

## 1. Purpose

Defines KCM's security subsystem: RBAC, encryption at rest, audit logging, and trust verification.

## 2. RBAC (Role-Based Access Control)

### 2.1 Permission Levels

| Permission | Bit | Description |
|-----------|-----|-------------|
| Read | 0x01 | Query facts |
| Write | 0x02 | Insert/update facts |
| Delete | 0x04 | Remove facts |
| Execute | 0x08 | Run inference rules |
| Admin | 0x10 | Manage users/roles |

### 2.2 ACLManager

```rust
pub struct ACLManager {
    users: HashMap<String, User>,
    roles: HashMap<String, Role>,
    context_grants: HashMap<u8, HashMap<String, Permission>>,
}
```

**Operations:**
| Method | Description |
|--------|-------------|
| `create_user(name)` | Create user |
| `assign_role(user, role)` | Assign role to user |
| `create_role(name, permissions)` | Create role with permissions |
| `grant_context(user, context, permission)` | Grant context-level permission |
| `check_permission(user, context, permission)` | Check effective permission |

### 2.3 Permission Resolution

1. Check context-level grants (most specific)
2. Check role permissions
3. Default: deny

### 2.4 Thread Safety

- `Arc<RwLock<ACLManager>>` (parking_lot)
- Readers concurrent, writers exclusive

## 3. Encryption at Rest

### 3.1 Algorithm

- **Algorithm:** AES-256-GCM (Galois/Counter Mode)
- **Key Size:** 256 bits (32 bytes)
- **Nonce Size:** 96 bits (12 bytes)
- **Tag Size:** 128 bits (16 bytes)

### 3.2 Key Derivation

- **Algorithm:** BLAKE3
- **Context String:** `"kcm-encryption"`
- **Input:** User-provided password/key material
- **Output:** 32-byte encryption key

### 3.3 Nonce Generation

- **Source:** `getrandom` crate (CSPRNG)
- **Size:** 12 bytes random
- **Uniqueness:** Each encryption operation uses fresh nonce

### 3.4 File Operations

| Operation | Description |
|-----------|-------------|
| `encrypt_file(path, key)` | Encrypt entire file in-place |
| `decrypt_file(path, key)` | Decrypt entire file in-place |

### 3.5 Key Management

- Key zeroized on drop via `write_volatile`
- Key never stored on disk
- Key material never logged

### 3.6 Invariants

| Invariant | Enforcement |
|-----------|-------------|
| Key zeroized on drop | `write_volatile` in Drop impl |
| Nonce unique per operation | CSPRNG generation |
| Authentication tag verified | GCM tag check on decrypt |
| No key in memory after drop | Zeroization |

## 4. Audit Logging

### 4.1 Event Types

| Event | Description |
|-------|-------------|
| QueryExecuted | Query performed |
| FactInserted | Fact added |
| FactDeleted | Fact removed |
| RuleExecuted | Inference rule applied |
| PermissionDenied | Access denied |

### 4.2 Event Structure

```rust
pub struct AuditEvent {
    pub event_type: AuditEventType,
    pub user_id: String,
    pub context: u8,
    pub timestamp: i64,
    pub details: String,
    pub previous_hash: [u8; 32],
    pub hash: [u8; 32],
}
```

### 4.3 Chain Integrity

- Each event's hash includes previous event's hash
- Hash algorithm: BLAKE3
- Chain verification: recompute all hashes and compare
- Tamper-evident: any modification breaks chain

### 4.4 Storage

- Ring buffer: `VecDeque<AuditEvent>` with max 100,000 events
- Protected by `Mutex<VecDeque<AuditEvent>>` (parking_lot, Arc)
- FIFO eviction when full

### 4.5 Integrity Verification

```rust
pub fn verify_integrity(&self) -> bool {
    // Recompute chain hashes from genesis
    // Compare with stored hashes
    // Return true if chain valid
}
```

## 5. FFI Safety

### 5.1 Null Pointer Guards

All FFI functions check null pointers before dereferencing:
```rust
if db.is_null() || fact.is_null() {
    return KCM_Error::KCM_ERR_INVALID_ARGUMENT;
}
```

### 5.2 Memory Management

- `Box::into_raw` for returning owned pointers
- `Box::from_raw` for reclaiming owned pointers
- Never pass uninitialized memory to FFI functions

### 5.3 Safety Documentation

All FFI functions have `# Safety` documentation specifying:
- Required pointer validity
- Lifetime requirements
- Caller responsibilities

## 6. Supply Chain Security

### 6.1 Dependency Auditing

- `cargo audit` checks for known vulnerabilities
- `cargo deny` enforces dependency policies
- Run in CI pipeline

### 6.2 No Hardcoded Secrets

- No secrets, API keys, or credentials in code
- Key material provided at runtime
- Environment variables for configuration

## 7. Security Test Matrix

| # | Test | Attack Vector | Expected |
|---|------|--------------|----------|
| 1 | Injection prevention | Malicious dictionary input | Stored safely |
| 2 | Buffer overflow | DenseVec capacity exceeded | Error returned |
| 3 | Integer overflow | Max ID values handled | No wrap |
| 4 | RBAC enforcement | Unauthorized access denied | Permission denied |
| 5 | Timing attack | Constant-time operations | No timing leak |
| 6 | Memory safety | No use-after-free | Clean access |
| 7 | Concurrent safety | Race condition prevention | No data corruption |
| 8 | Confidence boundary | NaN/Infinity rejected | Error returned |
| 9 | Context isolation | Cross-context data leakage | Isolation maintained |
| 10 | Audit integrity | Hash chain verification | Chain valid |

## 8. References

- **Implements:** PRD3.md §4 (Security)
- **Depends on:** KCM_DATA_MODEL_SPEC
- **Related:** KCM_API_SPEC, KCM_TRUST_SPEC
