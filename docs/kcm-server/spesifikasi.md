# kcm-server Technical Specification

## Overview

kcm-server is the HTTP and gRPC server binary crate for the KCM (Knowledge Columnar Model) engine. It provides network-facing services that expose KCM's knowledge storage, query, and reasoning capabilities over HTTP (actix-web) and gRPC (tonic) protocols. kcm-server owns server lifecycle, TLS configuration, middleware wiring, and graceful shutdown. All business logic is delegated to `kcm-interface`.

## Scope

This specification covers:

- HTTP server binary (`kcm-server`) with REST endpoints
- gRPC server binary (`kcm-grpc`) with protobuf services
- Server startup, configuration, and graceful shutdown
- Request routing and middleware integration
- TLS and connection management

This specification does not cover:

- Business logic implementation (owned by `kcm-interface`)
- Storage engine internals (owned by `kcm-storage`)
- Security primitives (owned by `kcm-security`)

## Responsibilities

| Responsibility | Owner | Description |
|---------------|-------|-------------|
| HTTP Server | kcm-server | actix-web binary serving REST endpoints |
| gRPC Server | kcm-server | tonic binary serving protobuf RPCs |
| Request Routing | kcm-server | Maps URLs/RPCs to interface handlers |
| Graceful Shutdown | kcm-server | Drains in-flight requests on SIGTERM/SIGINT |
| TLS Configuration | kcm-server | Loads certificates, configures TLS |
| Middleware Wiring | kcm-server | Configures auth, logging, CORS middleware |
| Server Lifecycle | kcm-server | Startup, health check, shutdown coordination |

## Technical Specification

### HTTP Server

- Framework: actix-web 4
- Protocol: HTTP/1.1 and HTTP/2
- TLS: Native actix-web TLS support via `actix-rt`
- Serialization: JSON via `serde_json`
- Request body limit: Configurable, default 1 MB
- Response compression: Configurable via middleware

### gRPC Server

- Framework: tonic 0.12
- Protocol: HTTP/2 with gRPC framing
- Code generation: tonic-build with prost
- Serialization: Protocol Buffers (protobuf3)
- TLS: tonic native TLS support
- Max message size: Configurable, default 4 MB

### Graceful Shutdown

- Signal handling: `tokio::signal::ctrl_c()`
- On signal: stop accepting new connections
- Drain period: Configurable, default 30 seconds
- Force termination after drain period

### Request Handling

- All requests pass through `kcm-interface` handlers
- Authentication middleware delegates to `kcm-security` RBAC
- Audit logging middleware logs all requests
- Error responses follow `KcmError` → HTTP status mapping

## Architecture

```
┌─────────────────────────────────────────────────┐
│                  kcm-server                      │
│                                                  │
│  ┌──────────┐  ┌──────────┐  ┌──────────────┐  │
│  │ main.rs  │  │grpc_main │  │ grpc_server.rs│  │
│  │ (HTTP)   │  │  .rs     │  │              │  │
│  └────┬─────┘  └────┬─────┘  └──────┬───────┘  │
│       │              │               │           │
│       ▼              ▼               ▼           │
│  ┌──────────────────────────────────────────┐   │
│  │             kcm-interface                 │   │
│  │   REST handlers │ gRPC handlers │ FFI    │   │
│  └──────────────────┬───────────────────────┘   │
│                     │                            │
│       ┌─────────────┼─────────────┐             │
│       ▼             ▼             ▼             │
│  ┌─────────┐  ┌──────────┐  ┌────────────┐    │
│  │kcm-core │  │kcm-      │  │kcm-security│    │
│  │         │  │runtime   │  │            │    │
│  └─────────┘  └──────────┘  └────────────┘    │
└─────────────────────────────────────────────────┘
```

## Internal Components

### main.rs — HTTP Server Binary

Entry point for the `kcm-server` binary. Responsibilities:

- Parse command-line arguments and environment variables
- Configure actix-web `HttpServer` with app state
- Register REST endpoint routes
- Configure middleware (auth, logging, CORS, compression)
- Start server on configured bind address
- Handle graceful shutdown on SIGTERM/SIGINT

### grpc_main.rs — gRPC Server Binary

Entry point for the `kcm-grpc` binary. Responsibilities:

