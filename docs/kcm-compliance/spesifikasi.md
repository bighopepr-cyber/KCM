# kcm-compliance Technical Specification

## Overview

`kcm-compliance` is the compliance and standards crate of the KCM (Knowledge Columnar Model) engine. It provides GDPR consent management and data classification (4 tiers) to ensure regulatory compliance and data protection across the KCM system.

## Scope

This specification covers the `kcm-compliance` crate only. It does not cover storage, compute, reasoning, or any higher-level functionality.

## Responsibilities

| Responsibility | Description |
|---------------|-------------|
| GDPR consent management | Consent lifecycle: register, grant, withdraw, export, delete |
| Data classification | 4-tier classification: Public, Internal, Confidential, Restricted |
| Encryption enforcement | Validate encryption for Confidential/Restricted data |
| Audit log requirements | Determine which tiers require audit logging |
| Data retention | Calculate maximum retention period per classification tier |

## Technical Specification

### GDPR Consent Management

#### Consent Record Structure

```rust
pub struct DataSubject {
    pub subject_id: String,
    pub email: String,
    pub consent: ConsentStatus,
}
```

#### Consent Status

```rust
pub enum ConsentStatus {
    Granted,
    Withdrawn,
    NotProvided,
}
```

**State transitions:**
- `NotProvided → Granted` (via `grant_consent`)
- `Granted → Withdrawn` (via `withdraw_consent`)
- No transition from `Withdrawn` — consent cannot be re-granted without re-registration

#### GDPR Manager

```rust
pub struct GDPRManager {
    subjects: Arc<RwLock<HashMap<String, DataSubject>>>,
}
```

**Operations:**

| Operation | Complexity | Description |
|-----------|-----------|-------------|
| `new()` | O(1) | Create empty manager |
| `register_subject(subject)` | O(1) | Register new data subject |
| `grant_consent(subject_id)` | O(1) | Grant consent for subject |
| `withdraw_consent(subject_id)` | O(1) | Withdraw consent for subject |
| `has_consent(subject_id)` | O(1) | Check if subject has granted consent |
| `export_data(subject_id)` | O(1) | Export subject data (right to portability) |
| `delete_data(subject_id)` | O(1) | Delete subject data (right to erasure) |

**Constraints:**
- Subject IDs must be unique — duplicate registration returns `KcmError::InvalidArgument`
- All operations on nonexistent subjects return `KcmError::NotFound`
- Thread-safe via `Arc<RwLock<...>>` (parking_lot)

### Data Classification

#### Classification Tiers

```rust
pub enum DataClassification {
    Public,
    Internal,
    Confidential,
    Restricted,
}
```

| Tier | Encryption | Audit Log | Max Retention (days) | Description |
|------|-----------|-----------|---------------------|-------------|
| Public | No | No | 365 | Non-sensitive, publicly available data |
| Internal | No | Yes | 730 | Internal business data |
| Confidential | Yes | Yes | 1825 | Sensitive data requiring protection |
| Restricted | Yes | Yes | 2555 | Highly sensitive, regulated data |

#### Classification Policy

```rust
impl DataClassification {
    pub fn requires_encryption(&self) -> bool;
    pub fn requires_audit_log(&self) -> bool;
    pub fn max_retention_days(&self) -> u32;
    pub fn validate_encryption(&self, is_encrypted: bool) -> Result<(), KcmError>;
}
```

#### Classified Fact

```rust
pub struct ClassifiedFact {
    pub fact: Fact,
    pub classification: DataClassification,
}
```

**Operations:**

| Operation | Complexity | Description |
|-----------|-----------|-------------|
| `should_retain(now)` | O(1) | Check if fact is within retention period |
| `is_expired(now)` | O(1) | Check if fact has exceeded retention period |

**Retention calculation:**
```
retained = (now - fact.timestamp) <= (max_retention_days * 86400)
```

## Architecture

```
kcm-compliance
  ├── gdpr.rs            → GDPRManager, DataSubject, ConsentStatus
  ├── data_classification.rs → DataClassification, ClassifiedFact
  └── lib.rs             → Module declarations
```

## Internal Components

### gdpr.rs

Implements GDPR consent management. Uses `Arc<RwLock<HashMap<String, DataSubject>>>` for thread-safe subject registry. All mutation operations acquire write lock; read operations acquire read lock.

### data_classification.rs

Implements data classification with 4 tiers. `DataClassification` is a simple enum with match-based methods. `ClassifiedFact` wraps a `Fact` with its classification tier and provides retention logic.

### lib.rs

Module declarations only. Re-exports `data_classification` and `gdpr` modules.

## Data Model

### Memory Layout

```
DataSubject (approximate):
  subject_id: String    → Heap-allocated string
  email: String         → Heap-allocated string
  consent: ConsentStatus → Enum (3 variants, stack-allocated)

GDPRManager:
  subjects: Arc<RwLock<HashMap<String, DataSubject>>>
    → Arc pointer (8 bytes)
    → RwLock (parking_lot, ~8 bytes)
    → HashMap<String, DataSubject>

DataClassification:
  Enum (4 variants, stack-allocated, 1 byte)

ClassifiedFact:
  fact: Fact (40 bytes)
  classification: DataClassification (1 byte)
```

## Execution Flow

### Consent Check Flow

```
1. Caller requests data operation
2. Call GDPRManager::has_consent(subject_id)
3. Read lock acquired on subjects HashMap
4. Lookup subject by ID
5. Check consent == ConsentStatus::Granted
6. Return true/false
7. If false → caller must deny operation
8. If true → caller proceeds with operation
```

