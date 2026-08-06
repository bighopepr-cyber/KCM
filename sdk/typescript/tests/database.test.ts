import { Database, KcmError, ErrorCode, FactData, Transaction } from '../src/index';
import * as fs from 'fs';
import * as path from 'path';

function makeFact(overrides: Partial<FactData> = {}): FactData {
    return {
        subject: 1,
        predicate: 0,
        object: 2,
        confidence: 0.95,
        evidence: 1,
        timestamp: 1700000000000000000,
        context: 1,
        version: 1,
        priority: 0,
        owner: 1,
        ...overrides,
    };
}

const TEST_DB_PATH = path.join(__dirname, '__test_db__.json');

describe('Database', () => {
    let db: Database;

    beforeEach(() => {
        db = new Database();
    });

    afterEach(() => {
        db.close();
        if (fs.existsSync(TEST_DB_PATH)) {
            fs.unlinkSync(TEST_DB_PATH);
        }
    });

    test('creates empty database', () => {
        expect(db.factCount()).toBe(0);
        expect(db.activeFactCount()).toBe(0);
    });

    test('insert returns sequential row ids', () => {
        const id0 = db.insert(makeFact());
        const id1 = db.insert(makeFact({ subject: 2 }));
        expect(id0).toBe(0);
        expect(id1).toBe(1);
        expect(db.factCount()).toBe(2);
    });

    test('insert updates fact and active counts', () => {
        db.insert(makeFact());
        db.insert(makeFact({ subject: 2 }));
        expect(db.factCount()).toBe(2);
        expect(db.activeFactCount()).toBe(2);
    });

    test('insert stores all 10 fact fields', () => {
        const fact = makeFact({
            subject: 10,
            predicate: 5,
            object: 20,
            confidence: 0.8,
            evidence: 3,
            timestamp: 1234567890,
            context: 7,
            version: 4,
            priority: 2,
            owner: 99,
        });
        db.insert(fact);
        const results = db.queryAll();
        expect(results.length).toBe(1);
        expect(results[0].subject).toBe(10);
        expect(results[0].predicate).toBe(5);
        expect(results[0].object).toBe(20);
        expect(results[0].confidence).toBe(0.8);
        expect(results[0].evidence).toBe(3);
        expect(results[0].timestamp).toBe(1234567890);
        expect(results[0].context).toBe(7);
        expect(results[0].version).toBe(4);
        expect(results[0].priority).toBe(2);
        expect(results[0].owner).toBe(99);
    });

    test('queryAll returns all active facts', () => {
        db.insert(makeFact({ subject: 1 }));
        db.insert(makeFact({ subject: 2 }));
        db.insert(makeFact({ subject: 3 }));
        const results = db.queryAll();
        expect(results.length).toBe(3);
    });

    test('query returns KQueryResult', () => {
        db.insert(makeFact({ subject: 1 }));
        db.insert(makeFact({ subject: 2 }));
        const result = db.query('SELECT * FROM facts');
        expect(result.count).toBe(2);
        expect(result.collect().length).toBe(2);
    });

    test('query result collect returns copies', () => {
        db.insert(makeFact({ subject: 1 }));
        const result = db.query('SELECT * FROM facts');
        const collected = result.collect();
        collected[0].subject = 999;
        const recollected = result.collect();
        expect(recollected[0].subject).toBe(1);
    });

    test('update replaces fact data', () => {
        const id = db.insert(makeFact({ subject: 1, confidence: 0.5 }));
        db.update(id, makeFact({ subject: 2, confidence: 0.99 }));
        const results = db.queryAll();
        expect(results.length).toBe(1);
        expect(results[0].subject).toBe(2);
        expect(results[0].confidence).toBe(0.99);
    });

    test('update preserves row id position', () => {
        const id0 = db.insert(makeFact({ subject: 1 }));
        const id1 = db.insert(makeFact({ subject: 2 }));
        db.update(id0, makeFact({ subject: 10 }));
        const results = db.queryAll();
        expect(results.length).toBe(2);
        expect(results[0].subject).toBe(10);
        expect(results[1].subject).toBe(2);
    });

    test('delete marks fact as inactive', () => {
        const id = db.insert(makeFact());
        expect(db.activeFactCount()).toBe(1);
        const deleted = db.delete(id);
        expect(deleted).toBe(true);
        expect(db.activeFactCount()).toBe(0);
        expect(db.factCount()).toBe(1);
    });

    test('delete returns false for invalid row id', () => {
        expect(db.delete(-1)).toBe(false);
        expect(db.delete(999)).toBe(false);
    });

    test('delete returns false for already deleted row', () => {
        const id = db.insert(makeFact());
        db.delete(id);
        expect(db.delete(id)).toBe(false);
    });

    test('queryAll excludes deleted facts', () => {
        const id = db.insert(makeFact({ subject: 1 }));
        db.insert(makeFact({ subject: 2 }));
        db.delete(id);
        const results = db.queryAll();
        expect(results.length).toBe(1);
        expect(results[0].subject).toBe(2);
    });
});

