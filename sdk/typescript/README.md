# KCM TypeScript SDK

Official SDK for the KCM Knowledge Columnar Model.

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![Version](https://img.shields.io/badge/version-1.0.0-blue)]()
[![License](https://img.shields.io/badge/license-MIT-green)]()
[![KCM Engine Compatible](https://img.shields.io/badge/KCM%20Engine-1.0.0-orange)]()

## Installation

```bash
npm install @kcm/ts
```

## Quickstart

```typescript
import { Database, FactData } from '@kcm/ts';

const db = new Database();

const fact: FactData = {
    subject: 1,
    predicate: 0,
    object: 2,
    confidence: 0.95,
    evidence: 1,
    timestamp: Date.now(),
    context: 1,
    version: 1,
    priority: 0,
    owner: 1,
};

const rowId = db.insert(fact);
console.log(`Inserted row: ${rowId}`);

const results = db.queryAll();
console.log(`Total facts: ${db.factCount()}`);

db.save('my_database.json');
db.close();
```

## API Reference

### Database

| Operation | Signature | Description |
|-----------|-----------|-------------|
| `new Database()` | Constructor | Create in-memory database |
| `insert(fact)` | `insert(fact: FactData): number` | Insert fact, returns row ID |
| `update(rowId, fact)` | `update(rowId: number, fact: FactData): void` | Update fact by row ID |
| `delete(rowId)` | `delete(rowId: number): boolean` | Delete fact by row ID |
| `query(kql)` | `query(kql: string): QueryResult` | Execute KQL query |
| `queryAll()` | `queryAll(): FactData[]` | Get all active facts |
| `getFact(rowId)` | `getFact(rowId: number): FactData` | Retrieve a fact by ID |
| `factCount()` | `factCount(): number` | Total fact count |
| `activeFactCount()` | `activeFactCount(): number` | Active fact count |
| `beginTransaction()` | `beginTransaction(): Transaction` | Start transaction |
| `save(path)` | `save(path: string): void` | Save to file |
| `load(path)` | `load(path: string): void` | Load from file |
| `static verify(path)` | `Database.verify(path: string): void` | Verify file integrity |
| `close()` | `close(): void` | Close database |

### FactData

```typescript
interface FactData {
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
}
```

### QueryResult

| Operation | Signature | Description |
|-----------|-----------|-------------|
| `next()` | `next(): FactData \| undefined` | Iterate results |
| `collect()` | `collect(): FactData[]` | Collect all results |
| `count` | `get count(): number` | Result count |

### Transaction

| Operation | Signature | Description |
|-----------|-----------|-------------|
| `commit()` | `commit(): void` | Commit transaction |
| `rollback()` | `rollback(): void` | Rollback transaction |

## Error Handling

All operations throw `KcmError` with a `code` property:

| Error | Code | Description |
|-------|------|-------------|
| `NotFound` | 1001 | Resource not found |
| `OutOfMemory` | 1002 | Insufficient memory |
| `InvalidArgument` | 1003 | Invalid argument |
| `Io` | 1004 | I/O error |
| `Corrupted` | 1005 | Data corruption |
| `Conflict` | 1006 | Concurrent conflict |
| `TransactionAborted` | 1007 | Transaction aborted |

```typescript
try {
    db.insert({ subject: 1 } as FactData);
} catch (err) {
    if (err instanceof KcmError && err.code === ErrorCode.InvalidArgument) {
        console.error('Invalid fact: missing required fields');
    }
}
```

## Use Cases

### Basic Query

```typescript
import { Database } from '@kcm/ts';

const db = new Database();

db.insert({ subject: 1, predicate: 0, object: 2, confidence: 0.95 } as any);
db.insert({ subject: 2, predicate: 1, object: 3, confidence: 0.90 } as any);

const results = db.queryAll();
results.forEach(fact => {
    console.log(`Subject: ${fact.subject}, Object: ${fact.object}`);
});

db.close();
```

### API Integration

```typescript
import { Database, FactData } from '@kcm/ts';

function fetchKnowledge(dbPath: string): FactData[] {
    const db = new Database();
    db.load(dbPath);
    const facts = db.queryAll();
    db.close();
    return facts;
}
```

### Transaction

```typescript
import { Database } from '@kcm/ts';

const db = new Database();
const txn = db.beginTransaction();

db.insert({
    subject: 10, predicate: 0, object: 20, confidence: 0.85,
    evidence: 0, timestamp: 0, context: 0, version: 0, priority: 0, owner: 0,
});

if (db.activeFactCount() > 0) {
    txn.commit();
} else {
    txn.rollback();
}

db.close();
```

## Full Documentation

See [docs/sdk/typescript.md](../../docs/sdk/typescript.md) for the complete reference.

## Examples

See [examples/](examples/) for more examples including:
- `basic.ts` — Getting started
- `01_basic_crud.ts` — CRUD operations
- `02_transactions.ts` — Transaction management
- `03_persistence.ts` — Save/load databases
- `04_query_patterns.ts` — KQL query patterns
- `05_error_handling.ts` — Error handling patterns

## License

MIT

Made by bighopepr-cyber
