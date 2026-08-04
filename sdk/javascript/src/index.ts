/**
 * KCM Knowledge Columnar Model - JavaScript/TypeScript SDK
 * 
 * Pure TypeScript implementation that mirrors the core engine API.
 * In production, this would use N-API bindings to the Rust engine.
 */

export interface FactData {
    subject: number;
    predicate: number;
    object: number;
    confidence: number;
    evidence?: number;
    timestamp?: number;
    context?: number;
    version?: number;
    priority?: number;
    owner?: number;
}

export interface QueryOptions {
    subject?: number;
    predicate?: number;
    object?: number;
    confidenceMin?: number;
}

export interface Stats {
    factCount: number;
    activeCount: number;
    memoryBytes: number;
}

export class DatabaseError extends Error {
    constructor(message: string) {
        super(message);
        this.name = "DatabaseError";
    }
}

export class Database {
    private facts: FactData[] = [];
    private deleted: Set<number> = new Set();
    private nextId: number = 0;
    private dictionary: Map<string, number> = new Map();
    private dictCounter: number = 0;

    /**
     * Insert a fact into the database.
     * @returns The row ID of the inserted fact.
     */
    insert(fact: FactData): number {
        if (fact.confidence < 0 || fact.confidence > 1) {
            throw new DatabaseError(`Confidence must be in [0, 1], got ${fact.confidence}`);
        }
        const rowId = this.nextId++;
        this.facts.push({ ...fact });
        return rowId;
    }

    /**
     * Query all active facts.
     */
    queryAll(): FactData[] {
        return this.facts
            .filter((_, i) => !this.deleted.has(i))
            .map(f => ({ ...f }));
    }

    /**
     * Query facts with filters.
     */
    query(options: QueryOptions = {}): FactData[] {
        return this.queryAll().filter(f => {
            if (options.subject !== undefined && f.subject !== options.subject) return false;
            if (options.predicate !== undefined && f.predicate !== options.predicate) return false;
            if (options.object !== undefined && f.object !== options.object) return false;
            if (options.confidenceMin !== undefined && f.confidence < options.confidenceMin) return false;
            return true;
        });
    }

    /**
     * Get a fact by row ID.
     */
    getFact(rowId: number): FactData | null {
        if (rowId < 0 || rowId >= this.facts.length || this.deleted.has(rowId)) {
            return null;
        }
        return { ...this.facts[rowId] };
    }

    /**
     * Delete a fact by row ID.
     */
    delete(rowId: number): boolean {
        if (rowId < 0 || rowId >= this.facts.length || this.deleted.has(rowId)) {
            return false;
        }
        this.deleted.add(rowId);
        return true;
    }

    /**
     * Get total fact count (including deleted).
     */
    factCount(): number {
        return this.facts.length;
    }

    /**
     * Get active (non-deleted) fact count.
     */
    activeFactCount(): number {
        return this.facts.length - this.deleted.size;
    }

    /**
     * Insert a subject into the dictionary.
     * @returns Dictionary ID for the subject.
     */
    dictInsertSubject(name: string): number {
        if (!this.dictionary.has(name)) {
            this.dictionary.set(name, this.dictCounter++);
        }
        return this.dictionary.get(name)!;
    }

    /**
     * Look up a subject in the dictionary.
     */
    dictLookupSubject(name: string): number | undefined {
        return this.dictionary.get(name);
    }

    /**
     * Get subject name by dictionary ID.
     */
    dictGetSubject(dictId: number): string | undefined {
        for (const [name, id] of this.dictionary) {
            if (id === dictId) return name;
        }
        return undefined;
    }

    /**
     * Get database statistics.
     */
    stats(): Stats {
        return {
            factCount: this.factCount(),
            activeCount: this.activeFactCount(),
            memoryBytes: this.factCount() * 34,
        };
    }

    /**
     * Close the database and free resources.
     */
    close(): void {
        this.facts = [];
        this.deleted.clear();
        this.dictionary.clear();
    }
}

export default Database;
