/**
 * KCM TypeScript SDK - Standalone reference implementation
 * Aligns with SSOT API specification (KCM_API_SPEC.md §2)
 */

export enum ErrorCode {
    OK = 0,
    NotFound = 1,
    OutOfMemory = 2,
    InvalidArgument = 3,
    Io = 4,
    Corrupted = 5,
    Conflict = 6,
    TransactionAborted = 7,
}

export class KcmError extends Error {
    readonly code: ErrorCode;

    constructor(code: ErrorCode, message: string) {
        super(message);
        this.name = 'KcmError';
        this.code = code;
    }
}

export interface FactData {
    subject: number;
    predicate: number;
    object: number;
    confidence: number;
    evidence: number;
    timestamp: number;
    context: number;
    version: number;
    priority: number;
    owner: number;
}

export function validateFact(fact: FactData): void {
    if (typeof fact.subject !== 'number' || !Number.isInteger(fact.subject)) {
        throw new KcmError(ErrorCode.InvalidArgument, 'subject must be an integer');
    }
    if (typeof fact.predicate !== 'number' || !Number.isInteger(fact.predicate)) {
        throw new KcmError(ErrorCode.InvalidArgument, 'predicate must be an integer');
    }
    if (typeof fact.object !== 'number' || !Number.isInteger(fact.object)) {
        throw new KcmError(ErrorCode.InvalidArgument, 'object must be an integer');
    }
    if (typeof fact.confidence !== 'number' || fact.confidence < 0 || fact.confidence > 1) {
        throw new KcmError(ErrorCode.InvalidArgument, 'confidence must be in [0, 1]');
    }
    if (typeof fact.evidence !== 'number' || !Number.isInteger(fact.evidence)) {
        throw new KcmError(ErrorCode.InvalidArgument, 'evidence must be an integer');
    }
    if (typeof fact.timestamp !== 'number' || !Number.isInteger(fact.timestamp)) {
        throw new KcmError(ErrorCode.InvalidArgument, 'timestamp must be an integer');
    }
    if (typeof fact.context !== 'number' || !Number.isInteger(fact.context)) {
        throw new KcmError(ErrorCode.InvalidArgument, 'context must be an integer');
    }
    if (typeof fact.version !== 'number' || !Number.isInteger(fact.version)) {
        throw new KcmError(ErrorCode.InvalidArgument, 'version must be an integer');
    }
    if (typeof fact.priority !== 'number' || !Number.isInteger(fact.priority)) {
        throw new KcmError(ErrorCode.InvalidArgument, 'priority must be an integer');
    }
    if (typeof fact.owner !== 'number' || !Number.isInteger(fact.owner)) {
        throw new KcmError(ErrorCode.InvalidArgument, 'owner must be an integer');
    }
}

function cloneFact(fact: FactData): FactData {
    return { ...fact };
}

interface InternalFact {
    data: FactData;
    deleted: boolean;
}

class QueryIterator {
    private readonly facts: FactData[];
    private index: number;

    constructor(facts: FactData[]) {
        this.facts = facts;
        this.index = 0;
    }

    next(): { value: FactData | undefined; done: boolean } {
        if (this.index >= this.facts.length) {
            return { value: undefined, done: true };
        }
        const value = this.facts[this.index];
        this.index++;
        return { value, done: false };
    }
}

export class QueryResult {
    private readonly facts: FactData[];
    private iterator: QueryIterator | null = null;

    constructor(facts: FactData[]) {
        this.facts = facts;
    }

    next(): FactData | undefined {
        if (this.iterator === null) {
            this.iterator = new QueryIterator(this.facts);
        }
        const result = this.iterator.next();
        return result.value;
    }

    collect(): FactData[] {
        return this.facts.map(cloneFact);
    }

    get count(): number {
        return this.facts.length;
    }
}

export class Transaction {
    private readonly database: Database;
    private pendingInserts: Array<{ fact: FactData; rowId: number }> = [];
    private pendingUpdates: Array<{ rowId: number; fact: FactData }> = [];
    private pendingDeletes: number[] = [];
    private committed = false;
    private rolledBack = false;

    constructor(database: Database) {
        this.database = database;
    }

    recordInsert(fact: FactData, rowId: number): void {
        this.pendingInserts.push({ fact, rowId });
    }

    recordUpdate(rowId: number, fact: FactData): void {
        this.pendingUpdates.push({ rowId, fact });
    }

    recordDelete(rowId: number): void {
        this.pendingDeletes.push(rowId);
    }

    commit(): void {
        if (this.committed || this.rolledBack) {
            throw new KcmError(ErrorCode.TransactionAborted, 'transaction already finalized');
        }
        this.database.applyTransaction(this);
        this.committed = true;
    }

    rollback(): void {
        if (this.committed || this.rolledBack) {
            throw new KcmError(ErrorCode.TransactionAborted, 'transaction already finalized');
        }
        this.pendingInserts = [];
        this.pendingUpdates = [];
        this.pendingDeletes = [];
        this.rolledBack = true;
    }

    isFinalized(): boolean {
        return this.committed || this.rolledBack;
    }

    getInserts(): Array<{ fact: FactData; rowId: number }> {
        return this.pendingInserts;
    }

    getUpdates(): Array<{ rowId: number; fact: FactData }> {
        return this.pendingUpdates;
    }

    getDeletes(): number[] {
        return this.pendingDeletes;
    }
}

export class Database {
    private facts: InternalFact[] = [];
    private nextRowId = 0;
    private closed = false;

