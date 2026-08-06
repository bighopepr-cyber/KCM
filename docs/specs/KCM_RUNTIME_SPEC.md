# KCM Runtime Specification

**Document ID:** KCM-RUNTIME-001
**Version:** 1.0.0
**Status:** Active
**Owner:** Specification Lock (P4)
**Authority:** P3 (PRD2.md §8)

---

## 1. Purpose

Defines KCM's runtime layer: KnowledgeDatabase, transactions, metrics, health checks, executor, and async executor.

## 2. KnowledgeDatabase

### 2.1 Structure

```rust
pub struct KnowledgeDatabase {
    schema: Arc<RwLock<Schema>>,
    dictionaries: Arc<Dictionaries>,
}

pub struct Dictionaries {
    pub subjects: SharedDictionary,
    pub objects: SharedDictionary,
    pub predicates: SharedDictionary,
    pub evidence: SharedDictionary,
    pub context: SharedDictionary,
    pub owner: SharedDictionary,
}
```

### 2.2 Public API

| Method | Signature | Description |
|--------|-----------|-------------|
| `new()` | `Result<Self>` | Create with 1M row capacity |
| `get_schema()` | `RwLockReadGuard<Schema>` | Read-lock schema |
| `get_schema_mut()` | `RwLockWriteGuard<Schema>` | Write-lock schema |
| `insert(&Fact)` | `Result<RowID>` | Insert fact, return row ID |
| `insert_batch(&[Fact])` | `Result<Vec<RowID>>` | Batch insert |
| `update(RowID, &Fact)` | `Result<()>` | Update fact at row |
| `delete(RowID)` | `Result<()>` | Tombstone delete |
| `query()` | `QueryBuilder` | Create query builder |
| `get_fact(RowID)` | `Result<Option<Fact>>` | Get fact by ID |
| `begin_transaction()` | `Transaction` | Start transaction |
| `fact_count()` | `usize` | Total rows (incl. deleted) |
| `active_fact_count()` | `usize` | Non-deleted rows |
| `compact()` | `Result<Self>` | Remove tombstones |
| `dict_insert_subject(&str)` | `Result<DictID>` | Dictionary insert |
| `dict_get_subject(DictID)` | `Option<String>` | Dictionary lookup |
| `dict_lookup_subject(&str)` | `Option<DictID>` | Dictionary reverse lookup |

### 2.3 Thread Safety

- Schema protected by `Arc<RwLock<Schema>>` (parking_lot)
- Dictionaries protected by `Arc<Dictionaries>` with internal `Arc<RwLock<Dictionary>>`
- Readers can concurrent; writers exclusive
- All public types are `Send + Sync`

### 2.4 Default Capacity

- Schema capacity: 1,000,000 rows
- Grows via `Schema::append_fact()` (DenseVec push)
- Compact via `Schema::compact()` to reclaim tombstone space

## 3. Transaction

### 3.1 Structure

```rust
pub struct Transaction {
    changes: Vec<TransactionChange>,
    state: TransactionState,
}

enum TransactionChange {
    Insert(Fact),
    Update(RowID, Fact),
    Delete(RowID),
}

enum TransactionState {
    Active,
    Committed,
    RolledBack,
    Aborted,
}
```

### 3.2 Lifecycle

```
begin_transaction() → Active
    ├── commit() → Committed (applies all changes atomically)
    ├── rollback() → RolledBack (discards all changes)
    └── abort() → Aborted (discards + error)
```

### 3.3 Behavior

- Changes buffered in memory
- `commit()` applies all changes atomically under write lock
- `rollback()` discards all changes
- State machine enforces valid transitions
- Double-commit returns `KcmError::Conflict`
- Double-rollback is no-op

## 4. Metrics

### 4.1 Counters (14 AtomicU64)

| Counter | Type | Description |
|---------|------|-------------|
| `queries_total` | AtomicU64 | Total queries executed |
| `queries_failed` | AtomicU64 | Failed queries |
| `query_duration_sum_ms` | AtomicU64 | Cumulative query time |
| `inserts_total` | AtomicU64 | Total inserts |
| `inserts_failed` | AtomicU64 | Failed inserts |
| `cache_hits` | AtomicU64 | Cache hits |
| `cache_misses` | AtomicU64 | Cache misses |
| `memory_bytes` | AtomicU64 | Memory usage |
| `inferences_total` | AtomicU64 | Inference operations |
| `facts_inferred` | AtomicU64 | Facts derived from inference |
| `estimated_memory_bytes` | AtomicU64 | Estimated memory footprint |
| `total_facts` | AtomicU64 | Total fact count |
| `active_facts` | AtomicU64 | Active (non-deleted) fact count |
| `tombstone_count` | AtomicU64 | Deleted row count |

### 4.2 Methods

| Method | Description |
|--------|-------------|
| `record_query(duration_ms, success)` | Record query execution |
| `record_insert(success)` | Record insert operation |
| `record_cache_hit()` | Record cache hit |
| `record_cache_miss()` | Record cache miss |
| `record_inference(facts_derived)` | Record inference run |
| `get_avg_query_latency_ms()` | Compute average latency |
| `get_cache_hit_ratio()` | Compute hit ratio |
| `get_insert_error_rate()` | Compute error rate |
| `update_memory_estimate(bytes)` | Update memory estimate |
| `update_schema_stats(total, active, tombstones)` | Update schema stats |
| `snapshot()` | Take consistent snapshot |

### 4.3 MetricsSnapshot

```rust
pub struct MetricsSnapshot {
    pub queries_total: u64,
    pub queries_failed: u64,
    pub avg_query_latency_ms: f64,
    pub inserts_total: u64,
    pub inserts_failed: u64,
    pub cache_hit_ratio: f64,
    pub memory_bytes: u64,
    pub inferences_total: u64,
    pub facts_inferred: u64,
    pub estimated_memory_bytes: u64,
    pub total_facts: u64,
    pub active_facts: u64,
    pub tombstone_count: u64,
}
```

### 4.4 Memory Layout

All 14 counters: 14 × 8 bytes = 112 bytes contiguous via single `Arc<MetricsInner>`.

## 5. Health Check

### 5.1 Status Determination

| Status | Condition |
|--------|-----------|
| **Healthy** | error_rate < 5% AND cache_hit_ratio > 50% |
| **Degraded** | error_rate < 5% AND cache_hit_ratio ≤ 50% |
| **Unhealthy** | error_rate ≥ 5% |

### 5.2 Inputs

- `error_rate = inserts_failed / inserts_total`
- `cache_hit_ratio = cache_hits / (cache_hits + cache_misses)`

## 6. Executor (Rayon)

### 6.1 Thread Pool

- Uses rayon `ThreadPool` with work-stealing parallelism
- Thread count = CPU core count
- Global thread pool (shared across instances)

### 6.2 Operations

| Method | Description |
|--------|-------------|
| `parallel_map(items, f)` | Parallel map over items |
| `parallel_filter(items, f)` | Parallel filter over items |

## 7. Async Executor (Tokio)

### 7.1 Runtime

- Uses tokio multi-threaded runtime
- `spawn_blocking` for compute-bound operations

### 7.2 Operations

| Method | Description |
|--------|-------------|
| `async_insert(fact)` | Async insert |
| `async_query_all()` | Async query all facts |
| `async_fact_count()` | Async fact count |

## 8. References

- **Implements:** PRD2.md §8 (Runtime Layer)
- **Depends on:** KCM_DATA_MODEL_SPEC, KCM_COLUMNAR_FORMAT_SPEC
- **Related:** KCM_API_SPEC, KCM_PERFORMANCE_SPEC
