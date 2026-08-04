# KCM Storage, Runtime & Interfaces Specification

**Document ID:** KCM-STORAGE-001
**Version:** 2.0.0
**Status:** Authoritative
**Authority:** P3 (Architecture)

---

## 1. Purpose

This document specifies KCM's persistence layer (WAL, file format, backup/recovery), runtime layer (database, transactions, metrics, health), optimizer, and interfaces (C FFI, REST, KQL). It derives from PRD.md (P4 authority) for type definitions.

## 2. Storage Engine

### 2.1 Column Storage

Each column is a `DenseVec<T>` with per-column encoding and compression.

| Column | Rust Type | Encoding | Compression |
|--------|-----------|----------|-------------|
| Subject | `u32` | Dictionary | Zstd |
| Predicate | `u8` | Dictionary | RLE |
| Object | `u32` | Dictionary | Zstd |
| Confidence | `f64` | Gorilla | Zstd |
| Evidence | `u8` | Dictionary | RLE |
| Timestamp | `i64` | Delta | Zstd |
| Context | `u8` | Dictionary | RLE |
| Version | `i32` | Delta | LZ4 |
| Priority | `i8` | Identity | RLE |
| Owner | `u16` | Dictionary | Zstd |

### 2.2 Compression Codecs

| Codec | Algorithm | Use Case |
|-------|-----------|----------|
| Zstd | Zstandard | General-purpose, high ratio |
| LZ4 | LZ4 block | Speed-optimized |
| RLE | Run-Length Encoding | Low-cardinality columns |
| Noop | Identity | Already compressed or small |

All codecs implement `Compressor` trait: `compress(&[u8]) → Vec<u8>`, `decompress(&[u8]) → Vec<u8>`.

### 2.3 Dictionary Encoding

`DictionaryCodec` maps string values to integer IDs. Batch operations: `encode_batch`, `decode_batch`.

Six shared dictionaries in `KnowledgeDatabase`:
- subjects, objects, predicates, evidence, context, owner

### 2.4 Encoding Types

| Encoding | Description | Columns |
|----------|-------------|---------|
| Identity | Raw bytes, no transformation | Priority |
| Dictionary | String→u32 mapping | Subject, Predicate, Object, Evidence, Context, Owner |
| Delta | Difference between consecutive values | Timestamp, Version |
| Gorilla | XOR-based float encoding | Confidence |
| RLE | Run-length encoding | Predicate, Evidence, Context, Priority |

## 3. Write-Ahead Log

### 3.1 WAL Entry Format

| Entry | Size | Layout |
|-------|------|--------|
| Insert | 38 bytes | op(1) + subject(4) + predicate(1) + object(4) + confidence(8) + timestamp(8) + context(1) + version(4) + priority(1) + owner(2) + crc32(4) |
| Delete | 13 bytes | op(1) + row_id(8) + crc32(4) |

Note: `evidence` is intentionally not stored in WAL entries. On replay, `evidence` defaults to `EvidenceID::UNKNOWN`. This is a design decision to minimize WAL entry size — evidence provenance is reconstructed from the source system, not from WAL replay. See KCM_COLUMNAR_FORMAT_SPEC §3 for the complete WAL entry layout.

### 3.2 WAL Properties

- Buffered writes (64KB threshold before flush)
- CRC32 checksums per entry
- Append-only, sequential I/O
- Replay on recovery
- Truncate after successful checkpoint

## 4. Binary File Format

### 4.1 File Layout

```
Header (31 bytes):
  Magic: [u8; 5] = "KCMDB"
  Version: u8 = 2
  Row Count: u64
  Column Count: u8 = 10
  Created Timestamp: i64
  Last Modified: i64

Column Blocks (×10):
  Element Count: u64
  Codec ID: u8 (0=None, 1=Zstd, 2=LZ4, 3=RLE)
  Data Length: u64
  Data: [u8]

Tombstone Bitmap:
  Row Count: u64
  Byte Length: u64
  Bitmap Data: [u8]

Checksum Trailer:
  BLAKE3 Hash: [u8; 32]
```

### 4.2 File Invariants

