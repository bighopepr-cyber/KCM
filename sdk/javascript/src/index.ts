/**
 * KCM Knowledge Columnar Model - JavaScript/TypeScript SDK
 *
 * Pure in-memory reference implementation that mirrors the core engine API.
 * In production, this would use N-API bindings to the Rust engine.
 */

export enum ErrorCode {
    OK = 0,
    NOT_FOUND = 1,
    OUT_OF_MEMORY = 2,
    INVALID_ARGUMENT = 3,
    IO = 4,
    CORRUPTED = 5,
    CONFLICT = 6,
    TRANSACTION_ABORTED = 7,
}

const ERROR_MESSAGES: Record<ErrorCode, string> = {
    [ErrorCode.OK]: "OK",
    [ErrorCode.NOT_FOUND]: "Not found",
    [ErrorCode.OUT_OF_MEMORY]: "Out of memory",
    [ErrorCode.INVALID_ARGUMENT]: "Invalid argument",
    [ErrorCode.IO]: "I/O error",
    [ErrorCode.CORRUPTED]: "Data corrupted",
    [ErrorCode.CONFLICT]: "Conflict",
    [ErrorCode.TRANSACTION_ABORTED]: "Transaction aborted",
};

export class KcmError extends Error {
    code: ErrorCode;

    constructor(code: ErrorCode, message?: string) {
        super(message ?? ERROR_MESSAGES[code] ?? "Unknown error");
        this.name = "KcmError";
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

export class QueryResult {
    private facts: FactData[];
    private index: number = 0;

    constructor(facts: FactData[]) {
        this.facts = facts;
    }

    next(): IteratorResult<FactData> {
        if (this.index < this.facts.length) {
            return { value: this.facts[this.index++], done: false };
        }
        return { value: undefined as unknown as FactData, done: true };
    }

    collect(): FactData[] {
        return this.facts.map(f => ({ ...f }));
    }

    [Symbol.iterator](): Iterator<FactData> {
        return this;
    }
}

export class Transaction {
    private database: Database;
    private pendingInserts: { rowId: number; fact: FactData }[] = [];
    private pendingUpdates: { rowId: number; fact: FactData }[] = [];
    private pendingDeletes: Set<number> = new Set();
    private finalized: boolean = false;

    constructor(database: Database) {
        this.database = database;
    }

    insert(fact: FactData): number {
        if (this.finalized) {
            throw new KcmError(ErrorCode.TRANSACTION_ABORTED, "Transaction already finalized");
        }
        const rowId = this.database._allocateRowId();
        this.pendingInserts.push({ rowId, fact: { ...fact } });
        return rowId;
    }

    update(rowId: number, fact: FactData): void {
        if (this.finalized) {
            throw new KcmError(ErrorCode.TRANSACTION_ABORTED, "Transaction already finalized");
        }
        this.database._validateFact(fact);
        this.pendingUpdates.push({ rowId, fact: { ...fact } });
    }

    delete(rowId: number): boolean {
        if (this.finalized) {
            throw new KcmError(ErrorCode.TRANSACTION_ABORTED, "Transaction already finalized");
        }
        this.pendingDeletes.add(rowId);
        return true;
    }

    commit(): void {
        if (this.finalized) {
            throw new KcmError(ErrorCode.TRANSACTION_ABORTED, "Transaction already finalized");
        }
        this.finalized = true;
        for (const ins of this.pendingInserts) {
            this.database._applyInsert(ins.rowId, ins.fact);
        }
        for (const upd of this.pendingUpdates) {
            this.database._applyUpdate(upd.rowId, upd.fact);
        }
        for (const del of this.pendingDeletes) {
            this.database._applyDelete(del);
        }
    }

    rollback(): void {
        if (this.finalized) {
            throw new KcmError(ErrorCode.TRANSACTION_ABORTED, "Transaction already finalized");
        }
        this.finalized = true;
        this.pendingInserts = [];
        this.pendingUpdates = [];
        this.pendingDeletes.clear();
    }
}

export class Database {
    private facts: FactData[] = [];
    private deleted: Set<number> = new Set();
    private nextId: number = 0;

    insert(fact: FactData): number {
        this._validateFact(fact);
        const rowId = this.nextId++;
        this.facts.push({ ...fact });
        return rowId;
    }

    update(rowId: number, fact: FactData): void {
        this._validateFact(fact);
        if (rowId < 0 || rowId >= this.facts.length || this.deleted.has(rowId)) {
            throw new KcmError(ErrorCode.NOT_FOUND, `Row ${rowId} not found`);
        }
        this.facts[rowId] = { ...fact };
    }

    delete(rowId: number): boolean {
        if (rowId < 0 || rowId >= this.facts.length || this.deleted.has(rowId)) {
            return false;
        }
        this.deleted.add(rowId);
        return true;
    }

    query(kql: string): QueryResult {
        const results = this._executeKql(kql);
        return new QueryResult(results);
    }

    queryAll(): FactData[] {
        return this.query("SELECT * FROM facts").collect();
    }

    factCount(): number {
        return this.facts.length;
    }

    activeFactCount(): number {
        return this.facts.length - this.deleted.size;
    }

    beginTransaction(): Transaction {
        return new Transaction(this);
    }

    save(_path: string): void {
        throw new KcmError(ErrorCode.IO, "Save is not supported in the reference implementation");
    }

    load(_path: string): void {
        throw new KcmError(ErrorCode.IO, "Load is not supported in the reference implementation");
    }

    static verify(_path: string): void {
        throw new KcmError(ErrorCode.IO, "Verify is not supported in the reference implementation");
    }

    close(): void {
        this.facts = [];
        this.deleted.clear();
        this.nextId = 0;
    }

    /** @internal */
    _allocateRowId(): number {
        return this.nextId++;
    }

    /** @internal */
    _applyInsert(rowId: number, fact: FactData): void {
        while (this.facts.length <= rowId) {
            this.facts.push(null as unknown as FactData);
            this.deleted.add(this.facts.length - 1);
        }
        this.facts[rowId] = fact;
        this.deleted.delete(rowId);
    }

    /** @internal */
    _applyUpdate(rowId: number, fact: FactData): void {
        if (rowId >= 0 && rowId < this.facts.length && !this.deleted.has(rowId)) {
            this.facts[rowId] = fact;
        }
    }

    /** @internal */
    _applyDelete(rowId: number): void {
        if (rowId >= 0 && rowId < this.facts.length && !this.deleted.has(rowId)) {
            this.deleted.add(rowId);
        }
    }

    /** @internal */
    _validateFact(fact: FactData): void {
        if (typeof fact !== "object" || fact === null) {
            throw new KcmError(ErrorCode.INVALID_ARGUMENT, "Fact must be a non-null object");
        }
        const required: (keyof FactData)[] = [
            "subject", "predicate", "object", "confidence",
            "evidence", "timestamp", "context", "version", "priority", "owner",
        ];
        for (const field of required) {
            if (typeof fact[field] !== "number") {
                throw new KcmError(ErrorCode.INVALID_ARGUMENT, `${field} must be a number`);
            }
        }
        if (fact.confidence < 0 || fact.confidence > 1) {
            throw new KcmError(ErrorCode.INVALID_ARGUMENT, `confidence must be in [0, 1], got ${fact.confidence}`);
        }
    }

    private _executeKql(kql: string): FactData[] {
        const trimmed = kql.trim();

        if (trimmed === "*" || trimmed.toUpperCase() === "SELECT * FROM FACTS") {
            return this._activeFacts();
        }

        const upper = trimmed.toUpperCase();
        const whereIndex = upper.indexOf("WHERE");
        if (whereIndex === -1) {
            throw new KcmError(ErrorCode.INVALID_ARGUMENT, `Invalid KQL: ${kql}`);
        }

        const whereClause = trimmed.substring(whereIndex + 5).trim();
        const filters = this._parseWhereClause(whereClause);
        const all = this._activeFacts();

        return all.filter(f => {
            for (const [field, value] of filters) {
                const fieldValue = (f as unknown as Record<string, number>)[field];
                if (fieldValue !== value) return false;
            }
            return true;
        });
    }

    private _activeFacts(): FactData[] {
        const result: FactData[] = [];
        for (let i = 0; i < this.facts.length; i++) {
            if (!this.deleted.has(i)) {
                result.push({ ...this.facts[i] });
            }
        }
        return result;
    }

    private _parseWhereClause(clause: string): [string, number][] {
        const filters: [string, number][] = [];
        const parts = clause.split(/\s+AND\s+/i);

        for (const part of parts) {
            const match = part.trim().match(/^(\w+)\s*=\s*(-?\d+(?:\.\d+)?)$/);
            if (!match) {
                throw new KcmError(ErrorCode.INVALID_ARGUMENT, `Invalid WHERE condition: ${part}`);
            }
            filters.push([match[1].toLowerCase(), Number(match[2])]);
        }

        return filters;
    }
}

export default Database;
