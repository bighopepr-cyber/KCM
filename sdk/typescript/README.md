# KCM TypeScript SDK

TypeScript wrapper over the JavaScript SDK with full type definitions.

## Status: Planned

## Architecture

- Typed wrapper over `@kcm/js`
- Full TypeScript type definitions
- Package: `@kcm/ts` on npm

## API Design

```typescript
import { Database, Fact, QueryResult } from '@kcm/ts';

const db = new Database('my_knowledge.db');

const fact: Fact = {
  subject: 'planet',
  predicate: 'orbits',
  object: 'sun',
  confidence: 0.99
};

db.insert(fact);

const results: QueryResult = db.query("SELECT * FROM facts WHERE subject = 'planet'");
db.close();
```

## Installation

```bash
npm install @kcm/ts
```
