# KCM JavaScript SDK

Official SDK for the KCM Knowledge Columnar Model.

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![Version](https://img.shields.io/badge/version-1.0.0-blue)]()
[![License](https://img.shields.io/badge/license-MIT-green)]()
[![KCM Engine Compatible](https://img.shields.io/badge/KCM%20Engine-1.0.0-orange)]()

## Installation

```bash
npm install @kcm/js
```

## Quickstart

```javascript
const { Database } = require('@kcm/js');

const db = new Database();

db.insert({ subject: 1, predicate: 0, object: 2, confidence: 0.95 });
db.insert({ subject: 2, predicate: 1, object: 3, confidence: 0.90 });

const facts = db.queryAll();
facts.forEach(fact => {
    console.log(`Subject: ${fact.subject}, Object: ${fact.object}`);
});

console.log(`Total facts: ${db.factCount()}`);
db.close();
```

## API Reference

### Database

| Operation | Signature | Description |
|-----------|-----------|-------------|
| `Database(options?)` | Constructor | Create a new database |
| `insert(fact)` | `insert(fact: Fact): number` | Insert a fact, returns row ID |
| `update(rowId, fact)` | `update(rowId: number, fact: Fact): void` | Update a fact |
| `delete(rowId)` | `delete(rowId: number): boolean` | Delete a fact |
| `query(kql)` | `query(kql: string): QueryResult` | Execute KQL query |
| `queryAll()` | `queryAll(): Fact[]` | Get all active facts |
| `getFact(rowId)` | `getFact(rowId: number): Fact` | Retrieve a fact by ID |
| `factCount()` | `factCount(): number` | Total fact count |
| `activeFactCount()` | `activeFactCount(): number` | Active fact count |
| `beginTransaction()` | `beginTransaction(): Transaction` | Begin transaction |
| `save(path)` | `save(path: string): void` | Save to file |
| `load(path)` | `load(path: string): void` | Load from file |
| `static verify(path)` | `Database.verify(path: string): void` | Verify file integrity |
| `close()` | `close(): void` | Close database |

### Fact

```javascript
{
  subject: number,
  predicate: number,
  object: number,
  confidence: number,
  evidence?: number,
  timestamp?: number,
  context?: number,
  version?: number,
  priority?: number,
  owner?: number,
}
```

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

```javascript
try {
    db.insert({ subject: 1 });
} catch (err) {
    if (err.code === 1003) {
        console.error('Invalid fact: missing required fields');
    }
}
```

## Use Cases

### Basic Query

```javascript
const { Database } = require('@kcm/js');

const db = new Database();
db.insert({ subject: 1, predicate: 0, object: 2, confidence: 0.95 });
db.insert({ subject: 2, predicate: 1, object: 3, confidence: 0.90 });

const facts = db.queryAll();
facts.forEach(fact => {
    console.log(`Subject: ${fact.subject}, Object: ${fact.object}`);
});

db.close();
```

### API Integration

```javascript
const { Database } = require('@kcm/js');

function fetchKnowledge(dbPath) {
    const db = new Database();
    db.load(dbPath);
    const facts = db.queryAll();
    db.close();
    return facts;
}
```

### Transaction

```javascript
const { Database } = require('@kcm/js');

const db = new Database();
const txn = db.beginTransaction();

db.insert({ subject: 10, predicate: 0, object: 20, confidence: 0.85 });

if (db.activeFactCount() > 0) {
    txn.commit();
} else {
    txn.rollback();
}

db.close();
```

## Development

```bash
npm install
npm run build
npm test
```

## Full Documentation

See [docs/sdk/javascript.md](../../docs/sdk/javascript.md) for the complete reference.

## Examples

See [examples/](examples/) for more examples including:
- `basic.js` — Getting started
- `01_basic_crud.js` — CRUD operations
- `02_transactions.js` — Transaction management
- `03_persistence.js` — Save/load databases
- `04_query_patterns.js` — KQL query patterns
- `05_error_handling.js` — Error handling patterns

## License

MIT

Made by bighopepr-cyber
