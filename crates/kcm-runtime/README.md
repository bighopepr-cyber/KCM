# kcm-runtime

Runtime and transaction layer for KCM. Provides `KnowledgeDatabase`, transactions, metrics, health checks, and executors.

## Purpose

Orchestrates storage, compute, and reasoning into a unified database interface with ACID transactions, monitoring, and async execution support.

## Modules

| Module | Purpose |
|--------|---------|
| `database` | `KnowledgeDatabase` — main entry point |
| `transaction` | ACID transactions with isolation levels |
| `executor` | Synchronous query executor |
| `async_executor` | Async query executor (tokio) |
| `metrics` | Performance metrics (11 atomic counters) |
| `health` | Health check endpoint logic |

## Dependencies

| Dependency | Purpose |
|------------|---------|
| `kcm-core` | Core types |
| `kcm-storage` | Persistent storage |
| `parking_lot` | Fast RwLock/Mutex |
| `rayon` | Work-stealing thread pool |
| `tokio` | Async runtime |

## Concurrency Model

| Component | Mechanism |
|-----------|-----------|
| Schema | `Arc<RwLock<Schema>>` (parking_lot) |
| Dictionaries | `Arc<RwLock<Dictionary>>` (parking_lot) |
| WAL | `Mutex<File>` (parking_lot) |
| Metrics | `AtomicU64` (11 counters, lock-free) |
| Thread Pool | rayon ThreadPool |
| Async | tokio Runtime |

## Transaction API

```rust
use kcm_runtime::database::KnowledgeDatabase;
use kcm_runtime::transaction::Transaction;

let db = KnowledgeDatabase::new()?;

// Read transaction
let txn = db.begin_transaction();
let facts = txn.read_facts()?;
txn.commit()?;

// Write transaction
let mut txn = db.begin_transaction();
txn.insert(fact)?;
txn.update(&old_fact, &new_fact)?;
txn.delete(&fact)?;
txn.commit()?;
```

## Metrics

11 atomic counters tracked:

| Metric | Description |
|--------|-------------|
| `facts_inserted` | Total facts inserted |
| `facts_deleted` | Total facts deleted |
| `facts_queried` | Total facts queried |
| `queries_executed` | Total queries executed |
| `transactions_started` | Total transactions started |
| `transactions_committed` | Total transactions committed |
| `transactions_aborted` | Total transactions aborted |
| `bytes_written` | Total bytes written |
| `bytes_read` | Total bytes read |
| `compression_ratio` | Average compression ratio |
| `cache_hit_ratio` | Buffer cache hit ratio |

## Benchmarks

```bash
cargo bench --bench micro -p kcm-runtime
```
