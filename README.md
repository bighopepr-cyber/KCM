# KCM — Knowledge Columnar Model

> A columnar storage and reasoning engine for knowledge representation, built in Rust for performance, correctness, and enterprise reliability.

---

## What is KCM

KCM stores knowledge as **columnar relation spaces** rather than pointer-based graphs. Each attribute of a knowledge fact — subject, predicate, object, confidence, evidence, timestamp, context, version, priority, owner — is stored in its own independent, contiguous, SIMD-aligned column.

This architecture enables the system to:

- Scan millions of facts per second using sequential memory access
- Apply SIMD-accelerated filtering across 32-byte AVX2 lanes
- Compress each column independently with codec-specific algorithms
- Execute deterministic forward-chaining inference with confidence propagation
- Maintain ACID transactions with WAL-based crash recovery

```
Traditional:     Pointer → Pointer → Pointer → Value
KCM:             Column[Subject] → Column[Predicate] → Column[Object]
                 Sequential     Sequential         Sequential
                 SIMD-ready     SIMD-ready         SIMD-ready
```

---

## Architecture

```
┌────────────────────────────────────────────────────────────┐
│                     Interface Layer                          │
│  C FFI (13 functions) · REST API · KQL Parser · Python     │
├────────────────────────────────────────────────────────────┤
│                     Runtime Layer                            │
│  KnowledgeDatabase · Transactions · Metrics · Health        │
│  Executor (rayon) · AsyncExecutor (tokio)                   │
├────────────────────────────────────────────────────────────┤
│                     Compute Layer                            │
│  5 Algebra Operators · SIMD (AVX2) · Query Optimizer        │
├────────────────────────────────────────────────────────────┤
│                     Reasoning Layer                          │
│  Forward-Chaining Inference · Confidence Calculus           │
├────────────────────────────────────────────────────────────┤
│                     Storage Layer                            │
│  10-Column Schema · Codecs (Delta/RLE/Gorilla)             │
│  Compression (Zstd/LZ4) · WAL · File Format · Indexes     │
├────────────────────────────────────────────────────────────┤
│                     Core Layer                               │
│  Types · DenseVec (64-byte aligned) · Bitmap · Dictionary   │
└────────────────────────────────────────────────────────────┘
```

**Dependency flow:** `kcm-core → kcm-storage → kcm-compute/reasoning/optimizer → kcm-runtime → kcm-interface`

---

## Crates

| Crate | Responsibility | Public API |
|-------|---------------|------------|
| `kcm-core` | Foundation types, DenseVec, Bitmap, Dictionary | Types, DenseVec, Bitmap, Dictionary |
| `kcm-storage` | Columnar storage, WAL, file format, codecs, compression, indexes | Schema, Column, WAL, BackupManager |
| `kcm-compute` | Query algebra operators, SIMD acceleration | ScanOp, FilterOp, ProjectOp, JoinOp, AggregateOp |
| `kcm-reasoning` | Rule engine, forward-chaining inference, confidence calculus | InferenceEngine, ConfidenceCalculator |
| `kcm-optimizer` | Cost model, query planner, statistics, adaptive execution | Planner, CostModel, Statistics |
| `kcm-runtime` | Database lifecycle, transactions, metrics, health checks | KnowledgeDatabase, Transaction, Metrics |
| `kcm-interface` | C FFI, Python bindings, REST API, KQL parser | 13 C functions, 8 REST handlers |
| `kcm-distributed` | Sharding (Hash/Range/Consistent Hash), 2PC coordinator | ShardMap, TransactionCoordinator |
| `kcm-ml` | Learned index models, confidence learner, rule discovery | LearnedIndex, ConfidenceLearner |
| `kcm-security` | RBAC, AES-256-GCM encryption, audit logging | ACLManager, EncryptedStorage, AuditLog |
| `kcm-compliance` | GDPR data subject management, data classification | GDPRManager |
| `kcm-testing` | Load/stress test runners, security tests, regression detector | LoadTestResults, StressTestResults |
| `kcm-server` | HTTP (actix-web) and gRPC (tonic) server binaries | `kcm-server`, `kcm-grpc` |

