#!/usr/bin/env node
"use strict";

/**
 * KCM JavaScript SDK — Persistence Example.
 *
 * Demonstrates: save, load, and verify database persistence.
 * Note: The JS reference implementation throws IO errors for save/load/verify.
 * This example shows the expected error handling pattern.
 */

const { Database, KcmError, ErrorCode } = require('../src/index.js');

console.log("=== KCM JavaScript SDK — Persistence Example ===\n");

const db = new Database();

// Insert facts
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
console.log(`Facts: ${db.factCount()} total, ${db.activeFactCount()} active\n`);

// --- SAVE (reference implementation) ---
console.log("--- Save Database ---");
try {
    db.save("/tmp/kcm_example.db");
    console.log("  Database saved");
} catch (e) {
    if (e instanceof KcmError) {
        console.log(`  save() threw KcmError: code=${ErrorCode[e.code]}, message="${e.message}"`);
        console.log("  (This is expected for the reference implementation)");
    }
}

// --- LOAD (reference implementation) ---
console.log("\n--- Load Database ---");
try {
    const db2 = new Database();
    db2.load("/tmp/kcm_example.db");
    console.log(`  Loaded: ${db2.factCount()} facts`);
    db2.close();
} catch (e) {
    if (e instanceof KcmError) {
        console.log(`  load() threw KcmError: code=${ErrorCode[e.code]}, message="${e.message}"`);
        console.log("  (This is expected for the reference implementation)");
    }
}

// --- VERIFY (reference implementation) ---
console.log("\n--- Verify Database ---");
try {
    Database.verify("/tmp/kcm_example.db");
    console.log("  Database verified");
} catch (e) {
    if (e instanceof KcmError) {
        console.log(`  verify() threw KcmError: code=${ErrorCode[e.code]}, message="${e.message}"`);
        console.log("  (This is expected for the reference implementation)");
    }
}

// --- VERIFY NON-EXISTENT FILE ---
console.log("\n--- Verify Non-Existent File ---");
try {
    Database.verify("/nonexistent/path/db.kcm");
    console.log("  FAIL: Should have thrown");
} catch (e) {
    if (e instanceof KcmError) {
        console.log(`  Caught: code=${ErrorCode[e.code]}, message="${e.message}"`);
    }
}

// --- LOAD NON-EXISTENT FILE ---
console.log("\n--- Load Non-Existent File ---");
try {
    db.load("/nonexistent/path/db.kcm");
    console.log("  FAIL: Should have thrown");
} catch (e) {
    if (e instanceof KcmError) {
        console.log(`  Caught: code=${ErrorCode[e.code]}, message="${e.message}"`);
    }
}

db.close();
console.log("\n=== All persistence operations completed ===");