### Classification Flow

```
1. Fact is created or loaded
2. Assign DataClassification tier
3. Call validate_encryption(is_encrypted)
4. If tier requires encryption and fact is not encrypted → return error
5. Wrap fact in ClassifiedFact
6. Store ClassifiedFact with classification metadata
7. On access, check should_retain(now)
8. If expired → flag for deletion
```

## Public API

See the existing [README.md](../../crates/kcm-compliance/README.md) for the complete public API reference.

## Configuration

No configuration options. `kcm-compliance` is a stateless compliance library. Classification tiers and retention periods are defined as constants in the `DataClassification` enum.

## Dependencies

| Dependency | Type | Justification |
|-----------|------|---------------|
| `kcm-core` | Runtime | Core types (`Fact`, `KcmError`) |
| `parking_lot` | Runtime | 3-5x faster RwLock/Mutex than std for thread-safe consent registry |

## Error Handling

All public APIs return `Result<T, KcmError>`. Error variants used:

| Variant | Usage |
|---------|-------|
| `KcmError::NotFound` | Subject not found in registry |
| `KcmError::InvalidArgument` | Duplicate subject registration, encryption validation failure |

## Performance Characteristics

| Operation | Target | Measurement |
|-----------|--------|-------------|
| `GDPRManager::has_consent` | <100ns | HashMap lookup |
| `GDPRManager::register_subject` | <1μs | HashMap insert |
| `GDPRManager::grant_consent` | <100ns | HashMap mutation |
| `GDPRManager::withdraw_consent` | <100ns | HashMap mutation |
| `GDPRManager::export_data` | <1μs | HashMap lookup + format |
| `GDPRManager::delete_data` | <100ns | HashMap remove |
| `DataClassification::requires_encryption` | <1ns | Match arm |
| `DataClassification::validate_encryption` | <1ns | Match arm + bool check |
| `ClassifiedFact::should_retain` | <1ns | Arithmetic comparison |

## Security Considerations

- Consent state transitions are deterministic and auditable
- Withdrawn consent is immediately effective (no grace period)
- `delete_data()` fully removes all subject records (GDPR Article 17)
- `export_data()` returns complete subject data (GDPR Article 20)
- No subject PII in error messages (email not exposed in `NotFound` errors)
- Thread-safe via `parking_lot::RwLock` (no data races)
- `validate_encryption()` enforces encryption for Confidential/Restricted data

## Integration

`kcm-compliance` is consumed by higher-level KCM crates:

```
kcm-compliance ← kcm-runtime    (consent checks before data operations)
kcm-compliance ← kcm-security   (RBAC integration with consent state)
kcm-compliance ← kcm-testing    (compliance test scenarios)
```

## Sequence Diagram

### Consent Registration Flow

```
Caller → GDPRManager::register_subject(subject)
  → Acquire write lock
  → Check subject_id not in HashMap
  → Insert subject into HashMap
  → Release lock
  → Return Ok(())
```

### Consent Withdrawal Flow

```
Caller → GDPRManager::withdraw_consent(subject_id)
  → Acquire write lock
  → Lookup subject in HashMap
  → Set consent = Withdrawn
  → Release lock
  → Return Ok(())
```

### Classification Validation Flow

```
Caller → DataClassification::validate_encryption(is_encrypted)
  → Match tier
  → If Confidential/Restricted and !is_encrypted
    → Return Err(KcmError::InvalidArgument)
  → Return Ok(())
```

## Architecture Diagram

```
┌─────────────────────────────────────────┐
│           kcm-compliance                │
├───────────────────┬─────────────────────┤
│      gdpr.rs      │ data_classification │
│                   │      .rs            │
├───────────────────┼─────────────────────┤
│ GDPRManager       │ DataClassification  │
│ DataSubject       │ ClassifiedFact      │
│ ConsentStatus     │                     │
├───────────────────┴─────────────────────┤
│           parking_lot (RwLock)          │
├─────────────────────────────────────────┤
│              kcm-core                   │
│         (Fact, KcmError)                │
└─────────────────────────────────────────┘
```

## References

- [PRD3.md](../PRD3.md) §32 — GDPR compliance specification
- [SSOT.md](../../SSOT.md) — Single Source of Truth
- [AGENTS.md](../../AGENTS.md) — Engineering constitution
- [README.md](../../crates/kcm-compliance/README.md) — Crate overview

## SSOT Alignment

| SSOT Requirement | Specification | Implementation | Test |
|-----------------|---------------|----------------|------|
| R-GDPR-001 | Consent record management | `gdpr.rs:GDPRManager` | `tests/test_compliance.rs` |
| R-GDPR-002 | Consent lifecycle (grant/withdraw) | `grant_consent()`, `withdraw_consent()` | `tests/test_compliance.rs` |
| R-GDPR-003 | Right to erasure | `delete_data()` | `tests/test_compliance.rs` |
| R-GDPR-004 | Right to portability | `export_data()` | `tests/test_compliance.rs` |
| R-CLASS-001 | 4-tier classification | `DataClassification` enum | `tests/test_compliance.rs` |
| R-CLASS-002 | Encryption enforcement | `validate_encryption()` | `tests/test_compliance.rs` |
| R-CLASS-003 | Audit log requirements | `requires_audit_log()` | `tests/test_compliance.rs` |
| R-CLASS-004 | Data retention periods | `max_retention_days()` | `tests/test_compliance.rs` |
