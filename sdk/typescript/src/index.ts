/**
 * KCM TypeScript SDK - Typed wrapper over JavaScript SDK
 */

export { Database, DatabaseError } from '../../javascript/src/index';
export type { FactData, QueryOptions, Stats } from '../../javascript/src/index';

export class Fact {
    readonly subject: number;
    readonly predicate: number;
    readonly object: number;
    readonly confidence: number;

    private constructor(subject: number, predicate: number, object: number, confidence: number) {
        this.subject = subject;
        this.predicate = predicate;
        this.object = object;
        this.confidence = confidence;
    }

    static create(subject: number, predicate: number, object: number, confidence: number): Fact {
        if (confidence < 0 || confidence > 1) {
            throw new RangeError(`Confidence must be in [0, 1], got ${confidence}`);
        }
        return new Fact(subject, predicate, object, confidence);
    }

    toData() {
        return { subject: this.subject, predicate: this.predicate, object: this.object, confidence: this.confidence };
    }
}

export class QueryBuilder {
    private _subject?: number;
    private _predicate?: number;
    private _object?: number;
    private _confidenceMin?: number;

    static create(): QueryBuilder { return new QueryBuilder(); }
    withSubject(s: number): this { this._subject = s; return this; }
    withPredicate(p: number): this { this._predicate = p; return this; }
    withObject(o: number): this { this._object = o; return this; }
    withConfidenceMin(m: number): this { this._confidenceMin = m; return this; }
    build() { return { subject: this._subject, predicate: this._predicate, object: this._object, confidenceMin: this._confidenceMin }; }
}
