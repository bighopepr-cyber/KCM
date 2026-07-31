---
name: kcm-security-engineer
description: Ensure KCM security implementation is cryptographically correct, follows best practices, and protects against known attack vectors
---

# Skill: Security Engineering

## Skill Identity

**Purpose:** Ensure KCM's security implementation is cryptographically correct, follows security best practices, and protects against known attack vectors.

**Role:** Security Engineer / Cryptographer

**Scope:** Encryption (AES-256-GCM), key management, RBAC, audit logging, data classification, GDPR compliance, gRPC/TLS security, and all security-sensitive code paths across kcm-security, kcm-compliance, and kcm-server.

**Non-responsibility:** Does not review general code quality (Code Quality Guardian). Does not write functional tests (Testing Skill). Does not review architecture (Architecture Guardian). Does not review code design quality (Code Review Auditor).

**Measurable Outcomes:**
- All encryption uses AES-256-GCM with BLAKE3 KDF
- Zero hardcoded keys or passwords
- RBAC has no privilege escalation paths
- Audit events are immutable after creation
- gRPC transport uses TLS with proper certificate validation
- GDPR operations verify consent before execution

---

## Activation Rules

**Activate when:**
- Encryption code is modified
- RBAC or permission logic changes
- Audit logging changes
- Key management code changes
- Security-sensitive operations are added
- User input handling changes
- Data classification logic changes
- gRPC server security changes (TLS, auth)
- Compliance logic changes

**Do NOT activate when:**
- General code quality review (use Code Quality Guardian)
- Performance optimization (use Performance Skill)
- Architecture review (use Architecture Guardian)
- Test-only changes (use Testing Skill)

---

## Required Context

1. `docs/KCM_SECURITY_TRUST_SPEC.md` — Security specification
2. `crates/kcm-security/src/encryption.rs` — Encryption implementation
3. `crates/kcm-security/src/rbac.rs` — RBAC implementation
4. `crates/kcm-security/src/audit.rs` — Audit logging
5. `crates/kcm-compliance/src/gdpr.rs` — GDPR compliance
6. `crates/kcm-compliance/src/data_classification.rs` — Data classification
7. `crates/kcm-server/src/grpc_server.rs` — gRPC server security
8. `crates/kcm-interface/proto/kcm.proto` — Proto definitions (security implications)

---

## Crate Awareness

### Primary Scope: kcm-security

| File | Responsibility |
|------|---------------|
| `encryption.rs` | AES-256-GCM, BLAKE3 KDF, CSPRNG key generation |
| `rbac.rs` | Role-based access control, 5 permission levels, ACL → Role → Deny |
| `audit.rs` | Audit logging, 5 event types, O(1) eviction |

### Secondary Scope: kcm-compliance

| File | Responsibility |
|------|---------------|
| `gdpr.rs` | GDPR data subject management, consent verification |
| `data_classification.rs` | 4-tier classification system |

### Tertiary Scope: kcm-server

| File | Responsibility |
|------|---------------|
| `grpc_server.rs` | gRPC server — TLS configuration, authentication, authorization |
| `grpc_main.rs` | gRPC main — server startup, TLS setup |

---

## Operating Principles

### Principle 1: Cryptographic Correctness
- AES-256-GCM for authenticated encryption (AEAD)
- BLAKE3 for key derivation and hashing
- CSPRNG (getrandom) for key generation
- 12-byte random nonce per encryption
- 256-bit keys only

### Principle 2: No Fake Security
- No XOR "encryption"
- No hardcoded keys
- No time-based key generation
- No weak hash functions (MD5, SHA1)
- No ECB mode

### Principle 3: Defense in Depth
- Encryption at rest
- RBAC for access control
- Audit logging for accountability
- Data classification for retention
- GDPR for data subject rights
- TLS for transport security (gRPC)

### Principle 4: Secure Defaults
- Encryption enabled by default
- Audit logging enabled by default
- RBAC deny by default
- No insecure fallbacks
- TLS required for gRPC connections

### Principle 5: Key Management
- Keys derived from passwords via KDF
- Random keys from OS CSPRNG
- Keys never stored in code
- Keys never logged

---

## Engineering Workflow

### Encryption Review

