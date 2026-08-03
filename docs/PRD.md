# KCM Architecture Specification

**Document ID:** KCM-ARCH-001
**Version:** 2.0.0
**Status:** Authoritative
**Authority:** P4 (Specification Lock)

---

## 1. Purpose

This document is the authoritative specification for KCM's core architecture: type system, storage engine, compute engine, and reasoning engine. All other documents derive from this specification.

## 2. Design Principles

1. **Columnar Native** — All knowledge is stored as independent typed columns
2. **Dictionary-Encoded** — All string/reference data mapped to integer IDs
3. **Deterministic** — Identical input always produces identical output
4. **Zero-Copy Access** — DenseVec provides direct slice access without allocation
5. **SIMD-Ready** — Data structures aligned for vector processing
6. **Production-Grade** — Full ACID, crash recovery, validation

## 3. Type System

All types defined in `kcm-core/src/types.rs`. This is the single source of truth for domain types.

### 3.1 Identity Types

| Type | Underlying | Range | Purpose |
|------|-----------|-------|---------|
| `RowID` | `u64` | 0..u64::MAX | Sequential row identifier |
| `SubjectID` | `u32` | 0..u32::MAX | Dictionary-encoded subject |
| `PredicateID` | `u8` | 0..255 | Dictionary-encoded predicate |
| `ObjectID` | `u32` | 0..u32::MAX | Dictionary-encoded object |
| `ContextID` | `u8` | 0..255 | Context scope (0 = null) |
| `EvidenceID` | `u8` | 0..255 | Evidence type (0 = unknown) |

### 3.2 Confidence

`Confidence(f64)` — validated to [0.0, 1.0], rejects NaN/Infinity.

Operations:
- `multiply(a, b)` → `a × b` (conjunction)
- `combine_or(a, b)` → `a + b - a×b` (disjunction)

### 3.3 Fact

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

Size: 34 bytes uncompressed. Validated at construction via `Fact::new()`.

### 3.4 Error Model

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

All public APIs return `Result<T, KcmError>`. `StorageError` converts via `From` impl.

### 3.5 Column Identifier

```rust
pub enum ColumnID {
    RowID = 0, Subject = 1, Predicate = 2, Object = 3,
    Confidence = 4, Evidence = 5, Timestamp = 6, Context = 7,
    Version = 8, Priority = 9, Owner = 10,
}
```

11 variants (RowID is virtual, not stored). 10 physical columns in storage.

## 4. Data Structures

### 4.1 DenseVec

SIMD-aligned (64-byte) contiguous memory allocator. Fixed capacity, no reallocation.

- `new(capacity)` — allocates aligned memory
- `push(value)` — O(1), returns error if full
- `as_slice()` — zero-copy access
- `Index`/`IndexMut` — O(1) random access

Implementation: raw `alloc/dealloc` with `Layout::from_size_align`. Drop deallocates.

### 4.2 Bitmap

64-bit word bit-vector with O(1) set/clear/get and O(n/64) bulk operations.

- `set(idx)`, `clear(idx)`, `get(idx)` — O(1)
- `and_inplace`, `or_inplace`, `not_inplace` — O(n/64)
- `count_ones()` — popcount per word
- `iter_set_bits()` — Brian Kernighan's algorithm, O(popcount)

### 4.3 Dictionary

Bidirectional string↔integer mapping. ID 0 is always NULL (empty string).

- `insert(value) → DictID` — O(1) amortized
- `get(id) → Option<&str>` — O(1)
- `lookup(value) → Option<DictID>` — O(1) via HashMap

`SharedDictionary` wraps in `Arc<RwLock<Dictionary>>` for concurrent access.

## 5. Compute Engine

### 5.1 Operator Trait

```rust
pub trait Operator: Send + Sync {
    fn execute(&self) -> Result<Vec<usize>, KcmError>;
    fn estimated_rows(&self) -> usize;
}
```

All operators produce row ID lists. Operators compose via row ID passing.

### 5.2 Operators

| Operator | Purpose | Complexity |
|----------|---------|------------|
| `ScanOp` | Full scan with optional context/confidence filter | O(n) |
| `FilterOp` | Predicate evaluation on row IDs | O(m) where m = input rows |
| `ProjectOp` | Column selection (pass-through) | O(1) |
| `JoinOp` | Hash join on object column | O(n + m) |
| `AggregateOp` | Count/Sum/Avg/Min/Max with optional group-by | O(n) |

### 5.3 SIMD Operations

AVX2-accelerated operations for `[u8]`:
- `filter_eq(value)` — equality filter, processes 32 bytes per instruction
- `filter_ge(value)` — greater-than-or-equal filter
- `count_nonzero()` — population count

Fallback scalar implementations for `[u32]` and `[f64]`.

## 6. Reasoning Engine

### 6.1 Rule System

```rust
pub enum RulePattern {
    Triple(Option<SubjectID>, PredicateID, Option<ObjectID>),
    And(Box<RulePattern>, Box<RulePattern>),
    Or(Box<RulePattern>, Box<RulePattern>),
    Not(Box<RulePattern>),
}
```

Rules have: id, name, pattern, consequent_predicate, confidence_formula (closure), enabled flag, priority.

### 6.2 Inference Engine

Forward-chaining inference:
- Iterates until convergence or max_iterations (default 1000)
- Confidence threshold: 0.3 (default)
- Timeout: 60 seconds
- Pattern matching traverses schema rows
- Derived facts appended to schema with rule provenance

## 7. Invariants

| Invariant | Enforcement |
|-----------|-------------|
| Confidence ∈ [0.0, 1.0] | `Confidence::new()` validates at construction |
| Row IDs monotonically increasing | `Schema::append_fact()` increments row_count |
| Column lengths equal | `Schema` enforces all columns same length |
| No unwrap in production | Clippy + code review |
| All errors propagated | `Result<T, KcmError>` return type |
| Deterministic execution | No randomness in query/inference paths |
| Dictionary ID 0 = NULL | Reserved at construction |

## 8. Performance Characteristics

| Operation | Target | Measurement |
|-----------|--------|-------------|
| Column scan 1M facts | < 10ms | Criterion benchmark |
| Dictionary lookup | < 100ns | Criterion benchmark |
| Bitmap AND 1M bits | < 1ms | Criterion benchmark |
| Insert throughput | > 50K facts/sec | Load test |
| Query latency P99 | < 100ms | Load test |
| Memory per fact | < 100 bytes | Memory profiling |

## 9. Testing Strategy

| Level | Scope | Count | Speed |
|-------|-------|-------|-------|
| Unit | Single function | 90+ | < 100ms |
| Integration | Cross-crate | 108+ | 1s-5s |
| Property | Invariant verification | 8+ | 1-5min |
| Security | Attack surface | 29+ | varies |

## 10. References

- **Depends on:** None (root specification)
- **Parent specs:** AGENTS.md (Engineering Constitution)
- **Derived specs:** KCM_DATA_MODEL_SPEC, KCM_COLUMNAR_FORMAT_SPEC, KCM_COMPRESSION_SPEC, KCM_QUERY_EXECUTION_SPEC
