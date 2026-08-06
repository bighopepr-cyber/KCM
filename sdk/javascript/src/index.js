"use strict";
Object.defineProperty(exports, "__esModule", { value: true });

var ErrorCode;
(function (ErrorCode) {
    ErrorCode[ErrorCode["OK"] = 0] = "OK";
    ErrorCode[ErrorCode["NOT_FOUND"] = 1] = "NOT_FOUND";
    ErrorCode[ErrorCode["OUT_OF_MEMORY"] = 2] = "OUT_OF_MEMORY";
    ErrorCode[ErrorCode["INVALID_ARGUMENT"] = 3] = "INVALID_ARGUMENT";
    ErrorCode[ErrorCode["IO"] = 4] = "IO";
    ErrorCode[ErrorCode["CORRUPTED"] = 5] = "CORRUPTED";
    ErrorCode[ErrorCode["CONFLICT"] = 6] = "CONFLICT";
    ErrorCode[ErrorCode["TRANSACTION_ABORTED"] = 7] = "TRANSACTION_ABORTED";
})(ErrorCode = exports.ErrorCode || (exports.ErrorCode = {}));

var ERROR_MESSAGES = {};
ERROR_MESSAGES[ErrorCode.OK] = "OK";
ERROR_MESSAGES[ErrorCode.NOT_FOUND] = "Not found";
ERROR_MESSAGES[ErrorCode.OUT_OF_MEMORY] = "Out of memory";
ERROR_MESSAGES[ErrorCode.INVALID_ARGUMENT] = "Invalid argument";
ERROR_MESSAGES[ErrorCode.IO] = "I/O error";
ERROR_MESSAGES[ErrorCode.CORRUPTED] = "Data corrupted";
ERROR_MESSAGES[ErrorCode.CONFLICT] = "Conflict";
ERROR_MESSAGES[ErrorCode.TRANSACTION_ABORTED] = "Transaction aborted";

class KcmError extends Error {
    constructor(code, message) {
        super(message !== null && message !== void 0 ? message : ERROR_MESSAGES[code] !== null && ERROR_MESSAGES[code] !== void 0 ? ERROR_MESSAGES[code] : "Unknown error");
        this.name = "KcmError";
        this.code = code;
    }
}
exports.KcmError = KcmError;

class QueryResult {
    constructor(facts) {
        this.index = 0;
        this.facts = facts;
    }

    next() {
        if (this.index < this.facts.length) {
            return { value: this.facts[this.index++], done: false };
        }
        return { value: undefined, done: true };
    }

    collect() {
        return this.facts.map(function (f) { return Object.assign({}, f); });
    }

    [Symbol.iterator]() {
        return this;
    }
}
exports.QueryResult = QueryResult;

class Transaction {
    constructor(database) {
        this.pendingInserts = [];
        this.pendingUpdates = [];
        this.pendingDeletes = new Set();
        this.finalized = false;
        this.database = database;
    }

    insert(fact) {
        if (this.finalized) {
            throw new KcmError(ErrorCode.TRANSACTION_ABORTED, "Transaction already finalized");
        }
        var rowId = this.database._allocateRowId();
        this.pendingInserts.push({ rowId: rowId, fact: Object.assign({}, fact) });
        return rowId;
    }

    update(rowId, fact) {
        if (this.finalized) {
            throw new KcmError(ErrorCode.TRANSACTION_ABORTED, "Transaction already finalized");
        }
        this.database._validateFact(fact);
        this.pendingUpdates.push({ rowId: rowId, fact: Object.assign({}, fact) });
    }

    delete(rowId) {
        if (this.finalized) {
            throw new KcmError(ErrorCode.TRANSACTION_ABORTED, "Transaction already finalized");
        }
        this.pendingDeletes.add(rowId);
        return true;
    }

    commit() {
        if (this.finalized) {
            throw new KcmError(ErrorCode.TRANSACTION_ABORTED, "Transaction already finalized");
        }
        this.finalized = true;
        var inserts = this.pendingInserts;
        for (var i = 0; i < inserts.length; i++) {
            this.database._applyInsert(inserts[i].rowId, inserts[i].fact);
        }
        var updates = this.pendingUpdates;
        for (var i = 0; i < updates.length; i++) {
            this.database._applyUpdate(updates[i].rowId, updates[i].fact);
        }
        var deletes = this.pendingDeletes;
        deletes.forEach(function (del) {
            this.database._applyDelete(del);
        }.bind(this));
    }

    rollback() {
        if (this.finalized) {
            throw new KcmError(ErrorCode.TRANSACTION_ABORTED, "Transaction already finalized");
        }
        this.finalized = true;
        this.pendingInserts = [];
        this.pendingUpdates = [];
        this.pendingDeletes.clear();
    }
}
exports.Transaction = Transaction;

