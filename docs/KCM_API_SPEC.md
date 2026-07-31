# KCM API Specification

**Document ID:** KCM-API-001  
**Version:** 1.0.0  
**Depends on:** KCM-DATA-001, KCM_ARCHITECTURE-001

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

### 2.2 Functions

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
| KCM_TransactionFree | (txn: *mut KCM_Transaction) | void |
| KCM_ErrorMessage | (err: KCM_Error) -> *const c_char | Static string |

### 2.3 Error Handling

All functions return `KCM_Error`. Non-null pointer outputs are written via the pointer parameter. `KCM_ErrorMessage` returns a static null-terminated string.

---

## 3. Rust Public API

### 3.1 KnowledgeDatabase

```rust
impl KnowledgeDatabase {
    pub fn new() -> Result<Self, KcmError>;
    pub fn insert(&self, fact: &Fact) -> Result<RowID, KcmError>;
    pub fn insert_batch(&self, facts: &[Fact]) -> Result<Vec<RowID>, KcmError>;
    pub fn update(&self, row_id: RowID, fact: &Fact) -> Result<(), KcmError>;
    pub fn delete(&self, row_id: RowID) -> Result<(), KcmError>;
    pub fn query(&self) -> QueryBuilder;
    pub fn get_fact(&self, row_id: RowID) -> Result<Option<Fact>, KcmError>;
    pub fn fact_count(&self) -> usize;
    pub fn active_fact_count(&self) -> usize;
    pub fn begin_transaction(&self) -> Transaction;
    pub fn dict_insert_subject(&self, name: &str) -> DictID;
    pub fn dict_get_subject(&self, id: DictID) -> Option<String>;
    pub fn dict_lookup_subject(&self, name: &str) -> Option<DictID>;
}
```

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

| Handler | Method | Parameters | Response |
|---------|--------|------------|----------|
| handle_health | GET /health | — | HealthReport JSON |
| handle_insert | POST /facts | subject, predicate, object, confidence | row_id |
| handle_query | GET /facts | subject?, predicate?, object?, confidence_min? | Facts array |
| handle_get_fact | GET /facts/:id | row_id | Fact JSON |
| handle_update | PUT /facts/:id | row_id, subject, predicate, object, confidence | status |
| handle_delete | DELETE /facts/:id | row_id | status |
| handle_stats | GET /stats | — | MetricsSnapshot JSON |

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

| Message | Fields |
|---------|--------|
| InsertFactRequest | subject: u32, predicate: u32, object: u32, confidence: double |
| InsertFactResponse | row_id: u64, status: string |
| QueryRequest | subject?, predicate?, object?, confidence_min?, limit? |
| QueryResponse | facts: repeated FactData, total_count: u32 |
| FactData | subject: u32, predicate: u32, object: u32, confidence: double, timestamp: int64, context: u32 |
| GetFactRequest | row_id: u64 |
| GetStatsRequest | — |
| StatsResponse | fact_count: u64, memory_bytes: u64, avg_confidence: double |

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
