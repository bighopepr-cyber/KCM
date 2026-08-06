/**
 * KCM TypeScript SDK — Persistence Example.
 *
 * Demonstrates: save, load, and verify database persistence.
 */

import { Database, FactData, KcmError, ErrorCode } from '../src/index';
import * as fs from 'fs';
import * as path from 'path';
import * as os from 'os';

console.log("=== KCM TypeScript SDK — Persistence Example ===\n");

const savePath: string = path.join(os.tmpdir(), 'kcm_ts_example.json');

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

// --- SAVE ---
console.log("--- Save Database ---");
db.save(savePath);
console.log(`  Saved to ${savePath}`);

// --- VERIFY ---
console.log("\n--- Verify Database ---");
Database.verify(savePath);
console.log("  Verification passed");

// --- LOAD ---
console.log("\n--- Load Database ---");
const db2 = new Database();
db2.load(savePath);
console.log(`  Loaded: ${db2.factCount()} total, ${db2.activeFactCount()} active`);
console.assert(db2.factCount() === 3);
console.assert(db2.activeFactCount() === 3);

// --- VERIFY DATA INTEGRITY ---
console.log("\n--- Verify Data Integrity ---");
const allFacts: FactData[] = db2.queryAll();
allFacts.forEach(f => {
    console.log(`  subject=${f.subject} predicate=${f.predicate} object=${f.object}`);
});
console.assert(allFacts.length === 3);

// --- SAVE-LOAD ROUND TRIP ---
console.log("\n--- Save-Load Round Trip ---");
db2.insert({
    subject: 10, predicate: 0, object: 20, confidence: 0.99,
    evidence: 0, timestamp: 0, context: 0, version: 1, priority: 0, owner: 0,
});
db2.save(savePath);
const db3 = new Database();
db3.load(savePath);
console.log(`  Round-trip: ${db3.factCount()} total, ${db3.activeFactCount()} active`);
console.assert(db3.factCount() === 4);
console.assert(db3.activeFactCount() === 4);

// --- CLEANUP ---
try { fs.unlinkSync(savePath); } catch {}
db.close();
db2.close();
db3.close();
console.log("\n=== All persistence operations completed ===");
