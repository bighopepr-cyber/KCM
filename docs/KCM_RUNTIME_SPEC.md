# KCM Runtime Specification

**Document ID:** KCM-RUNTIME-001  
**Version:** 1.0.0  
**Status:** Derived  
**Owner:** Database Engine Specialist (P6)  
**Depends on:** KCM-ARCH-001

---

## 1. Purpose

Defines runtime lifecycle, concurrency model, memory management, and resource control.

---

## 2. Runtime Lifecycle

```
┌──────────┐   ┌──────────────┐   ┌──────────┐   ┌───────────┐   ┌──────────┐
│ Startup  │──►│ Initialization│──►│  Loading  │──►│ Execution │──►│ Shutdown │
└──────────┘   └──────────────┘   └──────────┘   └───────────┘   └──────────┘
```

### 2.1 Startup

1. Allocate schema with pre-configured capacity
2. Initialize WAL file
3. Initialize dictionaries (subject, object, predicate, evidence, context, owner)
4. Start metrics collection
5. Start health check evaluator

### 2.2 Initialization (Recovery)

```
if db_path exists AND file_size > 32:
    load DatabaseFile
    if wal_path exists:
        replay WAL entries on loaded schema
else if wal_path exists:
    create empty schema
    replay WAL entries
else:
    create fresh schema
```

### 2.3 Execution

All operations are thread-safe through `Arc<RwLock<Schema>>`:

| Operation | Lock Type | Duration |
|-----------|-----------|----------|
| Insert | Write lock | Per-fact append |
| Query | Read lock (snapshot clone) | Schema clone + iteration |
| Update | Write lock | Column overwrite |
| Delete | Write lock | Tombstone set |

### 2.4 Shutdown

1. Flush WAL buffer
2. Persist schema to disk (optional)
3. Release all locks

---

## 3. Concurrency Model

### 3.1 Read/Write Pattern

```
Readers (queries):  ──read lock──► clone schema snapshot ──release lock──► iterate
Writers (insert):   ──write lock──► append to all columns ──release lock──
```

### 3.2 Lock Hierarchy

| Lock | Type | Protects |
|------|------|----------|
| schema RwLock | parking_lot::RwLock | Schema read/write |
| WAL Mutex | parking_lot::Mutex | WAL file write |
| WAL buffer Mutex | parking_lot::Mutex | WAL write buffer |
| Dictionary RwLock | parking_lot::RwLock | Dictionary read/write |
| AuditLog Mutex | parking_lot::Mutex | Audit event append |
| Metrics | AtomicU64 | Lock-free counters |

### 3.3 Async Support

```rust
pub async fn async_insert(db: Arc<Mutex<KnowledgeDatabase>>, fact: Fact) -> Result<RowID, KcmError>;
pub async fn async_query_all(db: Arc<Mutex<KnowledgeDatabase>>) -> Result<Vec<Fact>, KcmError>;
```

Uses `tokio::task::spawn_blocking` to offload blocking schema operations.

---

## 4. Memory Management

### 4.1 DenseVec

- Cache-line aligned (64 bytes minimum)
- Pre-allocated capacity
- Drop deallocates via Layout
- Clone allocates new buffer

### 4.2 Schema Cloning

Each query creates a full Schema clone. For datasets < 1M facts, this is acceptable. For larger datasets, MVCC snapshots should be considered.

### 4.3 WAL Buffering

WAL uses in-memory buffer (64KB threshold) before flushing to disk.

---

## 5. Resource Control

### 5.1 Executor (Parallel)

```rust
pub struct Executor {
    thread_pool: rayon::ThreadPool,
}
```

- Default: `std::thread::available_parallelism()` threads
- `parallel_map` / `parallel_filter` for data-parallel operations

### 5.2 AsyncExecutor

```rust
pub struct AsyncExecutor {
    runtime: tokio::runtime::Runtime,
}
```

- Multi-threaded tokio runtime
- `block_on` for synchronous bridge

### 5.3 Database Compaction

The `compact()` method creates a new database with only active (non-deleted) facts:

| Step | Description |
|------|-------------|
| 1 | Acquire write lock on schema |
| 2 | Create new empty schema |
| 3 | Iterate all rows, skip tombstone-deleted |
| 4 | Append active facts to new schema |
| 5 | Return new KnowledgeDatabase |

This operation is O(n) where n = total rows. Tombstone entries are eliminated.

### 5.4 WAL Evidence Field Handling

Evidence fields are intentionally not persisted in the WAL. On WAL replay, the evidence field defaults to `UNKNOWN` (value `0`). This aligns with PRD2.md §3.1, which specifies that evidence is a runtime-only field derived from the query context at insert time, not a durable property of the fact.

---

## 6. Metrics

14 atomic counters (lock-free):

| Metric | Type | Description |
|--------|------|-------------|
| queries_total | AtomicU64 | Total query count |
| queries_failed | AtomicU64 | Failed queries |
| query_duration_sum_ms | AtomicU64 | Cumulative query duration |
| inserts_total | AtomicU64 | Total insert count |
| inserts_failed | AtomicU64 | Failed inserts |
| cache_hits | AtomicU64 | Cache hit count |
| cache_misses | AtomicU64 | Cache miss count |
| memory_bytes | AtomicU64 | Memory usage estimate |
| inferences_total | AtomicU64 | Total inference operations |
| facts_inferred | AtomicU64 | Facts derived by inference |
| estimated_memory_bytes | AtomicU64 | Estimated memory footprint |
| total_facts | AtomicU64 | Total fact count |
| active_facts | AtomicU64 | Active (non-deleted) fact count |
| tombstone_count | AtomicU64 | Deleted row count |

### 6.1 Snapshot

```rust
struct MetricsSnapshot {
    queries_total, queries_failed, avg_query_latency_ms,
    inserts_total, inserts_failed, cache_hit_ratio,
    memory_bytes, inferences_total, facts_inferred,
    estimated_memory_bytes, total_facts, active_facts, tombstone_count,
}
```

---

## 7. Health Check

Threshold-based health determination (defaults: error_threshold=0.05, latency_threshold_ms=100.0, cache_hit_threshold=0.5):

| Status | Condition |
|--------|-----------|
| Unhealthy | error_rate > 5% (when inserts_total > 0) |
| Degraded | avg_query_latency_ms > 100ms OR cache_hit_ratio < 50% (when queries_total > 0) |
| Healthy | None of the above |

---

## 8. Constraints

| Constraint | Rationale |
|------------|-----------|
| WAL must be fsync'd on flush | Crash recovery guarantee |
| Schema capacity pre-allocated | Avoids reallocation |
| Maximum 100K audit events | Memory bound |
| Thread pool bounded by CPU count | Prevents thread explosion |

---

## 9. References

- **Depends on:** KCM_ARCHITECTURE (KCM_ARCHITECTURE)
- **Parent specs:** KCM_SPECIFICATION (KCM_SPECIFICATION)
- **Related:** KCM_DATA_MODEL_SPEC (KCM_DATA_MODEL_SPEC), KCM_API_SPEC (KCM_API_SPEC), KCM_SECURITY_TRUST_SPEC (KCM_SECURITY_TRUST_SPEC)
