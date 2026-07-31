# KCM — Knowledge Columnar Model

> **Columnar knowledge representation, storage, and reasoning engine for high-performance computing.**

KCM is a Rust-native engine that represents knowledge as **columnar relation spaces** instead of pointer-based graphs. This architecture enables SIMD-accelerated scanning, per-column compression, dictionary encoding, bitmap indexing, and deterministic forward-chaining inference — all within a single, unified storage and compute layer.

```
Knowledge is not an object graph.
Knowledge is a columnar relation space.
```

---

## Overview

Traditional knowledge systems store facts as adjacency lists or property graphs, which degrade to pointer-chasing on full scans and cannot leverage modern CPU features. KCM applies columnar storage principles — proven in analytical databases — to knowledge representation and reasoning.

Every attribute of a knowledge fact (subject, predicate, object, confidence, evidence, timestamp, context, version, priority, owner) is stored in its own **independent linear column**. Columns are SIMD-aligned, dictionary-encoded, and independently compressed. Queries scan columns rather than following pointers, yielding predictable latency and high throughput.

KCM is designed for applications that require:

- Knowledge storage with probabilistic confidence scores
- Query execution with filter pushdown and cost-based optimization
- Forward-chaining inference with rule-based reasoning
- ACID-compliant transactions with crash recovery
- AES-256-GCM encryption at rest with RBAC access control
- GDPR-compliant data subject management

---

## Key Features

### Storage Engine

- **10-column fact store** — Subject, Predicate, Object, Confidence, Evidence, Timestamp, Context, Version, Priority, Owner
- **Cache-aligned DenseVec** — 64-byte SIMD-aligned contiguous memory with zero-copy iteration
- **Tombstone-based soft delete** — Bitmap-tracked deletions with active count tracking
- **Write-Ahead Log** — Buffered WAL with fsync for crash recovery
- **Binary file format** — Blake3 checksummed, versioned columnar layout
- **Backup and restore** — Full and incremental backup with manifest tracking

### Compression and Encoding

| Encoding | Algorithm | Application |
|----------|-----------|-------------|
| Dictionary | String → integer mapping | Subject, Object, Predicate, Evidence, Context, Owner |
| Delta | Consecutive difference encoding | Timestamp, Version |
| Gorilla | XOR-based float encoding | Confidence |
| RLE | Run-length encoding | Predicate, Evidence, Context, Priority |

Physical compression via Zstd (level 3), LZ4, and custom RLE with per-column codec selection.

### Query Engine

- **Volcano-model execution** — Pull-based operator pipeline with `Operator` trait
- **5 algebraic operators** — ScanOp, FilterOp, ProjectOp, JoinOp (hash join), AggregateOp (Count/Sum/Avg/Min/Max)
- **SIMD-accelerated filtering** — AVX2 intrinsics with runtime feature detection and scalar fallback
- **Bitmap indexing** — One bitmap per unique value for O(1) membership testing
- **Zone maps** — Min/max block statistics for range query pruning
- **Bloom filters** — Probabilistic pre-filter with configurable false positive rate

### Query Optimization

- **Cost-based optimizer** — CPU, I/O, and memory cost estimation per operator
- **Filter pushdown** — Push predicates toward data source
- **Join reordering** — Smaller relations joined first
- **Index selection** — Automatic index choice based on predicate type
- **Adaptive execution** — Runtime cardinality feedback with re-optimization

### Reasoning Engine

- **Forward-chaining inference** — Iterative rule application with max iteration limit
- **Rule patterns** — Triple, And, Or, Not composition for complex matching
- **Confidence calculus** — Conjunction (P(A∧B)), disjunction (P(A∨B)), negation, weighted combination
- **Rule discovery** — Automated association pattern mining from fact co-occurrence

### Security and Compliance

- **AES-256-GCM encryption** — AEAD at-rest encryption with BLAKE3 key derivation and CSPRNG
- **RBAC access control** — 5 permission levels (Read, Write, Delete, Execute, Admin) with per-context ACL
- **Audit logging** — Structured event log with configurable capacity (100K events)
- **GDPR compliance** — Data subject management with consent tracking, data export, and right to deletion
- **Data classification** — 4-tier classification (Public, Internal, Confidential, Restricted) with retention policies

### Concurrency

