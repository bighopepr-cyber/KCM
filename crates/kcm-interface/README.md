# kcm-interface

C FFI API, Python bindings, REST handlers, and KQL parser for KCM.

## Purpose

Provides language-agnostic interfaces to KCM: a C FFI layer (18 functions) for system integration, Python bindings via PyO3, REST API handlers, and a KQL query parser.

## Modules

| Module | Purpose |
|--------|---------|
| `lib.rs` | C FFI exports (18 functions) |
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

18 exported functions in `lib.rs`:

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

8 endpoints (no prefix, implemented in `rest_api.rs`, served by `kcm-server`):

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/health` | Health check |
| POST | `/facts` | Insert fact |
| GET | `/facts` | Query facts |
| GET | `/facts/{id}` | Get fact by ID |
| PUT | `/facts/{id}` | Update fact |
| DELETE | `/facts/{id}` | Delete fact |
| GET | `/stats` | Metrics JSON |
| GET | `/metrics` | Prometheus metrics |
