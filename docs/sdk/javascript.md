# KCM JavaScript SDK

Official SDK for the KCM Knowledge Columnar Model

<!-- badges:start -->
[![Build Status](https://img.shields.io/badge/build-pending-lightgrey)]()
[![Version](https://img.shields.io/badge/version-1.0.0-blue)]()
[![License](https://img.shields.io/badge/license-MIT-green)]()
<!-- badges:end -->

The JavaScript SDK wraps the KCM C FFI via Node.js native bindings.

## Installation

```bash
npm install @kcm/js
```

## Quickstart

```javascript
const { Database, Fact } = require('@kcm/js');

const db = new Database();

const fact = new Fact({
    subject: 1,
    predicate: 2,
    object: 3,
    confidence: 0.95,
    evidence: 1,
    context: 1,
    priority: 0,
    owner: 1,
});

db.insert(fact);

console.log('Total facts:', db.factCount());
console.log('Active facts:', db.activeCount());

db.close();
```

## API Reference

### `Fact`

Knowledge fact with 10 attributes.

```javascript
class Fact {
    constructor(options = {});

    subject;       // uint32
    predicate;     // uint8
    object;        // uint32
    confidence;    // double
    evidence;      // uint8
    timestamp;     // int64
    context;       // uint8
    version;       // int32
    priority;      // int8
    owner;         // uint16
}
```

All fields default to `0` / `0.0`.

### `Database`

Main entry point for KCM operations.

```javascript
class Database {
    constructor();

    insert(fact);
    update(rowId, fact);
    delete(rowId);

    factCount();
    activeCount();

    query(kql);
    beginTransaction();

    save(path);
    load(path);
    close();

    static verify(path);
}
```

#### Constructor

```javascript
const db = new Database();
```

Create a new in-memory database. Throws `KcmError` on failure.

#### `insert`

```javascript
db.insert(fact);
```

Insert a `Fact`. Throws `KcmError` on failure.

#### `update`

```javascript
db.update(rowId, fact);
```

Update a fact by row ID. Throws `KcmError` if not found.

#### `delete`

```javascript
db.delete(rowId);
```

Delete a fact by row ID. Throws `KcmError` if not found.

#### `factCount`

```javascript
db.factCount(); // -> number
```

Returns total fact count (including deleted).

#### `activeCount`

```javascript
db.activeCount(); // -> number
```

Returns active (non-deleted) fact count.

#### `query`

```javascript
db.query(kql); // -> QueryResult
```

Execute a KQL query. Returns a `QueryResult`.

#### `beginTransaction`

```javascript
db.beginTransaction(); // -> Transaction
```

Begin a new transaction.

#### `save`

```javascript
db.save(path);
```

Save the database to a file. Throws `KcmError` on I/O failure.

#### `load`

```javascript
db.load(path);
```

Load a database from a file. Throws `KcmError` on I/O or corruption.

#### `close`

```javascript
db.close();
```

Free the database and release all resources.

#### `verify` (static)

```javascript
Database.verify(path);
```

Verify database file integrity. Throws `KcmError` if corrupted.

### `QueryResult`

Query result iterator.

```javascript
class QueryResult {
    next();       // -> { done: boolean, value?: Fact }
    [Symbol.iterator]();
    toArray();    // -> Fact[]
    close();
}
```

#### `next`

```javascript
const result = db.query("SELECT * FROM facts");
const { done, value } = result.next();
if (!done) {
    console.log(value.subject);
}
```

#### `toArray`

```javascript
const facts = db.query("SELECT * FROM facts").toArray();
```

Collect all results into an array.

#### Iteration with `for...of`

```javascript
for (const fact of db.query("SELECT * FROM facts")) {
    console.log(fact.subject);
}
```

### `Transaction`

```javascript
class Transaction {
    commit();
    rollback();
    close();
}
```

#### `commit`

```javascript
txn.commit();
```

Commit the transaction. Throws `KcmError` on failure.

#### `rollback`

```javascript
txn.rollback();
```

Rollback the transaction.

### `KcmError`

```javascript
class KcmError extends Error {
    code;    // ErrorCode
}
```

### `ErrorCode`

```javascript
const ErrorCode = {
    OK: 0,
    NOT_FOUND: 1,
    OUT_OF_MEMORY: 2,
    INVALID_ARGUMENT: 3,
    IO: 4,
    CORRUPTED: 5,
    CONFLICT: 6,
    TRANSACTION_ABORTED: 7,
};
```

## Error Handling

All errors are reported via `KcmError`:

```javascript
try {
    const db = new Database();
    db.insert(fact);
} catch (e) {
    if (e instanceof KcmError) {
        console.error(`Error [${e.code}]: ${e.message}`);
    }
}
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

```javascript
const db = new Database();
const txn = db.beginTransaction();

try {
    db.insert(new Fact({ subject: 1, predicate: 2, object: 3, confidence: 0.9 }));
    txn.commit();
} catch (e) {
    txn.rollback();
    throw e;
} finally {
    db.close();
}
```

### Query with Iteration

```javascript
const db = new Database();

for (const fact of db.query("SELECT * FROM facts")) {
    console.log(`Subject=${fact.subject}, Object=${fact.object}, Confidence=${fact.confidence}`);
}

db.close();
```

### Save and Load

```javascript
let db = new Database();
db.insert(new Fact({ subject: 1, predicate: 2, object: 3, confidence: 0.95 }));
db.save("knowledge.kcm");
db.close();

db = new Database();
db.load("knowledge.kcm");
console.log(`Loaded ${db.factCount()} facts`);
db.close();
```

### Integrity Verification

```javascript
try {
    Database.verify("knowledge.kcm");
    console.log("Database is valid");
} catch (e) {
    console.error(`Corruption detected: ${e.message}`);
}
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
