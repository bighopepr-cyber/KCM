"use strict";
Object.defineProperty(exports, "__esModule", { value: true });

class DatabaseError extends Error {
    constructor(message) {
        super(message);
        this.name = "DatabaseError";
    }
}

class Database {
    constructor() {
        this.facts = [];
        this.deleted = new Set();
        this.nextId = 0;
        this.dictionary = new Map();
        this.dictCounter = 0;
    }

    insert(fact) {
        if (fact.confidence < 0 || fact.confidence > 1) {
            throw new DatabaseError("Confidence must be in [0, 1], got " + fact.confidence);
        }
        const rowId = this.nextId++;
        this.facts.push({ ...fact });
        return rowId;
    }

    queryAll() {
        return this.facts.filter((_, i) => !this.deleted.has(i)).map(f => ({ ...f }));
    }

    query(options = {}) {
        return this.queryAll().filter(f => {
            if (options.subject !== undefined && f.subject !== options.subject) return false;
            if (options.predicate !== undefined && f.predicate !== options.predicate) return false;
            if (options.object !== undefined && f.object !== options.object) return false;
            if (options.confidenceMin !== undefined && f.confidence < options.confidenceMin) return false;
            return true;
        });
    }

    getFact(rowId) {
        if (rowId < 0 || rowId >= this.facts.length || this.deleted.has(rowId)) return null;
        return { ...this.facts[rowId] };
    }

    delete(rowId) {
        if (rowId < 0 || rowId >= this.facts.length || this.deleted.has(rowId)) return false;
        this.deleted.add(rowId);
        return true;
    }

    factCount() { return this.facts.length; }
    activeFactCount() { return this.facts.length - this.deleted.size; }

    dictInsertSubject(name) {
        if (!this.dictionary.has(name)) this.dictionary.set(name, this.dictCounter++);
        return this.dictionary.get(name);
    }
    dictLookupSubject(name) { return this.dictionary.get(name); }
    dictGetSubject(dictId) {
        for (const [name, id] of this.dictionary) { if (id === dictId) return name; }
        return undefined;
    }

    stats() {
        return { factCount: this.factCount(), activeCount: this.activeFactCount(), memoryBytes: this.factCount() * 34 };
    }

    close() { this.facts = []; this.deleted.clear(); this.dictionary.clear(); }
}

exports.Database = Database;
exports.DatabaseError = DatabaseError;
exports.default = Database;
