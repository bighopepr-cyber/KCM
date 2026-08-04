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
kcm-interface     → C FFI (18 functions), Python bindings (PyO3), REST handlers, KQL parser.
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
kcm-storage (core + log + zstd + lz4 + blake3 + thiserror)
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
kcm-testing (core + storage + runtime + reasoning + security + distributed + compliance)
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
| P5 | `docs/*.md` | Derived technical specifications (26 documents) |

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
| Audit Log | `Mutex<VecDeque<AuditEvent>>` (parking_lot, wrapped in Arc) | Serialized append, FIFO eviction at 100K |
| Metrics | `AtomicU64` (14 counters) | Lock-free counters |
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
| Unit | 89+ | < 100ms | Single function correctness |
| Integration | 470+ | 1s-5s | Cross-component correctness |
| Property | 8+ | 1-5min | Invariant verification |
| Security | 29+ | varies | Attack surface validation |

Quality gates: ≥95% coverage, 0 clippy warnings, 0 unwrap in production, benchmarks within 5% of baseline.

### Automated Validation

```bash
bash scripts/validate-ssot.sh  # 13 automated checks
```

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

---

## SSOT-First Development Rules

**The SSOT (Single Source of Truth) documentation is the absolute technical contract for the KCM project. No implementation may deviate from the SSOT without an approved SSOT update.**

### SSOT Authority

| Rule | Description |
|------|-------------|
| SSOT-01 | All public APIs, data structures, formats, algorithms, and behaviors are defined in SSOT documents |
| SSOT-02 | Implementation MUST match SSOT specifications exactly |
| SSOT-03 | No code change is permitted that deviates from SSOT without first updating the SSOT |
| SSOT-04 | SSOT updates require approval from the Specification Lock (P4) skill |
| SSOT-05 | When SSOT and code diverge, the SSOT is the reference — fix the code, not the SSOT |
| SSOT-06 | Every code change must trace back to a requirement in the SSOT |
| SSOT-07 | New features require SSOT specification before implementation begins |
| SSOT-08 | API changes require backward compatibility analysis before SSOT update |

### Requirement Traceability

Every implementation must be traceable to an SSOT requirement:

```
SSOT Requirement → Specification Document → Implementation File → Test File → Benchmark
```

| Traceability Level | Description |
|-------------------|-------------|
| L1 | Requirement exists in SSOT |
| L2 | Specification defines behavior |
| L3 | Implementation matches specification |
| L4 | Test validates implementation |
| L5 | Benchmark measures performance |

### SSOT Documents (Authoritative Sources)

| Document | Scope | Priority |
|----------|-------|----------|
| `docs/PRD.md` | Core types, storage, compute, reasoning | P4 |
| `docs/PRD2.md` | Storage, runtime, interfaces | P3 |
| `docs/PRD3.md` | Distributed, ML, security, compliance | P2 |
| `docs/PRD-TESTING& BRACHMARCK.md` | Testing, benchmarks, quality gates | P1 |
| `AGENTS.md` | Engineering constitution | P5 |

---

## Codebase Audit Procedures

### Pre-Implementation Audit

Before any code change, perform this audit:

| Step | Action | Output |
|------|--------|--------|
| 1 | Identify affected crates and modules | Affected file list |
| 2 | Map SSOT requirements for affected area | Requirement IDs |
| 3 | Verify current implementation matches SSOT | Drift report |
| 4 | Check dependency impact | Dependency graph |
| 5 | Assess backward compatibility | Compatibility report |
| 6 | Identify affected tests | Test list |
| 7 | Identify affected benchmarks | Benchmark list |

### Post-Implementation Audit

After any code change, perform this audit:

| Step | Action | Command |
|------|--------|---------|
| 1 | Verify compilation | `cargo build --workspace` |
| 2 | Run all tests | `cargo test --workspace` |
| 3 | Run clippy | `cargo clippy --workspace -- -D warnings` |
| 4 | Check formatting | `cargo fmt --all -- --check` |
| 5 | Verify SSOT compliance | `bash scripts/validate-ssot.sh` |
| 6 | Check for stubs/placeholders | `grep -r "todo!\|unimplemented!\|FIXME\|TODO" crates/ --include="*.rs"` |
| 7 | Check for unwrap in production | `grep -r "\.unwrap()" crates/ --include="*.rs" \| grep -v tests/ \| grep -v benches/` |

