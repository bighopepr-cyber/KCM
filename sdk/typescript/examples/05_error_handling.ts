/**
 * KCM TypeScript SDK — Error Handling Example.
 *
 * Demonstrates: proper error handling patterns with KcmError and ErrorCode.
 */

import { Database, FactData, KcmError, ErrorCode } from '../src/index';

console.log("=== KCM TypeScript SDK — Error Handling Example ===\n");

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
        console.assert(e.code === ErrorCode.InvalidArgument);
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
        console.assert(e.code === ErrorCode.NotFound);
    }
}

// --- NOT FOUND (delete non-existent row) ---
console.log("\n--- Not Found (delete non-existent row) ---");
const deleted: boolean = db.delete(99999);
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
    txn.commit();
    console.log("  FAIL: Should have thrown");
} catch (e) {
    if (e instanceof KcmError) {
        console.log(`  Caught KcmError: code=${ErrorCode[e.code]}, message="${e.message}"`);
    }
}

// --- FILE NOT FOUND ---
console.log("\n--- File Not Found (load) ---");
try {
    db.load("/nonexistent/path/db.json");
    console.log("  FAIL: Should have thrown");
} catch (e) {
    if (e instanceof KcmError) {
        console.log(`  Caught KcmError: code=${ErrorCode[e.code]}, message="${e.message}"`);
    }
}

// --- VERIFY ALL ERROR CODES ---
console.log("\n--- All Error Codes ---");
for (const key of Object.keys(ErrorCode).filter(k => isNaN(Number(k)))) {
    console.log(`  ErrorCode.${key} = ${ErrorCode[key as keyof typeof ErrorCode]}`);
}

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
    console.log(`  Query returned ${results.count} results`);
} catch (e) {
    if (e instanceof KcmError) {
        console.log(`  Database error: ${ErrorCode[e.code]}: ${e.message}`);
    } else {
        console.log(`  Unexpected error: ${(e as Error).constructor.name}: ${(e as Error).message}`);
    }
}

db.close();
console.log("\n=== All error handling patterns completed ===");
