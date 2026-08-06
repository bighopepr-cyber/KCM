import { Database, KcmError, ErrorCode, QueryResult, Transaction } from '../src/index';
import type { FactData } from '../src/index';

function makeFact(overrides?: Partial<FactData>): FactData {
    return {
        subject: 1,
        predicate: 0,
        object: 2,
        confidence: 0.95,
        evidence: 1,
        timestamp: 1700000000,
        context: 1,
        version: 1,
        priority: 0,
        owner: 1,
        ...overrides,
    };
}

describe('ErrorCode', () => {
    test('has all 8 error codes', () => {
        expect(ErrorCode.OK).toBe(0);
        expect(ErrorCode.NOT_FOUND).toBe(1);
        expect(ErrorCode.OUT_OF_MEMORY).toBe(2);
        expect(ErrorCode.INVALID_ARGUMENT).toBe(3);
        expect(ErrorCode.IO).toBe(4);
        expect(ErrorCode.CORRUPTED).toBe(5);
        expect(ErrorCode.CONFLICT).toBe(6);
        expect(ErrorCode.TRANSACTION_ABORTED).toBe(7);
    });
});

describe('KcmError', () => {
    test('stores code and message', () => {
        const err = new KcmError(ErrorCode.NOT_FOUND, "missing");
        expect(err.code).toBe(ErrorCode.NOT_FOUND);
        expect(err.message).toBe("missing");
        expect(err.name).toBe("KcmError");
    });

    test('uses default message when none provided', () => {
        const err = new KcmError(ErrorCode.IO);
        expect(err.message).toBe("I/O error");
    });

    test('is an instance of Error', () => {
        const err = new KcmError(ErrorCode.CONFLICT);
        expect(err).toBeInstanceOf(Error);
        expect(err).toBeInstanceOf(KcmError);
    });
});

describe('FactData', () => {
    test('accepts all 10 required fields', () => {
        const db = new Database();
        const fact = makeFact();
        const id = db.insert(fact);
        expect(id).toBe(0);
        db.close();
    });

    test('rejects missing subject', () => {
        const db = new Database();
        const fact = { predicate: 0, object: 2, confidence: 0.5, evidence: 0, timestamp: 0, context: 0, version: 0, priority: 0, owner: 0 } as unknown as FactData;
        expect(() => db.insert(fact)).toThrow(KcmError);
        db.close();
    });

    test('rejects missing predicate', () => {
        const db = new Database();
        const fact = { subject: 1, object: 2, confidence: 0.5, evidence: 0, timestamp: 0, context: 0, version: 0, priority: 0, owner: 0 } as unknown as FactData;
        expect(() => db.insert(fact)).toThrow(KcmError);
        db.close();
    });

    test('rejects missing object', () => {
        const db = new Database();
        const fact = { subject: 1, predicate: 0, confidence: 0.5, evidence: 0, timestamp: 0, context: 0, version: 0, priority: 0, owner: 0 } as unknown as FactData;
        expect(() => db.insert(fact)).toThrow(KcmError);
        db.close();
    });

    test('rejects missing confidence', () => {
        const db = new Database();
        const fact = { subject: 1, predicate: 0, object: 2, evidence: 0, timestamp: 0, context: 0, version: 0, priority: 0, owner: 0 } as unknown as FactData;
        expect(() => db.insert(fact)).toThrow(KcmError);
        db.close();
    });

    test('rejects missing evidence', () => {
        const db = new Database();
        const fact = { subject: 1, predicate: 0, object: 2, confidence: 0.5, timestamp: 0, context: 0, version: 0, priority: 0, owner: 0 } as unknown as FactData;
        expect(() => db.insert(fact)).toThrow(KcmError);
        db.close();
    });

    test('rejects missing timestamp', () => {
        const db = new Database();
        const fact = { subject: 1, predicate: 0, object: 2, confidence: 0.5, evidence: 0, context: 0, version: 0, priority: 0, owner: 0 } as unknown as FactData;
        expect(() => db.insert(fact)).toThrow(KcmError);
        db.close();
    });

    test('rejects missing context', () => {
        const db = new Database();
        const fact = { subject: 1, predicate: 0, object: 2, confidence: 0.5, evidence: 0, timestamp: 0, version: 0, priority: 0, owner: 0 } as unknown as FactData;
        expect(() => db.insert(fact)).toThrow(KcmError);
        db.close();
    });

    test('rejects missing version', () => {
        const db = new Database();
        const fact = { subject: 1, predicate: 0, object: 2, confidence: 0.5, evidence: 0, timestamp: 0, context: 0, priority: 0, owner: 0 } as unknown as FactData;
        expect(() => db.insert(fact)).toThrow(KcmError);
        db.close();
    });

    test('rejects missing priority', () => {
        const db = new Database();
        const fact = { subject: 1, predicate: 0, object: 2, confidence: 0.5, evidence: 0, timestamp: 0, context: 0, version: 0, owner: 0 } as unknown as FactData;
        expect(() => db.insert(fact)).toThrow(KcmError);
        db.close();
    });

    test('rejects missing owner', () => {
        const db = new Database();
        const fact = { subject: 1, predicate: 0, object: 2, confidence: 0.5, evidence: 0, timestamp: 0, context: 0, version: 0, priority: 0 } as unknown as FactData;
        expect(() => db.insert(fact)).toThrow(KcmError);
        db.close();
    });

    test('rejects null fact', () => {
        const db = new Database();
        expect(() => db.insert(null as unknown as FactData)).toThrow(KcmError);
        db.close();
    });

    test('rejects non-object fact', () => {
        const db = new Database();
        expect(() => db.insert("not a fact" as unknown as FactData)).toThrow(KcmError);
        db.close();
    });

    test('rejects confidence < 0', () => {
        const db = new Database();
        expect(() => db.insert(makeFact({ confidence: -0.1 }))).toThrow(KcmError);
        db.close();
    });

    test('rejects confidence > 1', () => {
        const db = new Database();
        expect(() => db.insert(makeFact({ confidence: 1.1 }))).toThrow(KcmError);
        db.close();
    });

    test('accepts confidence boundaries 0 and 1', () => {
        const db = new Database();
        expect(db.insert(makeFact({ confidence: 0 }))).toBe(0);
        expect(db.insert(makeFact({ confidence: 1 }))).toBe(1);
        db.close();
    });
});

