# SDK Technical Specification

## Table of Contents

- [SDK Architecture](#sdk-architecture)
- [SDK Design Principles](#sdk-design-principles)
- [Supported Languages](#supported-languages)
- [Standard API (16 Operations)](#standard-api-16-operations)
- [Data Model](#data-model)
- [ErrorCode Mapping](#errorcode-mapping)
- [Serialization Rules](#serialization-rules)
- [REST Mapping](#rest-mapping)
- [FFI Mapping](#ffi-mapping)
- [Thread Safety](#thread-safety)
- [Memory Ownership](#memory-ownership)
- [Examples](#examples)
- [Compatibility Matrix](#compatibility-matrix)
- [Versioning](#versioning)
- [Security](#security)
- [Testing](#testing)
- [SSOT Compliance](#ssot-compliance)
- [References](#references)

---

## SDK Architecture

The KCM SDK follows a layered architecture that isolates application code from the core Rust engine:

```
Application Layer
    ↓
Language SDK Layer (9 SDKs)
    ↓
KCM Core (Rust engine)
    ↓
Operating System / Hardware
```

### Layer Responsibilities

| Layer | Responsibility | Example |
|-------|---------------|---------|
| Application Layer | Business logic, user interaction | User's Rust program, Python script, JavaScript app |
| Language SDK Layer | Language-idiomatic API, type conversion, error mapping | `kcm-sdk` crate, `kcm` Python package, `@kcm/js` npm package |
| KCM Core | Columnar storage, query execution, compression, indexing | `kcm-core`, `kcm-storage`, `kcm-compute`, `kcm-reasoning` |
| OS / Hardware | Filesystem, memory, CPU (SIMD) | Linux ext4, macOS APFS, Windows NTFS |

### SDK Composition

Each SDK consists of:

1. **API Surface** — Public types, functions, and methods matching the Standard API
2. **Binding Layer** — Language-specific bridge to the Rust engine
3. **Type Mapping** — Conversion between language types and KCM core types
4. **Error Translation** — Mapping `KcmError` variants to language-native errors
5. **Serialization** — JSON/MessagePack encoding for cross-SDK interoperability
6. **Tests** — Unit, integration, and cross-language consistency tests

### Binding Strategies

| Language | Binding Strategy | Mechanism |
|----------|-----------------|-----------|
| Rust | Native | Direct crate dependency |
| C | FFI | `extern "C"` functions via `cbindgen` |
| C++ | FFI + Header | C FFI with RAII wrapper header |
| Python | PyO3 | Native Python extension module |
| JavaScript | N-API | Node.js native addon |
| TypeScript | N-API + Types | JavaScript binding with TypeScript declarations |
| Go | cgo | C FFI via cgo |
| Java | JNI | Java Native Interface via `jni` crate |
| .NET | P/Invoke | Platform Invocation Services |

---

## SDK Design Principles

The KCM SDKs are governed by seven design principles. Every SDK must satisfy all seven.

### 1. API Consistency

All SDKs expose identical operations with identical semantics. A `query(kql)` call in Python returns the same results as the same call in Rust, Go, or any other SDK. Semantic equivalence is enforced by cross-language consistency tests.

### 2. Idiomatic Code

Each SDK follows the conventions, patterns, and idioms of its target language:

- **Rust**: `Result<T, KcmError>`, ownership, traits
- **Python**: exceptions, context managers, duck typing
- **JavaScript/TypeScript**: Promises, async/await, object literals
- **Go**: `(value, error)` tuples, interfaces
- **Java**: checked exceptions, builder patterns
- **.NET**: IDisposable, async/await, properties
- **C**: opaque pointers, error codes, manual memory management
- **C++**: RAII, smart pointers, exceptions

### 3. Minimal Dependencies

Each SDK minimizes external dependencies to reduce supply chain risk:

| Language | Max Dependencies | Rationale |
|----------|-----------------|-----------|
| Rust | 0 (core SDK) | Direct engine integration |
| C | 0 | Standalone FFI |
| C++ | 0 | Header-only wrapper |
| Python | 1 (pyo3 for build) | Extension module |
| JavaScript | 0 (runtime) | Native addon |
| Go | 0 | cgo, no external deps |
| Java | 0 (runtime) | JNI |
| .NET | 0 (runtime) | P/Invoke |

### 4. Type Safety

Strong typing is enforced wherever the language supports it:

- Rust: Full type system with generics
- TypeScript: Strict type declarations
- Java: Generics and enums
- .NET: Generics and nullable reference types
- Go: Static typing with interfaces
- Python: Type hints (enforced by mypy/pyright)
- C/C++: Static typing with typedefs

### 5. Error Transparency

Errors propagate with full context through all layers. Every SDK maps `KcmError` variants to language-native error types while preserving the error message, error code, and call context.

### 6. Memory Safety

No SDK introduces memory leaks, use-after-free, double-free, or buffer overflows. Memory safety is achieved through:

- Rust: ownership system
- C++: RAII and smart pointers
- Managed languages: garbage collection
- C: strict ownership protocol with explicit free functions

### 7. Thread Safety

Thread-safety guarantees are documented per SDK and per operation. All SDKs document which operations are safe to call concurrently and which require external synchronization.

---

## Supported Languages

| Language | Status | Package | Build System | Test Framework | Linter |
|----------|--------|---------|--------------|----------------|--------|
| Rust | Stable | `kcm-sdk` (crate) | Cargo | `cargo test` | clippy |
| C | Stable | `libkcm` (FFI) | Make | `make test` | — |
| C++ | Stable | `libkcm` (header) | CMake | `ctest` | — |
| Python | Beta | `kcm` (PyPI) | pyproject.toml | pytest | ruff |
| JavaScript | Beta | `@kcm/js` (npm) | package.json | jest | eslint |
| TypeScript | Beta | `@kcm/ts` (npm) | package.json | jest | eslint + tsc |
| Go | Beta | `github.com/kcm/go-sdk` | go.mod | `go test` | `go vet` |
| Java | Beta | `io.kcm:sdk` (Maven) | pom.xml | `mvn test` | — |
| .NET | Beta | `Kcm.Sdk` (NuGet) | *.csproj | `dotnet test` | — |

### Status Definitions

| Status | Definition |
|--------|-----------|
| Stable | API frozen, backward compatibility guaranteed, full test coverage, production-ready |
| Beta | API may change, feature-complete but not yet API-frozen, test coverage ≥ 80% |
| Alpha | API unstable, incomplete feature set, for internal testing only |
| Deprecated | Will be removed in next major version |

---

## Standard API (16 Operations)

All SDKs implement exactly 16 operations. Each operation is documented with its signature in all 9 supported languages.

### 1. Database(path?)

Open or create a database at the given path. If no path is provided, creates an in-memory database.

**Semantics**: If the file exists, open it. If not, create a new database with default schema. Returns a database handle.

| Language | Signature |
|----------|-----------|
| Rust | `Database::new(path: Option<&str>) -> Result<Database, KcmError>` |
| C | `KCM_Database* KCM_DatabaseNew(const char* path, KCM_Result* result)` |
| C++ | `KcmDatabase::KcmDatabase(std::string_view path = {})` |
| Python | `kcm.Database(path: str \| None = None) -> Database` |
| JavaScript | `KcmDatabase.open(path?: string): KcmDatabase` |
| TypeScript | `KcmDatabase.open(path?: string): KcmDatabase` |
| Go | `Open(path string) (*Database, error)` |
| Java | `KcmDatabase.open(String path) throws KcmException` |
| .NET | `KcmDatabase.Open(string? path = null)` |

### 2. Insert(fact)

Insert a knowledge fact into the database.

**Semantics**: Validates the fact, assigns a `RowID`, stores in the columnar engine. Returns the assigned `RowID`.

| Language | Signature |
|----------|-----------|
| Rust | `db.insert(fact: Fact) -> Result<RowID, KcmError>` |
| C | `KCM_Result KCM_DatabaseInsert(KCM_Database* db, const KCM_Fact* fact, uint32_t* row_id)` |
| C++ | `uint32_t KcmDatabase::insert(const KcmFact& fact)` |
| Python | `db.insert(fact: Fact) -> int` |
| JavaScript | `db.insert(fact: KcmFact): number` |
| TypeScript | `db.insert(fact: KcmFact): number` |
| Go | `db.Insert(fact Fact) (uint32, error)` |
| Java | `long insert(Fact fact) throws KcmException` |
| .NET | `uint Insert(Fact fact)` |

### 3. Query(kql)

Execute a KQL (Knowledge Query Language) query against the database.

**Semantics**: Parses the KQL string, executes the query plan, returns matching facts.

| Language | Signature |
|----------|-----------|
| Rust | `db.query(kql: &str) -> Result<Vec<Fact>, KcmError>` |
| C | `KCM_Result KCM_DatabaseQuery(KCM_Database* db, const char* kql, KCM_Query** query)` |
| C++ | `std::vector<KcmFact> KcmDatabase::query(std::string_view kql)` |
| Python | `db.query(kql: str) -> list[Fact]` |
| JavaScript | `db.query(kql: string): KcmFact[]` |
| TypeScript | `db.query(kql: string): KcmFact[]` |
| Go | `db.Query(kql string) ([]Fact, error)` |
| Java | `List<Fact> query(String kql) throws KcmException` |
| .NET | `IReadOnlyList<Fact> Query(string kql)` |

### 4. QueryAll()

Retrieve all active (non-deleted) facts from the database.

**Semantics**: Returns all facts where `is_deleted == false`. Equivalent to `query("SELECT *")`.

| Language | Signature |
|----------|-----------|
| Rust | `db.query_all() -> Result<Vec<Fact>, KcmError>` |
| C | `KCM_Result KCM_DatabaseQueryAll(KCM_Database* db, KCM_Query** query)` |
| C++ | `std::vector<KcmFact> KcmDatabase::queryAll()` |
| Python | `db.query_all() -> list[Fact]` |
| JavaScript | `db.queryAll(): KcmFact[]` |
| TypeScript | `db.queryAll(): KcmFact[]` |
| Go | `db.QueryAll() ([]Fact, error)` |
| Java | `List<Fact> queryAll() throws KcmException` |
| .NET | `IReadOnlyList<Fact> QueryAll()` |

### 5. Delete(row_id)

Delete a fact by its `RowID`. The fact is logically deleted (tombstoned).

**Semantics**: Marks the fact as deleted. The row is not physically removed until compaction.

| Language | Signature |
|----------|-----------|
| Rust | `db.delete(row_id: RowID) -> Result<(), KcmError>` |
| C | `KCM_Result KCM_DatabaseDelete(KCM_Database* db, uint32_t row_id)` |
| C++ | `void KcmDatabase::delete(uint32_t row_id)` |
| Python | `db.delete(row_id: int) -> None` |
| JavaScript | `db.delete(rowId: number): void` |
| TypeScript | `db.delete(rowId: number): void` |
| Go | `db.Delete(rowID uint32) error` |
| Java | `void delete(long rowId) throws KcmException` |
| .NET | `void Delete(uint rowId)` |

### 6. Update(row_id, fact)

Update an existing fact identified by `RowID`.

**Semantics**: Replaces the fact at the given `RowID`. The `RowID` remains unchanged. Returns the updated `RowID`.

| Language | Signature |
|----------|-----------|
| Rust | `db.update(row_id: RowID, fact: Fact) -> Result<RowID, KcmError>` |
| C | `KCM_Result KCM_DatabaseUpdate(KCM_Database* db, uint32_t row_id, const KCM_Fact* fact)` |
| C++ | `void KcmDatabase::update(uint32_t row_id, const KcmFact& fact)` |
| Python | `db.update(row_id: int, fact: Fact) -> None` |
| JavaScript | `db.update(rowId: number, fact: KcmFact): void` |
| TypeScript | `db.update(rowId: number, fact: KcmFact): void` |
| Go | `db.Update(rowID uint32, fact Fact) error` |
| Java | `void update(long rowId, Fact fact) throws KcmException` |
| .NET | `void Update(uint rowId, Fact fact)` |

### 7. GetFact(row_id)

Retrieve a single fact by its `RowID`.

**Semantics**: Returns the fact at the given `RowID`, or `NotFound` if the row does not exist or is deleted.

| Language | Signature |
|----------|-----------|
| Rust | `db.get_fact(row_id: RowID) -> Result<Fact, KcmError>` |
| C | `KCM_Result KCM_DatabaseGetFact(KCM_Database* db, uint32_t row_id, KCM_Fact* fact)` |
| C++ | `KcmFact KcmDatabase::getFact(uint32_t row_id)` |
| Python | `db.get_fact(row_id: int) -> Fact` |
| JavaScript | `db.getFact(rowId: number): KcmFact` |
| TypeScript | `db.getFact(rowId: number): KcmFact` |
| Go | `db.GetFact(rowID uint32) (Fact, error)` |
| Java | `Fact getFact(long rowId) throws KcmException` |
| .NET | `Fact GetFact(uint rowId)` |

### 8. FactCount()

Get the total number of facts in the database (including deleted).

**Semantics**: Returns the total row count, regardless of deletion status.

| Language | Signature |
|----------|-----------|
| Rust | `db.fact_count() -> Result<u64, KcmError>` |
| C | `KCM_Result KCM_DatabaseFactCount(KCM_Database* db, uint64_t* count)` |
| C++ | `uint64_t KcmDatabase::factCount()` |
| Python | `db.fact_count() -> int` |
| JavaScript | `db.factCount(): number` |
| TypeScript | `db.factCount(): number` |
| Go | `db.FactCount() (uint64, error)` |
| Java | `long factCount() throws KcmException` |
| .NET | `ulong FactCount` |

### 9. ActiveFactCount()

Get the number of active (non-deleted) facts.

**Semantics**: Returns the count of facts where `is_deleted == false`.

| Language | Signature |
|----------|-----------|
| Rust | `db.active_fact_count() -> Result<u64, KcmError>` |
| C | `KCM_Result KCM_DatabaseActiveFactCount(KCM_Database* db, uint64_t* count)` |
| C++ | `uint64_t KcmDatabase::activeFactCount()` |
| Python | `db.active_fact_count() -> int` |
| JavaScript | `db.activeFactCount(): number` |
| TypeScript | `db.activeFactCount(): number` |
| Go | `db.ActiveFactCount() (uint64, error)` |
| Java | `long activeFactCount() throws KcmException` |
| .NET | `ulong ActiveFactCount` |

### 10. BeginTransaction()

Start a new transaction.

**Semantics**: Creates a transaction context. All subsequent operations (until commit or rollback) are part of this transaction. Returns a transaction handle.

| Language | Signature |
|----------|-----------|
| Rust | `db.begin_transaction() -> Result<Transaction, KcmError>` |
| C | `KCM_Result KCM_DatabaseBeginTransaction(KCM_Database* db, KCM_Transaction** txn)` |
| C++ | `KcmTransaction KcmDatabase::beginTransaction()` |
| Python | `db.begin_transaction() -> Transaction` |
| JavaScript | `db.beginTransaction(): KcmTransaction` |
| TypeScript | `db.beginTransaction(): KcmTransaction` |
| Go | `db.BeginTransaction() (*Transaction, error)` |
| Java | `Transaction beginTransaction() throws KcmException` |
| .NET | `Transaction BeginTransaction()` |

### 11. Commit(txn)

Commit a transaction, making all changes permanent.

**Semantics**: Validates all changes in the transaction, applies them atomically, and releases the transaction lock.

| Language | Signature |
|----------|-----------|
| Rust | `txn.commit() -> Result<(), KcmError>` |
| C | `KCM_Result KCM_TransactionCommit(KCM_Transaction* txn)` |
| C++ | `void KcmTransaction::commit()` |
| Python | `txn.commit() -> None` |
| JavaScript | `txn.commit(): void` |
| TypeScript | `txn.commit(): void` |
| Go | `txn.Commit() error` |
| Java | `void commit() throws KcmException` |
| .NET | `void Commit()` |

### 12. Rollback(txn)

Roll back a transaction, discarding all changes.

**Semantics**: Discards all changes made within the transaction and releases the transaction lock.

| Language | Signature |
|----------|-----------|
| Rust | `txn.rollback() -> Result<(), KcmError>` |
| C | `KCM_Result KCM_TransactionRollback(KCM_Transaction* txn)` |
| C++ | `void KcmTransaction::rollback()` |
| Python | `txn.rollback() -> None` |
| JavaScript | `txn.rollback(): void` |
| TypeScript | `txn.rollback(): void` |
| Go | `txn.Rollback() error` |
| Java | `void rollback() throws KcmException` |
| .NET | `void Rollback()` |

### 13. Save(path)

Save the database to a file at the given path.

**Semantics**: Serializes the entire database state (columns, indices, WAL) to the specified file path. If the path exists, it is overwritten.

| Language | Signature |
|----------|-----------|
| Rust | `db.save(path: &str) -> Result<(), KcmError>` |
| C | `KCM_Result KCM_DatabaseSave(KCM_Database* db, const char* path)` |
| C++ | `void KcmDatabase::save(std::string_view path)` |
| Python | `db.save(path: str) -> None` |
| JavaScript | `db.save(path: string): void` |
| TypeScript | `db.save(path: string): void` |
| Go | `db.Save(path string) error` |
| Java | `void save(String path) throws KcmException` |
| .NET | `void Save(string path)` |

### 14. Load(path)

Load a database from a file at the given path.

**Semantics**: Deserializes the database from the specified file. The file must have been created by `save()`. Returns a new database handle.

| Language | Signature |
|----------|-----------|
| Rust | `Database::load(path: &str) -> Result<Database, KcmError>` |
| C | `KCM_Database* KCM_DatabaseLoad(const char* path, KCM_Result* result)` |
| C++ | `KcmDatabase KcmDatabase::load(std::string_view path)` |
| Python | `kcm.Database.load(path: str) -> Database` |
| JavaScript | `KcmDatabase.load(path: string): KcmDatabase` |
| TypeScript | `KcmDatabase.load(path: string): KcmDatabase` |
| Go | `Load(path string) (*Database, error)` |
| Java | `static KcmDatabase load(String path) throws KcmException` |
| .NET | `static KcmDatabase Load(string path)` |

### 15. Verify(path)

Verify the integrity of a database file.

**Semantics**: Reads the database file and validates checksums, structure, and consistency. Returns `Ok(())` if valid, or an error describing the corruption.

| Language | Signature |
|----------|-----------|
| Rust | `Database::verify(path: &str) -> Result<(), KcmError>` |
| C | `KCM_Result KCM_DatabaseVerify(const char* path)` |
| C++ | `bool KcmDatabase::verify(std::string_view path)` |
| Python | `kcm.Database.verify(path: str) -> bool` |
| JavaScript | `KcmDatabase.verify(path: string): boolean` |
| TypeScript | `KcmDatabase.verify(path: string): boolean` |
| Go | `Verify(path string) error` |
| Java | `static boolean verify(String path) throws KcmException` |
| .NET | `static bool Verify(string path)` |

### 16. Close()

Close the database and release all resources.

**Semantics**: Flushes pending writes, releases file handles, frees memory. The database handle is invalid after `close()`.

| Language | Signature |
|----------|-----------|
| Rust | `db.close() -> Result<(), KcmError>` |
| C | `KCM_Result KCM_DatabaseClose(KCM_Database* db)` |
| C++ | `void KcmDatabase::close()` |
| Python | `db.close() -> None` (also supports `with` statement) |
| JavaScript | `db.close(): void` (also supports `await using` pattern) |
| TypeScript | `db.close(): void` (also supports `await using` pattern) |
| Go | `db.Close() error` (implements `io.Closer`) |
| Java | `void close() implements AutoCloseable` |
| .NET | `void Dispose()` (implements `IDisposable`) |

---

## Data Model

### Fact Structure

A `Fact` is the fundamental data unit in KCM. It contains 10 fields totaling 34 bytes:

| Field | Type | Size | Description |
|-------|------|------|-------------|
| `subject` | `u32` | 4 bytes | Subject identifier (SubjectID) |
| `predicate` | `u8` | 1 byte | Predicate identifier (PredicateID) |
| `object` | `u32` | 4 bytes | Object identifier (ObjectID) |
| `confidence` | `f64` | 8 bytes | Confidence score (0.0–1.0) |
| `evidence` | `u8` | 1 byte | Evidence level (0–255) |
| `timestamp` | `i64` | 8 bytes | Unix epoch timestamp (milliseconds) |
| `context` | `u8` | 1 byte | Context identifier |
| `version` | `i32` | 4 bytes | Fact version (for optimistic concurrency) |
| `priority` | `i8` | 1 byte | Priority level (-128 to 127) |
| `owner` | `u16` | 2 bytes | Owner identifier |
| **Total** | | **34 bytes** | |

### Type Definitions Per Language

| Language | Type Name | Fields |
|----------|-----------|--------|
| Rust | `Fact` | struct with named fields |
| C | `KCM_Fact` | struct with named fields |
| C++ | `KcmFact` | struct with named fields |
| Python | `Fact` | dataclass |
| JavaScript | `KcmFact` | plain object |
| TypeScript | `KcmFact` | interface |
| Go | `Fact` | struct with named fields |
| Java | `Fact` | class with fields |
| .NET | `Fact` | record/class with properties |

### RowID

`RowID` is a `u32` (0–4,294,967,295) assigned sequentially upon insertion. RowID 0 is reserved and never assigned.

### SubjectID / ObjectID

`SubjectID` and `ObjectID` are `u32` values mapped through the dictionary encoding layer. Actual string values are stored in the dictionary; the Fact stores the integer key.

### PredicateID

`PredicateID` is a `u8` (0–255). The first 32 predicates are reserved for system use. User-defined predicates start at 32.

### Confidence

`Confidence` is an `f64` in the range [0.0, 1.0]. Values outside this range are rejected with `InvalidArgument`.

---

## ErrorCode Mapping

| KCM Error | Rust | Python | JavaScript | Go | Java | .NET | C |
|-----------|------|--------|------------|-----|------|------|---|
| OK | `Ok(())` | `None` | `null` | `nil` | `null` | `null` | `KCM_OK` |
| NotFound | `Err(KcmError::NotFound(msg))` | `raise KcmNotFoundError(msg)` | `throw KcmNotFoundError(msg)` | `error` | `throw KcmNotFoundException(msg)` | `throw KcmNotFoundException(msg)` | `KCM_ERR_NOT_FOUND` |
| OutOfMemory | `Err(KcmError::OutOfMemory)` | `raise KcmOutOfMemoryError()` | `throw KcmOutOfMemoryError()` | `error` | `throw KcmOutOfMemoryException()` | `throw KcmOutOfMemoryException()` | `KCM_ERR_OUT_OF_MEMORY` |
| InvalidArgument | `Err(KcmError::InvalidArgument(msg))` | `raise KcmValueError(msg)` | `throw KcmValueError(msg)` | `error` | `throw KcmIllegalArgumentException(msg)` | `throw KcmArgumentException(msg)` | `KCM_ERR_INVALID_ARGUMENT` |
| Io | `Err(KcmError::Io(msg))` | `raise KcmIoError(msg)` | `throw KcmIoError(msg)` | `error` | `throw KcmIOException(msg)` | `throw KcmIoException(msg)` | `KCM_ERR_IO` |
| Corrupted | `Err(KcmError::Corrupted(msg))` | `raise KcmCorruptedError(msg)` | `throw KcmCorruptedError(msg)` | `error` | `throw KcmCorruptedException(msg)` | `throw KcmCorruptedException(msg)` | `KCM_ERR_CORRUPTED` |
| Conflict | `Err(KcmError::Conflict(msg))` | `raise KcmConflictError(msg)` | `throw KcmConflictError(msg)` | `error` | `throw KcmConflictException(msg)` | `throw KcmConflictException(msg)` | `KCM_ERR_CONFLICT` |
| TransactionAborted | `Err(KcmError::TransactionAborted)` | `raise KcmTransactionAbortedError()` | `throw KcmTransactionAbortedError()` | `error` | `throw KcmTransactionAbortedException()` | `throw KcmTransactionAbortedException()` | `KCM_ERR_TRANSACTION_ABORTED` |

### Error Properties

Each error carries:

| Property | Type | Description |
|----------|------|-------------|
| `code` | string | Error code identifier (e.g., `NOT_FOUND`) |
| `message` | string | Human-readable error message |
| `source` | string (optional) | Underlying cause (chained errors) |

---

## Serialization Rules

### JSON Format

Facts are serialized as JSON objects for cross-SDK interoperability:

```json
{
  "subject": 42,
  "predicate": 3,
  "object": 108,
  "confidence": 0.95,
  "evidence": 1,
  "timestamp": 1691328000000,
  "context": 0,
  "version": 1,
  "priority": 10,
  "owner": 1
}
```

### Rules

| Rule | Description |
|------|-------------|
| Field names | Always lowercase, match Rust field names exactly |
| Integer types | Serialized as JSON numbers (no quotes) |
| Float types | Serialized as JSON numbers (no quotes) |
| Timestamps | Unix epoch in milliseconds (`i64`) |
| Confidence | `f64` in range [0.0, 1.0] |
| Null handling | Null facts are not permitted |
| Unknown fields | Ignored on deserialization, preserved on round-trip |
| Encoding | UTF-8 for all string values |

### Binary Serialization

For in-process use (Rust ↔ Rust), the native binary format is used:

| Field | Encoding | Size |
|-------|----------|------|
| subject | Little-endian u32 | 4 |
| predicate | u8 | 1 |
| object | Little-endian u32 | 4 |
| confidence | Little-endian f64 | 8 |
| evidence | u8 | 1 |
| timestamp | Little-endian i64 | 8 |
| context | u8 | 1 |
| version | Little-endian i32 | 4 |
| priority | i8 | 1 |
| owner | Little-endian u16 | 2 |

---

## REST Mapping

### Endpoints

| Operation | Method | Endpoint | Request Body | Response |
|-----------|--------|----------|-------------|----------|
| Insert | `POST` | `/api/v1/facts` | `Fact` JSON | `{ "rowId": u32 }` |
| Query | `GET` | `/api/v1/facts?kql={kql}` | — | `{ "facts": Fact[] }` |
| QueryAll | `GET` | `/api/v1/facts` | — | `{ "facts": Fact[] }` |
| Delete | `DELETE` | `/api/v1/facts/{rowId}` | — | `{ "ok": true }` |
| Update | `PUT` | `/api/v1/facts/{rowId}` | `Fact` JSON | `{ "ok": true }` |
| GetFact | `GET` | `/api/v1/facts/{rowId}` | — | `{ "fact": Fact }` |
| FactCount | `GET` | `/api/v1/stats/fact-count` | — | `{ "count": u64 }` |
| ActiveFactCount | `GET` | `/api/v1/stats/active-fact-count` | — | `{ "count": u64 }` |
| BeginTransaction | `POST` | `/api/v1/transactions` | — | `{ "txnId": string }` |
| Commit | `POST` | `/api/v1/transactions/{txnId}/commit` | — | `{ "ok": true }` |
| Rollback | `POST` | `/api/v1/transactions/{txnId}/rollback` | — | `{ "ok": true }` |
| Save | `POST` | `/api/v1/database/save` | `{ "path": string }` | `{ "ok": true }` |
| Load | `POST` | `/api/v1/database/load` | `{ "path": string }` | `{ "ok": true }` |
| Verify | `GET` | `/api/v1/database/verify?path={path}` | — | `{ "valid": bool }` |
| Close | `POST` | `/api/v1/database/close` | — | `{ "ok": true }` |

### Error Responses

All error responses follow:

```json
{
  "error": {
    "code": "NOT_FOUND",
    "message": "Row with ID 42 not found"
  }
}
```

| HTTP Status | KCM Error |
|-------------|-----------|
| 200 | OK |
| 400 | InvalidArgument |
| 404 | NotFound |
| 409 | Conflict |
| 422 | Corrupted |
| 500 | Io, OutOfMemory, TransactionAborted |

---

## FFI Mapping

### C FFI Functions

| Operation | C Function | Parameters | Return Type |
|-----------|-----------|------------|-------------|
| Database(path) | `KCM_DatabaseNew` | `const char* path, KCM_Result* result` | `KCM_Database*` |
| Insert(fact) | `KCM_DatabaseInsert` | `KCM_Database* db, const KCM_Fact* fact, uint32_t* row_id` | `KCM_Result` |
| Query(kql) | `KCM_DatabaseQuery` | `KCM_Database* db, const char* kql, KCM_Query** query` | `KCM_Result` |
| QueryAll() | `KCM_DatabaseQueryAll` | `KCM_Database* db, KCM_Query** query` | `KCM_Result` |
| Delete(row_id) | `KCM_DatabaseDelete` | `KCM_Database* db, uint32_t row_id` | `KCM_Result` |
| Update(row_id, fact) | `KCM_DatabaseUpdate` | `KCM_Database* db, uint32_t row_id, const KCM_Fact* fact` | `KCM_Result` |
| GetFact(row_id) | `KCM_DatabaseGetFact` | `KCM_Database* db, uint32_t row_id, KCM_Fact* fact` | `KCM_Result` |
| FactCount() | `KCM_DatabaseFactCount` | `KCM_Database* db, uint64_t* count` | `KCM_Result` |
| ActiveFactCount() | `KCM_DatabaseActiveFactCount` | `KCM_Database* db, uint64_t* count` | `KCM_Result` |
| BeginTransaction() | `KCM_DatabaseBeginTransaction` | `KCM_Database* db, KCM_Transaction** txn` | `KCM_Result` |
| Commit(txn) | `KCM_TransactionCommit` | `KCM_Transaction* txn` | `KCM_Result` |
| Rollback(txn) | `KCM_TransactionRollback` | `KCM_Transaction* txn` | `KCM_Result` |
| Save(path) | `KCM_DatabaseSave` | `KCM_Database* db, const char* path` | `KCM_Result` |
| Load(path) | `KCM_DatabaseLoad` | `const char* path, KCM_Result* result` | `KCM_Database*` |
| Verify(path) | `KCM_DatabaseVerify` | `const char* path` | `KCM_Result` |
| Close() | `KCM_DatabaseClose` | `KCM_Database* db` | `KCM_Result` |

### FFI Types

```c
typedef struct KCM_Database KCM_Database;
typedef struct KCM_Transaction KCM_Transaction;
typedef struct KCM_Query KCM_Query;

typedef struct KCM_Fact {
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

typedef struct KCM_Result {
    int32_t  code;
    char*    message;
} KCM_Result;

typedef enum {
    KCM_OK = 0,
    KCM_ERR_NOT_FOUND = 1,
    KCM_ERR_OUT_OF_MEMORY = 2,
    KCM_ERR_INVALID_ARGUMENT = 3,
    KCM_ERR_IO = 4,
    KCM_ERR_CORRUPTED = 5,
    KCM_ERR_CONFLICT = 6,
    KCM_ERR_TRANSACTION_ABORTED = 7,
} KCM_ErrorCode;
```

### Query Result Functions

```c
KCM_Result KCM_QueryNext(KCM_Query* query, KCM_Fact* fact);
uint64_t   KCM_QueryLen(KCM_Query* query);
void       KCM_QueryFree(KCM_Query* query);
```

---

## Thread Safety

### Guarantees Per SDK

| Language | Thread Safety Model | Concurrent Reads | Concurrent Writes | Concurrent Read+Write |
|----------|-------------------|------------------|-------------------|----------------------|
| Rust | `Arc<Mutex<Database>>` | Via lock | Via lock | Via lock |
| C | User-managed locking | Manual | Manual | Manual |
| C++ | User-managed locking | Manual | Manual | Manual |
| Python | GIL-protected | Yes (GIL) | Yes (GIL) | Yes (GIL) |
| JavaScript | Single-threaded event loop | N/A | N/A | N/A |
| TypeScript | Single-threaded event loop | N/A | N/A | N/A |
| Go | `sync.RWMutex` | Yes | Via lock | Via lock |
| Java | `synchronized` methods | Yes | Via sync | Via sync |
| .NET | `ReaderWriterLockSlim` | Yes | Via lock | Via lock |

### Rules

1. No SDK operation is safe to call concurrently on the **same database handle** without external synchronization, except where noted.
2. **Different** database handles may be used concurrently from different threads.
3. Transaction handles are **not** thread-safe and must be used from a single thread.
4. Query handles are **not** thread-safe and must be used from a single thread.

---

## Memory Ownership

### Per-SDK Memory Management

| Language | Ownership Model | Resource Cleanup |
|----------|----------------|-----------------|
| Rust | Ownership system (RAII) | `Drop` trait |
| C | Manual ownership | `KCM_DatabaseFree`, `KCM_QueryFree`, `KCM_ResultFree`, `KCM_TransactionFree` |
| C++ | RAII (smart pointers) | `std::unique_ptr<KcmDatabase>`, destructors |
| Python | Reference counting + GC | `__del__`, context manager (`with`) |
| JavaScript | GC | `db.close()` (explicit), finalizer |
| TypeScript | GC | `db.close()` (explicit), `Disposable` interface |
| Go | GC + finalizer | `db.Close()`, `runtime.SetFinalizer` |
| Java | GC | `db.close()`, `AutoCloseable` |
| .NET | GC + Dispose | `db.Dispose()`, `IDisposable` |

### C Memory Protocol

| Function | Allocates | Frees |
|----------|-----------|-------|
| `KCM_DatabaseNew` | `KCM_Database*` | — |
| `KCM_DatabaseLoad` | `KCM_Database*` | — |
| `KCM_DatabaseClose` | — | `KCM_Database*` |
| `KCM_DatabaseQuery` | `KCM_Query*` | — |
| `KCM_QueryFree` | — | `KCM_Query*` |
| `KCM_ResultFree` | — | `KCM_Result.message` |

### Ownership Rules

1. The caller owns all pointers returned by FFI functions.
2. Every allocated pointer must be freed by the corresponding `Free` function.
3. After `Close`, the database pointer is invalid and must not be used.
4. After `QueryFree`, the query pointer is invalid and must not be used.
5. `KCM_Result` must be freed after checking the error code.

---

## Examples

### Rust

```rust
use kcm_sdk::{Database, Fact};

fn main() -> Result<(), kcm_sdk::KcmError> {
    let mut db = Database::new(Some("my_db.kcm"))?;

    let fact = Fact {
        subject: 42,
        predicate: 3,
        object: 108,
        confidence: 0.95,
        evidence: 1,
        timestamp: 1691328000000,
        context: 0,
        version: 1,
        priority: 10,
        owner: 1,
    };

    let row_id = db.insert(fact)?;
    println!("Inserted fact with RowID: {}", row_id);

    let results = db.query("SELECT * WHERE subject = 42")?;
    println!("Found {} facts", results.len());

    let count = db.active_fact_count()?;
    println!("Active facts: {}", count);

    db.save("my_db.kcm")?;
    db.close()?;

    Ok(())
}
```

### Python

```python
import kcm

db = kcm.Database("my_db.kcm")

fact = kcm.Fact(
    subject=42,
    predicate=3,
    object=108,
    confidence=0.95,
    evidence=1,
    timestamp=1691328000000,
    context=0,
    version=1,
    priority=10,
    owner=1,
)

row_id = db.insert(fact)
print(f"Inserted fact with RowID: {row_id}")

results = db.query("SELECT * WHERE subject = 42")
print(f"Found {len(results)} facts")

count = db.active_fact_count()
print(f"Active facts: {count}")

db.save("my_db.kcm")
db.close()
```

### JavaScript

```javascript
const { KcmDatabase, KcmFact } = require("@kcm/js");

async function main() {
  const db = await KcmDatabase.open("my_db.kcm");

  const fact = new KcmFact({
    subject: 42,
    predicate: 3,
    object: 108,
    confidence: 0.95,
    evidence: 1,
    timestamp: 1691328000000,
    context: 0,
    version: 1,
    priority: 10,
    owner: 1,
  });

  const rowId = db.insert(fact);
  console.log(`Inserted fact with RowID: ${rowId}`);

  const results = db.query("SELECT * WHERE subject = 42");
  console.log(`Found ${results.length} facts`);

  const count = db.activeFactCount();
  console.log(`Active facts: ${count}`);

  db.save("my_db.kcm");
  db.close();
}

main().catch(console.error);
```

---

## Compatibility Matrix

### OS and Architecture Support

| SDK | OS | Architecture | Engine Version | Status |
|-----|-----|-------------|---------------|--------|
| Rust | Linux, macOS, Windows | x86_64, aarch64 | 0.1.0 | Stable |
| C | Linux, macOS, Windows | x86_64, aarch64 | 0.1.0 | Stable |
| C++ | Linux, macOS, Windows | x86_64, aarch64 | 0.1.0 | Stable |
| Python | Linux, macOS, Windows | x86_64, aarch64 | 0.1.0 | Beta |
| JavaScript | Linux, macOS, Windows | x86_64, aarch64 | 0.1.0 | Beta |
| TypeScript | Linux, macOS, Windows | x86_64, aarch64 | 0.1.0 | Beta |
| Go | Linux, macOS, Windows | x86_64, aarch64 | 0.1.0 | Beta |
| Java | Linux, macOS, Windows | x86_64, aarch64 | 0.1.0 | Beta |
| .NET | Linux, macOS, Windows | x86_64, aarch64 | 0.1.0 | Beta |

### Language Version Requirements

| Language | Minimum Version | Recommended Version |
|----------|----------------|-------------------|
| Rust | 1.70 | Latest stable |
| C | C99 (C11 recommended) | C11 |
| C++ | C++17 | C++20 |
| Python | 3.9 | 3.12+ |
| Node.js | 18 LTS | 20 LTS |
| TypeScript | 5.0 | 5.3+ |
| Go | 1.21 | 1.22+ |
| Java | 11 (LTS) | 21 (LTS) |
| .NET | 6.0 | 8.0 |

---

## Versioning

### Version Scheme

SDK versions follow [Semantic Versioning](https://semver.org/):

```
MAJOR.MINOR.PATCH
```

| Change Type | Version Bump | Example |
|-------------|-------------|---------|
| Bug fix | Patch (x.y.Z) | Fix error mapping in Python SDK |
| New feature (additive) | Minor (x.Y.0) | Add `query_stream()` to Go SDK |
| Breaking API change | Major (X.0.0) | Change `insert()` return type |
| Engine update | Minor or Major | Depends on API impact |

### Version Tracking

| SDK | Package | Version Source |
|-----|---------|---------------|
| Rust | `kcm-sdk` | `Cargo.toml` |
| C | `libkcm` | `Makefile` VERSION |
| C++ | `libkcm` | `CMakeLists.txt` VERSION |
| Python | `kcm` | `pyproject.toml` version |
| JavaScript | `@kcm/js` | `package.json` version |
| TypeScript | `@kcm/ts` | `package.json` version |
| Go | `kcm` | Git tags |
| Java | `io.kcm:sdk` | `pom.xml` version |
| .NET | `Kcm.Sdk` | `*.csproj` version |

### Engine Compatibility

Each SDK declares the compatible engine version range:

```toml
[package]
name = "kcm-sdk"
version = "0.2.0"
# Requires engine 0.1.x
```

---

## Security

### FFI Safety

| Protection | Implementation |
|-----------|---------------|
| Null-pointer guards | All FFI functions check for null pointers before dereferencing |
| Bounds checking | Array and buffer operations validate bounds |
| String validation | C strings validated for null-termination before use |
| Integer overflow | Arithmetic operations checked for overflow |
| Buffer overflow | No raw buffer operations; all memory managed by engine |

### Input Validation

All SDK operations validate inputs before passing to the engine:

| Validation | Scope |
|-----------|-------|
| Path strings | Non-empty, valid characters, max length 4096 |
| Fact fields | Type checks, range checks (confidence: [0.0, 1.0]) |
| KQL strings | Syntax validation before execution |
| Row IDs | Non-zero, within valid range |
| Transaction handles | Non-null, not already committed/rolled back |

### Cryptographic Security

| Concern | Implementation |
|---------|---------------|
| Database encryption | Delegated to `kcm-security` (AES-256-GCM) |
| Checksums | Delegated to `kcm-storage` (BLAKE3) |
| Audit logging | Delegated to `kcm-security` (hash-chained audit log) |
| Key derivation | Delegated to `kcm-security` (Argon2) |

### Dependency Security

| Policy | Implementation |
|--------|---------------|
| Audit | All dependencies audited before inclusion |
| Minimal | Maximum dependency count enforced per SDK |
| Pinned | All dependency versions pinned in lock files |
| No unsafe | Unsafe code forbidden in SDK wrappers (Rust only) |

---

## Testing

### Test Pyramid

| Tier | Count | Speed | Purpose |
|------|-------|-------|---------|
| Unit | 89+ | < 100ms | Single function correctness |
| Integration | 470+ | 1s–5s | Cross-component correctness |
| Property | 8+ | 1–5min | Invariant verification |
| Security | 29+ | Varies | Attack surface validation |
| Cross-language | 16+ | 5–15min | API consistency across SDKs |

### Per-SDK Testing

| SDK | Unit Tests | Integration Tests | Property Tests | Security Tests |
|-----|-----------|------------------|----------------|----------------|
| Rust | `cargo test` | `cargo test --features integration` | `cargo test --features proptest` | `cargo test --features security` |
| C | `make test` | `make test-integration` | — | — |
| C++ | `ctest` | `ctest -E integration` | — | — |
| Python | `pytest tests/unit` | `pytest tests/integration` | — | — |
| JavaScript | `jest --unit` | `jest --integration` | — | — |
| TypeScript | `jest --unit` | `jest --integration` | — | — |
| Go | `go test ./...` | `go test -tags=integration ./...` | — | — |
| Java | `mvn test` | `mvn verify` | — | — |
| .NET | `dotnet test` | `dotnet test --filter Integration` | — | — |

### Cross-Language Consistency Tests

Each operation is tested across all SDKs to verify identical behavior:

1. Insert the same fact in all 9 SDKs
2. Query with the same KQL in all 9 SDKs
3. Verify identical results (modulo type differences)
4. Serialize/deserialize through JSON and verify round-trip

### API Compliance Validation

A compliance test suite validates that every SDK implements all 16 operations with correct signatures and semantics.

---

## SSOT Alignment

| SSOT Document | SDK Relevance |
|---------------|---------------|
| `PRD.md` | Core types (Fact, RowID, SubjectID, Confidence, KcmError), API semantics |
| `PRD2.md` | Storage format (column blocks, WAL entries), persistence layer, interfaces |
| `PRD3.md` | Distributed architecture, security model, compliance requirements |
| `sdk/README.md` | Cross-SDK API surface, language-specific notes |
| `docs/sdk/compatibility.md` | Compatibility matrix, OS/arch support |
| `docs/sdk/spesifikasi.md` | This document — SDK technical specification |
| `AGENTS.md` | Engineering constitution, non-negotiable rules, error model |

### Traceability

Every SDK operation traces back to an SSOT requirement:

| Operation | SSOT Source | Specification |
|-----------|------------|--------------|
| Database(path?) | PRD.md §4 | Database creation/opening semantics |
| Insert(fact) | PRD.md §4 | Fact insertion, RowID assignment |
| Query(kql) | PRD.md §5 | KQL parsing, query execution |
| Delete(row_id) | PRD.md §4 | Logical deletion (tombstone) |
| Update(row_id, fact) | PRD.md §4 | In-place update, version increment |
| GetFact(row_id) | PRD.md §4 | Point lookup by RowID |
| FactCount() | PRD2.md §18 | Total row count |
| ActiveFactCount() | PRD2.md §18 | Active row count |
| BeginTransaction() | PRD2.md §18 | Transaction creation |
| Commit(txn) | PRD2.md §18 | Atomic commit |
| Rollback(txn) | PRD2.md §18 | Transaction rollback |
| Save(path) | PRD2.md §15 | Database serialization to file |
| Load(path) | PRD2.md §15 | Database deserialization from file |
| Verify(path) | PRD2.md §15 | Integrity verification |
| Close() | PRD.md §4 | Resource cleanup |

---

## References

- [sdk/README.md](../../sdk/README.md) — SDK overview
- [sdk/CONTRIBUTING.md](../../sdk/CONTRIBUTING.md) — SDK contribution guide
- [sdk/SECURITY.md](../../sdk/SECURITY.md) — SDK security policy
- [docs/specs/KCM_API_SPEC.md](../specs/KCM_API_SPEC.md) — API specification
- [docs/sdk/compatibility.md](compatibility.md) — Compatibility matrix
- [SSOT.md](../../SSOT.md) — Single Source of Truth
- [AGENTS.md](../../AGENTS.md) — Engineering constitution