### Continuous Audit Schedule

| Audit Type | Frequency | Owner |
|------------|-----------|-------|
| SSOT compliance | Every PR | CI pipeline |
| Stub/placeholder detection | Every PR | CI pipeline |
| Benchmark regression | Weekly | benchmark.yml |
| Dependency audit | Monthly | Manual |
| Full codebase audit | Quarterly | kcm-engineering-orchestrator |

---

## Implementation Quality Standards

### Code Quality Requirements

| Requirement | Standard | Enforcement |
|-------------|----------|-------------|
| Error handling | All public APIs return `Result<T, KcmError>` | Compiler + clippy |
| No unwrap | Zero `unwrap()` in production code paths | CI gate |
| No panic | Zero `panic!()` in production code | CI gate |
| No TODO/FIXME | Zero markers in production code | CI gate |
| No placeholders | Every function has real implementation | Code review |
| No fake returns | Every return value is computed, not hardcoded | Code review |
| Thread safety | All shared types are `Send + Sync` | Compiler |
| Memory safety | No unsafe without documented justification | Code review |
| Determinism | Identical input produces identical output | Tests |

### Architecture Consistency

| Rule | Description |
|------|-------------|
| Single responsibility | Each crate has exactly one responsibility |
| Dependency direction | Dependencies flow upward only (no cycles) |
| Interface segregation | Public APIs are minimal and focused |
| Encapsulation | Internal details are not exposed through public API |
| Consistency | Similar operations have similar interfaces |

### Backward Compatibility

| Change Type | Compatibility Requirement |
|-------------|-------------------------|
| New public method | Additive, no breaking change |
| New crate | Additive, no breaking change |
| New dependency | Must justify existence per Dependency Policy |
| API signature change | Breaking — requires version bump |
| Remove public API | Breaking — requires version bump + migration |
| Format change | Breaking — requires version bump + migration |
| FFI change | Breaking — requires SDK version bump |

---

## Engineering Analysis Requirements

### Before Implementation

Every implementation task must complete this analysis:

1. **Requirements Analysis**: What SSOT requirements does this address?
2. **Architecture Analysis**: How does this fit the existing architecture?
3. **Dependency Analysis**: What dependencies are affected?
4. **Impact Analysis**: What other components are affected?
5. **Risk Analysis**: What could go wrong?
6. **Test Analysis**: What tests are needed?
7. **Benchmark Analysis**: What performance targets apply?
8. **Compatibility Analysis**: Is this backward compatible?

### During Implementation

Every implementation must follow:

1. **SSOT Alignment**: Implementation matches specification exactly
2. **Error Handling**: All error paths return `Result<T, KcmError>`
3. **Thread Safety**: All shared types are `Send + Sync`
4. **Memory Safety**: No unsafe without documented justification
5. **Determinism**: No randomness in query/inference paths
6. **Observability**: Metrics and logging where appropriate
7. **Testability**: Code is structured for testability

### After Implementation

Every implementation must verify:

1. **Correctness**: All tests pass
2. **Performance**: Benchmarks within 5% of baseline
3. **Quality**: No clippy warnings
4. **Format**: Code passes fmt check
5. **SSOT**: Automated validation passes
6. **Documentation**: Changes reflected in SSOT

---

## Refactoring Standards

### Safe Refactoring Rules

| Rule | Description |
|------|-------------|
| R-01 | Refactoring must not change external behavior |
| R-02 | All tests must pass before and after refactoring |
| R-03 | No new features during refactoring |
| R-04 | Refactoring must be verifiable by SSOT validation |
| R-05 | Refactoring must not break backward compatibility |
| R-06 | Refactoring must be reversible (git revert) |

### Refactoring Checklist

- [ ] SSOT requirements still met
- [ ] All tests pass
- [ ] No new stubs/placeholders introduced
- [ ] No new unwrap() in production code
- [ ] No new TODO/FIXME markers
- [ ] Clippy clean
- [ ] Format clean
- [ ] SSOT validation passes
- [ ] Benchmarks within 5% of baseline
- [ ] No dependency changes

---

## CI/CD and Release Validation

### CI Pipeline Requirements

