# KCM Data Model Specification

**Document ID:** KCM-DATA-001  
**Version:** 1.0.0  
**Depends on:** KCM-SPEC-001, KCM_ARCHITECTURE-001

---

## 1. Purpose

Defines the knowledge representation model, type system, validation rules, and schema design for KCM.

---

## 2. Knowledge Object (Fact)

A Knowledge Object in KCM is called a **Fact**. It represents a single assertion in the knowledge base.

### 2.1 Fact Structure

```
Fact {
    subject:    SubjectID(u32)      — Entity performing or described by the relation
    predicate:  PredicateID(u8)     — Type of relation (max 256 types)
    object:     ObjectID(u32)       — Entity or value being related to
    confidence: f64                  — Confidence score [0.0, 1.0]
    evidence:   EvidenceID(u8)      — Evidence source identifier
    timestamp:  i64                  — Nanosecond-precision creation time
    context:    ContextID(u8)       — Domain/scope identifier
    version:    i32                  — Fact version (incremented on update)
    priority:   i8                   — Priority level
    owner:      u16                  — Owner identifier
}
```

### 2.2 Type Definitions

| Type | Underlying | Range | Semantics |
|------|-----------|-------|-----------|
| RowID | u64 | [0, u64::MAX] | Sequential row identifier, 0-indexed |
| SubjectID | u32 | [0, u32::MAX] | Subject entity reference (dictionary-encoded) |
| PredicateID | u8 | [0, 255] | Relation type (max 256 distinct predicates) |
| ObjectID | u32 | [0, u32::MAX] | Object entity reference (dictionary-encoded) |
| ContextID | u8 | [0, 255] | Domain/scope (max 256 contexts) |
| EvidenceID | u8 | [0, 255] | Evidence source (max 256 sources) |
| Confidence | f64 | [0.0, 1.0] | Probabilistic confidence score |

### 2.3 Validation Rules

| Rule | Constraint | Error |
|------|-----------|-------|
| VR-001 | confidence ∈ [0.0, 1.0] and not NaN/Inf | InvalidArgument |
| VR-002 | subject, predicate, object are non-negative | Type system enforced |
| VR-003 | timestamp is nanoseconds since UNIX epoch | SystemTime::now() |
| VR-004 | version starts at 1 for new facts | Constructor default |
| VR-005 | Two facts are equal iff all fields match (except timestamp/version) | PartialEq on Fact |

---

## 3. Schema

### 3.1 Column Layout

The Schema stores 10 independent columns plus a tombstone bitmap:

```
Schema {
    subject_col:    Column<u32>     — Dictionary encoding, Zstd compression
    predicate_col:  Column<u8>      — Dictionary encoding, RLE compression
    object_col:     Column<u32>     — Dictionary encoding, Zstd compression
    confidence_col: Column<f64>     — Gorilla encoding, Zstd compression
    evidence_col:   Column<u8>      — Dictionary encoding, RLE compression
    timestamp_col:  Column<i64>     — Delta encoding, Zstd compression
    context_col:    Column<u8>      — Dictionary encoding, RLE compression
    version_col:    Column<i32>     — Delta encoding, LZ4 compression
    priority_col:   Column<i8>      — Identity encoding, RLE compression
    owner_col:      Column<u16>     — Dictionary encoding, Zstd compression
    tombstones:     Bitmap          — Soft-delete marker (1 bit per row)
}
```

### 3.2 Operations

| Operation | Behavior | Complexity |
|-----------|----------|------------|
| append_fact | Appends values to all 10 columns | O(1) amortized |
| get_fact | Reads from all 10 columns, returns None if tombstone | O(1) |
| delete_fact | Sets bit in tombstone bitmap | O(1) |
| update_fact | Overwrites values at index in all 10 columns | O(1) |
| active_count | len() - tombstones.count_ones() | O(W/64) |
| iter_active | Iterator skipping tombstoned rows | O(n) worst case |

### 3.3 Constraints