- Magic bytes always "KCMDB"
- Version byte determines format compatibility
- Column blocks written in fixed order (Subject→Owner)
- Tombstone bitmap tracks deleted rows
- BLAKE3 checksum covers entire file except checksum itself

## 5. Backup & Recovery

### 5.1 Backup Types

| Type | Content | Dependencies |
|------|---------|-------------|
| Full | Complete schema serialization | None |
| Incremental | Changes since last backup | Previous backup |

### 5.2 Recovery Strategy

1. Load primary database file
2. Verify BLAKE3 checksum
3. If corrupted, attempt backup restoration
4. Replay WAL entries to catch up
5. Verify data integrity

### 5.3 Crash Recovery Guarantees

- WAL replay is idempotent (re-insert is safe)
- Deleted rows remain deleted after recovery
- Partial writes are detected via checksum mismatch
- Empty database created if no valid files exist

## 6. Indexing

### 6.1 Index Types

| Index | Purpose | Cardinality |
|-------|---------|-------------|
| BitmapIndex | Equality lookup on u8 columns | Low (≤256 distinct) |
| ZoneMap | Range filtering per block | Any |
| BloomFilter | Probabilistic membership test | Any |
| CompositeIndex | (subject, predicate) pair lookup | Medium |

### 6.2 Index Selection Rules

- Predicate/Context/Evidence → BitmapIndex
- Timestamp → ZoneMap
- Subject/Object → BloomFilter for exclusion, CompositeIndex for pair queries
- ZoneMap block size: configurable (default 1000 rows)

## 7. Query Optimizer

### 7.1 Cost Model

```rust
pub struct OperatorCost {
    pub cpu_cost: f64,
    pub io_cost: f64,
    pub memory_cost: f64,
    pub estimated_rows: usize,
}
```

Cost weights: `total = cpu×1.0 + io×10.0 + memory×0.1`

### 7.2 Optimization Pipeline

Single pipeline, applied iteratively until convergence:

1. **Filter Pushdown** — Move filters closer to scan
2. **Column Pruning** — Remove unused columns from scan
3. **Join Reordering** — Smallest relations first
4. **Index Selection** — Choose best index per predicate

### 7.3 Adaptive Execution

Tracks prediction vs actual cardinality. Re-optimizes when error > 50%.

## 8. Runtime Layer

### 8.1 KnowledgeDatabase

Central database wrapping `Schema` with thread-safe CRUD.

```rust
pub struct KnowledgeDatabase {
    schema: Arc<RwLock<Schema>>,
    dictionaries: Arc<Dictionaries>,
}
```

Public API:
- `new() → Result<Self>`
- `get_schema() → RwLockReadGuard<Schema>`
- `get_schema_mut() → RwLockWriteGuard<Schema>`
- `insert(&Fact) → Result<RowID>`
- `insert_batch(&[Fact]) → Result<Vec<RowID>>`
- `update(RowID, &Fact) → Result<()>`
- `delete(RowID) → Result<()>`
- `query() → QueryBuilder`
- `get_fact(RowID) → Result<Option<Fact>>`
- `dict_insert_subject(&str) → Result<DictID>`
- `dict_get_subject(DictID) → Option<String>`
- `dict_lookup_subject(&str) → Option<DictID>`
- `begin_transaction() → Transaction`
- `fact_count() → usize`
- `active_fact_count() → usize`
- `compact() → Result<Self>`

### 8.2 QueryBuilder

Fluent API for building queries:

```rust
kb.query()
    .with_subject(SubjectID(1))
    .with_predicate(PredicateID(5))
    .with_confidence(0.8)
    .execute()?
```

### 8.3 Transaction

Buffering transaction system:
- Changes buffered in memory
- `commit()` applies all changes atomically
- `rollback()` discards all changes
- State machine: Active → Committed/RolledBack/Aborted

### 8.4 Metrics

14 atomic counters (lock-free):

