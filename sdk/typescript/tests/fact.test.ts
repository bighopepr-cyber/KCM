import { Database, KcmError, ErrorCode, FactData, validateFact } from '../src/index';

describe('FactData validation', () => {
    test('valid fact passes validation', () => {
        const fact: FactData = {
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
        };
        expect(() => validateFact(fact)).not.toThrow();
    });

    test('rejects non-number subject', () => {
        expect(() => validateFact({
            subject: 'abc' as any,
            predicate: 0,
            object: 2,
            confidence: 0.95,
            evidence: 1,
            timestamp: 0,
            context: 1,
            version: 1,
            priority: 0,
            owner: 1,
        })).toThrow(KcmError);
    });

    test('rejects float subject', () => {
        expect(() => validateFact({
            subject: 1.5,
            predicate: 0,
            object: 2,
            confidence: 0.95,
            evidence: 1,
            timestamp: 0,
            context: 1,
            version: 1,
            priority: 0,
            owner: 1,
        })).toThrow(KcmError);
    });

    test('rejects confidence out of range', () => {
        expect(() => validateFact({
            subject: 1,
            predicate: 0,
            object: 2,
            confidence: -0.1,
            evidence: 1,
            timestamp: 0,
            context: 1,
            version: 1,
            priority: 0,
            owner: 1,
        })).toThrow(KcmError);
        expect(() => validateFact({
            subject: 1,
            predicate: 0,
            object: 2,
            confidence: 1.1,
            evidence: 1,
            timestamp: 0,
            context: 1,
            version: 1,
            priority: 0,
            owner: 1,
        })).toThrow(KcmError);
    });

    test('accepts negative integer fields', () => {
        const fact: FactData = {
            subject: 1,
            predicate: 0,
            object: 2,
            confidence: 0.5,
            evidence: 1,
            timestamp: -100,
            context: 1,
            version: -1,
            priority: -5,
            owner: 1,
        };
        expect(() => validateFact(fact)).not.toThrow();
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

    test('collect returns all facts', () => {
        db.insert({
            subject: 1,
            predicate: 0,
            object: 2,
            confidence: 0.95,
            evidence: 1,
            timestamp: 0,
            context: 1,
            version: 1,
            priority: 0,
            owner: 1,
        });
        const result = db.query('SELECT * FROM facts');
        expect(result.collect().length).toBe(1);
    });

    test('count matches collect length', () => {
        db.insert({
            subject: 1,
            predicate: 0,
            object: 2,
            confidence: 0.95,
            evidence: 1,
            timestamp: 0,
            context: 1,
            version: 1,
            priority: 0,
            owner: 1,
        });
        db.insert({
            subject: 2,
            predicate: 0,
            object: 3,
            confidence: 0.8,
            evidence: 2,
            timestamp: 0,
            context: 2,
            version: 1,
            priority: 1,
            owner: 2,
        });
        const result = db.query('SELECT * FROM facts');
        expect(result.count).toBe(2);
        expect(result.collect().length).toBe(2);
    });
});