| Constraint | Rationale |
|------------|-----------|
| Schema capacity is pre-allocated at creation | Avoids reallocation in hot path |
| All columns must have identical length | Row alignment invariant |
| Tombstone bitmap size equals column capacity | Bit-addressable delete markers |
| Compress/decompress roundtrip must preserve data | Zero-loss compression requirement |

### 3.4 Tombstone Serialization

Tombstone bitmaps are persisted to disk as part of the file format:

| Field | Size | Description |
|-------|------|-------------|
| Row Count | 8 bytes (u64 LE) | Number of bits in the bitmap |
| Byte Length | 8 bytes (u64 LE) | Number of bytes in the packed bitmap |
| Bitmap Data | variable | Packed bits, one bit per row, LSB-first |

Serialization uses `Bitmap::as_bytes()` (raw words as bytes) and `Bitmap::from_bytes()` for reconstruction.

After save/load round-trip:
- Tombstoned rows are correctly marked as deleted
- `active_count()` is accurate
- `get_fact()` returns `None` for tombstoned rows

---

## 4. Dictionary

### 4.1 Structure

```
Dictionary {
    entries: Vec<String>           — ID-to-string mapping
    reverse_map: HashMap<String, DictID>  — String-to-ID mapping
}
```

### 4.2 Semantics

| Property | Value |
|----------|-------|
| Null entry | ID 0 maps to empty string "" |
| Deduplication | insert("hello") twice returns same ID |
| Thread safety | SharedDictionary wraps in Arc<RwLock<Dictionary>> |
| Ordering | IDs are assigned in insertion order |

### 4.3 Operations

| Operation | Signature | Behavior |
|-----------|-----------|----------|
| insert | (&mut self, &str) -> DictID | Returns existing ID if already present |
| get | (&self, DictID) -> Option<&str> | Lookup by ID |
| lookup | (&self, &str) -> Option<DictID> | Reverse lookup |
| len | (&self) -> usize | Entry count including null |

---

## 5. Column Type

### 5.1 Generic Structure

```
Column<T: Copy> {
    data: DenseVec<T>        — SIMD-aligned storage
    encoding: ColumnEncoding  — Logical encoding type
    compression: CompressionCodec — Physical compression
    row_count: u64           — Current element count
    raw_bytes: Vec<u8>       — Compressed data buffer
    compressed: bool         — Compression state flag
}
```

### 5.2 Encoding Types

| Encoding | Applicable Types | Algorithm |
|----------|-----------------|-----------|
| Identity | u8, i8, u16 | No transformation |
| Dictionary | u32, u8 | Maps to dictionary IDs |
| Delta | i64, i32 | Stores differences between consecutive values |
| Gorilla | f64 | XOR-based floating-point encoding |
| FrameOfReference | Any | Block-relative encoding |
| RLE | u8 | Run-length encoding for repeated values |

### 5.3 Column Type Aliases

| Alias | Type | Used For |
|-------|------|----------|
| SubjectColumn | Column<u32> | Subject entity references |
| ObjectColumn | Column<u32> | Object entity references |
| PredicateColumn | Column<u8> | Relation type identifiers |
| ConfidenceColumn | Column<f64> | Confidence scores |
| TimestampColumn | Column<i64> | Creation timestamps |
| ContextColumn | Column<u8> | Context identifiers |
| EvidenceColumn | Column<u8> | Evidence source IDs |
| VersionColumn | Column<i32> | Fact versions |
| PriorityColumn | Column<i8> | Priority levels |
| OwnerColumn | Column<u16> | Owner identifiers |

---

## 6. Error Types

```
enum KcmError {
    NotFound(String),
    OutOfMemory,
    InvalidArgument(String),
    Io(String),
    Corrupted(String),
    Conflict(String),
    TransactionAborted,
}
```

All public API methods return `Result<T, KcmError>`.

---

## 7. Validation

| Check | Method | Frequency |
|-------|--------|-----------|
| Confidence bounds | Fact::new() | Every insert |
| Column length consistency | Schema operations | Every append |
| Tombstone bitmap size | Schema::new() | Creation |
| Dictionary deduplication | Dictionary::insert() | Every insert |
| File checksum | DatabaseFile::verify() | After load |
