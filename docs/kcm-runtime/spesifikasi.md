# kcm-runtime Technical Specification

## Overview

`kcm-runtime` is the runtime and transaction layer for the KCM (Knowledge Columnar Model) system. It provides the primary entry point (`KnowledgeDatabase`), ACID transaction management, synchronous and asynchronous executors, metrics collection, and health monitoring. This crate orchestrates interactions between `kcm-core` types, `kcm-storage` persistence, and `kcm-optimizer` query planning.

## Scope

This document specifies the internal architecture, data models, execution flows, public API, configuration, dependencies, error handling, performance characteristics, security considerations, and integration points of the `kcm-runtime` crate.

## Responsibilities

| Responsibility | Description |
|---|---|
| KnowledgeDatabase lifecycle | Open, close, and manage the lifecycle of a knowledge database instance |
| ACID transactions | Provide begin/commit/rollback semantics with snapshot isolation |
| Sync executor | Execute parallel operations using rayon thread pool |
| Async executor | Execute async operations using tokio runtime |
| Metrics collection | Track 14 operational counters atomically |
| Health monitoring | Compute health status from error rate, latency, and cache hit ratio |

## Technical Specification

### KnowledgeDatabase

The `KnowledgeDatabase` is the main entry point for the runtime. It wraps `kcm-storage` for persistence and `kcm-optimizer` for query planning.

- Constructed with a storage path and configuration
- Manages schema, dictionaries, and WAL through `kcm-storage`
- Delegates query optimization to `kcm-optimizer`
- Provides thread-safe access via `Arc<RwLock<...>>` (parking_lot)
- Supports concurrent reads and exclusive writes

### Transaction

Transactions provide ACID guarantees:

- **Atomicity**: Changes are applied entirely on commit or not at all on rollback
- **Consistency**: Transaction state machine enforces valid transitions
- **Isolation**: Snapshot isolation via version tracking
- **Durability**: Committed changes are durable via WAL

State machine: `Idle → Active → Committed` or `Active → RolledBack`

- `begin()`: Transitions to `Active`; creates a snapshot view
- `commit()`: Validates all changes, applies atomically, transitions to `Committed`
- `rollback()`: Discards all changes, transitions to `RolledBack`
- Internal state protected by `parking_lot::Mutex`

### Metrics

14 `AtomicU64` counters tracking operational health:

| Counter | Description |
|---|---|
| `queries_total` | Total query operations |
| `inserts_total` | Total insert operations |
| `updates_total` | Total update operations |
| `deletes_total` | Total delete operations |
| `cache_hits` | Cache hit count |
| `cache_misses` | Cache miss count |
| `transaction_commits` | Successful transaction commits |
| `transaction_rollbacks` | Transaction rollbacks |
| `transaction_aborts` | Transaction aborts |
| `inference_count` | Inference operations executed |
| `facts_inferred` | Facts produced by inference |
| `rule_executions` | Rule execution count |
| `error_count` | Total errors encountered |
| `active_transactions` | Currently active transactions |

All counters are lock-free via `AtomicU64`. Snapshots return a consistent read of all 14 counters.

### Health

Health status is computed from live metrics:

- **Healthy**: error_rate < 5%, P99 latency < 100ms, cache_hit_ratio > 50%
- **Degraded**: P99 latency > 100ms OR cache_hit_ratio < 50%
- **Unhealthy**: error_rate > 5%

Status is computed on demand, not cached. This ensures health checks reflect current system state.

### Executor

The synchronous executor wraps a `rayon` thread pool for parallel operations:

- Configurable thread count (defaults to number of CPU cores)
- Supports map-reduce style parallel iteration
- Integrates with `kcm-compute` for parallel relational algebra operations
- Bounded by the rayon global or custom thread pool

### AsyncExecutor

The asynchronous executor wraps a `tokio` runtime for async operations:

- Configurable worker thread count
- Supports structured concurrency via `tokio::spawn`
- Used for I/O-bound operations (WAL writes, backup, recovery)
- Integrates with the sync executor for hybrid workloads

## Architecture

