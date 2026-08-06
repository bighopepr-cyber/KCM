#!/usr/bin/env node
"use strict";

/**
 * KCM JavaScript SDK — Query Patterns Example.
 *
 * Demonstrates: different KQL query patterns and filtering options.
 */

const { Database, KcmError, ErrorCode } = require('../src/index.js');

console.log("=== KCM JavaScript SDK — Query Patterns Example ===\n");

const db = new Database();

// Insert test data
db.insert({
    subject: 1, predicate: 0, object: 2, confidence: 0.95,
    evidence: 1, timestamp: 0, context: 1, version: 1, priority: 0, owner: 1,
});
db.insert({
    subject: 2, predicate: 1, object: 3, confidence: 0.90,
    evidence: 2, timestamp: 0, context: 1, version: 1, priority: 0, owner: 2,
});
db.insert({
    subject: 3, predicate: 2, object: 4, confidence: 0.85,
    evidence: 3, timestamp: 0, context: 2, version: 1, priority: 0, owner: 3,
});
db.insert({
    subject: 1, predicate: 3, object: 5, confidence: 0.80,
    evidence: 1, timestamp: 0, context: 2, version: 1, priority: 0, owner: 1,
});
db.insert({
    subject: 4, predicate: 0, object: 6, confidence: 0.75,
    evidence: 2, timestamp: 0, context: 1, version: 1, priority: 0, owner: 2,
});
console.log("Inserted 5 facts\n");

// --- SELECT ALL ---
console.log("--- SELECT * FROM facts ---");
const all = db.query("SELECT * FROM facts");
console.log(`  Returned ${all.collect().length} facts`);

// --- FILTER BY SUBJECT ---
console.log("\n--- SELECT * FROM facts WHERE subject = 1 ---");
const bySubject = db.query("SELECT * FROM facts WHERE subject = 1");
const subjectFacts = bySubject.collect();
console.log(`  Returned ${subjectFacts.length} facts`);
subjectFacts.forEach(f => {
    console.log(`  subject=${f.subject} predicate=${f.predicate} object=${f.object}`);
});
console.assert(subjectFacts.length === 2);

// --- FILTER BY PREDICATE ---
console.log("\n--- SELECT * FROM facts WHERE predicate = 0 ---");
const byPredicate = db.query("SELECT * FROM facts WHERE predicate = 0");
const predicateFacts = byPredicate.collect();
console.log(`  Returned ${predicateFacts.length} facts`);
predicateFacts.forEach(f => {
    console.log(`  subject=${f.subject} predicate=${f.predicate} object=${f.object}`);
});
console.assert(predicateFacts.length === 2);

// --- FILTER BY OBJECT ---
console.log("\n--- SELECT * FROM facts WHERE object = 4 ---");
const byObject = db.query("SELECT * FROM facts WHERE object = 4");
const objectFacts = byObject.collect();
console.log(`  Returned ${objectFacts.length} facts`);
console.assert(objectFacts.length === 1);

// --- MULTI-CONDITION FILTER ---
console.log("\n--- SELECT * FROM facts WHERE subject = 1 AND predicate = 3 ---");
const multiCond = db.query("SELECT * FROM facts WHERE subject = 1 AND predicate = 3");
const multiFacts = multiCond.collect();
console.log(`  Returned ${multiFacts.length} facts`);
console.assert(multiFacts.length === 1);

// --- QUERY ALL CONVENIENCE ---
console.log("\n--- queryAll() convenience method ---");
const allFacts = db.queryAll();
console.log(`  Returned ${allFacts.length} facts`);
console.assert(allFacts.length === 5);

// --- ITERATOR PATTERN ---
console.log("\n--- Iterator Pattern ---");
const iter = db.query("SELECT * FROM facts WHERE subject = 1");
for (const fact of iter) {
    console.log(`  subject=${fact.subject} predicate=${fact.predicate} object=${fact.object} confidence=${fact.confidence.toFixed(2)}`);
}

// --- NEXT() PATTERN ---
console.log("\n--- Next() Pattern ---");
const q = db.query("SELECT * FROM facts WHERE subject = 4");
let fact;
while ((fact = q.next()) !== undefined) {
    console.log(`  subject=${fact.subject} predicate=${fact.predicate} object=${fact.object}`);
}

db.close();
console.log("\n=== All query patterns completed ===");
