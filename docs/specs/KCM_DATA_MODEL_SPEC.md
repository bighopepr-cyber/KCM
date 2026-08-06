# KCM Data Model Specification

**Document ID:** KCM-DATA-001
**Version:** 1.0.0
**Status:** Active
**Owner:** Specification Lock (P4)
**Authority:** P4 (PRD.md §3)

---

## 1. Purpose

Defines KCM's knowledge representation model: the Fact structure, identity types, confidence calculus, and data invariants.

## 2. Fact Structure

### 2.1 Layout

```rust
pub struct Fact {
    pub subject: SubjectID,      // u32 — dictionary-encoded
    pub predicate: PredicateID,  // u8  — dictionary-encoded
    pub object: ObjectID,        // u32 — dictionary-encoded
    pub confidence: f64,         // validated [0.0, 1.0]
    pub evidence: EvidenceID,    // u8
    pub timestamp: i64,          // nanoseconds since epoch
    pub context: ContextID,      // u8
    pub version: i32,            // monotonic on update
    pub priority: i8,            // -128..127
    pub owner: u16,              // dictionary-encoded
}
```

### 2.2 Size

- Unserialized: 40 bytes (Rust struct with alignment)
- Wire format (FFI): 48 bytes (`KCM_Fact` repr(C))
- Columnar storage: variable per column after encoding/compression

### 2.3 Construction

`Fact::new(subject, predicate, object, confidence)` validates confidence via `Confidence::new()`, sets timestamp to current system time (nanoseconds), and initializes defaults:
- `evidence`: `EvidenceID::UNKNOWN` (0)
- `context`: `ContextID::NULL` (0)
- `version`: 1
- `priority`: 0
- `owner`: 0

## 3. Identity Types

| Type | Underlying | Range | Null Value | Purpose |
|------|-----------|-------|------------|---------|
| `RowID` | `u64` | 0..u64::MAX | N/A | Sequential row identifier |
| `SubjectID` | `u32` | 0..u32::MAX | 0 | Dictionary-encoded subject |
| `PredicateID` | `u8` | 0..255 | 0 | Dictionary-encoded predicate |
| `ObjectID` | `u32` | 0..u32::MAX | 0 | Dictionary-encoded object |
| `ContextID` | `u8` | 0..255 | 0 | Context scope |
| `EvidenceID` | `u8` | 0..255 | 0 | Evidence type |

All identity types implement: `Copy`, `Clone`, `Debug`, `Eq`, `PartialEq`, `Ord`, `PartialOrd`, `Hash`.

## 4. Confidence Calculus

### 4.1 Construction

`Confidence::new(value)` validates:
- Rejects NaN: `value.is_nan()` → `Err(KcmError::InvalidArgument)`
- Rejects Infinity: `value.is_infinite()` → `Err(KcmError::InvalidArgument)`
- Rejects out-of-range: `!(0.0..=1.0).contains(&value)` → `Err(KcmError::InvalidArgument)`

### 4.2 Operations

| Operation | Formula | Semantics |
|-----------|---------|-----------|
| `multiply(a, b)` | `(a × b).clamp(0.0, 1.0)` | Conjunction (AND) — both conditions hold |
| `combine_or(a, b)` | `(a + b - a×b).clamp(0.0, 1.0)` | Disjunction (OR) — either condition holds |

### 4.3 Invariants

- `multiply(x, 1.0) == x` (identity)
- `multiply(x, 0.0) == 0.0` (annihilation)
- `multiply(a, b) == multiply(b, a)` (commutativity)
- `0.0 <= result <= 1.0` (bounds)
- Results are finite (never NaN/Infinity)

## 5. Column Identifier

```rust
pub enum ColumnID {
    RowID = 0,       // Virtual — not stored physically
    Subject = 1,
    Predicate = 2,
    Object = 3,
    Confidence = 4,
    Evidence = 5,
    Timestamp = 6,
    Context = 7,
    Version = 8,
    Priority = 9,
    Owner = 10,
}
```

11 variants total. RowID is virtual (computed from array position). 10 physical columns in storage.

## 6. Error Model

```rust
pub enum KcmError {
    NotFound(String),           // Entity not found
    OutOfMemory,                // Allocation failure
    InvalidArgument(String),    // Invalid parameter
    Io(String),                 // I/O operation failed
    Corrupted(String),          // Data integrity violation
    Conflict(String),           // Resource conflict
    TransactionAborted,         // Transaction rolled back
}
```

### 6.1 Properties

- All public APIs return `Result<T, KcmError>`
- `StorageError` converts to `KcmError` via `From` impl
- `String` converts to `KcmError::InvalidArgument` via `From` impl
- `KcmError` implements `Display`, `Error`, `Debug`, `Clone`, `Eq`, `PartialEq`

## 7. Data Invariants

| Invariant | Enforcement |
|-----------|-------------|
| Confidence ∈ [0.0, 1.0] | `Confidence::new()` validates at construction |
| Row IDs monotonically increasing | `Schema::append_fact()` increments row_count |
| Column lengths equal | `Schema` enforces all columns same length |
| Dictionary ID 0 = NULL | Reserved at `Dictionary::new()` |
| No unwrap in production | CI gate + code review |
| All errors propagated | `Result<T, KcmError>` return type |
| Deterministic execution | No randomness in query/inference paths |
| Tombstone count ≤ row count | `Bitmap::count_ones()` ≤ `len()` |

## 8. References

- **Implements:** PRD.md §3 (Type System)
- **Depends on:** None (root specification)
- **Derived specs:** KCM_COLUMNAR_FORMAT_SPEC, KCM_API_SPEC
