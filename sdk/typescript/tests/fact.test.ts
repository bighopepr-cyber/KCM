import { Fact, QueryBuilder } from '../src/index';

describe('Fact', () => {
    test('creates fact with valid values', () => {
        const f = Fact.create(1, 0, 2, 0.95);
        expect(f.subject).toBe(1);
        expect(f.predicate).toBe(0);
        expect(f.object).toBe(2);
        expect(f.confidence).toBe(0.95);
    });

    test('rejects invalid confidence', () => {
        expect(() => Fact.create(1, 0, 2, 1.5)).toThrow(RangeError);
    });

    test('toData returns plain object', () => {
        const f = Fact.create(1, 0, 2, 0.95);
        expect(f.toData()).toEqual({ subject: 1, predicate: 0, object: 2, confidence: 0.95 });
    });
});

describe('QueryBuilder', () => {
    test('chains filters', () => {
        const q = QueryBuilder.create().withSubject(1).withConfidenceMin(0.9).build();
        expect(q).toEqual({ subject: 1, confidenceMin: 0.9 });
    });
});