```
┌─────────────────────────────────────────────────┐
│                  kcm-runtime                     │
│                                                  │
│  ┌──────────────┐  ┌──────────────────────────┐ │
│  │ KnowledgeDB  │  │       Transaction        │ │
│  │  (database)  │  │  (transaction)           │ │
│  └──────┬───────┘  └────────────┬─────────────┘ │
│         │                       │                │
│  ┌──────┴───────┐  ┌────────────┴─────────────┐ │
│  │   Executor   │  │     AsyncExecutor        │ │
│  │   (rayon)    │  │      (tokio)             │ │
│  └──────────────┘  └──────────────────────────┘ │
│                                                  │
│  ┌──────────────┐  ┌──────────────────────────┐ │
│  │   Metrics    │  │       Health             │ │
│  │ (AtomicU64)  │  │  (live computation)      │ │
│  └──────────────┘  └──────────────────────────┘ │
└─────────────────────────────────────────────────┘
         │                       │
         ▼                       ▼
┌────────────────┐  ┌────────────────┐
│  kcm-storage   │  │  kcm-optimizer │
│  (persistence) │  │  (query plan)  │
└────────────────┘  └────────────────┘
         │
         ▼
┌────────────────┐
│   kcm-core     │
│   (types)      │
└────────────────┘
```

## Internal Components

### database.rs

- `KnowledgeDatabase` struct: primary entry point
- Storage initialization and lifecycle management
- Query delegation to optimizer
- Insert/update/delete operations with metric tracking
- Schema and dictionary access through storage layer

### transaction.rs

- `Transaction` struct: ACID transaction implementation
- `TransactionState` enum: `Idle`, `Active`, `Committed`, `RolledBack`
- State machine transitions with mutex protection
- Snapshot creation on `begin()`
- Atomic commit and rollback logic

### executor.rs

- `Executor` struct: synchronous parallel execution
- Rayon thread pool management
- Parallel iteration primitives
- Integration with compute operators

### async_executor.rs

- `AsyncExecutor` struct: asynchronous execution
- Tokio runtime management
- Async task spawning and joining
- Structured concurrency support

### metrics.rs

- `Metrics` struct: 14 atomic counters
- Increment/query/reset operations
- Snapshot creation (consistent read of all counters)
- Health-relevant metric derivation

### health.rs

- `HealthStatus` enum: `Healthy`, `Degraded`, `Unhealthy`
- Status computation from live metrics
- Threshold configuration
- No caching; always computes from current state

## Data Model

### Metrics Struct

```rust
pub struct Metrics {
    queries_total: AtomicU64,
    inserts_total: AtomicU64,
    updates_total: AtomicU64,
    deletes_total: AtomicU64,
    cache_hits: AtomicU64,
    cache_misses: AtomicU64,
    transaction_commits: AtomicU64,
    transaction_rollbacks: AtomicU64,
    transaction_aborts: AtomicU64,
    inference_count: AtomicU64,
    facts_inferred: AtomicU64,
    rule_executions: AtomicU64,
    error_count: AtomicU64,
    active_transactions: AtomicU64,
}
```

### HealthStatus Enum

```rust
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}
```

### TransactionState

```rust
pub enum TransactionState {
    Idle,
    Active,
    Committed,
    RolledBack,
}
```

## Execution Flow

### Insert Flow

1. Client calls `KnowledgeDatabase::insert(fact)`
2. Permission check via `kcm-security` (Write permission)
3. Fact validated against schema
4. Storage layer encodes and writes to WAL
5. Metric `inserts_total` incremented atomically
6. `Result<RowID, KcmError>` returned

### Query Flow

1. Client calls `KnowledgeDatabase::query(plan)`
2. Permission check via `kcm-security` (Read permission)
3. Optimizer rewrites and optimizes the plan
4. Executor runs the plan via rayon thread pool
5. Results collected and returned
6. Metric `queries_total` incremented atomically
7. Cache metrics updated on hit/miss

### Transaction Commit Flow