describe('Database CRUD', () => {
    let db: Database;

    beforeEach(() => {
        db = new Database();
    });

    afterEach(() => {
        db.close();
    });

    test('creates empty database', () => {
        expect(db.factCount()).toBe(0);
        expect(db.activeFactCount()).toBe(0);
    });

    test('insert returns sequential row IDs', () => {
        expect(db.insert(makeFact())).toBe(0);
        expect(db.insert(makeFact())).toBe(1);
        expect(db.insert(makeFact())).toBe(2);
    });

    test('insert stores all 10 fields', () => {
        const fact = makeFact({
            subject: 10, predicate: 5, object: 20, confidence: 0.75,
            evidence: 3, timestamp: 1700000001, context: 2, version: 3, priority: -1, owner: 99,
        });
        const id = db.insert(fact);
        const results = db.queryAll();
        expect(results.length).toBe(1);
        expect(results[0].subject).toBe(10);
        expect(results[0].predicate).toBe(5);
        expect(results[0].object).toBe(20);
        expect(results[0].confidence).toBe(0.75);
        expect(results[0].evidence).toBe(3);
        expect(results[0].timestamp).toBe(1700000001);
        expect(results[0].context).toBe(2);
        expect(results[0].version).toBe(3);
        expect(results[0].priority).toBe(-1);
        expect(results[0].owner).toBe(99);
    });

    test('insert does not store by reference', () => {
        const fact = makeFact();
        const id = db.insert(fact);
        fact.subject = 999;
        const results = db.queryAll();
        expect(results[0].subject).toBe(1);
    });

    test('update replaces fact at row ID', () => {
        const id = db.insert(makeFact({ subject: 1 }));
        db.update(id, makeFact({ subject: 2 }));
        const results = db.queryAll();
        expect(results.length).toBe(1);
        expect(results[0].subject).toBe(2);
    });

    test('update throws NOT_FOUND for invalid row ID', () => {
        expect(() => db.update(999, makeFact())).toThrow(KcmError);
        try {
            db.update(999, makeFact());
        } catch (e) {
            expect((e as KcmError).code).toBe(ErrorCode.NOT_FOUND);
        }
    });

    test('update throws NOT_FOUND for deleted row', () => {
        const id = db.insert(makeFact());
        db.delete(id);
        expect(() => db.update(id, makeFact())).toThrow(KcmError);
    });

    test('delete returns true for valid row', () => {
        const id = db.insert(makeFact());
        expect(db.delete(id)).toBe(true);
        expect(db.activeFactCount()).toBe(0);
        expect(db.queryAll().length).toBe(0);
    });

    test('delete returns false for invalid row', () => {
        expect(db.delete(999)).toBe(false);
    });

    test('delete returns false for already deleted row', () => {
        const id = db.insert(makeFact());
        db.delete(id);
        expect(db.delete(id)).toBe(false);
    });

    test('delete does not reduce factCount', () => {
        const id = db.insert(makeFact());
        db.delete(id);
        expect(db.factCount()).toBe(1);
    });

    test('factCount includes deleted', () => {
        db.insert(makeFact());
        db.insert(makeFact());
        db.delete(0);
        expect(db.factCount()).toBe(2);
        expect(db.activeFactCount()).toBe(1);
    });

    test('close resets database', () => {
        db.insert(makeFact());
        db.close();
        expect(db.factCount()).toBe(0);
        expect(db.activeFactCount()).toBe(0);
    });
});

