# kcm-interface

C FFI API, Python bindings, REST handlers, and KQL parser for KCM.

## Purpose

Provides language-agnostic interfaces to KCM: a C FFI layer (15 functions) for system integration, Python bindings via PyO3, REST API handlers, and a KQL query parser.

## Modules

| Module | Purpose |
|--------|---------|
| `lib.rs` | C FFI exports (15 functions) |
| `rest_api.rs` | REST API request/response handlers |
| `kql_parser.rs` | KQL (Knowledge Query Language) parser |
| `python.rs` | Python bindings via PyO3 |

## Dependencies

| Dependency | Purpose |
|------------|---------|
| `kcm-core` | Core types |
| `kcm-storage` | Storage access |
| `kcm-runtime` | Database operations |
| `parking_lot` | Thread-safe state |
| `serde` / `serde_json` | JSON serialization |
| `pyo3` | Python bindings (feature-gated) |

## Features

| Feature | Default | Description |
|---------|---------|-------------|
| `python` | No | Enable PyO3 Python bindings |

## C FFI API

```c
// 15 exported functions
void* kcm_database_open(const char* path);
void  kcm_database_close(void* db);
int   kcm_insert(void* db, const char* fact_json);
int   kcm_delete(void* db, const char* fact_json);
int   kcm_query(void* db, const char* query, char** result);
int   kcm_transaction_begin(void* db);
int   kcm_transaction_commit(void* txn);
int   kcm_transaction_abort(void* txn);
int   kcm_backup(void* db, const char* path);
int   kcm_restore(void* db, const char* path);
int   kcm_health_check(void* db);
int   kcm_metrics(void* db, char** result);
int   kcm_wal_checkpoint(void* db);
int   kcm_compact(void* db);
int   kcm_get_fact_count(void* db, uint64_t* count);
```

## Python Bindings

```python
import kcm

db = kcm.Database("/data/kcm.db")
db.insert({"subject": 1, "predicate": 1, "object": 1, "confidence": 0.95})
results = db.query("SELECT * FROM facts WHERE subject = 1")
db.close()
```

## KQL Syntax

```sql
SELECT subject, object FROM facts
WHERE subject = 1 AND confidence > 0.8
ORDER BY timestamp DESC
LIMIT 10
```

## REST API

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | /api/facts | Insert fact |
| DELETE | /api/facts | Delete fact |
| GET | /api/facts | Query facts |
| POST | /api/transactions | Begin transaction |
| POST | /api/transactions/:id/commit | Commit transaction |
| POST | /api/transactions/:id/abort | Abort transaction |
| GET | /api/health | Health check |
| GET | /api/metrics | Metrics |
| POST | /api/backup | Backup database |
| POST | /api/restore | Restore database |
