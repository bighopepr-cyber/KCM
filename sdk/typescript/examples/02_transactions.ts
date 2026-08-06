/**
 * KCM TypeScript SDK — Transaction Example.
 *
 * Demonstrates: begin, commit, and rollback scenarios with transactions.
 */

import { Database, FactData, KcmError, ErrorCode } from '../src/index';

console.log("=== KCM TypeScript SDK — Transaction Example ===\n");

const db = new Database();

// Insert baseline facts
db.insert({
    subject: 1, predicate: 0, object: 2, confidence: 0.95,
    evidence: 0, timestamp: 0, context: 0, version: 1, priority: 0, owner: 0,
});
db.insert({
    subject: 2, predicate: 1, object: 3, confidence: 0.90,
    evidence: 1, timestamp: 0, context: 1, version: 1, priority: 0, owner: 1,
});
console.log(`Initial: ${db.activeFactCount()} active facts\n`);

// --- COMMITTED TRANSACTION ---
console.log("--- Committed Transaction ---");
const txn1 = db.beginTransaction();
txn1.recordInsert({
    subject: 3, predicate: 2, object: 4, confidence: 0.85,
    evidence: 2, timestamp: 0, context: 2, version: 1, priority: 0, owner: 2,
}, db.factCount());
console.log("  Inserted fact in transaction");
txn1.commit();
console.log(`  After commit: ${db.activeFactCount()} active facts`);
console.assert(db.activeFactCount() === 3);

// --- ROLLED BACK TRANSACTION ---
console.log("\n--- Rolled Back Transaction ---");
const txn2 = db.beginTransaction();
txn2.recordInsert({
    subject: 4, predicate: 3, object: 5, confidence: 0.80,
    evidence: 3, timestamp: 0, context: 2, version: 1, priority: 0, owner: 3,
}, db.factCount());
console.log("  Inserted fact in transaction");
txn2.rollback();
console.log(`  After rollback: ${db.activeFactCount()} active facts`);
console.assert(db.activeFactCount() === 3);

// --- TRANSACTION WITH UPDATE ---
console.log("\n--- Transaction with Update ---");
const txn3 = db.beginTransaction();
txn3.recordUpdate(0, {
    subject: 10, predicate: 0, object: 20, confidence: 0.99,
    evidence: 5, timestamp: 0, context: 3, version: 2, priority: 0, owner: 1,
});
console.log("  Updated fact in transaction");
txn3.commit();
console.log("  Transaction committed successfully");

// --- TRANSACTION WITH DELETE ---
console.log("\n--- Transaction with Delete ---");
const countBefore: number = db.activeFactCount();
const txn4 = db.beginTransaction();
txn4.recordDelete(2);
console.log("  Deleted fact in transaction");
txn4.commit();
console.log(`  After commit: ${db.activeFactCount()} active facts (was ${countBefore})`);
console.assert(db.activeFactCount() === countBefore - 1);

// --- MULTIPLE OPERATIONS ---
console.log("\n--- Multiple Operations in Transaction ---");
const txn5 = db.beginTransaction();
txn5.recordInsert({
    subject: 10, predicate: 0, object: 20, confidence: 0.99,
    evidence: 0, timestamp: 0, context: 0, version: 1, priority: 0, owner: 0,
}, db.factCount());
txn5.recordInsert({
    subject: 30, predicate: 1, object: 40, confidence: 0.88,
    evidence: 0, timestamp: 0, context: 0, version: 1, priority: 0, owner: 0,
}, db.factCount() + 1);
txn5.recordInsert({
    subject: 50, predicate: 2, object: 60, confidence: 0.77,
    evidence: 0, timestamp: 0, context: 0, version: 1, priority: 0, owner: 0,
}, db.factCount() + 2);
console.log("  3 pending operations");
txn5.commit();
console.log(`  After commit: ${db.activeFactCount()} active facts`);

db.close();
console.log("\n=== All transaction operations completed ===");
