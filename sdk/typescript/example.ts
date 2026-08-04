import { Database } from '../../javascript/src/index';
import { Fact, QueryBuilder } from './src/index';

console.log("=== KCM TypeScript SDK Example ===\n");

const db = new Database();

// Type-safe fact creation
const fact1 = Fact.create(1, 0, 2, 0.95);
const fact2 = Fact.create(2, 1, 3, 0.90);
db.insert(fact1.toData());
db.insert(fact2.toData());
console.log(`Inserted 2 facts (count=${db.factCount()})`);

// Query builder
const query = QueryBuilder.create()
    .withSubject(1)
    .withConfidenceMin(0.9)
    .build();
const results = db.query(query);
console.log(`Query results: ${results.length}`);

// Stats
console.log(`Stats: ${JSON.stringify(db.stats())}`);

db.close();
console.log("Done!");
