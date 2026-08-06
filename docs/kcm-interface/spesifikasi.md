# kcm-interface Technical Specification

## Overview

kcm-interface is the external-facing boundary of the KCM system. It provides all public interfaces through which external consumers interact with the KCM columnar knowledge engine: C FFI, Python bindings, REST API, and a KQL query parser.

## Scope

This specification covers:

- 18 C FFI functions with `#[repr(C)]` types
- Python bindings via PyO3 (optional, feature-gated)
- REST API handlers via actix-web
- KQL (KCM Query Language) parser
- OpenAPI specification generation
- Middleware: authentication, CORS, logging, rate limiting, request ID

## Responsibilities

| Component | Responsibility |
|-----------|---------------|
| C FFI | Provide safe, stable C-callable interface to KCM engine |
| Python bindings | Expose KCM engine to Python via PyO3 |
| REST API | HTTP CRUD operations on knowledge bases |
| KQL parser | Parse SQL-like query language into executable plans |
| OpenAPI | Generate machine-readable API specifications |
| Middleware | Enforce cross-cutting concerns (auth, CORS, logging, rate limits) |

## Technical Specification

### C FFI

18 `extern "C"` functions with `#[repr(C)]` types. All functions accept and return C-compatible types only. No Rust `String` or `Vec` crosses the FFI boundary.

Functions: `KCM_DatabaseNew`, `KCM_DatabaseFree`, `KCM_DatabaseInsert`, `KCM_DatabaseUpdate`, `KCM_DatabaseDelete`, `KCM_DatabaseFactCount`, `KCM_DatabaseActiveCount`, `KCM_DatabaseQuery`, `KCM_QueryNext`, `KCM_QueryFree`, `KCM_DatabaseBeginTransaction`, `KCM_TransactionFree`, `KCM_TransactionCommit`, `KCM_TransactionRollback`, `KCM_DatabaseSave`, `KCM_DatabaseLoad`, `KCM_DatabaseVerify`, `KCM_ErrorMessage`.

### REST API

actix-web handlers for CRUD operations on facts, queries, transactions, and database management. All endpoints are protected by authentication middleware.

### KQL Parser

SQL-like query language parser that transforms KQL strings into internal query representations. Supports `SELECT`, `WHERE`, `JOIN`, `GROUP BY`, and aggregate functions.

### Python Bindings

PyO3-based bindings (optional, `python` feature flag). Provides `KCMDatabase` and `KCMQuery` Python classes.

### Middleware

| Middleware | File | Purpose |
|-----------|------|---------|
| Auth | `middleware/auth.rs` | RBAC enforcement via kcm-security |
| CORS | `middleware/cors.rs` | Cross-origin request policy |
| Logging | `middleware/logging.rs` | Structured request/response logging |
| Rate Limit | `middleware/rate_limit.rs` | Per-client request throttling |
| Request ID | `middleware/request_id.rs` | Unique request identification |

## Architecture

```
External Consumers
    │
    ├── C/C++ Applications ──→ FFI (lib.rs)
    ├── Python Applications ──→ PyO3 Bindings (python.rs)
    ├── HTTP Clients ────────→ REST API (rest_api.rs)
    │                              │
    │                         Middleware
    │                         ├── Auth (auth.rs)
    │                         ├── CORS (cors.rs)
    │                         ├── Logging (logging.rs)
    │                         ├── Rate Limit (rate_limit.rs)
    │                         └── Request ID (request_id.rs)
    │
    └── KQL Queries ────────→ KQL Parser (kql_parser.rs)
                                    │
                               kcm-runtime
                                    │
                               kcm-storage
                                    │
                               kcm-core
```

## Internal Components

### `lib.rs`

Root module. Exports all public FFI functions and manages the crate's public API surface. Contains the `extern "C"` function definitions and safety wrappers.

### `kql_parser.rs`

KQL grammar definition and parser. Tokenizes and parses KQL query strings into internal AST representations. Validates query syntax and semantic correctness.

### `openapi.rs`

OpenAPI 3.0 specification generator. Produces machine-readable API documentation for all REST endpoints. Used by API gateway and client SDK generators.

### `python.rs`

PyO3 module definition. Exposes `KCMDatabase`, `KCMQuery`, and `KCMFact` as Python classes. Feature-gated behind the `python` flag.

### `rest_api.rs`

actix-web route definitions and handler functions. Implements CRUD endpoints for facts, queries, transactions, and database lifecycle operations.