describe('Fact validation', () => {
    let db: Database;

    beforeEach(() => {
        db = new Database();
    });

    afterEach(() => {
        db.close();
    });

    test('rejects non-integer subject', () => {
        expect(() => db.insert(makeFact({ subject: 1.5 }))).toThrow(KcmError);
    });

    test('rejects non-integer predicate', () => {
        expect(() => db.insert(makeFact({ predicate: 1.5 }))).toThrow(KcmError);
    });

    test('rejects non-integer object', () => {
        expect(() => db.insert(makeFact({ object: 1.5 }))).toThrow(KcmError);
    });

    test('rejects confidence < 0', () => {
        expect(() => db.insert(makeFact({ confidence: -0.1 }))).toThrow(KcmError);
    });

    test('rejects confidence > 1', () => {
        expect(() => db.insert(makeFact({ confidence: 1.5 }))).toThrow(KcmError);
    });

    test('rejects non-integer evidence', () => {
        expect(() => db.insert(makeFact({ evidence: 1.5 }))).toThrow(KcmError);
    });

    test('rejects non-integer timestamp', () => {
        expect(() => db.insert(makeFact({ timestamp: 1.5 }))).toThrow(KcmError);
    });

    test('rejects non-integer context', () => {
        expect(() => db.insert(makeFact({ context: 1.5 }))).toThrow(KcmError);
    });

    test('rejects non-integer version', () => {
        expect(() => db.insert(makeFact({ version: 1.5 }))).toThrow(KcmError);
    });

    test('rejects non-integer priority', () => {
        expect(() => db.insert(makeFact({ priority: 1.5 }))).toThrow(KcmError);
    });

    test('rejects non-integer owner', () => {
        expect(() => db.insert(makeFact({ owner: 1.5 }))).toThrow(KcmError);
    });

    test('accepts boundary confidence values', () => {
        expect(() => db.insert(makeFact({ confidence: 0 }))).not.toThrow();
        expect(() => db.insert(makeFact({ confidence: 1 }))).not.toThrow();
    });

    test('rejects missing fact fields', () => {
        expect(() => db.insert({ subject: 1 } as any)).toThrow(KcmError);
    });
});

describe('Error codes', () => {
    let db: Database;

    beforeEach(() => {
        db = new Database();
    });

    afterEach(() => {
        db.close();
    });

    test('InvalidArgument for bad confidence', () => {
        try {
            db.insert(makeFact({ confidence: 2 }));
            fail('should have thrown');
        } catch (e) {
            expect(e).toBeInstanceOf(KcmError);
            expect((e as KcmError).code).toBe(ErrorCode.InvalidArgument);
        }
    });

    test('NotFound for update of missing row', () => {
        try {
            db.update(999, makeFact());
            fail('should have thrown');
        } catch (e) {
            expect(e).toBeInstanceOf(KcmError);
            expect((e as KcmError).code).toBe(ErrorCode.NotFound);
        }
    });

    test('NotFound for update of deleted row', () => {
        const id = db.insert(makeFact());
        db.delete(id);
        try {
            db.update(id, makeFact());
            fail('should have thrown');
        } catch (e) {
            expect(e).toBeInstanceOf(KcmError);
            expect((e as KcmError).code).toBe(ErrorCode.NotFound);
        }
    });

    test('InvalidArgument for empty path in save', () => {
        try {
            db.save('');
            fail('should have thrown');
        } catch (e) {
            expect(e).toBeInstanceOf(KcmError);
            expect((e as KcmError).code).toBe(ErrorCode.InvalidArgument);
        }
    });

    test('NotFound for load of missing file', () => {
        try {
            db.load('/nonexistent/path.json');
            fail('should have thrown');
        } catch (e) {
            expect(e).toBeInstanceOf(KcmError);
            expect((e as KcmError).code).toBe(ErrorCode.NotFound);
        }
    });

    test('NotFound for verify of missing file', () => {
        try {
            Database.verify('/nonexistent/path.json');
            fail('should have thrown');
        } catch (e) {
            expect(e).toBeInstanceOf(KcmError);
            expect((e as KcmError).code).toBe(ErrorCode.NotFound);
        }
    });

    test('InvalidArgument for empty path in verify', () => {
        try {
            Database.verify('');
            fail('should have thrown');
        } catch (e) {
            expect(e).toBeInstanceOf(KcmError);
            expect((e as KcmError).code).toBe(ErrorCode.InvalidArgument);
        }
    });

    test('operations on closed database throw InvalidArgument', () => {
        db.close();
        expect(() => db.insert(makeFact())).toThrow(KcmError);
        expect(() => db.queryAll()).toThrow(KcmError);
        expect(() => db.factCount()).toThrow(KcmError);
        expect(() => db.beginTransaction()).toThrow(KcmError);
    });

    test('Corrupted for invalid JSON in load', () => {
        fs.writeFileSync(TEST_DB_PATH, 'not json', 'utf-8');
        try {
            db.load(TEST_DB_PATH);
            fail('should have thrown');
        } catch (e) {
            expect(e).toBeInstanceOf(KcmError);
            expect((e as KcmError).code).toBe(ErrorCode.Corrupted);
        }
    });

    test('Corrupted for invalid JSON in verify', () => {
        fs.writeFileSync(TEST_DB_PATH, 'not json', 'utf-8');
        try {
            Database.verify(TEST_DB_PATH);
            fail('should have thrown');
        } catch (e) {
            expect(e).toBeInstanceOf(KcmError);
            expect((e as KcmError).code).toBe(ErrorCode.Corrupted);
        }
    });
});

