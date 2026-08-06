# KCM Python SDK

Official SDK for the KCM Knowledge Columnar Model.

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![Version](https://img.shields.io/badge/version-0.1.0-blue)]()
[![License](https://img.shields.io/badge/license-MIT-green)]()
[![KCM Engine Compatible](https://img.shields.io/badge/KCM%20Engine-0.1.0-orange)]()

## Installation

```bash
pip install kcm
```

## Quickstart

```python
import kcm

db = kcm.Database()

db.insert(subject=1, predicate=0, object=2, confidence=0.95)
db.insert(subject=2, predicate=1, object=3, confidence=0.90)

for fact in db.query_all():
    print(f"Subject: {fact[0]}, Object: {fact[2]}")

print(f"Fact count: {db.fact_count()}")
db.close()
```

## API Reference

### Database

| Operation | Signature | Description |
|-----------|-----------|-------------|
| `Database()` | Constructor | Create a new in-memory database |
| `Database(path)` | Constructor | Open or create a database at path |
| `insert(subject, predicate, object, confidence)` | `int` | Insert a fact, returns row ID |
| `update(row_id, subject, predicate, object, confidence)` | `None` | Update a fact |
| `delete(row_id)` | `None` | Delete a fact |
| `query(kql)` | `list[Fact]` | Execute KQL query |
| `query_all()` | `list[Fact]` | Get all active facts |
| `get_fact(row_id)` | `Fact` | Retrieve a fact by ID |
| `fact_count()` | `int` | Total fact count |
| `active_fact_count()` | `int` | Active fact count |
| `begin_transaction()` | `Transaction` | Begin transaction |
| `save(path)` | `None` | Save to file |
| `load(path)` | `None` | Load from file |
| `verify(path)` | `None` (static) | Verify file integrity |
| `close()` | `None` | Close database |

### Fact

Facts are returned as tuples: `(subject, predicate, object, confidence)`

## Error Handling

All operations raise `KcmError` on failure:

| Error | Code | Description |
|-------|------|-------------|
| `NotFound` | 1001 | Resource not found |
| `OutOfMemory` | 1002 | Insufficient memory |
| `InvalidArgument` | 1003 | Invalid argument |
| `Io` | 1004 | I/O error |
| `Corrupted` | 1005 | Data corruption |
| `Conflict` | 1006 | Concurrent conflict |
| `TransactionAborted` | 1007 | Transaction aborted |

```python
try:
    db.insert(subject=1)
except kcm.KcmError as e:
    print(f"Error {e.code}: {e}")
```

## Use Cases

### Basic Query

```python
import kcm

db = kcm.Database()

db.insert(subject=1, predicate=0, object=2, confidence=0.95)
db.insert(subject=2, predicate=1, object=3, confidence=0.90)

for subject, predicate, obj, confidence in db.query_all():
    print(f"Subject: {subject}, Object: {obj}")

db.close()
```

### API Integration

```python
import kcm

def fetch_knowledge(db_path):
    db = kcm.Database(db_path)
    facts = db.query_all()
    db.close()
    return facts
```

### Transaction

```python
import kcm

db = kcm.Database()
txn = db.begin_transaction()

db.insert(subject=10, predicate=0, object=20, confidence=0.85)

if db.active_fact_count() > 0:
    txn.commit()
else:
    txn.rollback()

db.close()
```

## Development

```bash
pip install maturin
maturin develop
pytest tests/
```

## Full Documentation

See [docs/sdk/python.md](../../docs/sdk/python.md) for the complete reference.

## Examples

See [examples/](examples/) for more examples including:
- `basic.py` — Getting started
- `01_basic_crud.py` — CRUD operations
- `02_transactions.py` — Transaction management
- `03_persistence.py` — Save/load databases
- `04_query_patterns.py` — KQL query patterns
- `05_error_handling.py` — Error handling patterns

## License

MIT

Made by bighopepr-cyber