| Metric | Type | Purpose |
|--------|------|---------|
| queries_total | AtomicU64 | Total queries executed |
| queries_failed | AtomicU64 | Failed queries |
| query_duration_sum_ms | AtomicU64 | Cumulative query time |
| inserts_total | AtomicU64 | Total inserts |
| inserts_failed | AtomicU64 | Failed inserts |
| cache_hits | AtomicU64 | Cache hits |
| cache_misses | AtomicU64 | Cache misses |
| memory_bytes | AtomicU64 | Memory usage |
| inferences_total | AtomicU64 | Inference operations |
| facts_inferred | AtomicU64 | Facts derived from inference |
| estimated_memory_bytes | AtomicU64 | Estimated memory footprint |
| total_facts | AtomicU64 | Total fact count |
| active_facts | AtomicU64 | Active (non-deleted) fact count |
| tombstone_count | AtomicU64 | Deleted row count |

### 8.5 Health Check

Threshold-based health determination:
- **Healthy:** error_rate < 5%, cache_hit_ratio > 50%
- **Degraded:** error_rate < 5%, cache_hit_ratio ≤ 50%
- **Unhealthy:** error_rate ≥ 5%

### 8.6 Executor

Rayon thread pool with work-stealing parallelism:
- `parallel_map(items, f)` — parallel map
- `parallel_filter(items, f)` — parallel filter
- CPU count determines thread count

### 8.7 Async Executor

Tokio runtime bridge:
- `async_insert()`, `async_query_all()`, `async_fact_count()`
- Uses `spawn_blocking` for compute-bound operations

## 9. Interfaces

### 9.1 C FFI

18 `extern "C"` functions:

| Function | Purpose |
|----------|---------|
| `KCM_DatabaseNew` | Create database |
| `KCM_DatabaseFree` | Destroy database |
| `KCM_DatabaseInsert` | Insert fact |
| `KCM_DatabaseUpdate` | Update fact |
| `KCM_DatabaseDelete` | Delete fact |
| `KCM_DatabaseFactCount` | Get fact count |
| `KCM_DatabaseActiveCount` | Get active count |
| `KCM_DatabaseQuery` | Start query |
| `KCM_QueryNext` | Iterate results |
| `KCM_QueryFree` | Free query |
| `KCM_DatabaseBeginTransaction` | Start transaction |
| `KCM_TransactionFree` | Free transaction |
| `KCM_DatabaseSave` | Save database to file |
| `KCM_DatabaseLoad` | Load database from file |
| `KCM_DatabaseVerify` | Verify database integrity |
| `KCM_TransactionCommit` | Commit transaction |
| `KCM_TransactionRollback` | Rollback transaction |
| `KCM_ErrorMessage` | Get error string |

All functions check null pointers before dereferencing. Opaque types prevent direct struct access.

### 9.2 REST API

8 endpoints (no prefix, served by kcm-server):

| Method | Endpoint | Handler |
|--------|----------|---------|
| GET | `/health` | Health check |
| POST | `/facts` | Insert fact |
| GET | `/facts` | Query facts |
| GET | `/facts/{id}` | Get fact |
| PUT | `/facts/{id}` | Update fact |
| DELETE | `/facts/{id}` | Delete fact |
| GET | `/stats` | Metrics JSON |
| GET | `/metrics` | Prometheus format |

### 9.3 KQL Parser

Knowledge Query Language — SQL-like syntax:

```sql
SELECT subject, object FROM facts
WHERE predicate = 0 AND confidence >= 0.8
ORDER BY timestamp DESC
LIMIT 100
```

Tokens: 28 variants. Parser produces `SelectQuery` AST with WHERE, JOIN, ORDER BY, LIMIT clauses.

### 9.4 Python Bindings

Feature-gated (`python` feature). `PyKnowledgeBase` class with:
- `insert(subject, predicate, object, confidence)`
- `query_all() → list`
- `fact_count() → int`

### 9.5 gRPC Service

```protobuf
service KnowledgeService {
    rpc InsertFact(InsertFactRequest) returns (InsertFactResponse);
    rpc QueryFacts(QueryRequest) returns (QueryResponse);
    rpc GetFact(GetFactRequest) returns (FactData);
    rpc GetStats(GetStatsRequest) returns (StatsResponse);
}
```

## 10. References

- **Depends on:** PRD.md (P4 — type definitions)
- **Parent specs:** AGENTS.md
- **Derived specs:** KCM_COLUMNAR_FORMAT_SPEC, KCM_COMPRESSION_SPEC, KCM_API_SPEC, KCM_RUNTIME_SPEC