### `examples/mod.rs`

Module index for example applications.

### `examples/ecommerce.rs`

E-commerce knowledge base example. Demonstrates product catalog, recommendation, and inventory queries.

### `examples/medical.rs`

Medical knowledge base example. Demonstrates diagnosis support, drug interaction, and patient record queries.

### `middleware/mod.rs`

Module index for middleware components.

### `middleware/auth.rs`

Authentication and RBAC middleware. Integrates with `kcm-security` for permission checking. Extracts identity from request headers and validates against the RBAC engine.

### `middleware/cors.rs`

CORS middleware. Configurable origin whitelist. Rejects requests from unauthorized origins.

### `middleware/logging.rs`

Structured logging middleware. Records method, path, status, latency, and client IP for every request.

### `middleware/rate_limit.rs`

Rate limiting middleware. Per-client request throttling with configurable limits.

### `middleware/request_id.rs`

Request ID middleware. Generates and attaches a unique UUID to every request for tracing.

## Data Model

### KCM_Fact (`#[repr(C)]`)

```c
struct KCM_Fact {
    subject: KCM_SubjectID,    // u32
    predicate: KCM_Predicate,  // u8
    object: KCM_ObjectID,      // u32
    confidence: f64,
    evidence: KCM_Evidence,    // u8
    timestamp: i64,
    context: KCM_Context,      // u8
    version: i32,
    priority: i8,
    owner: KCM_OwnerID,        // u16
}
```

### KCM_Error (`#[repr(C)]`)

```c
struct KCM_Error {
    code: i32,
    message: *const c_char,
}
```

### KCM_Database

Opaque pointer to the internal `KnowledgeDatabase` instance. No internal layout is exposed.

### KCM_Transaction

Opaque pointer to an active transaction handle.

### KCM_Query

Opaque pointer to an active query cursor.

## Execution Flow

### FFI Call Flow

```
Caller → KCM_DatabaseNew()
    1. Validate arguments (null checks)
    2. Create KnowledgeDatabase via kcm-runtime
    3. Wrap in opaque handle
    4. Return handle or KCM_Error
```

### REST Request Flow

```
HTTP Request → Middleware Stack
    → Request ID (attach UUID)
    → Logging (start timer)
    → Rate Limit (check quota)
    → Auth (validate identity + RBAC)
    → CORS (check origin)
    → Handler (process request)
    → Response
```

### KQL Parse Flow

```
KQL String → kql_parser.rs
    1. Tokenize input
    2. Parse tokens into AST
    3. Validate syntax
    4. Validate semantics (table/column existence)
    5. Return AST or parse error
```

## Public API

### FFI Functions

| Function | Arguments | Returns | Description |
|----------|-----------|---------|-------------|
| `KCM_DatabaseNew` | `path: *const c_char` | `*mut KCM_Database` or `NULL` | Create or open a database |
| `KCM_DatabaseFree` | `db: *mut KCM_Database` | `KCM_Error` | Free a database handle |
| `KCM_DatabaseInsert` | `db, fact: *const KCM_Fact` | `KCM_Error` | Insert a fact |
| `KCM_DatabaseUpdate` | `db, fact: *const KCM_Fact` | `KCM_Error` | Update a fact |
| `KCM_DatabaseDelete` | `db, subject, predicate, object` | `KCM_Error` | Delete a fact |
| `KCM_DatabaseFactCount` | `db` | `u64` or `0` on error | Total fact count |
| `KCM_DatabaseActiveCount` | `db` | `u64` or `0` on error | Active fact count |
| `KCM_DatabaseQuery` | `db, query: *const c_char` | `*mut KCM_Query` or `NULL` | Execute a KQL query |
| `KCM_QueryNext` | `query, fact: *mut KCM_Fact` | `i32` (1=next, 0=done) | Iterate query results |
| `KCM_QueryFree` | `query: *mut KCM_Query` | `KCM_Error` | Free a query cursor |
| `KCM_DatabaseBeginTransaction` | `db` | `*mut KCM_Transaction` or `NULL` | Begin a transaction |
| `KCM_TransactionFree` | `txn: *mut KCM_Transaction` | `KCM_Error` | Free a transaction handle |
| `KCM_TransactionCommit` | `txn` | `KCM_Error` | Commit a transaction |
| `KCM_TransactionRollback` | `txn` | `KCM_Error` | Rollback a transaction |
| `KCM_DatabaseSave` | `db` | `KCM_Error` | Persist to disk |
| `KCM_DatabaseLoad` | `db` | `KCM_Error` | Reload from disk |
| `KCM_DatabaseVerify` | `db` | `KCM_Error` | Verify integrity |
| `KCM_ErrorMessage` | `error: *const KCM_Error` | `*const c_char` | Get error message |