---

## Quick Start

### Build

```bash
cargo build --release
```

### Insert and Query

```rust
use kcm_runtime::database::KnowledgeDatabase;
use kcm_core::types::*;

fn main() -> Result<(), KcmError> {
    let kb = KnowledgeDatabase::new()?;

    // Insert facts
    let fact = Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9)?;
    let row_id = kb.insert(&fact)?;

    // Query by subject
    let results = kb.query()
        .with_subject(SubjectID(1))
        .with_confidence(0.5)
        .execute()?;

    // Update
    let updated = Fact::new(SubjectID(5), PredicateID(2), ObjectID(6), 0.7)?;
    kb.update(row_id, &updated)?;

    // Delete
    kb.delete(row_id)?;

    // Get by ID
    if let Some(fact) = kb.get_fact(RowID(0))? {
        println!("Subject: {}", fact.subject.0);
    }

    Ok(())
}
```

### HTTP Server

```bash
cargo run --release -p kcm-server
# KCM HTTP API available at http://0.0.0.0:8080
# POST   /facts          — Insert
# GET    /facts          — Query
# GET    /facts/{id}     — Get by ID
# PUT    /facts/{id}     — Update
# DELETE /facts/{id}     — Delete
# GET    /health         — Health check
# GET    /stats          — System statistics
# GET    /metrics        — Prometheus metrics
```

### gRPC Server

```bash
cargo run --release -p kcm-grpc
# gRPC service at http://0.0.0.0:50051
```

---

## Storage Format

KCM uses a binary columnar format version 2 with per-column compression headers.

### File Layout

```
Header (31 bytes)
├── Magic: "KCMDB" (5 bytes)
├── Version: u8 (1 byte)
├── Row Count: u64 LE (8 bytes)
├── Column Count: u8 (1 byte)
├── Created Timestamp: i64 LE (8 bytes)
└── Modified Timestamp: i64 LE (8 bytes)

Column Blocks (×10)
├── Element Count: u64 LE (8 bytes)
├── Codec ID: u8 (1 byte)
│   0=None, 1=Zstd, 2=LZ4, 3=RLE
├── Compressed Size: u64 LE (8 bytes)
└── Data: variable

Checksum (32 bytes)
└── Blake3 hash of entire file
```

### WAL Format

```
INSERT Entry (38 bytes)
├── OpType: u8 (1 byte) = 0x01
├── Subject: u32 LE (4 bytes)
├── Predicate: u8 (1 byte)
├── Object: u32 LE (4 bytes)
├── Confidence: f64 LE (8 bytes)
├── Timestamp: i64 LE (8 bytes)
├── Context: u8 (1 byte)
├── Version: i32 LE (4 bytes)
├── Priority: i8 (1 byte)
├── Owner: u16 LE (2 bytes)
└── CRC32: u32 LE (4 bytes)

DELETE Entry (13 bytes)
├── OpType: u8 (1 byte) = 0x02
├── RowID: u64 LE (8 bytes)
└── CRC32: u32 LE (4 bytes)
```

---

## Compression

| Encoding | Algorithm | Column | Rationale |
|----------|-----------|--------|-----------|
| Dictionary | String → u32 | Subject, Object, Predicate, Evidence, Context, Owner | Low cardinality, repeated references |
| Delta | Consecutive difference | Timestamp, Version | Monotonic sequences |
| Gorilla | XOR-based | Confidence | Slowly changing floats |
| RLE | Run-length | Predicate, Evidence, Context, Priority | Repeated values |

Physical compression: Zstd (level 3) for general data, LZ4 for fast decompression, custom RLE for byte sequences.

---

## Query Engine

### Operators

