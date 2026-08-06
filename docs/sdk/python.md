# KCM Python SDK

Official SDK for the KCM Knowledge Columnar Model

<!-- badges:start -->
[![Build Status](https://img.shields.io/badge/build-pending-lightgrey)]()
[![Version](https://img.shields.io/badge/version-1.0.0-blue)]()
[![License](https://img.shields.io/badge/license-MIT-green)]()
<!-- badges:end -->

The Python SDK wraps the KCM C FFI via PyO3, providing a Pythonic API.

## Installation

```bash
pip install kcm
```

### Requirements

- Python 3.8+
- Linux x86_64 / macOS arm64 / Windows x64

## Quickstart

```python
import kcm

db = kcm.Database()

fact = kcm.Fact(
    subject=1,
    predicate=2,
    object=3,
    confidence=0.95,
    evidence=1,
    context=1,
    priority=0,
    owner=1,
)

db.insert(fact)

print(f"Total facts: {db.fact_count()}")
print(f"Active facts: {db.active_count()}")

db.close()
```

## API Reference

### `Fact`

Knowledge fact with 10 attributes.

```python
class Fact:
    subject: int       # uint32
    predicate: int     # uint8
    object: int        # uint32
    confidence: float  # double
    evidence: int      # uint8
    timestamp: int     # int64
    context: int       # uint8
    version: int       # int32
    priority: int      # int8
    owner: int         # uint16
```

All fields default to `0` / `0.0`.

### `Database`

Main entry point for KCM operations. Supports context manager protocol.

```python
class Database:
    def __init__(self):
        """Create a new in-memory database."""

    def close(self):
        """Free the database and release all resources."""

    def __enter__(self):
        """Context manager entry."""

    def __exit__(self, *args):
        """Context manager exit (calls close)."""

    def insert(self, fact: Fact) -> int:
        """Insert a fact. Returns the assigned row ID."""

    def update(self, row_id: int, fact: Fact):
        """Update a fact by row ID."""

    def delete(self, row_id: int):
        """Delete a fact by row ID."""

    def fact_count(self) -> int:
        """Get total fact count (including deleted)."""

    def active_count(self) -> int:
        """Get active (non-deleted) fact count."""

    def query(self, kql: str) -> QueryResult:
        """Execute a KQL query string."""

    def begin_transaction(self) -> Transaction:
        """Begin a new transaction."""

    def save(self, path: str):
        """Save the database to a file."""

    def load(self, path: str):
        """Load a database from a file."""

    @staticmethod
    def verify(path: str):
        """Verify database file integrity."""
```

#### Constructor

```python
db = kcm.Database()
```

Create a new in-memory database. Raises `KcmError` on failure.

#### Context Manager

```python
with kcm.Database() as db:
    db.insert(fact)
```

Database is automatically closed when exiting the context.

#### `insert`

```python
row_id = db.insert(fact)
```

Insert a fact. Returns the assigned row ID.

#### `update`

```python
db.update(row_id, fact)
```

Update a fact by row ID. Raises `KcmError` with `ErrorCode.NOT_FOUND` if not found.

#### `delete`

```python
db.delete(row_id)
```

Delete a fact by row ID. Raises `KcmError` with `ErrorCode.NOT_FOUND` if not found.

#### `fact_count`

```python
count = db.fact_count()  # -> int
```

Returns total fact count (including deleted).

#### `active_count`

```python
count = db.active_count()  # -> int
```

Returns active (non-deleted) fact count.

#### `query`

```python
results = db.query("SELECT * FROM facts")  # -> QueryResult
```

Execute a KQL query.

#### `begin_transaction`

```python
txn = db.begin_transaction()  # -> Transaction
```

Begin a new transaction.

#### `save`

```python
db.save("knowledge.kcm")
```

Save the database to a file.

#### `load`

```python
db.load("knowledge.kcm")
```

Load a database from a file.

#### `verify`

```python
kcm.Database.verify("knowledge.kcm")
```

Verify database file integrity.

### `QueryResult`

Query result iterator. Supports iteration protocol.

```python
class QueryResult:
    def __iter__(self):
        """Iterate over facts."""

    def __next__(self) -> Fact:
        """Get next fact."""

    def next(self) -> Fact | None:
        """Get next fact, or None if exhausted."""

    def collect(self) -> list[Fact]:
        """Collect all remaining facts into a list."""
```

### `Transaction`

```python
class Transaction:
    def commit(self):
        """Commit the transaction."""

    def rollback(self):
        """Rollback the transaction."""
```

### `KcmError`

```python
class KcmError(Exception):
    code: ErrorCode
```

### `ErrorCode`

```python
class ErrorCode:
    OK = 0
    NOT_FOUND = 1
    OUT_OF_MEMORY = 2
    INVALID_ARGUMENT = 3
    IO = 4
    CORRUPTED = 5
    CONFLICT = 6
    TRANSACTION_ABORTED = 7
```

## Error Handling

All errors are reported via `KcmError`:

```python
import kcm

try:
    db = kcm.Database()
    db.insert(fact)
except kcm.KcmError as e:
    print(f"Error [{e.code}]: {e}")
```

| `ErrorCode` | Description |
|-------------|-------------|
| `NOT_FOUND` | Requested row ID not found |
| `OUT_OF_MEMORY` | Memory allocation failed |
| `INVALID_ARGUMENT` | Invalid argument |
| `IO` | I/O error |
| `CORRUPTED` | Data corruption detected |
| `CONFLICT` | Conflict (e.g., duplicate key) |
| `TRANSACTION_ABORTED` | Transaction was aborted |

## Example Code

### Transactions

```python
with kcm.Database() as db:
    txn = db.begin_transaction()
    try:
        fact = kcm.Fact(subject=1, predicate=2, object=3, confidence=0.9)
        db.insert(fact)
        txn.commit()
    except Exception:
        txn.rollback()
        raise
```

### Query Iteration

```python
with kcm.Database() as db:
    for fact in db.query("SELECT * FROM facts"):
        print(f"Subject={fact.subject}, Object={fact.object}, Confidence={fact.confidence}")
```

### Collect All Results

```python
with kcm.Database() as db:
    facts = db.query("SELECT * FROM facts").collect()
    print(f"Found {len(facts)} facts")
```

### Save and Load

```python
with kcm.Database() as db:
    db.insert(kcm.Fact(subject=1, predicate=2, object=3, confidence=0.95))
    db.save("knowledge.kcm")

with kcm.Database() as db:
    db.load("knowledge.kcm")
    print(f"Loaded {db.fact_count()} facts")
```

### Integrity Verification

```python
try:
    kcm.Database.verify("knowledge.kcm")
    print("Database is valid")
except kcm.KcmError as e:
    print(f"Corruption detected: {e}")
```

## Benchmark

Build and run the benchmark suite:

```bash
cargo bench --workspace
```

| Metric | Target |
|--------|--------|
| Insert (1M facts) | < 2s |
| Query (100K results) | < 50ms |
| Save/Load (1M facts) | < 5s |
| Memory (1M facts) | < 512MB |

Results are published with each release. See `docs/PRD-TESTING& BRACHMARCK.md` for methodology.