1. Client calls `transaction.commit()`
2. State validated: must be `Active`
3. All pending changes collected from transaction buffer
4. Changes applied atomically to storage
5. WAL flushed for durability
6. State transitions to `Committed`
7. Metric `transaction_commits` incremented atomically
8. `Result<(), KcmError>` returned

## Public API

| Function | Signature | Description |
|---|---|---|
| `KnowledgeDatabase::open` | `fn open(path: &Path) -> Result<Self, KcmError>` | Open or create a database |
| `KnowledgeDatabase::insert` | `fn insert(&self, fact: Fact) -> Result<RowID, KcmError>` | Insert a fact |
| `KnowledgeDatabase::query` | `fn query(&self, plan: PlanNode) -> Result<Vec<Fact>, KcmError>` | Execute a query |
| `KnowledgeDatabase::begin_transaction` | `fn begin_transaction(&self) -> Result<Transaction, KcmError>` | Start a transaction |
| `Transaction::commit` | `fn commit(self) -> Result<(), KcmError>` | Commit the transaction |
| `Transaction::rollback` | `fn rollback(self) -> Result<(), KcmError>` | Rollback the transaction |
| `Metrics::snapshot` | `fn snapshot(&self) -> MetricsSnapshot` | Get consistent metrics snapshot |
| `Health::status` | `fn status(&self, metrics: &Metrics) -> HealthStatus` | Compute current health |
| `Executor::new` | `fn new(threads: usize) -> Self` | Create sync executor |
| `AsyncExecutor::new` | `fn new(workers: usize) -> Result<Self, KcmError>` | Create async executor |

## Configuration

| Parameter | Default | Description |
|---|---|---|
| `storage_path` | (required) | Path to database files |
| `executor_threads` | CPU count | Number of rayon worker threads |
| `async_workers` | CPU count | Number of tokio worker threads |
| `health_error_threshold` | 0.05 | Error rate threshold for unhealthy |
| `health_latency_threshold_ms` | 100 | P99 latency threshold for degraded |
| `health_cache_threshold` | 0.50 | Cache hit ratio threshold for degraded |

## Dependencies

| Dependency | Justification | Could Remove? |
|---|---|---|
| `kcm-core` | Types, DenseVec, Bitmap, Dictionary | No |
| `kcm-storage` | Columns, Codecs, WAL, FileFormat, Index | No |
| `kcm-optimizer` | Query planning, cost model, statistics | No |
| `parking_lot` | 3-5x faster RwLock/Mutex than std | Yes, measurable perf regression |
| `rayon` | Work-stealing parallel iterator library | Yes, manual threads (loses work-stealing) |
| `tokio` | Async runtime for I/O-bound operations | No |
| `log` | Structured logging | Yes, custom macros |
| `thiserror` | Error derive macro | Yes, manual impl |

## Error Handling

All public APIs return `Result<T, KcmError>`. The error hierarchy:

```
KcmError
├── NotFound(String)
├── OutOfMemory
├── InvalidArgument(String)
├── Io(String)
├── Corrupted(String)
├── Conflict(String)
└── TransactionAborted
```

- `NotFound`: Requested entity does not exist
- `OutOfMemory`: System cannot allocate required memory
- `InvalidArgument`: Invalid parameter or configuration
- `Io`: Underlying I/O failure
- `Corrupted`: Data integrity violation
- `Conflict`: Concurrent modification conflict
- `TransactionAborted`: Transaction was aborted due to conflict or error

No `unwrap()` or `panic!()` in production code paths. All errors propagate through the `Result` type.

## Performance Characteristics

| Operation | Target | Measurement |
|---|---|---|
| Insert throughput | > 50,000 facts/sec | Criterion benchmark, single thread |
| Query latency P99 | < 100ms | Criterion benchmark, 10K fact dataset |
| Transaction commit P99 | < 10ms | Criterion benchmark |
| Metric increment | < 100ns | Criterion benchmark |
| Health status computation | < 1ms | Criterion benchmark |
| Parallel insert (8 threads) | > 200,000 facts/sec | Criterion benchmark |
| Memory per 1M facts | < 500MB | Profiling benchmark |

## Security Considerations