### REST Endpoints

| Method | Path | Description |
|--------|------|-------------|
| POST | `/api/v1/facts` | Insert a fact |
| GET | `/api/v1/facts` | List facts |
| PUT | `/api/v1/facts` | Update a fact |
| DELETE | `/api/v1/facts/{id}` | Delete a fact |
| POST | `/api/v1/query` | Execute a KQL query |
| POST | `/api/v1/transactions/begin` | Begin a transaction |
| POST | `/api/v1/transactions/{id}/commit` | Commit a transaction |
| POST | `/api/v1/transactions/{id}/rollback` | Rollback a transaction |
| POST | `/api/v1/database/save` | Persist database |
| POST | `/api/v1/database/load` | Reload database |
| POST | `/api/v1/database/verify` | Verify integrity |

## Configuration

| Setting | Default | Description |
|---------|---------|-------------|
| `KCM_AUTH_ENABLED` | `true` | Enable authentication middleware |
| `KCM_CORS_ORIGINS` | `localhost` | Allowed CORS origins |
| `KCM_RATE_LIMIT_RPM` | `1000` | Requests per minute per client |
| `KCM_LOG_LEVEL` | `info` | Logging level |
| `KCM_FFI_SAFETY_CHECKS` | `true` | Enable null-pointer guards in FFI |

## Dependencies

| Crate | Purpose | Justification |
|-------|---------|---------------|
| kcm-core | Core types (Fact, KcmError) | Foundation types |
| kcm-storage | Storage engine | Column persistence |
| kcm-runtime | KnowledgeDatabase, transactions | High-level operations |
| kcm-security | RBAC, encryption | Auth middleware |
| parking_lot | RwLock, Mutex | 3-5x faster than std |
| serde / serde_json | Serialization | REST API JSON handling |
| pyo3 | Python bindings | Optional Python interface |

## Error Handling

All public APIs return `Result<T, KcmError>`. FFI functions return `KCM_Error` with a numeric code and message pointer. The `KCM_ErrorMessage` function extracts the message from an error handle.

```
KcmError::NotFound       → KCM_ERROR_NOT_FOUND (1)
KcmError::OutOfMemory    → KCM_ERROR_OUT_OF_MEMORY (2)
KcmError::InvalidArgument→ KCM_ERROR_INVALID_ARGUMENT (3)
KcmError::Io             → KCM_ERROR_IO (4)
KcmError::Corrupted      → KCM_ERROR_CORRUPTED (5)
KcmError::Conflict       → KCM_ERROR_CONFLICT (6)
KcmError::TransactionAborted → KCM_ERROR_TRANSACTION_ABORTED (7)
```

## Performance Characteristics

- FFI call overhead: < 1μs for simple operations
- REST middleware stack latency: < 100μs per request
- KQL parse time: < 1ms for queries under 1KB
- No allocations in hot FFI paths where avoidable

## Security Considerations

### FFI Safety

- Every FFI function validates all pointer arguments for null.
- No raw pointer arithmetic without bounds validation.
- FFI handles are opaque; internal layout is never exposed.
- All FFI functions return error codes on failure — never panic.
- Thread safety: FFI functions are safe to call from multiple threads (internal synchronization via parking_lot).

### REST Security

- Authentication middleware protects all non-public endpoints.
- CORS middleware enforces strict origin whitelist.
- Rate limiting prevents abuse.
- All input is validated before processing.

## Integration

kcm-interface integrates with:

- **kcm-core**: Core types (`Fact`, `RowID`, `KcmError`)
- **kcm-storage**: Storage engine for persistence
- **kcm-runtime**: `KnowledgeDatabase`, transaction management
- **kcm-security**: RBAC enforcement, encryption, audit logging

## Sequence Diagram — FFI Lifecycle