class Database {
    constructor() {
        this.facts = [];
        this.deleted = new Set();
        this.nextId = 0;
    }

    insert(fact) {
        this._validateFact(fact);
        var rowId = this.nextId++;
        this.facts.push(Object.assign({}, fact));
        return rowId;
    }

    update(rowId, fact) {
        this._validateFact(fact);
        if (rowId < 0 || rowId >= this.facts.length || this.deleted.has(rowId)) {
            throw new KcmError(ErrorCode.NOT_FOUND, "Row " + rowId + " not found");
        }
        this.facts[rowId] = Object.assign({}, fact);
    }

    delete(rowId) {
        if (rowId < 0 || rowId >= this.facts.length || this.deleted.has(rowId)) {
            return false;
        }
        this.deleted.add(rowId);
        return true;
    }

    query(kql) {
        var results = this._executeKql(kql);
        return new QueryResult(results);
    }

    queryAll() {
        return this.query("SELECT * FROM facts").collect();
    }

    factCount() {
        return this.facts.length;
    }

    activeFactCount() {
        return this.facts.length - this.deleted.size;
    }

    beginTransaction() {
        return new Transaction(this);
    }

    save(_path) {
        throw new KcmError(ErrorCode.IO, "Save is not supported in the reference implementation");
    }

    load(_path) {
        throw new KcmError(ErrorCode.IO, "Load is not supported in the reference implementation");
    }

    static verify(_path) {
        throw new KcmError(ErrorCode.IO, "Verify is not supported in the reference implementation");
    }

    close() {
        this.facts = [];
        this.deleted.clear();
        this.nextId = 0;
    }

    _allocateRowId() {
        return this.nextId++;
    }

    _applyInsert(rowId, fact) {
        while (this.facts.length <= rowId) {
            this.facts.push(null);
            this.deleted.add(this.facts.length - 1);
        }
        this.facts[rowId] = fact;
        this.deleted.delete(rowId);
    }

    _applyUpdate(rowId, fact) {
        if (rowId >= 0 && rowId < this.facts.length && !this.deleted.has(rowId)) {
            this.facts[rowId] = fact;
        }
    }

    _applyDelete(rowId) {
        if (rowId >= 0 && rowId < this.facts.length && !this.deleted.has(rowId)) {
            this.deleted.add(rowId);
        }
    }

    _validateFact(fact) {
        if (typeof fact !== "object" || fact === null) {
            throw new KcmError(ErrorCode.INVALID_ARGUMENT, "Fact must be a non-null object");
        }
        var required = [
            "subject", "predicate", "object", "confidence",
            "evidence", "timestamp", "context", "version", "priority", "owner",
        ];
        for (var i = 0; i < required.length; i++) {
            var field = required[i];
            if (typeof fact[field] !== "number") {
                throw new KcmError(ErrorCode.INVALID_ARGUMENT, field + " must be a number");
            }
        }
        if (fact.confidence < 0 || fact.confidence > 1) {
            throw new KcmError(ErrorCode.INVALID_ARGUMENT, "confidence must be in [0, 1], got " + fact.confidence);
        }
    }

    _executeKql(kql) {
        var trimmed = kql.trim();

        if (trimmed === "*" || trimmed.toUpperCase() === "SELECT * FROM FACTS") {
            return this._activeFacts();
        }

        var upper = trimmed.toUpperCase();
        var whereIndex = upper.indexOf("WHERE");
        if (whereIndex === -1) {
            throw new KcmError(ErrorCode.INVALID_ARGUMENT, "Invalid KQL: " + kql);
        }

        var whereClause = trimmed.substring(whereIndex + 5).trim();
        var filters = this._parseWhereClause(whereClause);
        var all = this._activeFacts();

        return all.filter(function (f) {
            for (var i = 0; i < filters.length; i++) {
                var field = filters[i][0];
                var value = filters[i][1];
                var fieldValue = f[field];
                if (fieldValue !== value) return false;
            }
            return true;
        });
    }

    _activeFacts() {
        var result = [];
        for (var i = 0; i < this.facts.length; i++) {
            if (!this.deleted.has(i)) {
                result.push(Object.assign({}, this.facts[i]));
            }
        }
        return result;
    }

    _parseWhereClause(clause) {
        var filters = [];
        var parts = clause.split(/\s+AND\s+/i);

        for (var i = 0; i < parts.length; i++) {
            var match = parts[i].trim().match(/^(\w+)\s*=\s*(-?\d+(?:\.\d+)?)$/);
            if (!match) {
                throw new KcmError(ErrorCode.INVALID_ARGUMENT, "Invalid WHERE condition: " + parts[i]);
            }
            filters.push([match[1].toLowerCase(), Number(match[2])]);
        }

        return filters;
    }
}
exports.Database = Database;
exports.default = Database;