- **Lock-free readers** — Query snapshots via `Arc<RwLock<Schema>>`
- **Write-locked modifications** — Exclusive lock for insert/update/delete
- **Rayon thread pool** — Data-parallel execution bounded by CPU count
- **Tokio async bridge** — Non-blocking async interface via `spawn_blocking`

### Interface Layer

- **C FFI** — 13 functions for cross-language integration (C, C++, Go, etc.)
- **Python bindings** — PyO3-based Python interface (feature-gated)
- **KQL parser** — Knowledge Query Language with SELECT/FROM/WHERE/JOIN/ORDER BY/LIMIT
- **REST API handlers** — Insert, Query, Get, Update, Delete, Health, Stats

---

## Architecture

```
┌──────────────────────────────────────────────────────┐
│                   Interface Layer                     │
│   C FFI  ·  Python (PyO3)  ·  REST  ·  KQL Parser   │
├──────────────────────────────────────────────────────┤
│                   Runtime Layer                       │
│   KnowledgeDatabase  ·  Transactions  ·  Metrics     │
│   Executor (rayon)  ·  AsyncExecutor (tokio)         │
├──────────────────────────────────────────────────────┤
│                   Compute Layer                       │
│   Algebra Operators  ·  SIMD (AVX2)  ·  Optimizer    │
├──────────────────────────────────────────────────────┤
│                   Reasoning Layer                     │
│   Rule Engine  ·  Inference  ·  Confidence Calculus  │
├──────────────────────────────────────────────────────┤
│                   Storage Layer                       │
│   Columns  ·  Codecs  ·  Compression  ·  Indexes     │
│   WAL  ·  File Format  ·  Backup/Recovery            │
├──────────────────────────────────────────────────────┤
│                   Core Layer                          │
│   Types  ·  DenseVec  ·  Bitmap  ·  Dictionary       │
└──────────────────────────────────────────────────────┘
```

### Crate Architecture

| Crate | Responsibility |
|-------|---------------|
| **kcm-core** | Foundation types (RowID, SubjectID, PredicateID, ObjectID, Confidence, Fact), DenseVec, Bitmap, Dictionary |
| **kcm-storage** | Columnar storage, WAL, file format, codecs, compression, indexes, backup/restore |
| **kcm-compute** | Query algebra operators, SIMD-accelerated execution |
| **kcm-reasoning** | Rule engine, forward-chaining inference, confidence calculus |
| **kcm-optimizer** | Cost model, query planner, statistics, adaptive execution |
| **kcm-runtime** | Database lifecycle, transactions, metrics, health checks, logging |
| **kcm-interface** | C FFI, Python bindings, REST API, KQL parser |
| **kcm-distributed** | Sharding (Hash, Range, Consistent Hash), 2PC transaction coordinator |
| **kcm-ml** | Learned index models, confidence learner, rule discovery |
| **kcm-security** | RBAC, AES-256-GCM encryption, audit logging |
| **kcm-compliance** | GDPR data subject management, data classification |
| **kcm-testing** | Load test runner, stress test runner, security tests, regression detector |

---

## Core Technology

### Rust

KCM is implemented entirely in Rust for memory safety without garbage collection, deterministic performance, and fearless concurrency. The codebase enforces:

- No `unsafe` in public API surfaces
- `Send + Sync` bounds on all shared types
- `Result<T, KcmError>` for all fallible operations
- Zero `unwrap()` in production code paths

### Columnar Architecture

Each knowledge attribute is an independent, contiguous column stored as a `DenseVec<T>` with 64-byte cache-line alignment. This layout enables:

- Sequential memory access (prefetch-friendly)
- SIMD-parallel filtering across 32-byte AVX2 lanes
- Independent compression per column (no inter-column dependencies)
- O(1) random access for single-row lookups

### Storage Engine

The storage engine manages the lifecycle of columnar data from append through compression to disk persistence:

1. **Append** — Values written to in-memory DenseVec (uncompressed, O(1))
2. **Flush** — DenseVec bytes compressed per-column using the assigned codec
3. **Write** — Compressed bytes written to binary file format with Blake3 checksum
4. **Recover** — WAL replay with crash-safe recovery (DB + WAL or WAL-only or fresh)

### Query Engine

Queries follow a pull-based volcano model:

```
Input → ScanOp → FilterOp → ProjectOp → Result
```

Each operator implements the `Operator` trait and returns a set of row IDs. The optimizer reorders and transforms the plan before execution. SIMD acceleration is applied automatically at runtime when AVX2 is detected.

### Compression