| Operator | Input | Output | Complexity |
|----------|-------|--------|------------|
| ScanOp | Schema | Row IDs | O(n) |
| FilterOp | Row IDs | Row IDs | O(n) |
| ProjectOp | Row IDs | Row IDs + column values | O(n) |
| JoinOp | 2× Row IDs | Joined row IDs | O(n+m) hash join |
| AggregateOp | Row IDs | Aggregated values | O(n) |

### KQL Syntax

```sql
SELECT subject, object
FROM facts
WHERE subject = 1
  AND confidence >= 0.5
ORDER BY object ASC
LIMIT 100
```

### Optimization

- Filter pushdown to data source
- Join reordering by cost estimation
- Statistics-driven selectivity estimation
- Adaptive execution with runtime feedback

---

## Reasoning Engine

### Forward-Chaining Inference

```
Rule: IF subject_2(X, Y) AND subject_3(Y, Z) THEN subject_1(X, Z)
      Confidence = min(source_confidences)
```

### Confidence Calculus

| Operation | Formula | Function |
|-----------|---------|----------|
| Conjunction | P(A∧B) = P(A) × P(B) | `multiply()` |
| Disjunction | P(A∨B) = P(A) + P(B) − P(A)×P(B) | `combine_or()` |
| Negation | P(¬A) = 1 − P(A) | `negation()` |
| Chain | P(A₁∧A₂∧...∧Aₙ) = Π P(Aᵢ) | `chain()` |
| Weighted | Σ(wᵢ·P(Aᵢ)) / Σ(wᵢ) | `weighted()` |

---

## Security

| Component | Implementation |
|-----------|---------------|
| Encryption | AES-256-GCM with 12-byte random nonces |
| Key derivation | BLAKE3 `derive_key` with 32-byte salt |
| Key generation | OS CSPRNG via `getrandom` crate |
| Key zeroization | `write_volatile` in `Drop` impl |
| RBAC | 5 permissions: Read, Write, Delete, Execute, Admin |
| Context ACL | Per-context permission overrides |
| Audit log | Hash-chained append-only event log |
| GDPR | Consent management, data export, right to deletion |
| Data classification | 4 tiers: Public, Internal, Confidential, Restricted |

---

## Testing

| Category | Count | Description |
|----------|-------|-------------|
| Unit | 200+ | Single function correctness |
| Integration | 108+ | Cross-crate interaction |
| Property | 31 | Roundtrip invariants, bounds checking |
| Security | 29 | Encryption, RBAC, GDPR, corruption |
| Concurrency | 6 | Thread safety under contention |
| Recovery | 12 | WAL replay, crash recovery, backup/restore |
| Server/HTTP | 19 | REST endpoint validation |
| KQL Parser | 27 | Lexer/parser edge cases |
| **Total** | **530** | **0 failures** |

### Run Tests

```bash
cargo test --workspace                              # All tests
cargo test -p kcm-core                              # Core only
cargo test -p kcm-storage --test test_property      # Property tests
cargo test -p kcm-security --test test_security     # Security tests
cargo test -p kcm-testing --test test_concurrent    # Concurrency tests
cargo test -p kcm-testing --test test_crash_recovery # Recovery tests
```

---

## Benchmarks

Benchmark groups with scaling from 1K to 1M:

| Group | Benchmarks | Scaling |
|-------|-----------|---------|
| Column Operations | sequential scan, random access, SIMD filter, push | 1K → 1M |
| Bitmap Operations | set, get, count, AND, OR, iter_set_bits | 10K → 1M |
| Dictionary Operations | insert, lookup, insert_existing | 1K → 100K |
| Database Operations | insert, query, filtered query, join | 100 → 1M |
| Reasoning | inference, pattern matching, confidence, rules | 1K → 100K |
| Storage I/O | WAL append, WAL replay, file save/load | 1K → 100K |
| Codecs | Delta, Gorilla, RLE encode | 100K elements |
| Distributed | Hash/Range/Consistent Hash routing | 10K routes |
| Memory | per-fact, bitmap, dictionary, DenseVec | 100K → 1M |

