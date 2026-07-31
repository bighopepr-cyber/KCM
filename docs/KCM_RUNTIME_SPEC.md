# KCM Runtime Specification

**Document ID:** KCM-RUNTIME-001  
**Version:** 1.0.0  
**Depends on:** KCM_ARCHITECTURE-001

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
| WAL Mutex | std::sync::Mutex | WAL file write |
| WAL buffer Mutex | std::sync::Mutex | WAL write buffer |
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

---

## 6. Metrics

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

### 6.1 Snapshot

```rust
struct MetricsSnapshot {
    queries_total, queries_failed, avg_query_latency_ms,
    inserts_total, inserts_failed, cache_hit_ratio,
    memory_bytes, inferences_total, facts_inferred,
}
```

---

## 7. Health Check

| Status | Condition |
|--------|-----------|
| Healthy | error_rate < 5%, latency < threshold, cache_hit_ratio > 50% |
| Degraded | latency > threshold OR cache_hit_ratio < 50% |
| Unhealthy | error_rate > 5% |

---

## 8. Constraints

| Constraint | Rationale |
|------------|-----------|
| WAL must be fsync'd on flush | Crash recovery guarantee |
| Schema capacity pre-allocated | Avoids reallocation |
| Maximum 100K audit events | Memory bound |
| Thread pool bounded by CPU count | Prevents thread explosion |