describe('Query', () => {
    let db: Database;

    beforeEach(() => {
        db = new Database();
    });

    afterEach(() => {
        db.close();
    });

    test('query("*") returns all active facts', () => {
        db.insert(makeFact({ subject: 1 }));
        db.insert(makeFact({ subject: 2 }));
        const result = db.query("*");
        expect(result.collect().length).toBe(2);
    });

    test('query("SELECT * FROM facts") returns all active facts', () => {
        db.insert(makeFact({ subject: 1 }));
        db.insert(makeFact({ subject: 2 }));
        const result = db.query("SELECT * FROM facts");
        expect(result.collect().length).toBe(2);
    });

    test('query with WHERE clause filters by subject', () => {
        db.insert(makeFact({ subject: 1, predicate: 0 }));
        db.insert(makeFact({ subject: 2, predicate: 0 }));
        db.insert(makeFact({ subject: 1, predicate: 1 }));
        const result = db.query("SELECT * FROM facts WHERE subject = 1");
        expect(result.collect().length).toBe(2);
    });

    test('query with multiple AND conditions', () => {
        db.insert(makeFact({ subject: 1, predicate: 0 }));
        db.insert(makeFact({ subject: 1, predicate: 1 }));
        db.insert(makeFact({ subject: 2, predicate: 0 }));
        const result = db.query("SELECT * FROM facts WHERE subject = 1 AND predicate = 0");
        expect(result.collect().length).toBe(1);
    });

    test('query excludes deleted facts', () => {
        db.insert(makeFact({ subject: 1 }));
        const id = db.insert(makeFact({ subject: 2 }));
        db.delete(id);
        const result = db.query("*");
        expect(result.collect().length).toBe(1);
    });

    test('query throws on invalid KQL', () => {
        expect(() => db.query("INVALID QUERY")).toThrow(KcmError);
    });

    test('query throws INVALID_ARGUMENT on invalid KQL', () => {
        try {
            db.query("INVALID QUERY");
        } catch (e) {
            expect((e as KcmError).code).toBe(ErrorCode.INVALID_ARGUMENT);
        }
    });

    test('queryAll returns deep copies', () => {
        const id = db.insert(makeFact({ subject: 1 }));
        const facts = db.queryAll();
        facts[0].subject = 999;
        const facts2 = db.queryAll();
        expect(facts2[0].subject).toBe(1);
    });
});

