import { Database, FactData, DatabaseError } from '../src/index';

describe('Database', () => {
    let db: Database;

    beforeEach(() => {
        db = new Database();
    });

    afterEach(() => {
        db.close();
    });

    test('should create empty database', () => {
        expect(db.factCount()).toBe(0);
        expect(db.activeFactCount()).toBe(0);
    });

    test('should insert fact', () => {
        const id = db.insert({ subject: 1, predicate: 0, object: 2, confidence: 0.95 });
        expect(id).toBe(0);
        expect(db.factCount()).toBe(1);
    });

    test('should insert multiple facts', () => {
        db.insert({ subject: 1, predicate: 0, object: 2, confidence: 0.95 });
        db.insert({ subject: 2, predicate: 1, object: 3, confidence: 0.90 });
        expect(db.factCount()).toBe(2);
    });

    test('should query all facts', () => {
        db.insert({ subject: 1, predicate: 0, object: 2, confidence: 0.95 });
        db.insert({ subject: 2, predicate: 1, object: 3, confidence: 0.90 });
        const facts = db.queryAll();
        expect(facts.length).toBe(2);
    });

    test('should query with filter', () => {
        db.insert({ subject: 1, predicate: 0, object: 2, confidence: 0.95 });
        db.insert({ subject: 2, predicate: 0, object: 3, confidence: 0.90 });
        db.insert({ subject: 1, predicate: 1, object: 4, confidence: 0.85 });
        const facts = db.query({ subject: 1 });
        expect(facts.length).toBe(2);
    });

    test('should delete fact', () => {
        const id = db.insert({ subject: 1, predicate: 0, object: 2, confidence: 0.95 });
        expect(db.activeFactCount()).toBe(1);
        db.delete(id);
        expect(db.activeFactCount()).toBe(0);
        expect(db.queryAll().length).toBe(0);
    });

    test('should get fact by id', () => {
        const id = db.insert({ subject: 1, predicate: 0, object: 2, confidence: 0.95 });
        const fact = db.getFact(id);
        expect(fact).not.toBeNull();
        expect(fact!.subject).toBe(1);
    });

    test('should reject invalid confidence', () => {
        expect(() => {
            db.insert({ subject: 1, predicate: 0, object: 2, confidence: 1.5 });
        }).toThrow(DatabaseError);
    });

    test('should manage dictionary', () => {
        const id1 = db.dictInsertSubject('planet');
        const id2 = db.dictInsertSubject('star');
        expect(id1).toBe(0);
        expect(id2).toBe(1);
        expect(db.dictLookupSubject('planet')).toBe(0);
        expect(db.dictGetSubject(1)).toBe('star');
    });

    test('should return stats', () => {
        db.insert({ subject: 1, predicate: 0, object: 2, confidence: 0.95 });
        const stats = db.stats();
        expect(stats.factCount).toBe(1);
        expect(stats.memoryBytes).toBe(34);
    });
});
