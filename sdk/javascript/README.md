# KCM JavaScript SDK

JavaScript/TypeScript bindings for the KCM Knowledge Columnar Model.

## Installation

```bash
npm install @kcm/js
```

## Quick Start

```typescript
import { Database } from '@kcm/js';

// Create a database
const db = new Database();

// Insert facts
db.insert({ subject: 1, predicate: 0, object: 2, confidence: 0.95 });
db.insert({ subject: 2, predicate: 1, object: 3, confidence: 0.90 });

// Query facts
const facts = db.queryAll();
console.log(facts);

// Check statistics
console.log(`Total facts: ${db.factCount()}`);

// Close database
db.close();
```

## API Reference

### Database

| Method | Description |
|--------|-------------|
| `Database(options?)` | Create a new database |
| `insert(fact)` | Insert a fact |
| `queryAll()` | Query all facts |
| `factCount()` | Get total fact count |
| `activeFactCount()` | Get active fact count |
| `close()` | Close the database |

### Fact

```typescript
interface Fact {
  subject: number;
  predicate: number;
  object: number;
  confidence: number;
}
```

## Development

```bash
# Install dependencies
npm install

# Build
npm run build

# Test
npm test
```

## License

MIT