KCM applies two-layer compression per column:

1. **Logical encoding** — Transform value representation (Delta, Gorilla, RLE, Dictionary)
2. **Physical compression** — Compress encoded bytes (Zstd, LZ4)

Each column independently selects the optimal encoding/compression pair based on its data characteristics.

### Security

All knowledge at rest is protected by AES-256-GCM authenticated encryption. Keys are derived from passwords via BLAKE3 key derivation or generated from the OS CSPRNG. Access is controlled by a role-based model with per-context permissions and full audit trail.

### Performance Optimization

- **SIMD** — AVX2 intrinsics for 32-byte parallel comparisons with scalar fallback
- **Bitmap operations** — 64-bit word-level AND/OR/NOT with trailing-zeros iteration
- **Hash join** — O(n+m) equi-join using hash table probing
- **Adaptive execution** — Runtime cardinality correction based on execution history
- **Compression** — Per-column codec selection minimizes storage while maintaining access speed

---

## Why KCM

### Traditional Knowledge Graphs vs KCM

| Aspect | Pointer-Based Graph | KCM Columnar |
|--------|-------------------|--------------|
| Storage layout | Adjacency lists with pointers | Independent linear columns |
| Scan pattern | Random pointer chasing | Sequential memory access |
| SIMD utilization | Not applicable | 32-byte parallel filtering |
| Compression | None or graph-specific | Per-column optimal codec |
| Index structure | B-tree, hash index | Bitmap, zone map, bloom filter |
| Query model | Traversal-based | Algebraic operators |
| Confidence scoring | Application-level | Built-in probabilistic model |
| Inference | External tool | Integrated forward-chaining |

### Columnar Databases vs KCM

| Aspect | Analytical DB (DuckDB, ClickHouse) | KCM |
|--------|-----------------------------------|-----|
| Data model | Generic tables | Knowledge triples with confidence |
| Query language | SQL | KQL + algebraic operators |
| Reasoning | None | Forward-chaining inference |
| Confidence | Not applicable | Built-in confidence calculus |
| Knowledge semantics | Not applicable | Subject-Predicate-Object-Evidence |
| Security model | RBAC + encryption | RBAC + AES-256-GCM + audit + GDPR |

---

## Use Cases

### Knowledge Graph Storage

Store and query large-scale knowledge graphs with columnar efficiency. Each triple is a row; each attribute is a column. Query by subject, predicate, object, confidence threshold, or any combination.

### Probabilistic Reasoning

Apply rules to derive new facts from existing knowledge with confidence propagation. Conjunction multiplies confidence; disjunction combines via P(A∪B) = P(A) + P(B) − P(A)·P(B). Results are deterministic and auditable.

### Enterprise Knowledge Management

Deploy as an embedded knowledge engine with full ACID transactions, crash recovery, encryption at rest, RBAC access control, audit logging, and GDPR compliance.

### Fraud Detection and Risk Scoring

Store entity relationships with confidence scores. Query patterns that indicate fraud (e.g., circular ownership chains) using filter operators on subject, object, and confidence columns simultaneously.

### Medical Knowledge Systems

Store drug-disease-treatment relationships with evidence provenance. Query contraindications and side effects. Apply inference rules to discover new drug interactions from existing knowledge.

---

## Performance

Benchmark targets measured with Criterion.rs on release builds with LTO:

| Operation | Target | Method |
|-----------|--------|--------|
| Column sequential scan | > 100M ops/sec | DenseVec iteration |
| Bitmap set/get | > 8M ops/sec | 64-bit word operations |
| Dictionary lookup | < 100ns | HashMap reverse lookup |
| Fact insert throughput | > 50K facts/sec | Schema append + WAL |
| Query latency P99 (1M facts) | < 100ms | Filter + scan |
| Memory per fact | < 34 bytes | Uncompressed column data |
| Compression ratio | > 5x | Zstd/Gorilla/Delta encoding |

Benchmarks are reproducible via `cargo bench --workspace`. Performance baselines are tracked by Criterion with automatic regression detection at 5% threshold.

---

## Documentation

### Technical Specifications

