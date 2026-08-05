# KCM API Specification

**Document ID:** KCM-API-001  
**Version:** 1.0.0  
**Status:** Derived  
**Owner:** Specification Lock (P4)  
**Depends on:** KCM-DATA-001, KCM-ARCH-001

---

## 1. Purpose

Defines the public API contracts for all KCM interfaces.

---

## 2. C FFI API

### 2.1 Types

```c
typedef struct KCM_Database KCM_Database;
typedef struct KCM_Transaction KCM_Transaction;
typedef struct KCM_Query KCM_Query;

typedef struct {
    uint32_t subject;
    uint8_t predicate;
    uint32_t object;
    double confidence;
    uint8_t evidence;
    int64_t timestamp;
    uint8_t context;
    int32_t version;
    int8_t priority;
    uint16_t owner;
} KCM_Fact;  // See KCM_DATA_MODEL_SPEC for canonical Fact definition

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

### 2.2 Functions

18 `extern "C"` functions:

| Function | Signature | Returns |
|----------|-----------|---------|
| KCM_DatabaseNew | (db: *mut *mut KCM_Database) -> KCM_Error | Status |
| KCM_DatabaseFree | (db: *mut KCM_Database) | void |
| KCM_DatabaseInsert | (db, fact: *const KCM_Fact) -> KCM_Error | Status |
| KCM_DatabaseUpdate | (db, row_id: u64, fact: *const KCM_Fact) -> KCM_Error | Status |
| KCM_DatabaseDelete | (db, row_id: u64) -> KCM_Error | Status |
| KCM_DatabaseFactCount | (db) -> u64 | Count |
| KCM_DatabaseActiveCount | (db) -> u64 | Count |
| KCM_DatabaseQuery | (db, query: *mut *mut KCM_Query) -> KCM_Error | Status |
| KCM_QueryNext | (query, fact_out: *mut KCM_Fact, has_next: *mut bool) -> KCM_Error | Status |
| KCM_QueryFree | (query: *mut KCM_Query) | void |
| KCM_DatabaseBeginTransaction | (db, txn: *mut *mut KCM_Transaction) -> KCM_Error | Status |
| KCM_TransactionCommit | (txn: *mut KCM_Transaction, db: *mut KCM_Database) -> KCM_Error | Status |
| KCM_TransactionRollback | (txn: *mut KCM_Transaction) -> KCM_Error | Status |
| KCM_TransactionFree | (txn: *mut KCM_Transaction) | void |
| KCM_DatabaseSave | (db, path: *const c_char) -> KCM_Error | Status |
| KCM_DatabaseLoad | (db, path: *const c_char) -> KCM_Error | Status |
| KCM_DatabaseVerify | (path: *const c_char) -> KCM_Error | Status |
| KCM_ErrorMessage | (err: KCM_Error) -> *const c_char | Static string |

### 2.3 Error Handling

All functions return `KCM_Error`. Non-null pointer outputs are written via the pointer parameter. `KCM_ErrorMessage` returns a static null-terminated string.

### 2.4 C FFI Struct Note

The C `KCM_Fact` struct exposes all 10 fields (subject, predicate, object, confidence, evidence, timestamp, context, version, priority, owner), matching the Rust `Fact` struct exactly.

---

## 3. Rust Public API

### 3.1 KnowledgeDatabase

```rust
impl KnowledgeDatabase {
    pub fn new() -> Result<Self, KcmError>;
    pub fn get_schema(&self) -> RwLockReadGuard<Schema>;
    pub fn get_schema_mut(&self) -> RwLockWriteGuard<Schema>;
    pub fn begin_transaction(&self) -> Transaction;
    pub fn insert(&self, fact: &Fact) -> Result<RowID, KcmError>;
    pub fn insert_batch(&self, facts: &[Fact]) -> Result<Vec<RowID>, KcmError>;
    pub fn update(&self, row_id: RowID, fact: &Fact) -> Result<(), KcmError>;
    pub fn delete(&self, row_id: RowID) -> Result<(), KcmError>;
    pub fn query(&self) -> QueryBuilder;
    pub fn get_fact(&self, row_id: RowID) -> Result<Option<Fact>, KcmError>;
    pub fn dict_insert_subject(&self, name: &str) -> Result<DictID, KcmError>;
    pub fn dict_get_subject(&self, id: DictID) -> Option<String>;
    pub fn dict_lookup_subject(&self, name: &str) -> Option<DictID>;
    pub fn fact_count(&self) -> usize;
    pub fn active_fact_count(&self) -> usize;
    pub fn compact(&self) -> Result<Self, KcmError>;
}
```

- `get_schema` / `get_schema_mut` — Provide direct RwLock read/write access to the underlying `Schema`. Returns `parking_lot::RwLockReadGuard<Schema>` and `parking_lot::RwLockWriteGuard<Schema>` respectively.
- `dict_insert_subject` — Insert a subject name into the dictionary and return its `DictID`. Returns `Result<DictID, KcmError>`.
- `dict_get_subject` — Retrieve a subject name by its dictionary ID. Returns `None` if the ID does not exist.
- `dict_lookup_subject` — Look up the `DictID` for a given subject name. Returns `None` if the name is not in the dictionary.
- `compact` — Compact the database by removing tombstoned rows. Returns a new `KnowledgeDatabase` containing only active facts. This is an expensive operation that rebuilds all columns.

### 3.2 QueryBuilder

```rust
impl QueryBuilder {
    pub fn with_subject(self, subject: SubjectID) -> Self;
    pub fn with_predicate(self, predicate: PredicateID) -> Self;
    pub fn with_object(self, object: ObjectID) -> Self;
    pub fn with_confidence(self, threshold: f64) -> Self;
    pub fn execute(self) -> Result<Vec<Fact>, KcmError>;
}
```

### 3.3 Python Bindings (PyO3)

Feature-gated with `python` feature flag.

```rust
#[pyclass]
struct PyKnowledgeBase {
    kb: Arc<Mutex<KnowledgeDatabase>>,
}