- Parse command-line arguments and environment variables
- Configure tonic `Server` with gRPC services
- Register protobuf service handlers
- Configure TLS if certificates are provided
- Start server on configured bind address
- Handle graceful shutdown on SIGTERM/SIGINT

### grpc_server.rs — gRPC Service Implementation

Implements the tonic service traits for gRPC RPCs. Responsibilities:

- Implement `KnowledgeService` trait methods
- Convert protobuf messages to/from KCM types
- Delegate to `kcm-interface` handlers
- Return protobuf responses or gRPC status errors

### build.rs — Code Generation

Build script for tonic code generation. Responsibilities:

- Compile `.proto` files using `tonic-build`
- Generate Rust types and service traits from protobuf definitions
- Output generated code to `$OUT_DIR`

## Data Model

### InsertRequest

```rust
pub struct InsertRequest {
    pub subject: String,
    pub predicate: String,
    pub object: String,
    pub confidence: f64,
    pub evidence: Option<String>,
    pub context: Option<String>,
}
```

### UpdateRequest

```rust
pub struct UpdateRequest {
    pub id: u64,
    pub subject: Option<String>,
    pub predicate: Option<String>,
    pub object: Option<String>,
    pub confidence: Option<f64>,
    pub evidence: Option<String>,
    pub context: Option<String>,
}
```

### BatchInsertRequest

```rust
pub struct BatchInsertRequest {
    pub facts: Vec<InsertRequest>,
}
```

### QueryParams

```rust
pub struct QueryParams {
    pub subject: Option<String>,
    pub predicate: Option<String>,
    pub object: Option<String>,
    pub min_confidence: Option<f64>,
    pub max_confidence: Option<f64>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}
```

## Execution Flow

### HTTP Request Flow

```
Client → TCP Connection → actix-web Router → Middleware Chain → Handler → kcm-interface → Response
```

1. Client establishes TCP connection (TLS if configured)
2. actix-web parses HTTP request
3. Request passes through middleware chain:
   - Security headers
   - CORS
   - Request logging / audit
   - Authentication (RBAC via kcm-interface → kcm-security)
   - Rate limiting
4. Router dispatches to appropriate handler
5. Handler delegates to `kcm-interface`
6. Response serialized and returned to client

### gRPC Call Flow

```
Client → HTTP/2 Connection → tonic Router → grpc_server → kcm-interface → Response
```

1. Client establishes HTTP/2 connection (TLS if configured)
2. tonic parses gRPC frame
3. Request passes through interceptor (auth)
4. Router dispatches to service method
5. Service method delegates to `kcm-interface`
6. Response serialized as protobuf and returned

### Startup Flow

```
main() → Load Config → Init Logger → Init Runtime → Configure Server → Bind → Start → Wait for Signal → Shutdown
```

1. Parse CLI args and environment variables
2. Initialize `env_logger` with configured level
3. Initialize `KnowledgeDatabase` via `kcm-runtime`
4. Configure actix-web/tonic server with handlers and middleware
5. Bind to configured address
6. Start accepting connections
7. Wait for SIGTERM/SIGINT
8. Initiate graceful shutdown (drain in-flight requests)
9. Close database connections
10. Exit

## Public API

### REST Endpoints

| Method | Path | Description | Auth Required |
|--------|------|-------------|---------------|
| GET | /health | Health check | No |
| GET | /metrics | Server metrics (JSON) | Yes |
| GET | /openapi.json | OpenAPI specification | No |
| POST | /api/v1/facts | Insert a fact | Yes (Write) |
| GET | /api/v1/facts/{id} | Get a fact by ID | Yes (Read) |
| GET | /api/v1/facts | Query facts with filters | Yes (Read) |
| PUT | /api/v1/facts/{id} | Update a fact | Yes (Write) |
| DELETE | /api/v1/facts/{id} | Delete a fact | Yes (Write) |
| POST | /api/v1/facts/batch | Batch insert facts | Yes (Write) |
| GET | /api/v1/stats | Database statistics | Yes (Read) |
| GET | /facts | Legacy fact listing | Yes (Read) |

### gRPC RPCs