### Run Benchmarks

```bash
cargo bench --workspace                           # All benchmarks
cargo bench --workspace --no-run                  # Compile only
scripts/bench-report.sh                           # Full report with metadata
```

---

## Development

### Prerequisites

- Rust 1.75+ (stable toolchain)
- Linux x86_64 (primary target)
- protobuf compiler (for gRPC: `protoc`)

### Build

```bash
cargo build --workspace                            # Debug
cargo build --release --workspace                  # Release with LTO
RUSTFLAGS="-C target-cpu=native" cargo build --release  # Native CPU
```

### Quality Gate

```bash
cargo fmt --all -- --check                         # Format
cargo clippy --workspace --all-targets -- -D warnings  # Lint
cargo test --workspace                             # Tests
cargo build --release --workspace                  # Release build
cargo bench --workspace --no-run                   # Benchmarks compile
```

### Project Structure

```
KCM/
├── crates/
│   ├── kcm-core/          Foundation: types, DenseVec, Bitmap, Dictionary
│   ├── kcm-storage/       Storage: columns, codecs, WAL, file format, indexes, backup
│   ├── kcm-compute/       Compute: algebra operators, SIMD (AVX2)
│   ├── kcm-reasoning/     Reasoning: rules, inference, confidence calculus
│   ├── kcm-optimizer/     Optimizer: cost model, planner, statistics, adaptive
│   ├── kcm-runtime/       Runtime: database, transactions, metrics, health, logging
│   ├── kcm-interface/     Interface: C FFI, Python, REST, KQL parser
│   ├── kcm-distributed/   Distributed: sharding, 2PC coordinator
│   ├── kcm-ml/            ML: learned index, confidence learner, rule discovery
│   ├── kcm-security/      Security: RBAC, AES-256-GCM, audit logging
│   ├── kcm-compliance/    Compliance: GDPR, data classification
│   ├── kcm-testing/       Testing: load/stress/recovery/concurrency tests
│   └── kcm-server/        Server: HTTP (actix-web) + gRPC (tonic) binaries
├── docs/                   18 technical specification documents
├── skills/                 16 engineering skill definitions
├── scripts/                Build and benchmark automation
├── benchmark-results/      Benchmark artifacts and metadata
├── k8s/                    Kubernetes deployment manifests
├── .github/workflows/      CI/CD pipeline
├── AGENTS.md               Engineering governance
├── kilo.json               AI agent configuration
├── PRD.md                  Core specification
├── PRD2.md                 Persistence, optimizer, deployment
├── PRD3.md                 Distributed, ML, security, compliance
└── GAP.md                  Gap analysis and tracking
```

---

## Specifications

All engineering decisions derive from authoritative specifications.

### Source Documents (Engineering Truth)

| Document | Scope |
|----------|-------|
| [PRD.md](PRD.md) | Core: types, storage, query, reasoning |
| [PRD2.md](PRD2.md) | Persistence, optimizer, monitoring, deployment |
| [PRD3.md](PRD3.md) | Distributed, ML, security, compliance |
| [PRD-TESTING&BRACHMARCK.md](PRD-TESTING\&BRACHMARCK.md) | Testing strategy, benchmarks, quality gates |
| [GAP.md](GAP.md) | Gap analysis and implementation tracking |

### Technical Specifications

