# KCM Repository Structure — Complete Reference

## Table of Contents

1. [Overview](#overview)
2. [Root Directory Layout](#root-directory-layout)
3. [Core Crates (`crates/`)](#core-crates)
4. [SDK Language Bindings (`sdk/`)](#sdk-language-bindings)
5. [CLI Tools (`scripts/kcm-cli/`)](#cli-tools)
6. [Documentation (`docs/`)](#documentation)
7. [Deployment (`deployment/`)](#deployment)
8. [Testing Infrastructure (`tests/`)](#testing-infrastructure)
9. [CI/CD Pipelines (`.github/workflows/`)](#cicd-pipelines)
10. [Engineering Skills (`skills/`)](#engineering-skills)
11. [Documentation Tools (`tools/`)](#documentation-tools)
12. [Examples (`examples/`)](#examples)
13. [Build & Tool Configuration](#build--tool-configuration)
14. [Dependency Flow](#dependency-flow)
15. [Quality Gates](#quality-gates)

---

## Overview

**KCM (Knowledge Columnar Model)** is a self-contained columnar knowledge representation, storage, query, and reasoning engine implemented in Rust. The repository is organized as a Cargo workspace containing **13 core crates**, **16 CLI tools**, **9 language SDKs**, and a complete deployment stack.

- **Language:** Rust (edition 2021, stable toolchain)
- **License:** MIT
- **Repository:** https://github.com/bighopepr-cyber/KCM
- **Minimum Rust Version:** 1.85+
- **Architecture:** Monorepo with Cargo workspace

---

## Root Directory Layout

```
KCM/
├── AGENTS.md                         # Engineering constitution & governance rules
├── ARCHITECTURE_CONSISTENCY_MATRIX.md # Component registry & consistency matrix
├── CODE_OF_CONDUCT.md                # Community code of conduct
├── CONTRIBUTING.md                   # Contribution guidelines
├── KCM_ENGINEERING_RULES.md          # Engineering rules reference
├── KCM_SPECIFICATION.md              # Technical specification
├── LICENSE                           # MIT License
├── README.md                         # Project overview & quick start
├── REPOSITORY_STRUCTURE.md           # This document
├── ROADMAP.md                        # Release plan & milestones
├── SECURITY.md                       # Security policy
├── SSOT.md                           # Single Source of Truth index
├── SSOT_CERTIFICATION_REPORT.md      # SSOT audit certification
├── Cargo.toml                        # Workspace root manifest
├── Cargo.lock                        # Dependency lockfile
├── deny.toml                         # cargo-deny configuration
├── kilo.json                         # AI agent configuration
├── repository-health.md              # Repository health report
├── rust-toolchain.toml               # Rust toolchain pinning
├── .gitignore                        # Git ignore rules
├── .dockerignore                     # Docker ignore rules
├── .markdownlint.json                # Markdown linting config
│
├── crates/                           # 13 core Rust crates
├── sdk/                              # 9 language SDK bindings
├── scripts/                          # Build automation, CLI tools, validation scripts
├── docs/                             # Documentation (SSOT v2.0)
├── deployment/                       # Docker, K8s, Helm, Terraform
├── tests/                            # Cross-language test infrastructure
├── examples/                         # Working code examples
├── tools/                            # Documentation tooling (5 tools)
├── assets/                           # Logo & static assets
├── benchmark-results/                # Benchmark baselines & reports
├── skills/                           # 16 AI engineering skills
├── .agents/                          # AI agent skill definitions (mirrors skills/)
├── .cargo/                           # Cargo configuration
├── .github/                          # CI/CD workflows & templates
├── .kilo/                            # Kilo AI configuration
├── .kcm_snapshots/                   # KCM snapshot data
└── target/                           # Build output (gitignored)
```

---

## Core Crates

### Crate Map Summary

| # | Crate | Responsibility | Dependencies |
|---|-------|---------------|--------------|
| 1 | `kcm-core` | Foundation types, DenseVec, Bitmap, Dictionary | parking_lot, ahash |
| 2 | `kcm-storage` | Columnar storage, Codecs, WAL, FileFormat, Index, Recovery | kcm-core, zstd, lz4, blake3, thiserror |
| 3 | `kcm-compute` | Relational algebra operators, SIMD AVX2 acceleration | kcm-core, kcm-storage |
| 4 | `kcm-reasoning` | Rule definitions, forward-chaining inference engine | kcm-core, kcm-storage |
| 5 | `kcm-optimizer` | Cost model, query planner, statistics, plan rewriting | kcm-core, kcm-storage |
| 6 | `kcm-runtime` | KnowledgeDatabase, Transactions, Metrics, Health, Executor | kcm-core, kcm-storage, kcm-optimizer, rayon, tokio |
| 7 | `kcm-interface` | C FFI (18 functions), Python bindings, REST handlers, KQL parser | kcm-core, kcm-storage, kcm-runtime, kcm-security, pyo3 |
| 8 | `kcm-distributed` | Sharding strategies (Hash/Range/ConsistentHash), 2PC coordinator | kcm-core, kcm-security, rayon |
| 9 | `kcm-ml` | Learned index, confidence learner, rule discovery | kcm-core, kcm-reasoning |
| 10 | `kcm-security` | RBAC (5 permissions), AES-256-GCM, audit log (hash-chained) | kcm-core, blake3, aes-gcm, getrandom |
| 11 | `kcm-compliance` | GDPR consent management, data classification (4 tiers) | kcm-core |
| 12 | `kcm-testing` | Load, stress, security, recovery, regression tests | kcm-core, kcm-storage, kcm-runtime, kcm-reasoning, kcm-security |
| 13 | `kcm-server` | HTTP (actix-web) + gRPC (tonic) server binaries | kcm-core, kcm-runtime, kcm-interface, kcm-security, actix-web, tonic |

### 1. `kcm-core` — Foundation

**Path:** `crates/kcm-core/`

The foundational crate containing all core types and data structures. Has zero internal KCM dependencies — only `parking_lot` and `ahash` as external deps.

```
crates/kcm-core/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs              # Module root — re-exports all public modules
│   ├── types.rs            # Core type definitions
│   ├── bitmap.rs           # Bitmap data structure (u64 word-based)
│   ├── dictionary.rs       # String dictionary encoding
│   └── vec.rs              # Custom aligned dense vector (64-byte alignment)
└── tests/
    ├── comprehensive_unit_tests.rs
    ├── property_tests.rs
    └── test_core.rs
```

**Public API:**

| Module | Key Types | Description |
|--------|-----------|-------------|
| `types` | `Fact`, `RowID`, `SubjectID`, `PredicateID`, `ObjectID`, `Confidence`, `ColumnID`, `KcmError`, `ErrorCode` | Foundational type definitions for the knowledge model |
| `bitmap` | `Bitmap` | Bit-vector with set/clear/get, bulk ops, AND/OR/NOT, iteration |
| `dictionary` | `Dictionary`, `SharedDictionary`, `DictID` | String-to-integer encoding with thread-safe wrapper |
| `vec` | `DenseVec<T>` | Cache-friendly aligned vector with raw allocation |

**Key Design Decisions:**
- `Fact` struct is 34 bytes uncompressed with 10 fields
- `Confidence` validates range [0.0, 1.0] with `multiply()` and `combine_or()` methods
- `DenseVec` uses 64-byte minimum alignment for SIMD-friendly access
- `KcmError` is the single error hierarchy root (7 variants)

### 2. `kcm-storage` — Data Layer

**Path:** `crates/kcm-storage/`

The columnar storage engine handling persistence, encoding, compression, indexing, WAL, and recovery.

```
crates/kcm-storage/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs              # Module root & re-exports
│   ├── column.rs           # Column trait + 10 typed column structs
│   ├── compress.rs         # Compressor trait + 4 codec implementations
│   ├── dict_codec.rs       # Dictionary encoding/decoding
│   ├── dict_cache.rs       # LRU cache for dictionary lookups
│   ├── file_format.rs      # DatabaseFile loader/saver/verifier
│   ├── wal.rs              # Write-Ahead Log implementation
│   ├── wal_state.rs        # WAL state management
│   ├── index.rs            # BitmapIndex, BloomFilter, CompositeIndex, ZoneMap
│   ├── backup.rs           # BackupManager / RestoreManager
│   ├── recovery.rs         # RecoveryManager for crash recovery
│   ├── robin_hood.rs       # Robin Hood hash map
│   └── errors.rs           # StorageError type
├── tests/
│   ├── integration_tests.rs
│   ├── property_tests.rs
│   ├── test_codec_property.rs
│   ├── test_corruption.rs
│   ├── test_persistence.rs
│   ├── test_property_roundtrips.rs
│   ├── test_property.rs
│   ├── test_storage.rs
│   └── test_wal_property.rs
└── benches/
    └── dictionary.rs
```

**Column Storage Model:**

| Column | Type | Encoding | Compression |
|--------|------|----------|-------------|
| Subject | u32 | Dictionary | Zstd |
| Predicate | u8 | Dictionary | RLE |
| Object | u32 | Dictionary | Zstd |
| Confidence | f64 | Gorilla | Zstd |
| Evidence | u8 | Dictionary | RLE |
| Timestamp | i64 | Delta | Zstd |
| Context | u8 | Dictionary | RLE |
| Version | i32 | Delta | LZ4 |
| Priority | i8 | Identity | RLE |
| Owner | u16 | Dictionary | Zstd |

**Compression Codecs:**

| Codec | Implementation | Use Case |
|-------|---------------|----------|
| Zstd | `ZstdCompressor` | High-ratio general compression |
| LZ4 | `Lz4Compressor` | Speed-optimized compression |
| RLE | `RleCompressor` | Run-length for repeated values |
| None | `NoopCompressor` | Already-compressed data |

**Index Structures:**

| Index | Purpose |
|-------|---------|
| `BitmapIndex` | Bitmap-based row set tracking |
| `BloomFilter` | Probabilistic membership test |
| `CompositeIndex` | Multi-column composite index |
| `ZoneMap` | Min/max per-zone for range pruning |

### 3. `kcm-compute` — Query Execution

**Path:** `crates/kcm-compute/`

Relational algebra operators with SIMD AVX2 acceleration.

```
crates/kcm-compute/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   ├── algebra.rs          # Relational algebra operators
│   └── simd.rs             # SIMD AVX2-accelerated operations
└── tests/
    └── test_compute.rs
```

**Operators:** Scan → Filter → Project → Join → Aggregate

### 4. `kcm-reasoning` — Inference Engine

**Path:** `crates/kcm-reasoning/`

Forward-chaining rule-based inference engine.

```
crates/kcm-reasoning/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   ├── rule.rs             # Rule definitions (RulePattern)
│   └── inference.rs        # Forward-chaining inference engine
└── tests/
    ├── test_reasoning.rs
    ├── test_provenance.rs
    └── test_fuzz_reasoning.rs
```

### 5. `kcm-optimizer` — Query Optimizer

**Path:** `crates/kcm-optimizer/`

Cost-based query optimization with statistics and plan rewriting.

```
crates/kcm-optimizer/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   ├── planner.rs          # Query planner (PlanNode, QueryPlan)
│   ├── cost_model.rs       # Cost estimation
│   ├── statistics.rs       # Column statistics & selectivity
│   ├── rewriting.rs        # Optimizer pipeline & rules
│   └── adaptive.rs         # Adaptive execution
└── tests/
    └── test_optimizer_advanced.rs
```

**Optimizer Pipeline:** Filter pushdown → Column pruning → Join reordering → Index selection

### 6. `kcm-runtime` — Orchestration Layer

**Path:** `crates/kcm-runtime/`

The main database facade orchestrating storage, compute, optimizer, and transactions.

```
crates/kcm-runtime/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   ├── database.rs         # KnowledgeDatabase (main entry point)
│   ├── transaction.rs      # Transaction management
│   ├── executor.rs         # Rayon-based parallel execution
│   ├── async_executor.rs   # Tokio-based async execution
│   ├── metrics.rs          # Lock-free atomic metrics (14 counters)
│   └── health.rs           # Health check (Healthy/Degraded/Unhealthy)
├── tests/
│   ├── test_full.rs
│   ├── test_integration.rs
│   ├── test_monitoring.rs
│   └── test_transaction_rollback.rs
└── benches/
    ├── advanced.rs
    ├── backup_recovery.rs
    ├── compression.rs
    └── micro.rs
```

**Key API:**
- `KnowledgeDatabase::new()` → `insert()`, `update()`, `delete()`, `query()`, `get_fact()`, `begin_transaction()`
- `Transaction::commit()` / `Transaction::rollback()`
- `Metrics::snapshot()` — 14 atomic counters
- `HealthCheck` — status based on error_rate, latency, cache_hit_ratio

### 7. `kcm-interface` — External Interfaces

**Path:** `crates/kcm-interface/`

Multi-protocol interface layer: C FFI, Python bindings, REST API, gRPC, KQL parser.

```
crates/kcm-interface/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   ├── rest_api.rs         # REST API handlers
│   ├── openapi.rs          # OpenAPI 3.1.0 spec generator
│   ├── kql_parser.rs       # KQL lexer + parser
│   ├── python.rs           # PyO3 Python bindings
│   ├── middleware/
│   │   ├── mod.rs
│   │   ├── auth.rs         # Authentication
│   │   ├── cors.rs         # CORS handling
│   │   ├── logging.rs      # Request logging
│   │   ├── rate_limit.rs   # Rate limiter
│   │   └── request_id.rs   # Request ID generation
│   └── examples/
│       ├── mod.rs
│       ├── ecommerce.rs    # E-commerce use case
│       └── medical.rs      # Medical domain use case
├── proto/
│   └── kcm.proto           # gRPC service definition
└── tests/
    ├── test_interface.rs
    ├── test_fuzz_parsing.rs
    └── test_kql_edge_cases.rs
```

**C FFI Functions (18):**

| Function | Purpose |
|----------|---------|
| `KCM_DatabaseNew` | Create database |
| `KCM_DatabaseFree` | Destroy database |
| `KCM_DatabaseInsert` | Insert fact |
| `KCM_DatabaseUpdate` | Update fact |
| `KCM_DatabaseDelete` | Delete fact |
| `KCM_DatabaseFactCount` | Total fact count |
| `KCM_DatabaseActiveCount` | Active fact count |
| `KCM_DatabaseQuery` | Execute query |
| `KCM_QueryNext` | Iterate query results |
| `KCM_QueryFree` | Free query iterator |
| `KCM_DatabaseBeginTransaction` | Begin transaction |
| `KCM_TransactionCommit` | Commit transaction |
| `KCM_TransactionRollback` | Rollback transaction |
| `KCM_TransactionFree` | Free transaction |
| `KCM_DatabaseSave` | Save to file |
| `KCM_DatabaseLoad` | Load from file |
| `KCM_DatabaseVerify` | Verify integrity |
| `KCM_ErrorMessage` | Error code to string |

**REST Endpoints:**

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/health` | Health check |
| GET | `/metrics` | Prometheus metrics |
| GET | `/openapi.json` | OpenAPI spec |
| POST | `/api/v1/facts` | Insert fact |
| GET | `/api/v1/facts` | Query facts |
| POST | `/api/v1/facts/batch` | Batch insert |
| GET | `/api/v1/facts/{id}` | Get fact by ID |
| PUT | `/api/v1/facts/{id}` | Update fact |
| DELETE | `/api/v1/facts/{id}` | Delete fact |
| GET | `/api/v1/stats` | Statistics |

**gRPC Service (`KnowledgeService`):**

| RPC | Request | Response |
|-----|---------|----------|
| `InsertFact` | `Fact` | `InsertResponse` |
| `QueryFacts` | `QueryRequest` | `QueryResponse` |
| `GetFact` | `GetFactRequest` | `Fact` |
| `GetStats` | `Empty` | `StatsResponse` |

### 8. `kcm-distributed` — Distributed Architecture

**Path:** `crates/kcm-distributed/`

Sharding, replication, and distributed transaction coordination.

```
crates/kcm-distributed/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   ├── sharding.rs         # Hash, Range, ConsistentHash strategies
│   ├── coordinator.rs      # 2PC transaction coordinator
│   ├── replication.rs      # Data replication across nodes
│   └── transport.rs        # Network transport abstraction
└── tests/
    ├── test_distributed.rs
    └── test_transport.rs
```

### 9. `kcm-ml` — Machine Learning

**Path:** `crates/kcm-ml/`

ML integration for learned indexes, confidence adjustment, and rule discovery.

```
crates/kcm-ml/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   ├── learned_index.rs    # Regression-based learned index
│   ├── confidence_learner.rs # ML confidence score adjustment
│   └── rule_discovery.rs   # Automated rule discovery
└── tests/
    └── test_ml.rs
```

### 10. `kcm-security` — Security & Access Control

**Path:** `crates/kcm-security/`

RBAC, AES-256-GCM encryption, and tamper-evident audit logging.

```
crates/kcm-security/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   ├── rbac.rs             # 5-level permission model
│   ├── encryption.rs       # AES-256-GCM authenticated encryption
│   ├── audit.rs            # Hash-chained audit log (100K FIFO)
│   └── secrets.rs          # Secret/credential management
└── tests/
    └── test_security.rs
```

**Permission Levels:** Read, Write, Delete, Execute, Admin

### 11. `kcm-compliance` — Regulatory Compliance

**Path:** `crates/kcm-compliance/`

GDPR compliance and data classification.

```
crates/kcm-compliance/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   ├── gdpr.rs             # Consent management
│   └── data_classification.rs # 4-tier classification
└── tests/
    └── test_compliance.rs
```

**Data Classification Tiers:** Public, Internal, Confidential, Restricted

### 12. `kcm-testing` — Test Infrastructure

**Path:** `crates/kcm-testing/`

Comprehensive testing framework: load, stress, security, recovery, regression detection.

```
crates/kcm-testing/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   ├── bench_fixtures.rs   # Shared benchmark fixtures
│   ├── chaos.rs            # Chaos testing (random failures)
│   ├── load_tests.rs       # Load testing infrastructure
│   ├── stress_tests.rs     # Stress testing (high-concurrency)
│   ├── metrics_dashboard.rs # Test observability dashboard
│   ├── regression_detector.rs # Performance regression detection
│   └── security_tests.rs   # Security test utilities (cfg(test))
└── tests/
    ├── test_additional.rs
    ├── test_concurrent_access.rs
    ├── test_crash_recovery.rs
    ├── test_distributed.rs
    ├── test_fuzz_kql.rs
    ├── test_gdpr.rs
    ├── test_integration_cli.rs
    ├── test_recovery.rs
    ├── test_soak.rs
    ├── test_stress_concurrent.rs
    ├── test_stress_scale.rs
    └── test_wal_recovery.rs
```

### 13. `kcm-server` — Server Binaries

**Path:** `crates/kcm-server/`

Production server binaries for HTTP and gRPC.

```
crates/kcm-server/
├── build.rs                # Protobuf compilation (tonic-build)
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs             # kcm-server binary (actix-web HTTP)
│   ├── grpc_main.rs        # kcm-grpc binary (tonic gRPC)
│   └── grpc_server.rs      # gRPC service implementation
└── tests/
    ├── test_endpoints.rs
    └── test_server.rs
```

**Binaries:**

| Binary | Protocol | Default Port | Env Var |
|--------|----------|-------------|---------|
| `kcm-server` | HTTP (actix-web) | 8080 | `KCM_BIND_ADDR` |
| `kcm-grpc` | gRPC (tonic) | 50051 | `KCM_GRPC_ADDR` |

---

## SDK Language Bindings

**Path:** `sdk/`

9 language SDKs with a standardized 16-operation API surface.

| Language | Directory | Package | Build System | Status |
|----------|-----------|---------|-------------|--------|
| Rust | `sdk/rust/` | `kcm-sdk` (crate) | Cargo | Stable |
| C | `sdk/c/` | `libkcm` (FFI header) | Makefile | Stable |
| C++ | `sdk/cpp/` | `libkcm` (header-only) | CMake | Stable |
| Python | `sdk/python/` | `kcm` (PyPI) | maturin + pyproject.toml | Beta |
| JavaScript | `sdk/javascript/` | `@kcm/js` (npm) | npm | Beta |
| TypeScript | `sdk/typescript/` | `@kcm/ts` (npm) | npm + tsc | Beta |
| Go | `sdk/go/` | `github.com/kcm/go-sdk` | go.mod | Beta |
| Java | `sdk/java/` | `io.kcm:sdk` (Maven) | Maven (pom.xml) | Beta |
| .NET | `sdk/dotnet/` | `Kcm.Sdk` (NuGet) | dotnet | Beta |

### Standardized API (16 Operations)

Every SDK implements:

1. `Database(path?)` — Open/create database
2. `insert(fact)` — Insert a knowledge fact
3. `query(kql)` — Execute KQL query
4. `query_all()` — Retrieve all active facts
5. `delete(row_id)` — Delete fact by ID
6. `update(row_id, fact)` — Update existing fact
7. `get_fact(row_id)` — Retrieve single fact
8. `fact_count()` — Total fact count
9. `active_fact_count()` — Active fact count
10. `begin_transaction()` — Start transaction
11. `commit(txn)` — Commit transaction
12. `rollback(txn)` — Rollback transaction
13. `save(path)` — Save database to file
14. `load(path)` — Load database from file
15. `verify(path)` — Verify database integrity
16. `close()` — Close database

### SDK Documentation

Each SDK has a dedicated specification document in `docs/sdk/`:

| File | Language |
|------|----------|
| `docs/sdk/rust.md` | Rust SDK spec |
| `docs/sdk/c.md` | C SDK spec |
| `docs/sdk/cpp.md` | C++ SDK spec |
| `docs/sdk/python.md` | Python SDK spec |
| `docs/sdk/javascript.md` | JavaScript SDK spec |
| `docs/sdk/typescript.md` | TypeScript SDK spec |
| `docs/sdk/go.md` | Go SDK spec |
| `docs/sdk/java.md` | Java SDK spec |
| `docs/sdk/dotnet.md` | .NET SDK spec |
| `docs/sdk/compatibility.md` | Cross-SDK compatibility matrix |
| `docs/sdk/spesifikasi.md` | SDK spesifikasi document |

---

## CLI Tools

**Path:** `scripts/kcm-cli/`

16 Rust CLI binary crates for database management and operations.

| Tool | Purpose | Key Commands |
|------|---------|--------------|
| `kcm-cli` | Main CLI entry point | query, insert, serve, version |
| `kcm-backup` | Backup management | create, list, verify |
| `kcm-restore` | Restore from backup | from, list, verify |
| `kcm-migrate` | Schema migration | up, down, status |
| `kcm-bench` | Benchmarking | run, compare, report |
| `kcm-inspect` | Database inspection | schema, columns, data, stats |
| `kcm-doctor` | Health checks | check, fix, report |
| `kcm-profile` | Performance profiling | start, insert, report |
| `kcm-snapshot` | Database snapshots | create, list, restore |
| `kcm-import` | Data import | csv, json, parquet |
| `kcm-export` | Data export | csv, json, parquet |
| `kcm-compact` | Compaction | run, status, analyze |
| `kcm-diagnose` | Full diagnostics | full, perf, report |
| `kcm-cluster` | Cluster management | status, rebalance |
| `kcm-schema` | Schema management | show, generate, validate |
| `kcm-docs` | Documentation generation | generate, serve |
| `kcm-perf` | Performance analysis | analyze, baseline |

### Build & Validation Scripts

**Path:** `scripts/`

| Script | Purpose |
|--------|---------|
| `validate-ssot.sh` | SSOT validation — 24 automated checks |
| `validate-sdk-api.sh` | SDK API compliance validation (9 SDKs) |
| `bench-regression.py` | Benchmark regression detector (5% warn, 10% fail) |
| `bench-compare.sh` | Benchmark comparison (shell wrapper) |
| `bench-compare.py` | Benchmark comparison (Python) |
| `bench-report.sh` | Benchmark report generator |
| `kcm-cli/build.sh` | CLI tools build script |
| `kcm-cli/test.sh` | CLI tools test script |

---

## Documentation

**Path:** `docs/`

```
docs/
├── INDEX.md                        # Documentation index
├── README.md                       # Documentation root readme
├── search-index.json               # Search index for documentation
│
├── specs/                          # PRDs and technical specifications (19 files)
│   ├── PRD.md                      # P4 — Core types, storage, compute, reasoning
│   ├── PRD2.md                     # P3 — Storage, runtime, interfaces
│   ├── PRD3.md                     # P2 — Distributed, ML, security, compliance
│   ├── PRD-TESTING-AND-BENCHMARK.md # P1 — Testing strategy, benchmarks
│   ├── KCM_API_SPEC.md             # API specification
│   ├── KCM_COLUMNAR_FORMAT_SPEC.md # Columnar format specification
│   ├── KCM_COMPRESSION_SPEC.md     # Compression codec specification
│   ├── KCM_DATA_MODEL_SPEC.md      # Data model specification
│   ├── KCM_DEPLOYMENT_SPEC.md      # Deployment specification
│   ├── KCM_DOCUMENT_AUDIT_REPORT.md # Document audit report
│   ├── KCM_GLOSSARY.md             # Project glossary
│   ├── KCM_INDEXING_SPEC.md        # Indexing specification
│   ├── KCM_PERFORMANCE_SPEC.md     # Performance targets
│   ├── KCM_QUERY_EXECUTION_SPEC.md # Query execution specification
│   ├── KCM_RUNTIME_SPEC.md         # Runtime specification
│   ├── KCM_SECURITY_TRUST_SPEC.md  # Security trust specification
│   ├── KCM_SPECIFICATION.md        # Master specification
│   ├── KCM_TESTING_SPEC.md         # Testing specification
│   └── KCM_VERSIONING_SPEC.md      # Versioning specification
│
├── adr/                            # Architecture Decision Records (10 files)
│   └── ADR-001.md through ADR-010.md
│
├── sdk/                            # SDK documentation (11 files)
│   ├── c.md, cpp.md, dotnet.md, go.md, java.md
│   ├── javascript.md, python.md, rust.md, typescript.md
│   ├── compatibility.md
│   └── spesifikasi.md
│
├── handbook/
│   └── handbook.md                 # Engineering handbook
│
├── runbook/
│   ├── OPERATIONAL_RUNBOOK.md      # Operational procedures
│   └── DISASTER_RECOVERY.md        # Disaster recovery procedures
│
├── governance/
│   └── documentation-governance.md # Documentation governance rules
│
├── templates/                      # Document templates (8 files)
│   ├── ADR-template.md
│   ├── benchmark-report-template.md
│   ├── CODE_OF_CONDUCT-template.md
│   ├── CONTRIBUTING-template.md
│   ├── README-template.md
│   ├── runbook-template.md
│   ├── SECURITY-template.md
│   └── spesifikasi-template.md
│
├── metrics/                        # Documentation metrics
│   ├── README.md
│   ├── coverage.md
│   ├── coverage.html
│   └── coverage.json
│
├── validation/                     # Documentation validation
│   └── README.md
│
├── automation/                     # Documentation automation
│   └── README.md
│
├── index/
│   └── README.md                   # Index documentation
│
├── agents/
│   └── spesifikasi.md              # Agent spesifikasi
│
├── assets/
│   └── spesifikasi.md              # Asset spesifikasi
│
├── benchmark-results/
│   └── spesifikasi.md              # Benchmark results spesifikasi
│
├── cargo/
│   └── spesifikasi.md              # Cargo spesifikasi
│
├── deployment/
│   └── spesifikasi.md              # Deployment spesifikasi
│
├── docs/
│   └── spesifikasi.md              # Docs spesifikasi
│
├── examples/
│   └── spesifikasi.md              # Examples spesifikasi
│
├── github/
│   └── spesifikasi.md              # GitHub spesifikasi
│
├── scripts/
│   └── spesifikasi.md              # Scripts spesifikasi
│
├── skills/
│   └── spesifikasi.md              # Skills spesifikasi
│
├── tests/
│   └── spesifikasi.md              # Tests spesifikasi
│
├── kcm-core/
│   └── spesifikasi.md              # kcm-core spesifikasi
│
├── kcm-storage/
│   └── spesifikasi.md              # kcm-storage spesifikasi
│
├── kcm-compute/
│   └── spesifikasi.md              # kcm-compute spesifikasi
│
├── kcm-reasoning/
│   └── spesifikasi.md              # kcm-reasoning spesifikasi
│
├── kcm-optimizer/
│   └── spesifikasi.md              # kcm-optimizer spesifikasi
│
├── kcm-runtime/
│   └── spesifikasi.md              # kcm-runtime spesifikasi
│
├── kcm-interface/
│   └── spesifikasi.md              # kcm-interface spesifikasi
│
├── kcm-distributed/
│   └── spesifikasi.md              # kcm-distributed spesifikasi
│
├── kcm-ml/
│   └── spesifikasi.md              # kcm-ml spesifikasi
│
├── kcm-security/
│   └── spesifikasi.md              # kcm-security spesifikasi
│
├── kcm-compliance/
│   └── spesifikasi.md              # kcm-compliance spesifikasi
│
├── kcm-testing/
│   └── spesifikasi.md              # kcm-testing spesifikasi
│
└── kcm-server/
    └── spesifikasi.md              # kcm-server spesifikasi
```

### Document Hierarchy (SSOT Authority)

| Priority | Document | Authority |
|----------|----------|-----------|
| P1 | `docs/specs/PRD-TESTING-AND-BENCHMARK.md` | Performance targets, validation methodology, testing strategy |
| P2 | `docs/specs/PRD3.md` | Distributed architecture, ML integration, security, compliance |
| P3 | `docs/specs/PRD2.md` | Persistence layer, optimizer, monitoring, interfaces |
| P4 | `docs/specs/PRD.md` | Core types, storage engine, compute engine, reasoning engine |
| P5 | `AGENTS.md` | Engineering constitution |

---

## Deployment

**Path:** `deployment/`

Complete deployment stack for production environments.

```
deployment/
├── Dockerfile                      # Multi-stage build (rust:1.88 → debian:bookworm-slim)
├── docker-compose.yml              # Single-node local development
├── docker-compose.monitoring.yml   # Full stack: KCM + Prometheus + Grafana + Alertmanager
├── README.md                       # Deployment guide
│
├── k8s/
│   └── deployment.yaml             # StatefulSet + Service + NetworkPolicy
│
├── helm/kcm/
│   ├── Chart.yaml                  # Helm chart v0.1.0
│   ├── values.yaml                 # Default values
│   ├── .helmignore
│   └── templates/
│       ├── _helpers.tpl
│       ├── deployment.yaml         # StatefulSet template
│       ├── service.yaml            # LoadBalancer service
│       ├── serviceaccount.yaml
│       ├── ingress.yaml            # Ingress (disabled by default)
│       ├── hpa.yaml                # Horizontal Pod Autoscaler
│       ├── networkpolicy.yaml
│       └── tests/test-connection.yaml
│
├── terraform/
│   ├── modules/kcm/                # Shared Terraform module
│   │   ├── main.tf
│   │   ├── variables.tf
│   │   └── outputs.tf
│   ├── aws/main.tf                 # AWS EKS
│   ├── azure/main.tf               # Azure AKS
│   └── gcp/main.tf                 # GCP GKE
│
├── grafana/
│   ├── kcm-dashboard.json          # KCM metrics dashboard
│   └── kcm-compliance-dashboard.json # Compliance dashboard
│
└── prometheus/
    ├── prometheus.yml              # Scrape config (kcm:8080, kcm:50051)
    └── kcm_alerts.yml              # 4 alert rules
```

### Docker Configuration

- **Builder stage:** `rust:1.88`, builds `kcm-server` binary
- **Runtime stage:** `debian:bookworm-slim` with ca-certificates
- **Health check:** `wget -q --spider http://localhost:8080/health`
- **Ports:** 8080 (HTTP), 50051 (gRPC)

### Kubernetes Configuration

- **Workload type:** StatefulSet (persistent storage)
- **Security:** runAsNonRoot, runAsUser: 1000, seccomp RuntimeDefault
- **Resources:** 512Mi–2Gi memory, 500m–2000m CPU
- **Storage:** 100Gi PVC (ReadWriteOnce)
- **NetworkPolicy:** Ingress on 8080/50051, Egress on 443/53

### Prometheus Alert Rules

| Alert | Condition | Severity |
|-------|-----------|----------|
| HighErrorRate | error_rate > 10/s | critical |
| HighLatency | p99_latency > 100ms | warning |
| LowCacheHitRatio | cache_hit_ratio < 50% | warning |
| HighMemoryUsage | memory_bytes > 1GB | warning |

---

## Testing Infrastructure

### Test Categories

| Category | Location | Purpose | Count |
|----------|----------|---------|-------|
| Unit tests | `crates/*/tests/` | Single function correctness | 89+ |
| Integration tests | `crates/*/tests/` | Cross-component correctness | 470+ |
| Property tests | `crates/*/tests/*_property.rs` | Invariant verification | 8+ (1000+ cases each) |
| Security tests | `crates/kcm-testing/src/security_tests.rs` | Attack surface validation | 29+ |
| Stress tests | `crates/kcm-testing/tests/test_stress_*.rs` | Performance under load | varies |
| Recovery tests | `crates/kcm-testing/tests/test_recovery.rs` | Crash recovery | varies |
| Concurrent tests | `crates/kcm-testing/tests/test_concurrent_access.rs` | Thread safety | varies |

### Cross-Language Test Infrastructure

**Path:** `tests/sdk/`

| File | Purpose |
|------|---------|
| `consistency_matrix.json` | Cross-language test matrix (10 test cases + 8 REST API tests) |
| `cross_language_test.py` | Cross-SDK consistency test suite |
| `validate_sdk_api.py` | SDK API compliance validator (18 FFI, 10 fields, 8 errors) |
| `mock_server.py` | Mock REST server (8 endpoints, in-memory storage) |
| `run_all_tests.sh` | Test runner script |

### Cross-Language Test Matrix

| Test Case | Description |
|-----------|-------------|
| TC-001 | Insert fact with all fields |
| TC-002 | Query all facts |
| TC-003 | Delete fact and verify removal |
| TC-004 | Update existing fact |
| TC-005 | Transaction commit |
| TC-006 | Transaction rollback |
| TC-007 | Save/load database |
| TC-008 | Error not found |
| TC-009 | Multiple inserts |
| TC-010 | Error messages |

---

## CI/CD Pipelines

**Path:** `.github/workflows/`

| Workflow | Trigger | Purpose |
|----------|---------|---------|
| `ci.yml` | push/PR to main | Core CI: format, clippy, build, tests, SSOT |
| `ci-full.yml` | push to main/develop, PR to main | Extended: stress, benchmarks, Docker, CLI |
| `benchmark.yml` | Weekly (Mon 6AM) + manual | Dedicated benchmark runs (90-day retention) |
| `sdk-ci.yml` | Changes to sdk/ or kcm-interface/ | All 9 SDKs: lint, type-check, test |
| `sdk-publish.yml` | Version tags (v*) | Multi-registry publishing |
| `docs.yml` | Changes to docs/ | Documentation validation & deployment |

### `ci.yml` Jobs (13 jobs → quality gate)

1. `format` — `cargo fmt --all -- --check`
2. `clippy` — `cargo clippy --workspace --all-targets -- -D warnings`
3. `build` — `cargo build --workspace`
4. `build-release` — `cargo build --release --workspace`
5. `unit-tests` — `cargo test --lib --all`
6. `integration-tests` — `cargo test --test '*' --all`
7. `property-tests` — `cargo test property_tests --all`
8. `security-tests` — `cargo test security_tests --all`
9. `benchmarks` — `cargo bench --workspace --no-run` (compile only)
10. `ssot-validation` — `bash scripts/validate-ssot.sh`
11. `cargo-audit` — Dependency vulnerability scan
12. `cargo-deny` — License and advisory check
13. `quality-gate` — Final gate requiring all above

### SDK Publishing Targets

| Registry | Language | Package |
|----------|----------|---------|
| crates.io | Rust | `kcm-sdk` |
| PyPI | Python | `kcm` |
| npm | JavaScript | `@kcm/js` |
| npm | TypeScript | `@kcm/ts` |
| Maven Central | Java | `io.kcm:sdk` |
| NuGet | .NET | `Kcm.Sdk` |

---

## Engineering Skills

**Path:** `skills/`

16 specialized AI engineering skills with defined authority boundaries.

| Priority | Skill | Authority |
|----------|-------|-----------|
| P1 | `kcm-engineering-orchestrator` | Master coordinator — overrides all |
| P2 | `kcm-task-planner` | Can block implementation without plan |
| P3 | `kcm-change-impact-analysis` | Can block changes with unassessed impact |
| P4 | `kcm-specification-lock` | Can veto format/API/FFI changes |
| P5 | `kcm-architecture-guardian` | Can block architecture violations |
| P6 | `kcm-database-engine-specialist` | Can block storage/query changes |
| P7 | `kcm-security-engineer` | Can block security/compliance violations |
| P8 | `kcm-performance-engineer` | Can block performance regressions |
| P9 | `kcm-testing-verification` | Can block changes without tests |
| P10 | `kcm-code-quality-guardian` | Can reject code quality issues |
| P11 | `kcm-documentation-guardian` | Can block undocumented changes |
| P12 | `kcm-release-readiness` | Can block releases |
| P13 | `kcm-code-review-auditor` | Provides review feedback |
| P14 | `kcm-debugging-root-cause` | Provides diagnostic analysis |
| P15 | `kcm-engineering-decision-record` | Documents decisions |
| P16 | `kcm-repository-intelligence` | Provides codebase understanding |

Each skill is a directory under `skills/` containing a `SKILL.md` file with detailed instructions and workflows.

---

## Documentation Tools

**Path:** `tools/`

5 documentation tooling scripts for coverage, drift detection, generation, link checking, and validation.

```
tools/
├── doc-coverage/
│   ├── calculate-coverage.sh       # Documentation coverage calculator
│   └── README.md
│
├── doc-drift/
│   ├── detect-drift.sh             # Documentation drift detector
│   ├── ssot-check.sh               # SSOT alignment checker
│   └── README.md
│
├── doc-generator/
│   ├── generate-index.sh           # Documentation index generator
│   ├── generate-search-index.sh    # Search index generator
│   └── README.md
│
├── doc-link-checker/
│   ├── check-links.sh              # Broken link checker
│   └── README.md
│
└── doc-validator/
    └── (empty)
```

---

## Benchmark Results

**Path:** `benchmark-results/`

```
benchmark-results/
├── baseline.json                   # Current performance baseline
├── README.md                       # Benchmark documentation
│
├── metadata/
│   ├── benchmark-version.json      # Benchmark tooling version
│   ├── environment.json            # Test environment details
│   └── git.json                    # Git commit reference
│
├── raw/                            # Raw benchmark output (empty)
│
└── reports/
    ├── KCM_BENCHMARK_REPORT.json   # Machine-readable report
    ├── KCM_BENCHMARK_REPORT.md     # Human-readable report
    └── KCM_PERFORMANCE_MATRIX.csv  # Performance matrix (CSV)
```

---

## Examples

**Path:** `examples/`

| Language | Directory | Examples |
|----------|-----------|----------|
| Rust | `examples/rust/` | `basic_usage.rs`, `reasoning.rs`, `transactions.rs` |
| Python | `examples/python/` | Planned |
| JavaScript | `examples/javascript/` | Planned |
| Go | `examples/go/` | Planned |
| Java | `examples/java/` | Planned |

### Rust Examples

- **`basic_usage.rs`**: Create database → insert facts → query → dictionary operations → transactions → delete → count
- **`reasoning.rs`**: Create schema → insert base facts → define transitive rule → run inference → print derived facts
- **`transactions.rs`**: Transaction commit (insert 2 facts) → rollback (insert then rollback) → direct insert

---

## Build & Tool Configuration

### `Cargo.toml` (Workspace Root)

- **Resolver:** v2
- **Edition:** 2021
- **Workspace members:** 13 crates + 16 CLI tools + SDK rust binding
- **Workspace dependencies:** 20+ shared dependencies
- **Lints:** `unwrap_used = "warn"`, `panic = "warn"`, `todo = "warn"`, `unimplemented = "warn"`, `dbg_macro = "warn"`
- **Release profile:** `opt-level = 3`, LTO enabled, single codegen unit, stripped

### `rust-toolchain.toml`

```toml
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
targets = ["x86_64-unknown-linux-gnu", "x86_64-apple-darwin", "aarch64-apple-darwin"]
```

### `deny.toml`

- **Advisories:** vulnerability = deny, yanked = deny
- **Licenses:** MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, Unicode-3.0, Zlib, BSL-1.0
- **Sources:** Only crates.io allowed

### `kilo.json`

```json
{
  "skills": { "paths": ["./skills"] },
  "instructions": ["AGENTS.md"]
}
```

### `.markdownlint.json`

Markdown linting configuration for documentation consistency.

---

## Dependency Flow

```
kcm-core (zero KCM deps)
  │
  ├── kcm-storage (core + zstd + lz4 + blake3 + thiserror)
  │     │
  │     ├── kcm-compute (core + storage)
  │     ├── kcm-reasoning (core + storage)
  │     ├── kcm-optimizer (core + storage)
  │     │
  │     └── kcm-runtime (core + storage + optimizer + rayon + tokio)
  │           │
  │           └── kcm-interface (core + storage + runtime + kcm-security + serde + pyo3)
  │                 │
  │                 └── kcm-server (core + runtime + interface + actix-web + tonic)
  │
  ├── kcm-distributed (core + kcm-security + rayon)
  ├── kcm-ml (core + kcm-reasoning)
  ├── kcm-security (core + blake3 + aes-gcm + getrandom)
  ├── kcm-compliance (core)
  └── kcm-testing (core + storage + runtime + reasoning + security)
```

**Dependency Policy:** Every external dependency must justify its existence. The `quickcheck` crate is marked for removal (redundant with `proptest`).

---

## Quality Gates

| Gate | Command | Blocks Merge |
|------|---------|-------------|
| Format | `cargo fmt --all -- --check` | Yes |
| Clippy | `cargo clippy --workspace -- -D warnings` | Yes |
| Build | `cargo build --workspace` | Yes |
| Unit Tests | `cargo test --lib --all` | Yes |
| Integration Tests | `cargo test --test '*' --all` | Yes |
| Property Tests | `cargo test property_tests --all` | Yes |
| Security Tests | `cargo test security_tests --all` | Yes |
| SSOT Validation | `bash scripts/validate-ssot.sh` | Yes |
| Cargo Audit | `cargo audit` | Yes |
| Cargo Deny | `cargo deny check` | Yes |

### SSOT Validation Checks (24)

1. C FFI function count = 18
2. Metrics counter count = 14
3. Test count ≥ 550
4. REST endpoint count ≥ 8
5. gRPC RPC count = 4
6. No TODO/FIXME in production code
7. Unwrap count ≤ 80
8. Workspace compiles
9–10. Root and community docs exist
11. No phantom document references
12. Deleted directories don't exist
13. `docs/` has 4 subfolders
14. `skills/` has 16 skills
15. `.agents/skills/` exists
16. `crates/` has 13 crates
17–24. Edition, configuration, CODEOWNERS, `.agents/` checks

---

*Document generated for KCM — Knowledge Columnar Model*
*Last updated: 2026-08-06*