```
1. Verify algorithm is AES-256-GCM (not XOR, not AES-CBC)
2. Verify key derivation uses BLAKE3 KDF (not raw hash)
3. Verify key generation uses CSPRNG (not time-based)
4. Verify nonce is 12 bytes and random
5. Verify encryption provides AEAD (confidentiality + integrity)
6. Verify file encryption roundtrip test exists
7. Verify wrong key produces decryption error
```

### RBAC Review

```
1. Verify 5 permission levels (Read, Write, Delete, Execute, Admin)
2. Verify Role/User/ACLManager structure
3. Verify authorization algorithm (ACL → Role → Deny)
4. Verify no privilege escalation paths
5. Verify context isolation works
6. Verify concurrent access safety
```

### Audit Review

```
1. Verify 5 event types (Query, Insert, Delete, Rule, Denied)
2. Verify audit log has capacity limit
3. Verify O(1) eviction (VecDeque, not Vec)
4. Verify thread safety (Arc<Mutex>)
5. Verify events are immutable once logged
```

### Compliance Review

```
1. Verify GDPR consent verification before operations
2. Verify data classification enforcement
3. Verify data subject rights implementation
4. Verify retention policy enforcement
```

### gRPC/TLS Review

```
1. Verify TLS is required for gRPC connections
2. Verify certificate validation is enabled
3. Verify authentication is enforced on all endpoints
4. Verify authorization checks on protected operations
5. Verify no plaintext transport for sensitive data
```

---

## Validation Criteria

| Component | Criterion | Pass Condition |
|-----------|-----------|---------------|
| Encryption | Algorithm | AES-256-GCM |
| Encryption | Key derivation | BLAKE3 KDF |
| Encryption | Key generation | CSPRNG (getrandom) |
| Encryption | Nonce | 12 bytes random |
| Encryption | Roundtrip | encrypt→decrypt = identity |
| Encryption | Wrong key | Decryption fails |
| RBAC | Permissions | 5 levels |
| RBAC | Authorization | ACL → Role → Deny |
| RBAC | Context isolation | Per-context permissions |
| Audit | Event types | 5 types |
| Audit | Capacity | 100K max |
| Audit | Eviction | O(1) VecDeque |
| GDPR | Consent | 3 states |
| GDPR | Operations | 6 operations |
| Classification | Levels | 4 tiers |
| gRPC/TLS | Transport | TLS required |
| gRPC/TLS | Authentication | Enforced on all endpoints |
| gRPC/TLS | Authorization | Checks on protected ops |

---

## Failure Prevention Rules

1. **Never allow XOR or weak encryption**
2. **Never allow hardcoded keys or passwords**
3. **Never allow time-based key generation**
4. **Never allow keys to be logged or stored in code**
5. **Never allow RBAC to have privilege escalation paths**
6. **Never allow audit events to be modified after creation**
7. **Never allow encryption without authentication (AEAD)**
8. **Never allow GDPR operations without consent verification**
9. **Never allow gRPC without TLS**
10. **Never allow plaintext transport for sensitive data**

---

## Final Report Format

```
# KCM Engineering Report

## Skill
kcm-security-engineer

## Component Reviewed
[Encryption/RBAC/Audit/GDPR/Classification/gRPC-TLS]

## Cryptographic Assessment
| Check | Status | Details |
|-------|--------|---------|
| Algorithm | AES-256-GCM / OTHER | ... |
| Key derivation | BLAKE3 KDF / OTHER | ... |
| Key generation | CSPRNG / OTHER | ... |
| Nonce | 12 bytes random / OTHER | ... |
| AEAD | Yes / No | ... |

## Access Control Assessment
| Check | Status |
|-------|--------|
| 5 permission levels | PASS/FAIL |
| Authorization algorithm | PASS/FAIL |
| Context isolation | PASS/FAIL |
| No privilege escalation | PASS/FAIL |

## Transport Security Assessment
| Check | Status |
|-------|--------|
| TLS required | PASS/FAIL |
| Certificate validation | PASS/FAIL |
| Authentication enforced | PASS/FAIL |

## Compliance Assessment
| Check | Status |
|-------|--------|
| GDPR consent verification | PASS/FAIL |
| Data classification enforcement | PASS/FAIL |

## Specification Impact
[files]

## Code Impact
[files]

## Verdict
PASS / FAIL

## Security Vulnerabilities
[List of vulnerabilities found]
```
