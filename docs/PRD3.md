# KCM Distributed, Security, Compliance & ML Specification

**Document ID:** KCM-ADVANCED-001
**Version:** 2.0.0
**Status:** Authoritative
**Authority:** P2 (Architecture)

---

## 1. Purpose

This document specifies KCM's advanced subsystems: distributed architecture, machine learning integration, security, and regulatory compliance. It derives from PRD.md (P4) for type definitions and PRD2.md (P3) for runtime interfaces.

## 2. Distributed Architecture

### 2.1 Sharding Strategies

| Strategy | Algorithm | Use Case |
|----------|-----------|----------|
| HashSharding | `hash(key) % num_shards` | Uniform distribution |
| RangeSharding | Binary search on sorted boundaries | Range queries |
| ConsistentHashSharding | Virtual nodes on hash ring | Minimal reshuffling on scale |

### 2.2 ShardMap

Maps keys to shard locations. Supports:
- `locate_key(key) → ShardInfo`
- `get_all_shards() → Vec<ShardInfo>`
- Register/unregister shards dynamically

### 2.3 Transaction Coordinator

Two-phase commit protocol:

1. **Prepare Phase:** Send PREPARE to all participant shards, collect votes
2. **Commit Phase:** If all vote YES, send COMMIT; otherwise send ABORT

`ParticipantTransport` trait abstracts network communication. `LocalTransport` for single-node testing.

### 2.4 Distributed Query Execution

Fan-out queries to all shards in parallel, merge results:
- Each shard executes locally
- Results collected and deduplicated
- Filter predicates pushed to each shard

## 3. Machine Learning Integration

### 3.1 Learned Index

Piecewise linear regression models for index prediction:
- `RegressionModel` — single linear model (y = ax + b)
- `LearnedIndex` — collection of models, each covering a key range
- `search(value) → (lower, upper)` — approximate position ±100

Training: O(n) per model. Search: O(log k) where k = number of models.

### 3.2 Confidence Learner

Online learning of confidence calibration:
- Tracks fact correctness history per fact hash
- Tracks rule accuracy per rule ID
- Exponential moving average (α = 0.1) for adaptation
- `adjust_rule_confidence(rule_id, base) → adjusted`

### 3.3 Rule Discovery

Association rule mining from fact patterns:
- Counts predicate chain patterns (X →pred1→ Y →pred2→ Z)
- Filters by min_support and min_confidence thresholds
- Outputs `RulePattern` objects for `InferenceEngine`

## 4. Security

### 4.1 RBAC (Role-Based Access Control)

5 permission levels:

| Permission | Description |
|-----------|-------------|
| Read | Query facts |
| Write | Insert/update facts |
| Delete | Remove facts |
| Execute | Run inference rules |
| Admin | Manage users/roles |

`ACLManager` with:
- User management (create, assign roles)
- Role management (create, assign permissions)
- Context-level ACL grants
- Thread-safe via `Arc<RwLock<>>`

### 4.2 Encryption at Rest

- Algorithm: AES-256-GCM (authenticated encryption)
- Key derivation: BLAKE3 with context string "kcm-encryption"
- Nonce: 12-byte random via `getrandom`
- Key zeroization on drop via `write_volatile`
- File-level encrypt/decrypt operations

### 4.3 Audit Logging

Hash-chained audit log:
- Events: QueryExecuted, FactInserted, FactDeleted, RuleExecuted, PermissionDenied
- Each event includes: type, user_id, context, timestamp, details
- Chain integrity: each event's hash includes previous event's hash
- Ring buffer: max 100,000 events
- Integrity verification: recompute chain and compare

## 5. Compliance

### 5.1 GDPR Data Subject Management

| Operation | Description |
|-----------|-------------|
| Register | Create data subject record |
| Grant Consent | Mark consent as granted |
| Withdraw Consent | Mark consent as withdrawn |
| Export | Export all facts for a subject |
| Delete | Right to be forgotten — remove all facts |
| Check Status | Query consent state |

Consent states: Granted, Withdrawn, NotProvided.

### 5.2 Data Classification

4-tier classification system:

| Tier | Name | Encryption Required | Audit Required | Max Retention |
|------|------|-------------------|----------------|---------------|
| 1 | Public | No | No | 365 days |
| 2 | Internal | No | Yes | 730 days |
| 3 | Confidential | Yes | Yes | 1825 days |
| 4 | Restricted | Yes | Yes | 2555 days |

`ClassifiedFact` wraps `Fact` with classification tier. Policy methods:
- `requires_encryption() → bool`
- `requires_audit_log() → bool`
- `max_retention_days() → u32`
- `is_expired() → bool`

## 6. Invariants

| Subsystem | Invariant | Enforcement |
|-----------|-----------|-------------|
| Distributed | All shards vote before commit | Coordinator waits for all responses |
| Encryption | Key zeroized on drop | `write_volatile` in Drop impl |
| Audit | Chain integrity maintained | BLAKE3 hash chain verification |
| GDPR | Deleted facts cannot be recovered | Permanent removal from schema |
| Classification | Restricted facts always encrypted | Policy check before storage |

## 7. References

- **Depends on:** PRD.md (P4 — types), PRD2.md (P3 — runtime interfaces)
- **Parent specs:** AGENTS.md
- **Derived specs:** KCM_SECURITY_TRUST_SPEC, KCM_INDEXING_SPEC