| Document | Scope |
|----------|-------|
| [KCM_SPECIFICATION.md](docs/KCM_SPECIFICATION.md) | Technical constitution |
| [KCM_ARCHITECTURE.md](docs/KCM_ARCHITECTURE.md) | System architecture |
| [KCM_DATA_MODEL_SPEC.md](docs/KCM_DATA_MODEL_SPEC.md) | Knowledge model, types |
| [KCM_COLUMNAR_FORMAT_SPEC.md](docs/KCM_COLUMNAR_FORMAT_SPEC.md) | Binary format, WAL |
| [KCM_QUERY_EXECUTION_SPEC.md](docs/KCM_QUERY_EXECUTION_SPEC.md) | Query pipeline, KQL |
| [KCM_COMPRESSION_SPEC.md](docs/KCM_COMPRESSION_SPEC.md) | Encodings, codecs |
| [KCM_INDEXING_SPEC.md](docs/KCM_INDEXING_SPEC.md) | Bitmap, zone map, bloom filter |
| [KCM_SECURITY_TRUST_SPEC.md](docs/KCM_SECURITY_TRUST_SPEC.md) | RBAC, encryption, GDPR |
| [KCM_API_SPEC.md](docs/KCM_API_SPEC.md) | C FFI, REST, gRPC contracts |
| [KCM_RUNTIME_SPEC.md](docs/KCM_RUNTIME_SPEC.md) | Concurrency, metrics, health |
| [KCM_PERFORMANCE_SPEC.md](docs/KCM_PERFORMANCE_SPEC.md) | Benchmark targets |
| [KCM_TESTING_SPEC.md](docs/KCM_TESTING_SPEC.md) | Test standards, CI pipeline |
| [KCM_ENGINEERING_RULES.md](docs/KCM_ENGINEERING_RULES.md) | Development rules |
| [KCM_VERSIONING_SPEC.md](docs/KCM_VERSIONING_SPEC.md) | Versioning, compatibility |
| [KCM_DEPLOYMENT_SPEC.md](docs/KCM_DEPLOYMENT_SPEC.md) | Docker, Kubernetes |
| [KCM_BENCHMARK_REPORTING_SPEC.md](docs/KCM_BENCHMARK_REPORTING_SPEC.md) | Benchmark artifacts |
| [KCM_GLOSSARY.md](docs/KCM_GLOSSARY.md) | Terminology |
| [KCM_DOCUMENT_AUDIT_REPORT.md](docs/KCM_DOCUMENT_AUDIT_REPORT.md) | Documentation audit |

---

## CI/CD Pipeline

| Job | Trigger | What it validates |
|-----|---------|-------------------|
| Format Check | Every push | `cargo fmt --all -- --check` |
| Build | Every push | `cargo build --workspace` |
| Clippy | Every push | `cargo clippy --workspace -- -D warnings` |
| Unit Tests | Every push | `cargo test --lib --all` |
| Integration Tests | Every push | `cargo test --test '*' --all` |
| Security Tests | After unit tests | `cargo test security_tests --all` |
| Property Tests | Every push | `cargo test property_tests --all` |
| Load Tests | After unit tests | `cargo test load_tests --all` |
| Stress Tests | After unit tests | `cargo test stress_tests --all` |
| Recovery Tests | After unit tests | `cargo test recovery --all` |
| Benchmarks | After unit tests | `cargo bench --workspace --no-run` |
| Quality Gate | All above pass | Final merge decision |

---

## Engineering Governance

KCM uses a 16-skill engineering system enforced by AI agents:

| Priority | Skill | Role |
|----------|-------|------|
| P1 | Engineering Orchestrator | Master coordinator |
| P2 | Task Planner | Implementation planning |
| P3 | Change Impact Analysis | Pre-change assessment |
| P4 | Specification Lock | Frozen contract protection |
| P5 | Architecture Guardian | Architecture integrity |
| P6 | Database Engine Specialist | Storage/query correctness |
| P7 | Security Engineer | Security and compliance |
| P8 | Performance Engineer | Performance validation |
| P9 | Testing Verification | Test coverage |
| P10 | Code Quality Guardian | Rust code quality |
| P11 | Documentation Guardian | Spec consistency |
| P12 | Release Readiness | Production validation |
| P13 | Code Review Auditor | Senior review |
| P14 | Debugging Root Cause | Bug investigation |
| P15 | Engineering Decision Record | Decision documentation |
| P16 | Repository Intelligence | Codebase understanding |

**Execution priority:** Correctness → Specification → Data Integrity → Security → Reliability → Performance → Maintainability → Speed

---

## License

MIT
