# KCM Engineering Constitution

## Identity

**KCM — Knowledge Columnar Model** is a self-contained columnar knowledge representation, storage, query, and reasoning engine implemented in Rust. KCM owns its core technology stack: storage, execution, query engine, compression, dictionary encoding, bitmap engine, optimizer, reasoning engine, transaction engine, recovery, benchmarking, testing, monitoring, and documentation.

## Engineering Philosophy

KCM optimizes for:
1. **Correctness** over performance
2. **Determinism** over flexibility
3. **Simplicity** over generality
4. **Engineering ownership** over external abstraction
5. **Implementation completeness** over feature breadth
6. **Long-term sustainability** over short-term convenience

KCM rejects:
- Framework-driven design
- Technology coupling
- Unnecessary middleware
- Hidden runtime behavior
- Unnecessary serialization layers
- Unnecessary protocol layers
- Dynamic architecture where deterministic architecture suffices
- Multiple implementations for the same responsibility

## System Architecture

### Crate Map (13 crates, single responsibility each)

```
kcm-core          → Types, DenseVec, Bitmap, Dictionary. Depends on parking_lot only.
kcm-storage       → Columns, Codecs, WAL, FileFormat, Index, Backup, Recovery, DictCodec.
kcm-compute       → Relational algebra operators, SIMD AVX2 acceleration.
kcm-reasoning     → Rule definitions, forward-chaining inference engine.
kcm-optimizer     → Cost model, query planner, statistics, plan rewriting, adaptive execution.
kcm-runtime       → KnowledgeDatabase, Transactions, Metrics, Health, Executor, AsyncExecutor.
kcm-interface     → C FFI (15 functions), Python bindings (PyO3), REST handlers, KQL parser.
kcm-distributed   → Sharding strategies (Hash/Range/ConsistentHash), 2PC coordinator.
kcm-ml            → Learned index (regression), confidence learner, rule discovery.
kcm-security      → RBAC (5 permission levels), AES-256-GCM encryption, audit log (hash-chained).
kcm-compliance    → GDPR consent management, data classification (4 tiers).
kcm-testing       → Load, stress, security, recovery, regression detection, metrics dashboard.
kcm-server        → HTTP (actix-web) + gRPC (tonic) server binaries.
```

### Dependency Flow

```
kcm-core (zero deps)
  ↑
kcm-storage (core + zstd + lz4 + blake3 + thiserror)
  ↑
kcm-compute (core + storage)
kcm-reasoning (core + storage)
kcm-optimizer (core + storage)
  ↑
kcm-runtime (core + storage + parking_lot + rayon + tokio)
  ↑
kcm-interface (core + storage + runtime + parking_lot + serde + serde_json)
  ↑
kcm-server (core + runtime + interface + actix-web + tonic + prost + tokio)

kcm-distributed (core + parking_lot)
kcm-ml (core + reasoning)
kcm-security (core + parking_lot + blake3 + aes-gcm + getrandom)
kcm-compliance (core + parking_lot)
kcm-testing (core + storage + runtime + security + distributed + compliance)
```

### Dependency Policy

Every external dependency must justify its existence:

| Dependency | Justification | Could Remove? |
|------------|---------------|---------------|
| parking_lot | 3-5x faster RwLock/Mutex than std. Used in 7 crates. | Yes, measurable perf regression |
| zstd | Industry-standard compression codec. Complex to reimplement. | No |
| lz4 | Speed-optimized compression. Complex to reimplement. | No |
| blake3 | Fastest cryptographic hash. Used for checksums + key derivation. | No |
| thiserror | Derive macro for Error trait. Boilerplate only. | Yes, manual impl |
| rayon | Work-stealing parallel iterator library. | Yes, manual threads (loses work-stealing) |
| tokio | Async runtime. No practical replacement. | No |
| serde/serde_json | Serialization framework. No practical replacement. | No |
| aes-gcm | Authenticated encryption. Must use audited crypto. | No |
| getrandom | CSPRNG. Platform-specific alternative possible. | Yes, portability loss |
| actix-web | HTTP server. | Yes, could use hyper directly |
| tonic/prost | gRPC framework. No replacement for gRPC compliance. | No |
| pyo3 | Python bindings. Feature-gated. | No (when python feature enabled) |
| log/env_logger | Logging. | Yes, custom macros |
| criterion | Dev-only benchmarking. | Yes, manual timing |
| proptest | Dev-only property testing. | Yes, custom fuzzing |
| quickcheck | **UNUSED.** Redundant with proptest. | **Remove** |