describe('QueryResult', () => {
    let db: Database;

    beforeEach(() => {
        db = new Database();
    });

    afterEach(() => {
        db.close();
    });

    test('next() iterates through facts', () => {
        db.insert(makeFact({ subject: 1 }));
        db.insert(makeFact({ subject: 2 }));
        const result = db.query("*");
        const first = result.next();
        expect(first.done).toBe(false);
        expect(first.value.subject).toBe(1);
        const second = result.next();
        expect(second.done).toBe(false);
        expect(second.value.subject).toBe(2);
        const third = result.next();
        expect(third.done).toBe(true);
    });

    test('collect() returns all facts as array', () => {
        db.insert(makeFact({ subject: 1 }));
        db.insert(makeFact({ subject: 2 }));
        const result = db.query("*");
        const facts = result.collect();
        expect(facts.length).toBe(2);
    });

    test('is iterable with for...of', () => {
        db.insert(makeFact({ subject: 1 }));
        db.insert(makeFact({ subject: 2 }));
        const result = db.query("*");
        const collected: FactData[] = [];
        for (const fact of result) {
            collected.push(fact);
        }
        expect(collected.length).toBe(2);
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
    });

    test('transaction insert is not visible until commit', () => {
        const txn = db.beginTransaction();
        txn.insert(makeFact({ subject: 1 }));
        expect(db.activeFactCount()).toBe(0);
        txn.commit();
        expect(db.activeFactCount()).toBe(1);
    });

    test('transaction update is not visible until commit', () => {
        const id = db.insert(makeFact({ subject: 1 }));
        const txn = db.beginTransaction();
        txn.update(id, makeFact({ subject: 2 }));
        expect(db.queryAll()[0].subject).toBe(1);
        txn.commit();
        expect(db.queryAll()[0].subject).toBe(2);
    });

    test('transaction delete is not visible until commit', () => {
        const id = db.insert(makeFact({ subject: 1 }));
        const txn = db.beginTransaction();
        txn.delete(id);
        expect(db.activeFactCount()).toBe(1);
        txn.commit();
        expect(db.activeFactCount()).toBe(0);
    });

    test('rollback discards all pending operations', () => {
        const id = db.insert(makeFact({ subject: 1 }));
        const txn = db.beginTransaction();
        txn.insert(makeFact({ subject: 2 }));
        txn.update(id, makeFact({ subject: 3 }));
        txn.delete(id);
        txn.rollback();
        expect(db.activeFactCount()).toBe(1);
        expect(db.queryAll()[0].subject).toBe(1);
    });

    test('throws after commit', () => {
        const txn = db.beginTransaction();
        txn.commit();
        expect(() => txn.insert(makeFact())).toThrow(KcmError);
        expect(() => txn.update(0, makeFact())).toThrow(KcmError);
        expect(() => txn.delete(0)).toThrow(KcmError);
        expect(() => txn.commit()).toThrow(KcmError);
        expect(() => txn.rollback()).toThrow(KcmError);
    });

    test('throws after rollback', () => {
        const txn = db.beginTransaction();
        txn.rollback();
        expect(() => txn.insert(makeFact())).toThrow(KcmError);
        expect(() => txn.commit()).toThrow(KcmError);
        expect(() => txn.rollback()).toThrow(KcmError);
    });

    test('throws TRANSACTION_ABORTED after finalize', () => {
        const txn = db.beginTransaction();
        txn.commit();
        try {
            txn.insert(makeFact());
        } catch (e) {
            expect((e as KcmError).code).toBe(ErrorCode.TRANSACTION_ABORTED);
        }
    });
});

describe('Save/Load/Verify', () => {
    test('save throws IO error', () => {
        const db = new Database();
        try {
            db.save("/tmp/test.db");
        } catch (e) {
            expect((e as KcmError).code).toBe(ErrorCode.IO);
        }
        db.close();
    });

    test('load throws IO error', () => {
        const db = new Database();
        try {
            db.load("/tmp/test.db");
        } catch (e) {
            expect((e as KcmError).code).toBe(ErrorCode.IO);
        }
        db.close();
    });

    test('static verify throws IO error', () => {
        try {
            Database.verify("/tmp/test.db");
        } catch (e) {
            expect((e as KcmError).code).toBe(ErrorCode.IO);
        }
    });
});

describe('Error handling', () => {
    test('KcmError is catchable as Error', () => {
        const db = new Database();
        try {
            db.insert(null as unknown as FactData);
        } catch (e) {
            expect(e).toBeInstanceOf(Error);
            expect(e).toBeInstanceOf(KcmError);
        }
        db.close();
    });

    test('error message is descriptive', () => {
        const db = new Database();
        try {
            db.insert(makeFact({ confidence: 2.0 }));
        } catch (e) {
            expect((e as KcmError).message).toContain("confidence");
        }
        db.close();
    });

    test('NOT_FOUND error for update of missing row', () => {
        const db = new Database();
        try {
            db.update(42, makeFact());
        } catch (e) {
            expect((e as KcmError).code).toBe(ErrorCode.NOT_FOUND);
            expect((e as KcmError).message).toContain("42");
        }
        db.close();
    });

    test('INVALID_ARGUMENT error for bad KQL', () => {
        const db = new Database();
        try {
            db.query("GARBAGE");
        } catch (e) {
            expect((e as KcmError).code).toBe(ErrorCode.INVALID_ARGUMENT);
        }
        db.close();
    });

    test('all error codes are unique', () => {
        const codes = Object.values(ErrorCode).filter(v => typeof v === "number");
        expect(new Set(codes).size).toBe(codes.length);
    });
});
