# KCM Security & Trust Specification

**Document ID:** KCM-SEC-001  
**Version:** 1.0.0  
**Depends on:** KCM-SPEC-001

---

## 1. Purpose

Defines security architecture, encryption, access control, and audit logging.

---

## 2. Security Architecture

```
┌────────────────────────────────────────────┐
│              Security Layer                 │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐ │
│  │   RBAC   │  │Encryption│  │  Audit   │ │
│  │  (ACL)   │  │(AES-256) │  │  (Log)   │ │
│  └──────────┘  └──────────┘  └──────────┘ │
└────────────────────────────────────────────┘
```

---

## 3. Role-Based Access Control (RBAC)

### 3.1 Permission Model

```rust
enum Permission {
    Read,      // Query facts
    Write,     // Insert/update facts
    Delete,    // Delete facts
    Execute,   // Run inference rules
    Admin,     // Full access
}
```

### 3.2 Entities

| Entity | Fields | Purpose |
|--------|--------|---------|
| User | user_id: String, roles: HashSet<String> | Identity |
| Role | name: String, permissions: HashSet<Permission> | Permission group |
| ACL | ContextID → Vec<(user_id, Permission)> | Per-context overrides |

### 3.3 ACLManager

| Method | Behavior |
|--------|----------|
| create_user(id) | Register new user |
| create_role(name) | Register new role |
| add_permission_to_role(role, perm) | Grant permission to role |
| assign_role(user, role) | Assign role to user |
| grant_context_permission(user, ctx, perm) | Direct context permission |
| check_permission(user, ctx, perm) -> bool | Check: direct ACL → role permissions |

### 3.4 Authorization Algorithm

```
check_permission(user, context, permission):
  1. Check direct ACL: if (user, permission) exists in context_acl[context] → ALLOW
  2. For each role of user:
       If role has permission → ALLOW
  3. DENY
```

---

## 4. Encryption

### 4.1 Key Derivation

```rust
let mut key = [0u8; 32];
blake3::derive_key("kcm-encryption", password.as_bytes(), salt, &mut key);
```

| Property | Value |
|----------|-------|
| Algorithm | BLAKE3 key derivation |
| Salt size | 32 bytes |
| Key size | 256 bits |
| Context string | "kcm-encryption" |

### 4.2 Random Key Generation

```rust
let mut key = [0u8; 32];
getrandom::getrandom(&mut key).expect("Failed to generate random key");
```

| Property | Value |
|----------|-------|
| Source | OS CSPRNG via getrandom |
| Entropy | 256 bits |

### 4.3 Encryption Algorithm

```rust
let cipher = Aes256Gcm::new_from_slice(&key)?;
let nonce = OsRng.fill_bytes(&mut nonce_bytes);
let ciphertext = cipher.encrypt(nonce, plaintext)?;
```

| Property | Value |
|----------|-------|
| Algorithm | AES-256-GCM (AEAD) |
| Nonce | 12 bytes, random per encryption |
| Key size | 256 bits |
| Provides | Confidentiality + Integrity |

### 4.4 Encrypted File Format

```
[12 bytes nonce][ciphertext bytes][16 bytes GCM tag]
```

---

## 5. Audit Logging

### 5.1 Event Types

```rust
enum AuditEventType {
    QueryExecuted,
    FactInserted,
    FactDeleted,
    RuleExecuted,
    PermissionDenied,
}
```

### 5.2 AuditEvent Structure

```rust
struct AuditEvent {
    event_type: AuditEventType,
    user_id: String,
    context: String,
    timestamp: i64,  // seconds since epoch
    details: String,
}
```

### 5.3 Constraints

| Property | Value |
|----------|-------|
| Storage | VecDeque with max 100,000 events |
| Eviction | Pop oldest on overflow |
| Thread safety | Arc<Mutex<VecDeque>> |

---

## 6. GDPR Compliance

### 6.1 DataSubject Management

| Operation | Behavior |
|-----------|----------|
| register_subject | Create new data subject |
| grant_consent | Set consent to Granted |
| withdraw_consent | Set consent to Withdrawn |
| has_consent | Check if consent == Granted |
| export_data | Serialize subject data |
| delete_data | Remove subject entirely |

### 6.2 Data Classification

| Classification | Requires Encryption | Requires Audit | Max Retention |
|---------------|--------------------|--------------|--------------|
| Public | No | No | 7 years |
| Internal | No | No | 3 years |
| Confidential | Yes | No | 1 year |
| Restricted | Yes | Yes | 6 months |

---

## 7. Validation

| Test | Description |
|------|-------------|
| AES-256-GCM roundtrip | Encrypt + decrypt produces original |
| Wrong key fails | Decrypt with wrong key returns error |
| RBAC denies unauthorized | User without role cannot access |
| Audit log caps at 100K | Oldest events evicted |
| GDPR consent workflow | Register → Grant → Withdraw → Delete |

---

## 8. References

- **Depends on:** KCM_SPECIFICATION (KCM_SPECIFICATION)
- **Parent specs:** KCM_SPECIFICATION (KCM_SPECIFICATION)
- **Related:** KCM_RUNTIME_SPEC (KCM_RUNTIME_SPEC), KCM_DATA_MODEL_SPEC (KCM_DATA_MODEL_SPEC), KCM_DEPLOYMENT_SPEC (KCM_DEPLOYMENT_SPEC)
