#!/usr/bin/env node
"use strict";

/**
 * KCM JavaScript SDK — Error Handling Example.
 *
 * Demonstrates: proper error handling patterns with KcmError and ErrorCode.
 */

const { Database, KcmError, ErrorCode } = require('../src/index.js');

console.log("=== KCM JavaScript SDK — Error Handling Example ===\n");

const db = new Database();

// --- INVALID CONFIDENCE ---
console.log("--- Invalid Confidence (out of range) ---");
try {
    db.insert({
        subject: 1, predicate: 0, object: 2, confidence: 1.5,
        evidence: 0, timestamp: 0, context: 0, version: 1, priority: 0, owner: 0,
    });
    console.log("  FAIL: Should have thrown");
} catch (e) {
    if (e instanceof KcmError) {
        console.log(`  Caught KcmError: code=${ErrorCode[e.code]}, message="${e.message}"`);
        console.assert(e.code === ErrorCode.INVALID_ARGUMENT);
    }
}

// --- NOT FOUND (update non-existent row) ---
console.log("\n--- Not Found (update non-existent row) ---");
try {
    db.update(99999, {
        subject: 1, predicate: 0, object: 2, confidence: 0.5,
        evidence: 0, timestamp: 0, context: 0, version: 1, priority: 0, owner: 0,
    });
    console.log("  FAIL: Should have thrown");
} catch (e) {
    if (e instanceof KcmError) {
        console.log(`  Caught KcmError: code=${ErrorCode[e.code]}, message="${e.message}"`);
        console.assert(e.code === ErrorCode.NOT_FOUND);
    }
}

// --- NOT FOUND (delete non-existent row) ---
console.log("\n--- Not Found (delete non-existent row) ---");
const deleted = db.delete(99999);
console.log(`  Delete returned: ${deleted} (not an exception for delete)`);

// --- INVALID KQL QUERY ---
console.log("\n--- Invalid KQL Query ---");
try {
    db.query("INVALID QUERY");
    console.log("  FAIL: Should have thrown");
} catch (e) {
    if (e instanceof KcmError) {
        console.log(`  Caught KcmError: code=${ErrorCode[e.code]}, message="${e.message}"`);
    }
}

// --- DATABASE CLOSED ---
console.log("\n--- Database Closed ---");
const db2 = new Database();
db2.close();
try {
    db2.insert({
        subject: 1, predicate: 0, object: 2, confidence: 0.5,
        evidence: 0, timestamp: 0, context: 0, version: 1, priority: 0, owner: 0,
    });
    console.log("  FAIL: Should have thrown");
} catch (e) {
    if (e instanceof KcmError) {
        console.log(`  Caught KcmError: code=${ErrorCode[e.code]}, message="${e.message}"`);
    }
}

// --- TRANSACTION ALREADY FINALIZED ---
console.log("\n--- Transaction Already Finalized ---");
const txn = db.beginTransaction();
txn.commit();
try {
    txn.insert({
        subject: 1, predicate: 0, object: 2, confidence: 0.5,
        evidence: 0, timestamp: 0, context: 0, version: 1, priority: 0, owner: 0,
    });
    console.log("  FAIL: Should have thrown");
} catch (e) {
    if (e instanceof KcmError) {
        console.log(`  Caught KcmError: code=${ErrorCode[e.code]}, message="${e.message}"`);
    }
}

// --- FILE NOT FOUND (save reference impl) ---
console.log("\n--- Save (reference implementation) ---");
try {
    db.save("/tmp/test.db");
    console.log("  Database saved");
} catch (e) {
    if (e instanceof KcmError) {
        console.log(`  save() threw: code=${ErrorCode[e.code]}, message="${e.message}"`);
        console.log("  (Expected for reference implementation)");
    }
}

// --- VERIFY ALL ERROR CODES ---
console.log("\n--- All Error Codes ---");
Object.keys(ErrorCode).forEach(key => {
    if (typeof ErrorCode[key] === 'number') {
        console.log(`  ${key} = ${ErrorCode[key]}`);
    }
});

// --- TRY-CATCH PATTERN ---
console.log("\n--- Try-Catch Pattern ---");
try {
    db.insert({
        subject: 1, predicate: 0, object: 2, confidence: 0.95,
        evidence: 0, timestamp: 0, context: 0, version: 1, priority: 0, owner: 0,
    });
    db.insert({
        subject: 2, predicate: 1, object: 3, confidence: 0.90,
        evidence: 0, timestamp: 0, context: 0, version: 1, priority: 0, owner: 0,
    });
    const results = db.query("SELECT * FROM facts WHERE subject = 1");
    console.log(`  Query returned ${results.collect().length} results`);
} catch (e) {
    if (e instanceof KcmError) {
        console.log(`  Database error: ${ErrorCode[e.code]}: ${e.message}`);
    } else {
        console.log(`  Unexpected error: ${e.constructor.name}: ${e.message}`);
    }
}

db.close();
console.log("\n=== All error handling patterns completed ===");
