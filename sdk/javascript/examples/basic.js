#!/usr/bin/env node
"use strict";

const { Database, ErrorCode, KcmError } = require('../src/index.js');

console.log("=== KCM JavaScript SDK - Basic Example ===\n");

const db = new Database();

// Insert facts with all 10 fields
db.insert({
    subject: 1, predicate: 0, object: 2, confidence: 0.95,
    evidence: 1, timestamp: 1700000000, context: 1, version: 1, priority: 0, owner: 1,
});
db.insert({
    subject: 2, predicate: 1, object: 3, confidence: 0.90,
    evidence: 2, timestamp: 1700000001, context: 1, version: 1, priority: 1, owner: 2,
});
db.insert({
    subject: 1, predicate: 2, object: 3, confidence: 0.85,
    evidence: 3, timestamp: 1700000002, context: 2, version: 1, priority: -1, owner: 1,
});
console.log(`Inserted 3 facts (count=${db.factCount()}, active=${db.activeFactCount()})`);

// Query all
const all = db.queryAll();
console.log(`\nqueryAll() returned ${all.length} facts:`);
all.forEach(f => {
    console.log(`  subject=${f.subject} predicate=${f.predicate} object=${f.object} confidence=${f.confidence.toFixed(2)}`);
});

// KQL query - all facts
const result = db.query("SELECT * FROM facts");
console.log(`\nquery("SELECT * FROM facts") returned ${result.collect().length} facts`);

// KQL query - filter by subject
const filtered = db.query("SELECT * FROM facts WHERE subject = 1");
console.log(`query("SELECT * FROM facts WHERE subject = 1") returned ${filtered.collect().length} facts`);

// Update
db.update(0, {
    subject: 1, predicate: 0, object: 5, confidence: 0.99,
    evidence: 1, timestamp: 1700000010, context: 1, version: 2, priority: 0, owner: 1,
});
console.log(`\nUpdated row 0: object changed from 2 to 5`);

// Delete
const deleted = db.delete(2);
console.log(`Deleted row 2: ${deleted}, active=${db.activeFactCount()}`);

// Transaction
console.log("\n--- Transaction demo ---");
const txn = db.beginTransaction();
txn.insert({
    subject: 99, predicate: 9, object: 99, confidence: 0.50,
    evidence: 0, timestamp: 0, context: 0, version: 1, priority: 0, owner: 0,
});
console.log(`After transactional insert: active=${db.activeFactCount()} (not yet visible)`);
txn.commit();
console.log(`After commit: active=${db.activeFactCount()} (now visible)`);

// Transaction rollback
const txn2 = db.beginTransaction();
txn2.insert({
    subject: 100, predicate: 0, object: 100, confidence: 0.10,
    evidence: 0, timestamp: 0, context: 0, version: 1, priority: 0, owner: 0,
});
txn2.rollback();
console.log(`After rollback: active=${db.activeFactCount()} (insert discarded)`);

// Iterator
console.log("\n--- Iterator demo ---");
const iter = db.query("SELECT * FROM facts WHERE subject = 1");
for (const fact of iter) {
    console.log(`  subject=${fact.subject} predicate=${fact.predicate} object=${fact.object}`);
}

// Error handling
console.log("\n--- Error handling demo ---");
try {
    db.insert({ subject: 1, predicate: 0, object: 2, confidence: 2.0, evidence: 0, timestamp: 0, context: 0, version: 1, priority: 0, owner: 0 });
} catch (e) {
    if (e instanceof KcmError) {
        console.log(`Caught KcmError: code=${ErrorCode[e.code]}, message="${e.message}"`);
    }
}

try {
    db.update(999, {
        subject: 1, predicate: 0, object: 2, confidence: 0.5,
        evidence: 0, timestamp: 0, context: 0, version: 1, priority: 0, owner: 0,
    });
} catch (e) {
    if (e instanceof KcmError) {
        console.log(`Caught KcmError: code=${ErrorCode[e.code]}, message="${e.message}"`);
    }
}

// Save/Load/Verify (not supported in reference implementation)
console.log("\n--- Save/Load/Verify (IO errors expected) ---");
try {
    db.save("/tmp/test.db");
} catch (e) {
    if (e instanceof KcmError) {
        console.log(`save() threw: code=${ErrorCode[e.code]}`);
    }
}

try {
    Database.verify("/tmp/test.db");
} catch (e) {
    if (e instanceof KcmError) {
        console.log(`verify() threw: code=${ErrorCode[e.code]}`);
    }
}

// Close
db.close();
console.log(`\nDatabase closed. Final factCount=${db.factCount()}`);
console.log("\nAll examples completed.");