| Service | Method | Description | Auth Required |
|---------|--------|-------------|---------------|
| KnowledgeService | InsertFact | Insert a single fact | Yes (Write) |
| KnowledgeService | GetFact | Retrieve a fact by ID | Yes (Read) |
| KnowledgeService | QueryFacts | Query facts with filters | Yes (Read) |
| KnowledgeService | UpdateFact | Update a fact | Yes (Write) |
| KnowledgeService | DeleteFact | Delete a fact | Yes (Write) |
| KnowledgeService | BatchInsert | Batch insert facts | Yes (Write) |
| KnowledgeService | GetStats | Database statistics | Yes (Read) |
| KnowledgeService | HealthCheck | Health check | No |

## Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| KCM_BIND_ADDRESS | 0.0.0.0:8080 | HTTP server bind address |
| KCM_GRPC_BIND_ADDRESS | 0.0.0.0:9090 | gRPC server bind address |
| KCM_DATABASE_PATH | /tmp/kcm.db | Path to database file |
| KCM_LOG_LEVEL | info | Log level (error, warn, info, debug, trace) |
| KCM_TLS_CERT_PATH | (none) | Path to TLS certificate (PEM) |
| KCM_TLS_KEY_PATH | (none) | Path to TLS private key (PEM) |
| KCM_MAX_REQUEST_SIZE | 1048576 | Maximum request body size in bytes |
| KCM_REQUEST_TIMEOUT | 30 | Request timeout in seconds |
| KCM_SHUTDOWN_TIMEOUT | 30 | Graceful shutdown timeout in seconds |
| KCM_RATE_LIMIT | (none) | Requests per second per client (optional) |
| KCM_CORS_ORIGIN | (none) | Allowed CORS origin (optional) |
| KCM_API_KEY | (none) | API key for authentication (optional) |

## Dependencies

| Dependency | Version | Purpose |
|-----------|---------|---------|
| kcm-core | path | Core types and error model |
| kcm-runtime | path | KnowledgeDatabase, transactions, metrics |
| kcm-interface | path | REST/gRPC handlers, middleware |
| kcm-security | path | RBAC, encryption, audit log |
| actix-web | 4 | HTTP server framework |
| actix-rt | 4 | actix async runtime |
| tonic | 0.12 | gRPC server framework |
| prost | 0.13 | Protocol Buffers serialization |
| serde | 1 | Serialization framework |
| serde_json | 1 | JSON serialization |
| tokio | 1 | Async runtime |
| env_logger | 0.11 | Logging initialization |
| log | 0.4 | Logging facade |

### Build Dependencies

| Dependency | Version | Purpose |
|-----------|---------|---------|
| tonic-build | 0.12 | Protobuf code generation |
| protoc-bin-vendored | 3 | Vendored protoc compiler |

## Error Handling

All errors follow the `KcmError` hierarchy defined in `kcm-core`. HTTP status code mapping:

| KcmError Variant | HTTP Status | gRPC Status |
|-----------------|-------------|-------------|
| NotFound | 404 | NOT_FOUND |
| InvalidArgument | 400 | INVALID_ARGUMENT |
| Conflict | 409 | ALREADY_EXISTS |
| OutOfMemory | 503 | RESOURCE_EXHAUSTED |
| Corrupted | 500 | INTERNAL |
| Io | 500 | INTERNAL |
| TransactionAborted | 409 | ABORTED |

Error responses include a JSON body with `error` and `message` fields. No internal details (stack traces, file paths) are exposed to clients.

## Performance Characteristics

| Metric | Target |
|--------|--------|
| REST P99 latency | < 100ms |
| gRPC P99 latency | < 50ms |
| Max concurrent connections | 10,000 |
| Max request body size | 1 MB (configurable) |
| Graceful shutdown drain | 30s (configurable) |
| Startup time | < 2s |

## Security Considerations

- TLS must be enabled in production deployments
- All non-health endpoints require authentication via RBAC middleware
- Request body size limits prevent memory exhaustion
- Request timeouts prevent slow-loris attacks
- Security headers (HSTS, CSP, X-Frame-Options) are set on all responses
- CORS policy is restrictive by default
- No secrets are logged or included in error responses
- Audit logging records all requests with client IP and authenticated user

## Integration

kcm-server integrates with:

