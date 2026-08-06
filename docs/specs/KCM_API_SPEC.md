# KCM API Specification

**Document ID:** KCM-API-001
**Version:** 1.0.0
**Status:** Active
**Owner:** Specification Lock (P4)
**Authority:** P3 (PRD2.md §9)

---

## 1. Purpose

Defines KCM's public API contracts: C FFI, REST endpoints, gRPC RPCs, Python bindings, and KQL parser.

## 2. C FFI (18 Functions)

All functions use `extern "C"` ABI. Opaque types prevent direct struct access.

### 2.1 Type Definitions

```c
typedef struct KCM_Database KCM_Database;
typedef struct KCM_Transaction KCM_Transaction;
typedef struct KCM_Query KCM_Query;

typedef struct {
    uint32_t subject;
    uint8_t  predicate;
    uint32_t object;
    double   confidence;
    uint8_t  evidence;
    int64_t  timestamp;
    uint8_t  context;
    int32_t  version;
    int8_t   priority;
    uint16_t owner;
} KCM_Fact;

typedef enum {
    KCM_OK = 0,
    KCM_ERR_NOT_FOUND = 1,
    KCM_ERR_OUT_OF_MEMORY = 2,
    KCM_ERR_INVALID_ARGUMENT = 3,
    KCM_ERR_IO = 4,
    KCM_ERR_CORRUPTED = 5,
    KCM_ERR_CONFLICT = 6,
    KCM_ERR_TRANSACTION_ABORTED = 7,
} KCM_Error;
```

### 2.2 Function Signatures

| # | Function | Signature | Description |
|---|----------|-----------|-------------|
| 1 | `KCM_DatabaseNew` | `(db_out: *mut *mut KCM_Database) -> KCM_Error` | Create database |
| 2 | `KCM_DatabaseFree` | `(db: *mut KCM_Database)` | Destroy database |
| 3 | `KCM_DatabaseInsert` | `(db: *mut KCM_Database, fact: *const KCM_Fact) -> KCM_Error` | Insert fact |
| 4 | `KCM_DatabaseUpdate` | `(db: *mut KCM_Database, row_id: u64, fact: *const KCM_Fact) -> KCM_Error` | Update fact |
| 5 | `KCM_DatabaseDelete` | `(db: *mut KCM_Database, row_id: u64) -> KCM_Error` | Delete fact |
| 6 | `KCM_DatabaseFactCount` | `(db: *mut KCM_Database) -> u64` | Get total count |
| 7 | `KCM_DatabaseActiveCount` | `(db: *mut KCM_Database) -> u64` | Get active count |
| 8 | `KCM_DatabaseQuery` | `(db: *mut KCM_Database, query_out: *mut *mut KCM_Query) -> KCM_Error` | Execute query |
| 9 | `KCM_QueryNext` | `(query: *mut KCM_Query, fact_out: *mut KCM_Fact, has_next: *mut bool) -> KCM_Error` | Iterate results |
| 10 | `KCM_QueryFree` | `(query: *mut KCM_Query)` | Free query |
| 11 | `KCM_DatabaseBeginTransaction` | `(db: *mut KCM_Database, txn_out: *mut *mut KCM_Transaction) -> KCM_Error` | Begin transaction |
| 12 | `KCM_TransactionFree` | `(txn: *mut KCM_Transaction)` | Free transaction |
| 13 | `KCM_DatabaseSave` | `(db: *mut KCM_Database, path: *const c_char) -> KCM_Error` | Save to file |
| 14 | `KCM_DatabaseLoad` | `(db: *mut *mut KCM_Database, path: *const c_char) -> KCM_Error` | Load from file |
| 15 | `KCM_DatabaseVerify` | `(db: *mut KCM_Database) -> KCM_Error` | Verify integrity |
| 16 | `KCM_TransactionCommit` | `(db: *mut KCM_Database, txn: *mut KCM_Transaction) -> KCM_Error` | Commit transaction |
| 17 | `KCM_TransactionRollback` | `(txn: *mut KCM_Transaction)` | Rollback transaction |
| 18 | `KCM_ErrorMessage` | `(err: KCM_Error) -> *const c_char` | Get error string |

### 2.3 Safety Rules

- All functions check null pointers before dereferencing
- `KCM_DatabaseNew` returns owned pointer; caller must free with `KCM_DatabaseFree`
- `KCM_DatabaseQuery` returns owned pointer; caller must free with `KCM_QueryFree`
- `KCM_DatabaseLoad` returns owned pointer; caller must free with `KCM_DatabaseFree`
- All FFI functions have `# Safety` documentation

## 3. REST API (8 Endpoints)

Served by `kcm-server` (actix-web).

### 3.1 Endpoints

| Method | Path | Handler | Description |
|--------|------|---------|-------------|
| GET | `/health` | `health_check` | Health check |
| POST | `/facts` | `insert_fact` | Insert fact |
| GET | `/facts` | `list_facts` | Query facts |
| GET | `/facts/{id}` | `get_fact` | Get fact by ID |
| PUT | `/facts/{id}` | `update_fact` | Update fact |
| DELETE | `/facts/{id}` | `delete_fact` | Delete fact |
| GET | `/stats` | `get_stats` | Metrics JSON |
| GET | `/metrics` | `get_metrics` | Prometheus format |

