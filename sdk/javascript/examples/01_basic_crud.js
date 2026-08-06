#!/usr/bin/env node
"use strict";

/**
 * KCM JavaScript SDK — Basic CRUD Example.
 *
 * Demonstrates: insert, query, update, delete operations on facts.
 */

const { Database, KcmError, ErrorCode } = require('../src/index.js');

console.log("=== KCM JavaScript SDK — Basic CRUD Example ===\n");

const db = new Database();

// --- INSERT ---
console.log("--- Insert Facts ---");
const row0 = db.insert({
    subject: 1, predicate: 0, object: 2, confidence: 0.95,
    evidence: 0, timestamp: 0, context: 0, version: 1, priority: 0, owner: 0,
});
const row1 = db.insert({
    subject: 2, predicate: 1, object: 3, confidence: 0.90,
    evidence: 1, timestamp: 0, context: 1, version: 1, priority: 0, owner: 1,
});
const row2 = db.insert({
    subject: 3, predicate: 2, object: 4, confidence: 0.85,
    evidence: 2, timestamp: 0, context: 2, version: 1, priority: 0, owner: 2,
});
const row3 = db.insert({
    subject: 1, predicate: 3, object: 5, confidence: 0.80,
    evidence: 3, timestamp: 0, context: 2, version: 1, priority: -1, owner: 7,
});
console.log(`  Inserted rows: ${row0}, ${row1}, ${row2}, ${row3}`);
console.log(`  Total facts: ${db.factCount()}, Active: ${db.activeFactCount()}`);

// --- QUERY ALL ---
console.log("\n--- Query All Facts ---");
const allFacts = db.queryAll();
allFacts.forEach(f => {
    console.log(`  subject=${f.subject} predicate=${f.predicate} object=${f.object} confidence=${f.confidence.toFixed(2)}`);
});

// --- QUERY WITH KQL ---
console.log("\n--- KQL Query: SELECT * FROM facts WHERE subject = 1 ---");
const result = db.query("SELECT * FROM facts WHERE subject = 1");
const resultFacts = result.collect();
console.log(`  Returned ${resultFacts.length} facts`);
resultFacts.forEach(f => {
    console.log(`  subject=${f.subject} predicate=${f.predicate} object=${f.object}`);
});

// --- UPDATE ---
console.log("\n--- Update Fact ---");
db.update(row0, {
    subject: 10, predicate: 0, object: 20, confidence: 0.99,
    evidence: 5, timestamp: 0, context: 3, version: 2, priority: 2, owner: 10,
});
console.log(`  Updated row ${row0}: subject changed to 10`);

// --- DELETE ---
console.log("\n--- Delete Fact ---");
const deleted = db.delete(row3);
console.log(`  Deleted row ${row3}: ${deleted}`);
console.log(`  Total: ${db.factCount()}, Active: ${db.activeFactCount()}`);

// --- VERIFY COUNTS ---
console.log("\n--- Verify Counts ---");
console.assert(db.factCount() === 4, `Expected 4 total, got ${db.factCount()}`);
console.assert(db.activeFactCount() === 3, `Expected 3 active, got ${db.activeFactCount()}`);
console.log("  Counts verified: 4 total, 3 active");

// --- ITERATOR PATTERN ---
console.log("\n--- Iterator Pattern ---");
const iter = db.query("SELECT * FROM facts WHERE subject = 2");
for (const fact of iter) {
    console.log(`  subject=${fact.subject} predicate=${fact.predicate} object=${fact.object}`);
}

db.close();
console.log("\n=== All operations completed ===");