- All data mutations require `Write` permission via `kcm-security` RBAC
- All data reads require `Read` permission via `kcm-security` RBAC
- Transaction state is mutex-protected against concurrent modification
- Metric counters use `AtomicU64` to prevent data races
- Health checks compute from live data to prevent stale status
- No secrets stored or logged by the runtime
- Audit logging for all security-relevant events

## Integration

### kcm-core

- Uses `Fact`, `RowID`, `SubjectID`, `Confidence` types
- Uses `DenseVec`, `Bitmap`, `Dictionary` for internal data structures
- Uses `KcmError` for error propagation

### kcm-storage

- Delegates persistence to storage layer
- Uses WAL for transaction durability
- Uses column encoding and compression for storage efficiency
- Uses index structures for query acceleration

### kcm-optimizer

- Delegates query optimization to optimizer
- Uses cost model for plan selection
- Uses statistics for cardinality estimation

### kcm-security

- Permission checks for all data operations
- Audit logging for security events
- RBAC integration for access control

## Sequence Diagram

### Transaction Lifecycle

```
Client          KnowledgeDatabase     Transaction      Storage        Metrics
  │                    │                   │              │              │
  │── begin_txn() ────>│                   │              │              │
  │                    │── create txn ─────>│              │              │
  │                    │                   │── snapshot ──>│              │
  │<─ Transaction ─────│<── return txn ────│              │              │
  │                    │                   │              │              │
  │── txn.insert(f) ──>│── buffer change ─>│              │              │
  │── txn.insert(g) ──>│── buffer change ─>│              │              │
  │                    │                   │              │              │
  │── txn.commit() ───>│── validate state >│              │              │
  │                    │── apply atomically ─────────────>│              │
  │                    │── flush WAL ────────────────────>│              │
  │                    │── update state ───>│              │              │
  │                    │──────────────────────────────────────────────>│
  │<─ Ok(()) ─────────│<── committed ──────│              │              │
```

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                         kcm-runtime                             │
│                                                                  │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │                   KnowledgeDatabase                      │    │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────────────────┐  │    │
│  │  │ Schema   │  │ Dicts    │  │ WAL                  │  │    │
│  │  │ (RwLock) │  │ (RwLock) │  │ (Mutex)              │  │    │
│  │  └──────────┘  └──────────┘  └──────────────────────┘  │    │
│  └───────────────────────┬─────────────────────────────────┘    │
│                          │                                       │
│  ┌───────────────┐  ┌────┴──────────┐  ┌──────────────────┐    │
│  │  Transaction  │  │   Executor    │  │  AsyncExecutor   │    │
│  │  (Mutex state)│  │  (rayon pool) │  │  (tokio runtime) │    │
│  └───────────────┘  └───────────────┘  └──────────────────┘    │
│                                                                  │
│  ┌───────────────┐  ┌───────────────┐                           │
│  │   Metrics     │  │    Health     │                           │
│  │ (14 AtomicU64)│  │ (live calc)  │                           │
│  └───────────────┘  └───────────────┘                           │
└─────────────────────────────────────────────────────────────────┘
```

## References

- `docs/PRD2.md §18` — Runtime specification (SSOT)
- `docs/PRD.md §3` — Core types (SSOT)
- `docs/PRD.md §5` — Query engine (SSOT)
- `AGENTS.md` — Engineering constitution
- `SSOT.md` — Single Source of Truth index
- `kcm-core` crate — Types and data structures
- `kcm-storage` crate — Persistence layer
- `kcm-optimizer` crate — Query optimization
- `kcm-security` crate — RBAC and encryption

## SSOT Alignment

This specification aligns with the following SSOT documents:

| SSOT Document | Section | Alignment |
|---|---|---|
| `docs/PRD2.md` | §18 | Runtime architecture, transaction model, metrics, health |
| `docs/PRD.md` | §3 | Core types used by runtime |
| `docs/PRD.md` | §5 | Query execution through executor |
| `AGENTS.md` | Engineering Gates | All 6 gates apply |
| `AGENTS.md` | Non-Negotiable Rules | Rules 1-12 enforced |

Any deviation from SSOT specifications requires an approved SSOT update before implementation.
