# kcm-core Technical Specification

## Overview

`kcm-core` is the foundational crate of the KCM (Knowledge Columnar Model) engine. It defines the core data types, dense vectors, bitmap engine, and dictionary encoding used by all other KCM crates.

## Scope

This specification covers the `kcm-core` crate only. It does not cover storage, compute, reasoning, or any higher-level functionality.

## Responsibilities

| Responsibility | Description |
|---------------|-------------|
| Core types | `Fact`, `RowID`, `SubjectID`, `PredicateID`, `ObjectID`, `Confidence`, `KcmError` |
| Dense vector | `DenseVec<T>` — growable, cache-friendly vector with O(1) index access |
| Bitmap engine | `Bitmap` — bit-vector with rank, select, and next-set-bit operations |
| Dictionary | `Dictionary` — string interning with bidirectional mapping (u32 ↔ String) |

## Technical Specification

### Fact Structure

```rust
pub struct Fact {
    pub subject: SubjectID,      // u32
    pub predicate: PredicateID,  // u8
    pub object: ObjectID,        // u32
    pub confidence: Confidence,  // f64
    pub evidence: Evidence,      // u8
    pub timestamp: Timestamp,    // i64
    pub context: Context,        // u8
    pub version: Version,        // i32
    pub priority: Priority,      // i8
    pub owner: Owner,            // u16
}
```

**Size:** 34 bytes uncompressed.

**Constraints:**
- `Confidence` must be in range `[0.0, 1.0]`
- All IDs are non-negative
- `timestamp` represents Unix epoch milliseconds

### DenseVec\<T\>

A growable, cache-friendly vector with O(1) index access.

| Operation | Complexity | Description |
|-----------|-----------|-------------|
| `new()` | O(1) | Create empty vector |
| `push(value)` | O(1) amortized | Append element |
| `get(index)` | O(1) | Access by index |
| `len()` | O(1) | Number of elements |
| `is_empty()` | O(1) | Check if empty |

**Bounds:** `T: Copy + Default`

### Bitmap

A bit-vector supporting rank, select, and next-set-bit operations.

| Operation | Complexity | Description |
|-----------|-----------|-------------|
| `new(capacity)` | O(n/64) | Create with pre-allocated capacity |
| `set(bit)` | O(1) | Set bit to 1 |
| `get(bit)` | O(1) | Read bit value |
| `clear(bit)` | O(1) | Set bit to 0 |
| `count_ones()` | O(n/64) | Population count |
| `next_set_bit(from)` | O(n/64) | Find next set bit |

### Dictionary

String interning with bidirectional mapping.

| Operation | Complexity | Description |
|-----------|-----------|-------------|
| `new()` | O(1) | Create empty dictionary |
| `intern(string)` | O(len) | Map string → u32 ID |
| `resolve(id)` | O(1) | Map u32 ID → &str |
| `len()` | O(1) | Number of entries |

**Constraints:**
- Maximum `u32::MAX` entries
- Strings are stored by reference (lifetime tied to `Dictionary`)
- Duplicate strings return the same ID

### Error Model

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

All public APIs return `Result<T, KcmError>`.

## Architecture

```
kcm-core (zero external dependencies)
  ├── types.rs      → Fact, RowID, SubjectID, PredicateID, ObjectID, Confidence, KcmError
  ├── vec.rs        → DenseVec<T>
  ├── bitmap.rs     → Bitmap
  └── dictionary.rs → Dictionary
```

## Internal Components

### types.rs

Defines all core type aliases and the `Fact` struct. The `Fact::new()` constructor validates all fields.

### vec.rs

Implements `DenseVec<T>` using a raw allocation with manual length tracking. Uses `T::default()` for zero-initialization.

### bitmap.rs

Implements `Bitmap` using `Vec<u64>` as the backing store. Bit operations use word-level manipulation with shift/mask.

### dictionary.rs

Implements `Dictionary` using `Vec<String>` for ID→String mapping and `HashMap<String, u32>` for String→ID mapping.

## Data Model

### Memory Layout

```
Fact (34 bytes):
  Subject:   [0..4)   u32
  Predicate: [4..5)   u8
  Object:    [5..9)   u32
  Confidence:[9..17)  f64
  Evidence:  [17..18) u8
  Timestamp: [18..26) i64
  Context:   [26..27) u8
  Version:   [27..31) i32
  Priority:  [31..32) i8
  Owner:     [32..34) u16
```