| Job | Trigger | Blocks Merge |
|-----|---------|-------------|
| Format Check | Every push | Yes |
| Clippy Lint | Every push | Yes |
| Build | Every push | Yes |
| Unit Tests | Every push | Yes |
| Integration Tests | Every push | Yes |
| Property Tests | Every push | Yes |
| Security Tests | After unit tests | Yes |
| Benchmarks (compile) | After unit tests | Yes |
| SSOT Validation | Every push | Yes |
| Quality Gate | All above pass | Yes |

### Release Validation

Before any release:

| Step | Action | Gate |
|------|--------|------|
| 1 | All CI jobs pass | `ci.yml` |
| 2 | SSOT validation passes | `validate-ssot.sh` |
| 3 | No regressions from baseline | Benchmark comparison |
| 4 | All public APIs match SSOT | API audit |
| 5 | All FFI functions match SSOT | FFI audit |
| 6 | All REST endpoints match SSOT | REST audit |
| 7 | All gRPC RPCs match SSOT | gRPC audit |
| 8 | Deployment configs valid | Docker/K8s build |
| 9 | Documentation up to date | SSOT review |
| 10 | Changelog updated | Manual review |

### Version Bumping Rules

| Change Type | Version Bump | Example |
|-------------|-------------|---------|
| Bug fix | Patch (0.0.x) | WAL replay fix |
| New feature | Minor (0.x.0) | New codec, new index |
| Breaking API change | Major (x.0.0) | Remove FFI function |
| Format change | Major (x.0.0) | Header layout change |
| Dependency change | Patch or Minor | Depends on impact |

---

## Development Playbooks

### New Feature Playbook

```
1. SSOT: Define requirement in appropriate PRD document
2. SSOT: Define specification in appropriate KCM_*_SPEC document
3. Plan: Identify affected crates, files, tests, benchmarks
4. Implement: Write code matching SSOT specification
5. Test: Write tests validating implementation
6. Benchmark: Write/verify benchmarks for performance
7. Validate: Run full quality gate suite
8. SSOT: Update documentation if implementation differs from spec
9. Review: Code review against SSOT
10. Release: Version bump, changelog, release
```

### Bug Fix Playbook

```
1. Reproduce: Write a test that demonstrates the bug
2. Root Cause: Identify the exact cause using debugging skills
3. Fix: Implement minimal fix matching SSOT behavior
4. Verify: Ensure fix resolves the bug without regressions
5. Validate: Run full quality gate suite
6. Review: Code review focusing on fix correctness
7. Release: Patch version bump, changelog
```

### Performance Optimization Playbook

```
1. Baseline: Run benchmarks to establish current performance
2. Profile: Identify bottleneck using CPU/memory profiling
3. SSOT: Verify performance target exists in SSOT
4. Optimize: Implement optimization matching SSOT target
5. Measure: Verify improvement with benchmarks
6. Validate: Ensure no correctness regressions
7. Document: Update SSOT if behavior changed
8. Review: Performance engineer review
```

### Security Fix Playbook

```
1. Assess: Determine severity and impact
2. SSOT: Verify security requirement in SSOT
3. Fix: Implement fix matching security specification
4. Test: Write security test validating fix
5. Audit: Run security test suite
6. Validate: Full quality gate suite
7. Review: Security engineer review
8. Release: Immediate patch if critical
```

---

## Monitoring and Observability

### Metrics Requirements

| Component | Required Metrics |
|-----------|-----------------|
| KnowledgeDatabase | queries_total, inserts_total, cache_hit_ratio, memory_bytes |
| Transaction | commit_count, rollback_count, abort_count |
| WAL | append_count, replay_count, flush_count |
| Inference | inference_count, facts_inferred, rule_execution_count |
| Security | permission_check_count, encryption_count, audit_event_count |

### Logging Standards

| Level | Usage |
|-------|-------|
| ERROR | Unrecoverable errors requiring immediate attention |
| WARN | Recoverable errors that may indicate issues |
| INFO | Significant state changes (startup, shutdown, recovery) |
| DEBUG | Detailed operational information |
| TRACE | Most detailed, lowest priority |

### Health Check Requirements

| Status | Condition |
|--------|-----------|
| Healthy | error_rate < 5%, latency < 100ms, cache_hit_ratio > 50% |
| Degraded | latency > 100ms OR cache_hit_ratio < 50% |
| Unhealthy | error_rate > 5% |
