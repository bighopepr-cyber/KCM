# KCM JavaScript SDK

JavaScript/Node.js bindings for KCM.

## Status: Planned

## Architecture

- N-API bindings to `kcm-interface` C FFI
- Package: `@kcm/js` on npm

## API Design

```javascript
const { Database } = require('@kcm/js');

// Open database
const db = new Database('my_knowledge.db');

// Insert fact
db.insert({
  subject: 'planet',
  predicate: 'orbits',
  object: 'sun',
  confidence: 0.99
});

// Query
const results = db.query("SELECT * FROM facts WHERE subject = 'planet'");
results.forEach(fact => console.log(fact.subject, fact.object));

// Close
db.close();
```

## Installation

```bash
npm install @kcm/js
```

## Examples

See `examples/javascript/` for complete examples.