## Single Source of Truth

### Document Hierarchy

| Priority | Document | Authority |
|----------|----------|-----------|
| P1 | `docs/PRD-TESTING& BRACHMARCK.md` | Performance targets, validation methodology, testing strategy |
| P2 | `docs/PRD3.md` | Distributed architecture, ML integration, security, compliance |
| P3 | `docs/PRD2.md` | Persistence layer, optimizer, monitoring, interfaces |
| P4 | `docs/PRD.md` | Core types, storage engine, compute engine, reasoning engine |
| P5 | `docs/*.md` | Derived technical specifications (23 documents) |

When documents conflict, the higher-priority document wins.

### Specification Ownership

| Domain | Authoritative Document | Type Definitions |
|--------|----------------------|-----------------|
| Core types | PRD.md §3 | Fact, RowID, SubjectID, Confidence, KcmError |
| Storage format | PRD2.md §15 | DB header, column blocks, WAL entries |
| Query engine | PRD.md §5 | Operator trait, ScanOp, FilterOp, JoinOp |
| Optimizer | PRD2.md §16 | PlanNode, CostModel, Planner |
| Runtime | PRD2.md §18 | KnowledgeDatabase, Transaction, Metrics |
| Interfaces | PRD2.md §19 | KCM_Database (FFI), REST handlers, KQL parser |
| Distributed | PRD3.md §27 | ShardMap, TransactionCoordinator |
| Security | PRD3.md §30 | ACLManager, EncryptionKey, AuditLog |
| Compliance | PRD3.md §32 | GDPRManager, DataClassification |
| Testing | PRD-TESTING§1-8 | Test pyramid, quality gates, benchmark suite |
| Benchmarks | PRD-TESTING§4 | Criterion configuration, results template |

## Engineering Gates

Every task passes 6 mandatory gates. No exceptions.

### Gate 1 — Repository Understanding
- Crate structure understood
- Affected modules identified
- Dependencies mapped
- Existing implementations located

### Gate 2 — Specification Validation
- Frozen contracts identified
- Format compatibility confirmed
- Architecture alignment verified
- Dependency boundaries respected

### Gate 3 — Implementation Planning
- Implementation strategy defined
- Affected files listed
- Impact assessment complete
- Risks identified

### Gate 4 — Implementation Validation
- No placeholders or stubs
- Error handling complete
- Tests written and passing
- No unwrap in production code

### Gate 5 — Domain Validation
- Storage/query changes reviewed by database-engine-specialist
- Security changes reviewed by security-engineer
- Performance changes benchmarked

### Gate 6 — Production Readiness
- `cargo build --release` passes
- `cargo test --workspace` all pass
- `cargo clippy --workspace -- -D warnings` clean
- `cargo fmt --all -- --check` clean

## Non-Negotiable Rules

1. All public APIs return `Result<T, KcmError>`
2. No `unwrap()` in production code paths
3. No `panic!()` in production code
4. No TODO/FIXME/HACK in production code
5. No placeholder implementations
6. No fake success responses
7. All tests must pass before commit
8. All clippy warnings must be resolved
9. Every requirement maps to an implementation
10. Every implementation maps to a test
11. Every benchmark validates a documented requirement
12. No documentation describes behavior that doesn't exist

## Error Model

Single error hierarchy rooted at `KcmError`:

```
KcmError
├── NotFound(String)
├── OutOfMemory
├── InvalidArgument(String)
├── Io(String)
├── Corrupted(String)
├── Conflict(String)
└── TransactionAborted
```