| Document | Description |
|----------|-------------|
| [KCM_SPECIFICATION.md](docs/KCM_SPECIFICATION.md) | Technical constitution — overview, scope, requirements |
| [KCM_ARCHITECTURE.md](docs/KCM_ARCHITECTURE.md) | System architecture, crate responsibilities, data flow |
| [KCM_DATA_MODEL_SPEC.md](docs/KCM_DATA_MODEL_SPEC.md) | Knowledge model, type system, schema design |
| [KCM_COLUMNAR_FORMAT_SPEC.md](docs/KCM_COLUMNAR_FORMAT_SPEC.md) | Binary file format, WAL format, compression |
| [KCM_QUERY_EXECUTION_SPEC.md](docs/KCM_QUERY_EXECUTION_SPEC.md) | Query pipeline, operators, optimizer, KQL |
| [KCM_COMPRESSION_SPEC.md](docs/KCM_COMPRESSION_SPEC.md) | Encoding algorithms, compression codecs |
| [KCM_INDEXING_SPEC.md](docs/KCM_INDEXING_SPEC.md) | Bitmap index, zone map, bloom filter |
| [KCM_SECURITY_TRUST_SPEC.md](docs/KCM_SECURITY_TRUST_SPEC.md) | RBAC, encryption, audit, GDPR |
| [KCM_API_SPEC.md](docs/KCM_API_SPEC.md) | C FFI, Python, REST, gRPC API contracts |
| [KCM_RUNTIME_SPEC.md](docs/KCM_RUNTIME_SPEC.md) | Runtime lifecycle, concurrency, metrics |
| [KCM_PERFORMANCE_SPEC.md](docs/KCM_PERFORMANCE_SPEC.md) | Benchmark targets, load/stress scenarios |
| [KCM_TESTING_SPEC.md](docs/KCM_TESTING_SPEC.md) | Test standards, coverage, CI pipeline |
| [KCM_ENGINEERING_RULES.md](docs/KCM_ENGINEERING_RULES.md) | Development rules and coding standards |
| [KCM_VERSIONING_SPEC.md](docs/KCM_VERSIONING_SPEC.md) | Versioning strategy, compatibility |
| [KCM_GLOSSARY.md](docs/KCM_GLOSSARY.md) | Technical terminology definitions |
| [KCM_DOCUMENT_AUDIT_REPORT.md](docs/KCM_DOCUMENT_AUDIT_REPORT.md) | Documentation consistency audit |

### Source Documents

| Document | Description |
|----------|-------------|
| [PRD.md](PRD.md) | Core specification — types, storage, query, reasoning |
| [PRD2.md](PRD2.md) | Persistence, optimizer, monitoring, deployment |
| [PRD3.md](PRD3.md) | Distributed architecture, ML, security, compliance |
| [PRD-TESTING&BRACHMARCK.md](PRD-TESTING\&BRACHMARCK.md) | Testing strategy, benchmarks, quality gates |

---

## Development

### Prerequisites

- Rust 1.75+ (stable)
- Linux x86_64 (primary target)

### Build

```bash
# Debug build
cargo build --workspace

# Release build with LTO
cargo build --release --workspace

# Build with native CPU optimizations
RUSTFLAGS="-C target-cpu=native" cargo build --release --workspace
```

### Test

```bash
# Run all tests (313+)
cargo test --workspace

# Run with output
cargo test --workspace -- --nocapture

# Run specific crate
cargo test -p kcm-core
cargo test -p kcm-storage
cargo test -p kcm-compute
```

### Benchmark

```bash
# Run all benchmarks
cargo bench --workspace

# Run specific benchmark group
cargo bench --bench micro
```

### Lint

```bash
cargo fmt --all
cargo clippy --workspace -- -D warnings
```

### Project Structure

```
KCM/
├── crates/
│   ├── kcm-core/          Foundation types, DenseVec, Bitmap, Dictionary
│   ├── kcm-storage/       Columns, codecs, WAL, file format, indexes
│   ├── kcm-compute/       Algebra operators, SIMD
│   ├── kcm-reasoning/     Rules, inference, confidence
│   ├── kcm-optimizer/     Cost model, planner, statistics
│   ├── kcm-runtime/       Database, transactions, metrics
│   ├── kcm-interface/     C FFI, Python, REST, KQL
│   ├── kcm-distributed/   Sharding, 2PC coordinator
│   ├── kcm-ml/            Learned index, rule discovery
│   ├── kcm-security/      RBAC, encryption, audit
│   ├── kcm-compliance/    GDPR, data classification
│   └── kcm-testing/       Load/stress test runners
├── docs/                  16 technical specification documents
└── .github/workflows/     CI/CD pipeline
```

---

## License

MIT