#[pymethods]
impl PyKnowledgeBase {
    #[new]
    fn new() -> PyResult<Self>;
    fn insert(&self, subject: u32, predicate: u8, object: u32, confidence: f64) -> PyResult<()>;
    fn query_all(&self) -> PyResult<Vec<(u32, u8, u32, f64)>>;
    fn fact_count(&self) -> usize;
}
```

### 3.4 Builder Pattern

All QueryBuilder methods consume `self` and return `Self`, enabling chaining:
```rust
kb.query().with_subject(SubjectID(1)).with_confidence(0.5).execute()
```

---

## 4. REST API Handlers

The server implementation in `crates/kcm-server/src/main.rs` exposes both the compatibility routes and the versioned API surface on port 8080. The canonical runtime contract is therefore the route table below, which is the dual-surface implementation that is actually compiled today.

| Handler | Method | Endpoint | Parameters | Response |
|---------|--------|----------|------------|----------|
| handle_health | GET | `/health` | — | Health status JSON |
| handle_insert | POST | `/facts` and `/api/v1/facts` | subject, predicate, object, confidence | row_id |
| handle_query | GET | `/facts` and `/api/v1/facts` | subject?, predicate?, object?, confidence_min? | Facts array |
| handle_get_fact | GET | `/facts/{id}` and `/api/v1/facts/{id}` | row_id | Fact JSON |
| handle_update | PUT | `/facts/{id}` and `/api/v1/facts/{id}` | row_id, subject, predicate, object, confidence | status |
| handle_delete | DELETE | `/facts/{id}` and `/api/v1/facts/{id}` | row_id | status |
| handle_stats | GET | `/stats` and `/api/v1/stats` | — | MetricsSnapshot JSON |
| handle_metrics | GET | `/metrics` | — | Prometheus text format |
| openapi_handler | GET | `/openapi.json` | — | OpenAPI JSON document |
| handle_batch_insert | POST | `/api/v1/facts/batch` | array of facts | batch status |

### 4.1 Response Formats

**POST /facts** — Insert a fact. Returns HTTP 201.
```json
{"row_id": 42, "status": "created"}
```
Error responses: 400 `{"error":"Invalid fact: ...","status":400}`, 500 `{"error":"Insert failed: ...","status":500}`.

**GET /facts** — Query facts with optional filters. Returns HTTP 200.
```json
{"facts":[{"subject":1,"predicate":2,"object":3,"confidence":0.95}],"count":1}
```
Error response: 500 `{"error":"Query failed: ...","status":500}`.

**GET /facts/{id}** — Get a single fact by row ID. Returns HTTP 200.
```json
{"row_id":42,"subject":1,"predicate":2,"object":3,"confidence":0.95}
```
Error responses: 404 `{"error":"Fact 42 not found","status":404}`, 500 `{"error":"Error: ...","status":500}`.

**PUT /facts/{id}** — Update a fact. Returns HTTP 200.
```json
{"row_id":42,"status":"updated"}
```
Error responses: 400 `{"error":"Invalid fact: ...","status":400}`, 500 `{"error":"Update failed: ...","status":500}`.

**DELETE /facts/{id}** — Delete a fact. Returns HTTP 200.
```json
{"row_id":42,"status":"deleted"}
```
Error response: 500 `{"error":"Delete failed: ...","status":500}`.

**GET /health** — Health check. Returns HTTP 200 (healthy/degraded) or 500 (unhealthy).
```json
{"status":"healthy"}
```
Status values: `"healthy"`, `"degraded"`, `"unhealthy"`.

**GET /stats** — Database statistics snapshot. Returns HTTP 200.
```json
{
  "fact_count": 1000,
  "active_count": 950,
  "total_inserts": 1200,
  "total_queries": 500,
  "avg_latency_ms": 1.23,
  "memory_bytes": 4096
}
```

**GET /metrics** — Prometheus text exposition format. Returns HTTP 200 with `Content-Type: text/plain`.
```
# HELP kcm_queries_total Total queries executed
# TYPE kcm_queries_total counter
kcm_queries_total 500
# HELP kcm_inserts_total Total inserts executed
# TYPE kcm_inserts_total counter
kcm_inserts_total 1200
# HELP kcm_cache_hit_ratio Cache hit ratio
# TYPE kcm_cache_hit_ratio gauge
kcm_cache_hit_ratio 0.8750
# HELP kcm_memory_bytes Memory usage in bytes
# TYPE kcm_memory_bytes gauge
kcm_memory_bytes 4096
```

---

## 5. gRPC API

### 5.1 Proto Definition

```protobuf
service KnowledgeService {
  rpc InsertFact(InsertFactRequest) returns (InsertFactResponse);
  rpc QueryFacts(QueryRequest) returns (QueryResponse);
  rpc GetFact(GetFactRequest) returns (FactData);
  rpc GetStats(GetStatsRequest) returns (StatsResponse);
}
```

### 5.2 Messages

Verified against `crates/kcm-interface/proto/kcm.proto`.

| Message | Fields | Proto Types |
|---------|--------|-------------|
| InsertFactRequest | subject: u32, predicate: u32, object: u32, confidence: double | uint32, uint32, uint32, double |
| InsertFactResponse | row_id: u64, status: string | uint64, string |
| QueryRequest | subject?, predicate?, object?, confidence_min?, limit? | optional uint32, optional uint32, optional uint32, optional double, optional uint32 |
| QueryResponse | facts: repeated FactData, total_count: u32 | repeated FactData, uint32 |
| FactData | subject: u32, predicate: u32, object: u32, confidence: double, timestamp: int64, context: u32 | uint32, uint32, uint32, double, int64, uint32 |
| GetFactRequest | row_id: u64 | uint64 |
| GetStatsRequest | — | — |
| StatsResponse | fact_count: u64, memory_bytes: u64, avg_confidence: double | uint64, uint64, double |

All five `QueryRequest` fields use proto3 `optional` semantics, meaning they default to absent when not provided by the caller.

---

## 6. Error Code Mapping

| KcmError Variant | C API Code | HTTP Status |
|------------------|-----------|-------------|
| NotFound | KCM_ERR_NOT_FOUND | 404 |
| OutOfMemory | KCM_ERR_OUT_OF_MEMORY | 507 |
| InvalidArgument | KCM_ERR_INVALID_ARGUMENT | 400 |
| Io | KCM_ERR_IO | 500 |
| Corrupted | KCM_ERR_CORRUPTED | 500 |
| Conflict | KCM_ERR_CONFLICT | 409 |
| TransactionAborted | KCM_ERR_TRANSACTION_ABORTED | 409 |

---

## 7. Constraints

| Constraint | Rationale |
|------------|-----------|
| C API returns static error strings | No dangling pointers |
| All pointers validated for null | Prevents segfault |
| QueryBuilder consumes self | Prevents accidental reuse |
| REST handlers are stateless functions | No framework dependency |

---

## 8. References

- **Depends on:** KCM-DATA-001 (KCM_DATA_MODEL_SPEC), KCM-ARCH-001 (KCM_ARCHITECTURE)
- **Parent specs:** KCM_SPECIFICATION (KCM_SPECIFICATION)
- **Related:** KCM_QUERY_EXECUTION_SPEC (KCM_QUERY_EXECUTION_SPEC), KCM_RUNTIME_SPEC (KCM_RUNTIME_SPEC)
