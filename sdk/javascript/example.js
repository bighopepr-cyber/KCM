#!/usr/bin/env node
"use strict";
const { Database } = require('./src/index.js');

console.log("=== KCM JavaScript SDK Example ===\n");

const db = new Database();

// Insert facts
db.insert({ subject: 1, predicate: 0, object: 2, confidence: 0.95 });
db.insert({ subject: 2, predicate: 1, object: 3, confidence: 0.90 });
db.insert({ subject: 3, predicate: 2, object: 4, confidence: 0.85 });
console.log(`Inserted 3 facts (count=${db.factCount()})`);

// Query all
const all = db.queryAll();
console.log(`\nQuery all (${all.length} results):`);
all.forEach(f => console.log(`  Subject=${f.subject} Predicate=${f.predicate} Object=${f.object} Confidence=${f.confidence.toFixed(2)}`));

// Query filter
const filtered = db.query({ subject: 1 });
console.log(`\nFiltered by subject=1: ${filtered.length} results`);

// Dictionary
const id1 = db.dictInsertSubject("planet");
const id2 = db.dictInsertSubject("star");
console.log(`\nDictionary: planet=${id1}, star=${id2}`);
console.log(`  Lookup 'planet': ${db.dictLookupSubject('planet')}`);
console.log(`  Get id ${id2}: ${db.dictGetSubject(id2)}`);

// Delete
const row = db.insert({ subject: 99, predicate: 9, object: 99, confidence: 0.5 });
console.log(`\nInserted row ${row}, count=${db.factCount()}, active=${db.activeFactCount()}`);
db.delete(row);
console.log(`After delete: count=${db.factCount()}, active=${db.activeFactCount()}`);

// Stats
const stats = db.stats();
console.log(`\nStats: facts=${stats.factCount}, active=${stats.activeCount}, memory=${stats.memoryBytes} bytes`);

// Close
db.close();
console.log("Database closed");

// Stress test
const db2 = new Database();
for (let i = 0; i < 10000; i++) {
    db2.insert({ subject: i % 1000, predicate: 0, object: i, confidence: 0.5 });
}
console.log(`Stress test: ${db2.factCount()} facts inserted`);
db2.close();

console.log("\nAll JavaScript SDK examples completed!");