describe('Transaction', () => {
    let db: Database;

    beforeEach(() => {
        db = new Database();
    });

    afterEach(() => {
        db.close();
    });

    test('beginTransaction returns Transaction', () => {
        const txn = db.beginTransaction();
        expect(txn).toBeInstanceOf(Transaction);
        txn.rollback();
    });

    test('commit persists insertions', () => {
        const txn = db.beginTransaction();
        const id = txn.getInserts().length;
        db.insert(makeFact({ subject: 1 }));
        txn.recordInsert(makeFact({ subject: 1 }), db.factCount() - 1);
        txn.commit();
        expect(db.activeFactCount()).toBe(1);
    });

    test('rollback discards changes', () => {
        const txn = db.beginTransaction();
        db.insert(makeFact({ subject: 1 }));
        txn.recordInsert(makeFact({ subject: 1 }), db.factCount() - 1);
        txn.rollback();
        expect(db.activeFactCount()).toBe(1);
    });

    test('double commit throws TransactionAborted', () => {
        const txn = db.beginTransaction();
        db.insert(makeFact());
        txn.recordInsert(makeFact(), 0);
        txn.commit();
        expect(() => txn.commit()).toThrow(KcmError);
    });

    test('commit after rollback throws TransactionAborted', () => {
        const txn = db.beginTransaction();
        txn.rollback();
        expect(() => txn.commit()).toThrow(KcmError);
    });

    test('rollback after commit throws TransactionAborted', () => {
        const txn = db.beginTransaction();
        db.insert(makeFact());
        txn.recordInsert(makeFact(), 0);
        txn.commit();
        expect(() => txn.rollback()).toThrow(KcmError);
    });

    test('double rollback throws TransactionAborted', () => {
        const txn = db.beginTransaction();
        txn.rollback();
        expect(() => txn.rollback()).toThrow(KcmError);
    });
});

describe('Save/Load/Verify', () => {
    let db: Database;

    beforeEach(() => {
        db = new Database();
    });

    afterEach(() => {
        db.close();
        if (fs.existsSync(TEST_DB_PATH)) {
            fs.unlinkSync(TEST_DB_PATH);
        }
    });

    test('save creates file', () => {
        db.insert(makeFact({ subject: 1 }));
        db.insert(makeFact({ subject: 2 }));
        db.save(TEST_DB_PATH);
        expect(fs.existsSync(TEST_DB_PATH)).toBe(true);
    });

    test('load restores facts', () => {
        db.insert(makeFact({ subject: 10 }));
        db.insert(makeFact({ subject: 20 }));
        db.save(TEST_DB_PATH);
        db.close();
        db = new Database();
        db.load(TEST_DB_PATH);
        expect(db.factCount()).toBe(2);
        const facts = db.queryAll();
        expect(facts[0].subject).toBe(10);
        expect(facts[1].subject).toBe(20);
    });

    test('load replaces existing data', () => {
        db.insert(makeFact({ subject: 1 }));
        db.save(TEST_DB_PATH);
        db.close();
        db = new Database();
        db.insert(makeFact({ subject: 99 }));
        db.load(TEST_DB_PATH);
        expect(db.factCount()).toBe(1);
        expect(db.queryAll()[0].subject).toBe(1);
    });

    test('verify passes for valid file', () => {
        db.insert(makeFact({ subject: 1 }));
        db.save(TEST_DB_PATH);
        expect(() => Database.verify(TEST_DB_PATH)).not.toThrow();
    });

    test('verify fails for corrupted file', () => {
        fs.writeFileSync(TEST_DB_PATH, 'corrupted data', 'utf-8');
        expect(() => Database.verify(TEST_DB_PATH)).toThrow(KcmError);
    });

    test('save preserves all 10 fact fields', () => {
        const fact = makeFact({
            subject: 10,
            predicate: 5,
            object: 20,
            confidence: 0.8,
            evidence: 3,
            timestamp: 1234567890,
            context: 7,
            version: 4,
            priority: 2,
            owner: 99,
        });
        db.insert(fact);
        db.save(TEST_DB_PATH);
        db.close();
        db = new Database();
        db.load(TEST_DB_PATH);
        const results = db.queryAll();
        expect(results.length).toBe(1);
        expect(results[0]).toEqual(fact);
    });
});