```
┌──────────┐     ┌──────────────┐     ┌───────────┐     ┌──────────┐
│  Caller  │     │  kcm-interface│    │ kcm-runtime│    │kcm-storage│
└────┬─────┘     └──────┬───────┘     └─────┬─────┘     └─────┬────┘
     │                  │                    │                   │
     │  KCM_DatabaseNew │                    │                   │
     │─────────────────→│  validate null     │                   │
     │                  │───────────────────→│  create DB        │
     │                  │                    │──────────────────→│
     │                  │                    │←──────────────────│
     │                  │←───────────────────│                   │
     │  return handle   │                    │                   │
     │←─────────────────│                    │                   │
     │                  │                    │                   │
     │  KCM_DatabaseInsert                   │                   │
     │─────────────────→│  validate          │                   │
     │                  │───────────────────→│  insert fact      │
     │                  │                    │──────────────────→│
     │                  │                    │←──────────────────│
     │                  │←───────────────────│                   │
     │  return OK       │                    │                   │
     │←─────────────────│                    │                   │
     │                  │                    │                   │
     │  KCM_DatabaseSave │                    │                   │
     │─────────────────→│  validate          │                   │
     │                  │───────────────────→│  persist to disk  │
     │                  │                    │──────────────────→│
     │                  │                    │←──────────────────│
     │                  │←───────────────────│                   │
     │  return OK       │                    │                   │
     │←─────────────────│                    │                   │
     │                  │                    │                   │
     │  KCM_DatabaseFree │                    │                   │
     │─────────────────→│  validate          │                   │
     │                  │───────────────────→│  destroy DB       │
     │                  │                    │──────────────────→│
     │                  │                    │←──────────────────│
     │                  │←───────────────────│                   │
     │  return OK       │                    │                   │
     │←─────────────────│                    │                   │
```

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                      kcm-interface                          │
│                                                             │
│  ┌─────────┐  ┌──────────┐  ┌──────────┐  ┌─────────────┐ │
│  │  FFI    │  │  REST    │  │  Python  │  │  KQL Parser │ │
│  │ (lib.rs)│  │(rest_api)│  │(python.rs│  │(kql_parser) │ │
│  └────┬────┘  └────┬─────┘  └────┬─────┘  └──────┬──────┘ │
│       │            │             │                │         │
│  ┌────┴────────────┴─────────────┴────────────────┴──────┐  │
│  │                    Middleware Layer                     │  │
│  │  ┌──────┐ ┌──────┐ ┌─────────┐ ┌──────────┐ ┌──────┐ │  │
│  │  │ Auth │ │ CORS │ │ Logging │ │Rate Limit│ │Req ID│ │  │
│  │  └──────┘ └──────┘ └─────────┘ └──────────┘ └──────┘ │  │
│  └────────────────────────────────────────────────────────┘  │
│                          │                                   │
└──────────────────────────┼───────────────────────────────────┘
                           │
┌──────────────────────────┼───────────────────────────────────┐
│                      kcm-runtime                             │
│  KnowledgeDatabase, Transactions, Metrics, Health            │
└──────────────────────────┼───────────────────────────────────┘
                           │
┌──────────────────────────┼───────────────────────────────────┐
│                      kcm-storage                             │
│  Columns, Codecs, WAL, FileFormat, Index                     │
└──────────────────────────┼───────────────────────────────────┘
                           │
┌──────────────────────────┼───────────────────────────────────┐
│                       kcm-core                               │
│  Fact, RowID, SubjectID, Confidence, KcmError                │
└──────────────────────────────────────────────────────────────┘
```

## References

| Document | Section | Relevance |
|----------|---------|-----------|
| `docs/PRD2.md` | §19 | Interface specification (SSOT) |
| `docs/PRD.md` | §7 | FFI function definitions (SSOT) |
| `docs/SSOT.md` | All | Single source of truth |
| `AGENTS.md` | All | Engineering constitution |

## SSOT Alignment

| This Specification | SSOT Source | Alignment |
|--------------------|-------------|-----------|
| 18 FFI functions | `docs/PRD.md §7` | Aligned |
| `#[repr(C)]` types | `docs/PRD.md §7` | Aligned |
| REST API endpoints | `docs/PRD2.md §19` | Aligned |
| Middleware stack | `docs/PRD2.md §19` | Aligned |
| KQL parser | `docs/PRD2.md §19` | Aligned |
| Python bindings | `docs/PRD2.md §19` | Aligned |
| Error codes | `AGENTS.md` Error Model | Aligned |
| RBAC integration | `docs/PRD3.md §30` | Aligned |