| Component | Integration Type |
|-----------|-----------------|
| kcm-core | Direct dependency (types, errors) |
| kcm-runtime | Direct dependency (database, transactions) |
| kcm-interface | Direct dependency (handlers, middleware) |
| kcm-security | Indirect via kcm-interface (RBAC, audit) |
| kcm-storage | Indirect via kcm-runtime (persistence) |
| kcm-compute | Indirect via kcm-runtime (query execution) |
| kcm-reasoning | Indirect via kcm-runtime (inference) |
| kcm-optimizer | Indirect via kcm-runtime (query planning) |

## Sequence Diagram

### HTTP Request Lifecycle

```
Client          kcm-server        middleware         kcm-interface      kcm-runtime
  │                 │                 │                  │                  │
  │── POST /api/v1/facts ──────────▶│                  │                  │
  │                 │── parse request ─▶│                │                  │
  │                 │                 │── auth check ───▶│                  │
  │                 │                 │  (RBAC)          │                  │
  │                 │                 │◀── auth OK ─────│                  │
  │                 │                 │── audit log ────▶│                  │
  │                 │                 │── rate check ───▶│                  │
  │                 │                 │◀── pass ────────│                  │
  │                 │                 │── forward ──────▶│                  │
  │                 │                 │                  │── insert fact ──▶│
  │                 │                 │                  │◀── row_id ──────│
  │                 │                 │◀── 201 Created ──│                  │
  │                 │◀── response ────│                  │                  │
  │◀── 201 Created ─│                │                  │                  │
```

## Architecture Diagram

```
┌──────────────────────────────────────────────────────────────────┐
│                        Network Layer                              │
│  ┌─────────────────────┐          ┌─────────────────────┐       │
│  │   HTTP (actix-web)  │          │   gRPC (tonic)      │       │
│  │   :8080             │          │   :9090             │       │
│  └──────────┬──────────┘          └──────────┬──────────┘       │
│             │                                │                   │
│  ┌──────────▼────────────────────────────────▼──────────┐       │
│  │              Middleware Pipeline                       │       │
│  │  Security Headers → CORS → Auth → Audit → Rate Limit │       │
│  └──────────────────────────┬───────────────────────────┘       │
│                             │                                    │
│  ┌──────────────────────────▼───────────────────────────┐       │
│  │              kcm-interface                            │       │
│  │  REST Handlers    gRPC Handlers    FFI Bridge         │       │
│  └──────────────────────────┬───────────────────────────┘       │
│                             │                                    │
│  ┌──────────────────────────▼───────────────────────────┐       │
│  │              kcm-runtime                              │       │
│  │  KnowledgeDatabase    Transactions    Metrics         │       │
│  └──────────┬───────────────────┬───────────────────────┘       │
│             │                   │                                │
│  ┌──────────▼────────┐  ┌──────▼──────────┐                    │
│  │    kcm-storage    │  │  kcm-compute    │                    │
│  │  Columns, Codecs  │  │  Operators, SIMD│                    │
│  └───────────────────┘  └─────────────────┘                    │
└──────────────────────────────────────────────────────────────────┘
```

## References

- PRD2.md §19 — Interface and server layer specification
- PRD3.md §28 — Distributed and server architecture
- SSOT.md — Single Source of Truth for all specifications
- AGENTS.md — Engineering constitution
- `kcm-interface` crate — Handler and middleware implementations
- `kcm-runtime` crate — Database and transaction management
- `kcm-security` crate — RBAC, encryption, audit log

## SSOT Alignment

| Specification | SSOT Reference | Status |
|--------------|---------------|--------|
| HTTP server with REST endpoints | PRD2.md §19 | Implemented |
| gRPC server with protobuf | PRD3.md §28 | Implemented |
| Graceful shutdown | PRD2.md §19 | Implemented |
| TLS configuration | PRD3.md §30 | Implemented |
| RBAC middleware integration | PRD3.md §30 | Implemented |
| Audit logging | PRD3.md §30 | Implemented |
| Health check endpoint | PRD2.md §19 | Implemented |
| Metrics endpoint | PRD2.md §18 | Implemented |
| Request size limits | PRD-TESTING | Implemented |
| Timeout enforcement | PRD-TESTING | Implemented |