### 3.2 Request/Response Formats

#### POST /facts
```json
// Request
{
  "subject": 1,
  "predicate": 2,
  "object": 3,
  "confidence": 0.95,
  "evidence": 1,
  "context": 1,
  "priority": 0,
  "owner": 1
}

// Response
{
  "row_id": 42,
  "success": true
}
```

#### GET /facts
```json
// Response
{
  "facts": [
    {
      "row_id": 0,
      "subject": 1,
      "predicate": 2,
      "object": 3,
      "confidence": 0.95,
      "evidence": 1,
      "timestamp": 1700000000000000000,
      "context": 1,
      "version": 1,
      "priority": 0,
      "owner": 1
    }
  ],
  "count": 1
}
```

#### GET /stats
```json
{
  "queries_total": 100,
  "queries_failed": 2,
  "avg_query_latency_ms": 1.5,
  "inserts_total": 500,
  "inserts_failed": 0,
  "cache_hit_ratio": 0.85,
  "memory_bytes": 1048576,
  "inferences_total": 10,
  "facts_inferred": 50,
  "estimated_memory_bytes": 2097152,
  "total_facts": 1000,
  "active_facts": 950,
  "tombstone_count": 50
}
```

## 4. gRPC Service (4 RPCs)

```protobuf
service KnowledgeService {
    rpc InsertFact(InsertFactRequest) returns (InsertFactResponse);
    rpc QueryFacts(QueryRequest) returns (QueryResponse);
    rpc BeginTransaction(BeginTransactionRequest) returns (BeginTransactionResponse);
    rpc GetStats(GetStatsRequest) returns (StatsResponse);
}
```

### 4.1 Messages

```protobuf
message InsertFactRequest {
    FactData fact = 1;
}

message FactData {
    uint32 subject = 1;
    uint32 predicate = 2;
    uint32 object = 3;
    double confidence = 4;
    uint32 evidence = 5;
    int64 timestamp = 6;
    uint32 context = 7;
    int32 version = 8;
    int32 priority = 9;
    uint32 owner = 10;
}

message InsertFactResponse {
    int64 row_id = 1;
    bool success = 2;
}

message QueryRequest {
    optional uint32 subject = 1;
    optional uint32 predicate = 2;
    optional uint32 object = 3;
    optional double min_confidence = 4;
    int32 limit = 5;
}

message QueryResponse {
    repeated FactData facts = 1;
    int32 count = 2;
}

message BeginTransactionRequest {}
message BeginTransactionResponse { int64 txn_id = 1; }
message GetStatsRequest {}
message StatsResponse { /* MetricsSnapshot fields */ }
```

## 5. Python Bindings

Feature-gated (`python` feature). PyO3-based.

### 5.1 Classes

```python
class Database:
    def insert(subject, predicate, object, confidence) -> int
    def query_all() -> list
    def fact_count() -> int

class Fact:
    subject: int
    predicate: int
    object: int
    confidence: float
```

## 6. KQL Parser

### 6.1 Token Types (28 variants)

Keywords: SELECT, FROM, WHERE, AND, OR, NOT, ORDER, BY, DESC, ASC, LIMIT, JOIN, ON, INSERT, DELETE, UPDATE, SET, VALUES, INTO, CREATE, DROP, TABLE, FACTS, INDEX, SHOW, DESCRIBE, HELP, EXIT

Operators: =, !=, <, >, <=, >=, LIKE, IN

### 6.2 AST Types

```rust
pub struct SelectQuery {
    pub columns: Vec<String>,
    pub table: String,
    pub where_clause: Option<WhereClause>,
    pub order_by: Option<(String, SortDirection)>,
    pub limit: Option<usize>,
}

pub enum SortDirection { Asc, Desc }
```

## 7. Error Mapping

| KcmError | KCM_Error (FFI) | HTTP Status | gRPC Status |
|----------|-----------------|-------------|-------------|
| NotFound | KCM_ERR_NOT_FOUND | 404 | NOT_FOUND |
| OutOfMemory | KCM_ERR_OUT_OF_MEMORY | 507 | RESOURCE_EXHAUSTED |
| InvalidArgument | KCM_ERR_INVALID_ARGUMENT | 400 | INVALID_ARGUMENT |
| Io | KCM_ERR_IO | 500 | INTERNAL |
| Corrupted | KCM_ERR_CORRUPTED | 500 | DATA_LOSS |
| Conflict | KCM_ERR_CONFLICT | 409 | ALREADY_EXISTS |
| TransactionAborted | KCM_ERR_TRANSACTION_ABORTED | 409 | ABORTED |

## 8. References

- **Implements:** PRD2.md §9 (Interfaces)
- **Depends on:** KCM_DATA_MODEL_SPEC, KCM_RUNTIME_SPEC
- **Related:** KCM_API_SPEC, KCM_SECURITY_TRUST_SPEC