### Bitmap Internal Layout

```
Bitmap (n bits):
  words: Vec<u64>  → ceil(n/64) words
  capacity: usize  → total bit capacity
```

## Execution Flow

### Fact Creation

```
1. Validate Confidence ∈ [0.0, 1.0]
2. Validate IDs are non-negative
3. Construct Fact struct
4. Return Ok(Fact)
```

### Dictionary Interning

```
1. Check if string already exists in HashMap
2. If yes → return existing ID
3. If no → push string to Vec, assign new ID
4. Insert (string, ID) into HashMap
5. Return new ID
```

## Public API

See the existing [README.md](../../crates/kcm-core/README.md) for the complete public API reference.

## Configuration

No configuration options. `kcm-core` is a stateless foundational library.

## Dependencies

| Dependency | Type | Justification |
|-----------|------|---------------|
| `parking_lot` | Runtime | 3-5x faster RwLock/Mutex than std |
| `serde` | Optional | Serialization support behind `serialization` feature |
| `ahash` | Runtime | Fast hash map for Dictionary |

## Error Handling

All public APIs return `Result<T, KcmError>`. Error variants are documented in the Error Model section above.

## Performance Characteristics

| Operation | Target | Measurement |
|-----------|--------|-------------|
| `DenseVec::push` | <1ns | Criterion micro-benchmark |
| `DenseVec::get` | <1ns | Criterion micro-benchmark |
| `Bitmap::set` | <1ns | Criterion micro-benchmark |
| `Bitmap::get` | <1ns | Criterion micro-benchmark |
| `Dictionary::intern` | <100ns | Criterion micro-benchmark |
| `Dictionary::resolve` | <10ns | Criterion micro-benchmark |

## Security Considerations

- `Confidence` validation prevents invalid confidence scores
- Bounds checking on all `DenseVec` and `Bitmap` operations
- `Dictionary` handles `u32::MAX` overflow
- No `unsafe` code without documented justification

## Integration

`kcm-core` is consumed by every other KCM crate:

```
kcm-core ← kcm-storage
kcm-core ← kcm-compute
kcm-core ← kcm-reasoning
kcm-core ← kcm-optimizer
kcm-core ← kcm-runtime
kcm-core ← kcm-interface
kcm-core ← kcm-distributed
kcm-core ← kcm-ml
kcm-core ← kcm-security
kcm-core ← kcm-compliance
kcm-core ← kcm-testing
```

## Sequence Diagram

### Fact Insertion Flow

```
Caller → Fact::new() → Validate fields → Return Ok(Fact)
```

### Dictionary Intern Flow

```
Caller → dict.intern("hello")
  → Check HashMap → Not found
  → Push to Vec (id=0)
  → Insert ("hello", 0) into HashMap
  → Return 0
```

## Architecture Diagram

```
┌─────────────────────────────────────┐
│            kcm-core                 │
├─────────┬─────────┬────────┬────────┤
│ types   │ vec     │ bitmap │ dict   │
├─────────┴─────────┴────────┴────────┤
│         parking_lot (optional)      │
│         serde (optional)            │
│         ahash                       │
└─────────────────────────────────────┘
```

## References

- [PRD.md](../specs/PRD.md) §3 — Core types specification
- [SSOT.md](../../SSOT.md) — Single Source of Truth
- [AGENTS.md](../../AGENTS.md) — Engineering constitution
- [KCM_SPECIFICATION.md](../../KCM_SPECIFICATION.md) — Technical constitution
- [README.md](../../crates/kcm-core/README.md) — Crate overview

## SSOT Alignment

| SSOT Requirement | Specification | Implementation | Test |
|-----------------|---------------|----------------|------|
| R-TYPES-001 | Fact structure (34 bytes) | `types.rs:Fact` | `tests/test_core.rs` |
| R-TYPES-002 | Confidence validation [0.0, 1.0] | `Fact::new()` | `tests/test_core.rs` |
| R-TYPES-003 | KcmError enum (7 variants) | `types.rs:KcmError` | `tests/test_core.rs` |
| R-VEC-001 | DenseVec O(1) access | `vec.rs:DenseVec` | `tests/test_core.rs` |
| R-BITMAP-001 | Bitmap rank/select | `bitmap.rs:Bitmap` | `tests/property_tests.rs` |
| R-DICT-001 | Dictionary bidirectional mapping | `dictionary.rs:Dictionary` | `tests/test_core.rs` |