Storage-specific errors (`StorageError`) convert to `KcmError` via `From` impl.

## Concurrency Model

| Component | Mechanism | Rationale |
|-----------|-----------|-----------|
| Schema | `Arc<RwLock<Schema>>` (parking_lot) | Readers concurrent, writers exclusive |
| Dictionaries | `Arc<RwLock<Dictionary>>` (parking_lot) | Same pattern as Schema |
| WAL | `Mutex<File>` (parking_lot) | Serialized writes |
| Audit Log | `Mutex<Vec<AuditEvent>>` (parking_lot) | Serialized append |
| Metrics | `AtomicU64` (11 counters) | Lock-free counters |
| Thread Pool | rayon ThreadPool | Work-stealing parallelism |
| Async | tokio Runtime | I/O-bound async operations |

## Storage Model

Single storage model: columnar with per-column encoding and compression.

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

## Query Model

Single query model: Volcano-style pull-based execution with cost-based optimization.

Operators: Scan → Filter → Project → Join → Aggregate
Optimizer: Filter pushdown → Column pruning → Join reordering → Index selection

## Testing Strategy

Single testing strategy: 4-tier pyramid.

| Tier | Count | Speed | Purpose |
|------|-------|-------|---------|
| Unit | 90+ | < 100ms | Single function correctness |
| Integration | 108+ | 1s-5s | Cross-component correctness |
| Property | 8+ | 1-5min | Invariant verification |
| Security | 29+ | varies | Attack surface validation |

Quality gates: ≥95% coverage, 0 clippy warnings, 0 unwrap in production, benchmarks within 5% of baseline.

## Build and Test Commands

```bash
cargo build --workspace
cargo build --release --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check
cargo bench --workspace
```

## Skill Governance

16 engineering skills with defined authority boundaries:

| Priority | Skill | Authority |
|----------|-------|-----------|
| P1 | kcm-engineering-orchestrator | Master coordinator — overrides all |
| P2 | kcm-task-planner | Can block implementation without plan |
| P3 | kcm-change-impact-analysis | Can block changes with unassessed impact |
| P4 | kcm-specification-lock | Can veto format/API/FFI changes |
| P5 | kcm-architecture-guardian | Can block architecture violations |
| P6 | kcm-database-engine-specialist | Can block storage/query changes |
| P7 | kcm-security-engineer | Can block security/compliance violations |
| P8 | kcm-performance-engineer | Can block performance regressions |
| P9 | kcm-testing-verification | Can block changes without tests |
| P10 | kcm-code-quality-guardian | Can reject code quality issues |
| P11 | kcm-documentation-guardian | Can block undocumented changes |
| P12 | kcm-release-readiness | Can block releases |
| P13 | kcm-code-review-auditor | Provides review feedback |
| P14 | kcm-debugging-root-cause | Provides diagnostic analysis |
| P15 | kcm-engineering-decision-record | Documents decisions |
| P16 | kcm-repository-intelligence | Provides codebase understanding |

### Execution Flow

```
1. Repository Understanding    → kcm-repository-intelligence (P16)
2. Specification Validation    → kcm-specification-lock (P4), kcm-architecture-guardian (P5)
3. Planning                    → kcm-task-planner (P2), kcm-change-impact-analysis (P3)
4. Implementation              → Domain skills (P6, P7, P8)
5. Verification                → kcm-testing-verification (P9), kcm-code-quality-guardian (P10), kcm-code-review-auditor (P13)
6. Release                     → kcm-release-readiness (P12)
```

### Authority Boundaries

- **Specification Lock (P4)** owns frozen contracts. Can VETO.
- **Database Engine Specialist (P6)** owns implementation. Cannot change contracts.
- **Architecture Guardian (P5)** owns system architecture. Defers to P4 for format changes.
- **Task Planner (P2)** answers "What should be done?"
- **Change Impact Analysis (P3)** answers "What will break?"
- **Code Quality Guardian (P10)** = automated prevention. Runs FIRST.
- **Code Review Auditor (P13)** = senior review. Runs AFTER CQG.