    insert(fact: FactData): number {
        this.ensureOpen();
        validateFact(fact);
        const rowId = this.nextRowId++;
        this.facts.push({ data: cloneFact(fact), deleted: false });
        return rowId;
    }

    update(rowId: number, fact: FactData): void {
        this.ensureOpen();
        this.ensureValidRowId(rowId);
        validateFact(fact);
        this.facts[rowId].data = cloneFact(fact);
    }

    delete(rowId: number): boolean {
        this.ensureOpen();
        if (rowId < 0 || rowId >= this.facts.length || this.facts[rowId].deleted) {
            return false;
        }
        this.facts[rowId].deleted = true;
        return true;
    }

    query(_kql: string): QueryResult {
        this.ensureOpen();
        const results = this.facts
            .filter(f => !f.deleted)
            .map(f => cloneFact(f.data));
        return new QueryResult(results);
    }

    queryAll(): FactData[] {
        this.ensureOpen();
        return this.facts
            .filter(f => !f.deleted)
            .map(f => cloneFact(f.data));
    }

    factCount(): number {
        this.ensureOpen();
        return this.facts.length;
    }

    activeFactCount(): number {
        this.ensureOpen();
        return this.facts.filter(f => !f.deleted).length;
    }

    beginTransaction(): Transaction {
        this.ensureOpen();
        return new Transaction(this);
    }

    applyTransaction(txn: Transaction): void {
        for (const { fact, rowId } of txn.getInserts()) {
            if (rowId < this.facts.length) {
                this.facts[rowId] = { data: cloneFact(fact), deleted: false };
            } else {
                while (this.facts.length <= rowId) {
                    this.facts.push({ data: cloneFact(fact), deleted: true });
                }
                this.facts[rowId] = { data: cloneFact(fact), deleted: false };
            }
        }
        for (const { rowId, fact } of txn.getUpdates()) {
            if (rowId >= 0 && rowId < this.facts.length && !this.facts[rowId].deleted) {
                this.facts[rowId].data = cloneFact(fact);
            }
        }
        for (const rowId of txn.getDeletes()) {
            if (rowId >= 0 && rowId < this.facts.length) {
                this.facts[rowId].deleted = true;
            }
        }
    }

    save(_path: string): void {
        this.ensureOpen();
        if (typeof _path !== 'string' || _path.length === 0) {
            throw new KcmError(ErrorCode.InvalidArgument, 'path must be a non-empty string');
        }
        const data = JSON.stringify({
            version: 1,
            facts: this.facts
                .filter(f => !f.deleted)
                .map(f => f.data),
        });
        const fs = require('fs');
        fs.writeFileSync(_path, data, 'utf-8');
    }

    load(_path: string): void {
        this.ensureOpen();
        if (typeof _path !== 'string' || _path.length === 0) {
            throw new KcmError(ErrorCode.InvalidArgument, 'path must be a non-empty string');
        }
        const fs = require('fs');
        if (!fs.existsSync(_path)) {
            throw new KcmError(ErrorCode.NotFound, `file not found: ${_path}`);
        }
        let raw: string;
        try {
            raw = fs.readFileSync(_path, 'utf-8');
        } catch {
            throw new KcmError(ErrorCode.Io, `failed to read file: ${_path}`);
        }
        let parsed: { version: number; facts: FactData[] };
        try {
            parsed = JSON.parse(raw);
        } catch {
            throw new KcmError(ErrorCode.Corrupted, `failed to parse file: ${_path}`);
        }
        if (!parsed || !Array.isArray(parsed.facts)) {
            throw new KcmError(ErrorCode.Corrupted, `invalid file format: ${_path}`);
        }
        this.facts = [];
        this.nextRowId = 0;
        for (const fact of parsed.facts) {
            validateFact(fact);
            this.facts.push({ data: cloneFact(fact), deleted: false });
            this.nextRowId++;
        }
    }

    static verify(_path: string): void {
        if (typeof _path !== 'string' || _path.length === 0) {
            throw new KcmError(ErrorCode.InvalidArgument, 'path must be a non-empty string');
        }
        const fs = require('fs');
        if (!fs.existsSync(_path)) {
            throw new KcmError(ErrorCode.NotFound, `file not found: ${_path}`);
        }
        let raw: string;
        try {
            raw = fs.readFileSync(_path, 'utf-8');
        } catch {
            throw new KcmError(ErrorCode.Io, `failed to read file: ${_path}`);
        }
        let parsed: { version: number; facts: FactData[] };
        try {
            parsed = JSON.parse(raw);
        } catch {
            throw new KcmError(ErrorCode.Corrupted, `failed to parse file: ${_path}`);
        }
        if (!parsed || !Array.isArray(parsed.facts)) {
            throw new KcmError(ErrorCode.Corrupted, `invalid file format: ${_path}`);
        }
        for (const fact of parsed.facts) {
            validateFact(fact);
        }
    }

    close(): void {
        this.facts = [];
        this.nextRowId = 0;
        this.closed = true;
    }

    private ensureOpen(): void {
        if (this.closed) {
            throw new KcmError(ErrorCode.InvalidArgument, 'database is closed');
        }
    }

    private ensureValidRowId(rowId: number): void {
        if (rowId < 0 || rowId >= this.facts.length) {
            throw new KcmError(ErrorCode.NotFound, `row not found: ${rowId}`);
        }
        if (this.facts[rowId].deleted) {
            throw new KcmError(ErrorCode.NotFound, `row is deleted: ${rowId}`);
        }
    }
}
