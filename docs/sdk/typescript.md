# KCM TypeScript SDK

Official SDK for the KCM Knowledge Columnar Model

<!-- badges:start -->
[![Build Status](https://img.shields.io/badge/build-pending-lightgrey)]()
[![Version](https://img.shields.io/badge/version-1.0.0-blue)]()
[![License](https://img.shields.io/badge/license-MIT-green)]()
<!-- badges:end -->

The TypeScript SDK wraps the KCM C FFI via Node.js native bindings with full type definitions.

## Installation

```bash
npm install @kcm/ts
```

## Quickstart

```typescript
import { Database, Fact } from '@kcm/ts';

const db = new Database();

const fact: Fact = {
    subject: 1,
    predicate: 2,
    object: 3,
    confidence: 0.95,
    evidence: 1,
    context: 1,
    priority: 0,
    owner: 1,
};

db.insert(fact);

console.log('Total facts:', db.factCount());
console.log('Active facts:', db.activeCount());

db.close();
```

## API Reference

### `FactData`

Knowledge fact with 10 attributes.

```typescript
interface FactData {
    subject: number;       // uint32
    predicate: number;     // uint8
    object: number;        // uint32
    confidence: number;    // double
    evidence: number;      // uint8
    timestamp: number;     // int64
    context: number;       // uint8
    version: number;       // int32
    priority: number;      // int8
    owner: number;         // uint16
}
```

### `Fact`

Concrete fact class implementing `FactData`.

```typescript
class Fact implements FactData {
    subject: number;
    predicate: number;
    object: number;
    confidence: number;
    evidence: number;
    timestamp: number;
    context: number;
    version: number;
    priority: number;
    owner: number;

    constructor(data?: Partial<FactData>);
}
```

All fields default to `0` / `0.0`.

### `Database`

Main entry point for KCM operations.

```typescript
class Database {
    constructor();

    insert(fact: FactData): number;
    update(rowId: number, fact: FactData): void;
    delete(rowId: number): void;

    factCount(): number;
    activeCount(): number;

    query(kql: string): QueryResult;
    beginTransaction(): Transaction;

    save(path: string): void;
    load(path: string): void;
    close(): void;

    static verify(path: string): void;
}
```

#### Constructor

```typescript
const db = new Database();
```

Create a new in-memory database. Throws `KcmError` on failure.

#### `insert`

```typescript
const rowId = db.insert(fact);
```

Insert a fact. Returns the assigned row ID.

#### `update`

```typescript
db.update(rowId, fact);
```

Update a fact by row ID. Throws `KcmError` if not found.

#### `delete`

```typescript
db.delete(rowId);
```

Delete a fact by row ID. Throws `KcmError` if not found.

#### `factCount`

```typescript
db.factCount(); // -> number
```

Returns total fact count (including deleted).

#### `activeCount`

```typescript
db.activeCount(); // -> number
```

Returns active (non-deleted) fact count.

#### `query`

```typescript
db.query(kql); // -> QueryResult
```

Execute a KQL query.

#### `beginTransaction`

```typescript
db.beginTransaction(); // -> Transaction
```

Begin a new transaction.

#### `save`

```typescript
db.save(path);
```

Save the database to a file.

#### `load`

```typescript
db.load(path);
```

Load a database from a file.

#### `close`

```typescript
db.close();
```

Free the database and release all resources.

#### `verify` (static)

```typescript
Database.verify(path);
```

Verify database file integrity.

### `QueryResult`

Query result iterator with full iteration protocol.

```typescript
class QueryResult implements Iterable<Fact> {
    next(): IteratorResult<Fact>;
    [Symbol.iterator](): IterableIterator<Fact>;
    toArray(): Fact[];
    close(): void;
}
```

### `Transaction`

```typescript
class Transaction {
    commit(): void;
    rollback(): void;
    close(): void;
}
```

#### `commit`

```typescript
txn.commit();
```

Commit the transaction.

#### `rollback`

```typescript
txn.rollback();
```

Rollback the transaction.

### `KcmError`

```typescript
class KcmError extends Error {
    code: ErrorCode;
}
```

### `ErrorCode`

```typescript
const ErrorCode = {
    OK: 0,
    NOT_FOUND: 1,
    OUT_OF_MEMORY: 2,
    INVALID_ARGUMENT: 3,
    IO: 4,
    CORRUPTED: 5,
    CONFLICT: 6,
    TRANSACTION_ABORTED: 7,
} as const;

type ErrorCode = typeof ErrorCode[keyof typeof ErrorCode];
```

## Error Handling

All errors are reported via `KcmError`:

```typescript
import { Database, KcmError, ErrorCode } from '@kcm/ts';

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

```typescript
import { Database, Fact } from '@kcm/ts';

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

```typescript
const db = new Database();

for (const fact of db.query("SELECT * FROM facts")) {
    console.log(`Subject=${fact.subject}, Object=${fact.object}, Confidence=${fact.confidence}`);
}

db.close();
```

### Collect to Array

```typescript
const db = new Database();
const facts = db.query("SELECT * FROM facts").toArray();
console.log(`Found ${facts.length} facts`);
db.close();
```

### Save and Load

```typescript
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

```typescript
import { Database, KcmError } from '@kcm/ts';

try {
    Database.verify("knowledge.kcm");
    console.log("Database is valid");
} catch (e) {
    if (e instanceof KcmError) {
        console.error(`Corruption detected: ${e.message}`);
    }
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
