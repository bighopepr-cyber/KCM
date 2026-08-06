# KCM Engineering Constitution

> Document ID: KCM-CONSTITUTION-001
> Version: 4.0.0
> Status: Active
> Owner: Engineering Orchestrator (P1)
> Authority: Highest — supersedes all other governance documents except SSOT.md

## Table of Contents

1. [Mission](#1-mission)
2. [Vision](#2-vision)
3. [Engineering Philosophy](#3-engineering-philosophy)
4. [Core Principles](#4-core-principles)
5. [Repository Constitution](#5-repository-constitution)
6. [Repository Structure Rules](#6-repository-structure-rules)
7. [SSOT Authority](#7-ssot-authority)
8. [Documentation Hierarchy](#8-documentation-hierarchy)
9. [Decision Hierarchy](#9-decision-hierarchy)
10. [Change Management](#10-change-management)
11. [Engineering Workflow](#11-engineering-workflow)
12. [Review Workflow](#12-review-workflow)
13. [Security Rules](#13-security-rules)
14. [Performance Rules](#14-performance-rules)
15. [Testing Rules](#15-testing-rules)
16. [Documentation Rules](#16-documentation-rules)
17. [Versioning Rules](#17-versioning-rules)
18. [API Stability Rules](#18-api-stability-rules)
19. [Benchmark Policy](#19-benchmark-policy)
20. [Release Policy](#20-release-policy)
21. [AI Agent Behaviour](#21-ai-agent-behaviour)
22. [Conflict Resolution](#22-conflict-resolution)
23. [Quality Gates](#23-quality-gates)
24. [Enforcement Rules](#24-enforcement-rules)
25. [Skill Governance](#25-skill-governance)
26. [References](#26-references)

---

## 1. Mission

KCM (Knowledge Columnar Model) is a self-contained columnar knowledge representation, storage, query, and reasoning engine implemented in Rust. KCM owns its entire technology stack: storage, execution, query engine, compression, dictionary encoding, bitmap engine, optimizer, reasoning engine, transaction engine, recovery, benchmarking, testing, monitoring, and documentation.

## 2. Vision

To become the definitive open-source columnar knowledge engine — enterprise-grade, fully auditable, deterministically governed, and powered by an AI engineering system that maintains perfect consistency between specification, implementation, and documentation.

## 3. Engineering Philosophy

KCM optimizes for:
- **Correctness** over performance
- **Determinism** over flexibility
- **Simplicity** over generality
- **Engineering ownership** over external abstraction
- **Implementation completeness** over feature breadth
- **Long-term sustainability** over short-term convenience

KCM rejects:
- Framework-driven design
- Technology coupling
- Unnecessary middleware
- Hidden runtime behavior
- Unnecessary serialization layers
- Unnecessary protocol layers
- Dynamic architecture where deterministic architecture suffices
- Multiple implementations for the same responsibility

## 4. Core Principles

1. **SSOT First** — Single Source of Truth governs all decisions
2. **Specification Before Implementation** — No code without spec
3. **Test Before Merge** — No merge without passing tests
4. **Security By Design** — Security is not an afterthought
5. **Documentation As Code** — Documentation is versioned, reviewed, tested
6. **Deterministic Governance** — AI decisions are auditable and reproducible
7. **Zero Trust Assumptions** — Every input is validated, every output is verified
8. **Minimal Dependencies** — Every dependency must justify its existence
9. **Backward Compatibility** — Breaking changes require explicit approval
10. **Continuous Validation** — Automation enforces quality gates

## 5. Repository Constitution

### 5.1 Document Hierarchy

| Priority | Document | Authority |
|----------|----------|-----------|
| P1 | SSOT.md | Single Source of Truth — highest authority |
| P2 | AGENTS.md (this document) | Engineering Constitution |
| P3 | docs/specs/PRD-TESTING-AND-BENCHMARK.md | Testing & benchmark targets |
| P4 | docs/specs/PRD3.md | Distributed, ML, security, compliance |
| P5 | docs/specs/PRD2.md | Storage, runtime, interfaces |
| P6 | docs/specs/PRD.md | Core types, storage, compute, reasoning |
| P7 | docs/specs/KCM_*.md | Derived specifications (15 documents) |
| P8 | docs/adr/ADR-*.md | Architecture Decision Records |

### 5.2 Conflict Resolution

When documents conflict:
1. Higher priority document wins
2. If equal priority, SSOT.md wins
3. If SSOT.md is silent, AGENTS.md wins
4. If both are silent, Engineering Orchestrator (P1) decides
5. All conflicts are documented as ADRs

### 5.3 Immutable Contracts

The following are FROZEN and cannot be changed without P4 (Specification Lock) approval:
- Binary file format (DB_MAGIC, DB_VERSION, header layout)
- WAL entry format (WAL_INSERT_SIZE, WAL_DELETE_SIZE)
- C FFI signatures (18 functions)
- Error code enum (7 variants)
- Fact structure (34 bytes, 10 fields)
- gRPC proto definitions
- Public API return types (`Result<T, KcmError>`)
- `#[repr(C)]` struct layouts

## 6. Repository Structure Rules

### 6.1 Crate Map (13 crates, single responsibility each)

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

### 6.2 Dependency Flow

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

### 6.3 Dependency Policy

Every external dependency must justify its existence. See the dependency table in the workspace Cargo.toml.

## 7. SSOT Authority

### 7.1 SSOT-First Development Rules

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

### 7.2 Requirement Traceability

```
SSOT Requirement → Specification Document → Implementation File → Test File → Benchmark
```

| Level | Description |
|-------|-------------|
| L1 | Requirement exists in SSOT |
| L2 | Specification defines behavior |
| L3 | Implementation matches specification |
| L4 | Test validates implementation |
| L5 | Benchmark measures performance |

## 8. Documentation Hierarchy

| Priority | Document | Authority |
|----------|----------|-----------|
| P1 | SSOT.md | Single Source of Truth |
| P2 | AGENTS.md | Engineering Constitution |
| P3 | docs/specs/PRD-TESTING-AND-BENCHMARK.md | Testing targets |
| P4 | docs/specs/PRD3.md | Distributed, ML, security |
| P5 | docs/specs/PRD2.md | Storage, runtime, interfaces |
| P6 | docs/specs/PRD.md | Core types, storage, compute |
| P7 | docs/specs/KCM_*.md | Derived specifications |
| P8 | docs/governance/documentation-governance.md | Documentation governance |

## 9. Decision Hierarchy

```
SSOT Requirement
  ↓
Specification Lock (P4) — validates contract compatibility
  ↓
Architecture Guardian (P5) — validates architectural integrity
  ↓
Domain Specialist (P6/P7/P8) — validates domain correctness
  ↓
Code Quality Guardian (P10) — validates code quality
  ↓
Testing Verification (P9) — validates correctness
  ↓
Documentation Guardian (P11) — validates documentation
  ↓
Release Readiness (P12) — validates production readiness
  ↓
Engineering Orchestrator (P1) — final coordination
```

## 10. Change Management

### 10.1 Change Categories

| Category | Approval Required | Skills Involved |
|----------|------------------|-----------------|
| Bug fix | P10 + P9 | Code Quality, Testing |
| New feature | P2 + P4 + P5 + P9 + P11 | Task Planner, Spec Lock, Arch Guardian, Testing, Doc Guardian |
| API change | P4 + P5 + P9 + P11 | Spec Lock, Arch Guardian, Testing, Doc Guardian |
| FFI change | P4 + P7 + P5 | Spec Lock, Security, Arch Guardian |
| Storage format | P4 + P5 + P6 + P7 | Spec Lock, Arch Guardian, DB Specialist, Security |
| Security fix | P7 + P4 | Security, Spec Lock |
| Performance | P8 + P9 | Performance Engineer, Testing |
| Documentation | P11 | Documentation Guardian |
| Release | P12 + P1 | Release Readiness, Orchestrator |

### 10.2 Change Process

```
1. Task Planning (P2)
2. Impact Analysis (P3)
3. Specification Validation (P4)
4. Architecture Validation (P5)
5. Implementation (Domain Specialist)
6. Code Quality (P10)
7. Testing (P9)
8. Benchmark (P8) — if performance-related
9. Documentation (P11)
10. Code Review (P13)
11. Release Readiness (P12)
12. Final Coordination (P1)
```

## 11. Engineering Workflow

### 11.1 Standard Workflow

```
Task → Planning → Impact Analysis → Specification Validation → Architecture Validation
  → Implementation → Code Quality → Testing → Benchmark → Documentation
  → Code Review → Release Readiness → Done
```

### 11.2 Emergency Workflow (Critical Bugs)

```
Bug Report → Root Cause (P14) → Minimal Fix → Testing (P9) → Code Quality (P10) → Release (P12)
```

### 11.3 Security Workflow

```
Security Issue → Security Engineer (P7) → Spec Lock (P4) → Fix → Security Test → Release (P12)
```

## 12. Review Workflow

### 12.1 Code Review Process

```
1. Author completes implementation
2. Code Quality Guardian (P10) — automated check
3. Testing Verification (P9) — test validation
4. Code Review Auditor (P13) — senior review
5. Domain Specialist — domain review
6. Final approval — owner
```

### 12.2 Review SLA

| Change Type | Review SLA | Approval SLA |
|-------------|-----------|-------------|
| Critical (security) | 24 hours | 48 hours |
| High (API, breaking) | 24 hours | 48 hours |
| Medium (feature) | 48 hours | 72 hours |
| Low (typo, doc) | 12 hours | 24 hours |

## 13. Security Rules

### 13.1 Non-Negotiable Security Rules

1. All encryption uses AES-256-GCM with BLAKE3 KDF
2. Zero hardcoded keys, tokens, or credentials
3. CSPRNG for all random number generation
4. Hash-chained audit logging for all write operations
5. RBAC enforcement on every sensitive operation
6. TLS for all network communication
7. Input validation on all public interfaces
8. Null-pointer guards on all FFI functions
9. No `unsafe` without documented `// SAFETY:` justification
10. Security changes require P7 (Security Engineer) approval

### 13.2 Security Model

| Component | Mechanism |
|-----------|-----------|
| Encryption | AES-256-GCM, 256-bit key, 96-bit nonce |
| Key Derivation | BLAKE3 |
| RBAC | 5 permission levels (Read, Write, Admin, SuperAdmin, Owner) |
| Audit Log | Hash-chained, FIFO at 100K events |
| Compliance | GDPR consent management, 4-tier data classification |

## 14. Performance Rules

### 14.1 Performance Targets

| Metric | Target |
|--------|--------|
| Column scan | > 100M ops/sec |
| Bitmap operations | > 8M ops/sec |
| Dictionary lookup | < 100ns |
| Insert throughput | > 50K facts/sec |
| Query P99 latency | < 100ms |
| Memory per fact | < 34 bytes (uncompressed) |
| Compression ratio | > 5x |

### 14.2 Regression Thresholds

| Regression | Action |
|-----------|--------|
| < 5% | Acceptable — no action |
| 5-10% | WARNING — requires justification |
| > 10% | FAILURE — blocks merge |

### 14.3 Benchmark Policy

- Every performance claim must have a benchmark
- All benchmarks use criterion with statistical analysis
- Baseline stored in benchmark-results/
- Regression detection via bench-compare.py
- Benchmark results are part of release validation

## 15. Testing Rules

### 15.1 Test Pyramid

| Tier | Count | Speed | Purpose |
|------|-------|-------|---------|
| Unit | 89+ | < 100ms | Single function correctness |
| Integration | 470+ | 1s-5s | Cross-component correctness |
| Property | 8+ | 1-5min | Invariant verification |
| Security | 29+ | varies | Attack surface validation |

### 15.2 Testing Requirements

1. Every public function must have at least one unit test
2. Every bug fix must have a regression test
3. Every storage change must have recovery tests
4. Every security change must have security tests
5. Every numeric operation must have property tests
6. No fake tests (tests that always pass)
7. No placeholder assertions
8. 100% test pass rate required for merge

### 15.3 Quality Gates

```bash
cargo fmt --all -- --check          # Format
cargo clippy --workspace -- -D warnings  # Lint
cargo build --workspace              # Build
cargo test --workspace               # Test
bash scripts/validate-ssot.sh        # SSOT
```

## 16. Documentation Rules

### 16.1 Documentation Requirements

1. Every crate must have README.md, SECURITY.md, CONTRIBUTING.md, CODE_OF_CONDUCT.md
2. Every crate must have docs/<crate>/spesifikasi.md
3. Every folder must follow the 5-document blueprint
4. All documentation is version controlled
5. All documentation is reviewed before merge
6. Documentation changes follow the same CI pipeline as code

### 16.2 Documentation Standards

- Enterprise-grade quality
- SSOT-compliant
- No duplication with root docs
- Cross-references to root documents
- Consistent heading structure
- Table of Contents for long documents

## 17. Versioning Rules

| Change Type | Version Bump | Example |
|-------------|-------------|---------|
| Bug fix | Patch (0.0.x) | WAL replay fix |
| New feature | Minor (0.x.0) | New codec, new index |
| Breaking API change | Major (x.0.0) | Remove FFI function |
| Format change | Major (x.0.0) | Header layout change |
| Dependency change | Patch or Minor | Depends on impact |

## 18. API Stability Rules

### 18.1 API Contract

- All public APIs return `Result<T, KcmError>`
- API changes require P4 (Specification Lock) approval
- Breaking changes require major version bump
- Backward compatibility analysis required before any API change
- API documentation must be updated with every change

### 18.2 FFI Stability Rules

- All FFI functions have `# Safety` documentation
- All FFI functions validate null pointers
- Memory management uses `Box::into_raw` / `Box::from_raw`
- FFI changes require P4 + P7 approval
- FFI changes require SDK version bump

### 18.3 SDK Stability Rules

- All SDKs expose identical API surface
- SDK changes require cross-SDK consistency validation
- SDK breaking changes require major version bump
- SDK changes require test validation across all languages

## 19. Benchmark Policy

- Benchmarks use criterion with statistical analysis
- Results stored in benchmark-results/
- Regression detection via scripts/bench-compare.py
- Thresholds: 5% warning, 10% failure
- Baseline must be updated before merge if benchmarks change
- Performance claims must be backed by benchmarks

## 20. Release Policy

### 20.1 Release Validation

1. All CI jobs pass
2. SSOT validation passes
3. No regressions from baseline
4. All public APIs match SSOT
5. All FFI functions match SSOT
6. All REST endpoints match SSOT
7. All gRPC RPCs match SSOT
8. Deployment configs valid
9. Documentation up to date
10. Changelog updated

### 20.2 Release Process

1. P12 (Release Readiness) validates all gates
2. P1 (Orchestrator) gives final approval
3. Version bump per versioning rules
4. Changelog update
5. Git tag
6. CI/CD triggers release

## 21. AI Agent Behaviour

### 21.1 Mandatory Behaviour

All AI agents must:
1. Follow the Engineering Workflow (Section 11)
2. Respect the Authority System (Section 22)
3. Validate against SSOT before any change
4. Run quality gates before reporting completion
5. Produce deterministic, reproducible output
6. Document all decisions
7. Never modify frozen contracts without P4 approval
8. Never modify security model without P7 approval
9. Never modify API without P4 approval
10. Never modify benchmark targets without P8 approval

### 21.2 Forbidden Behaviour

AI agents must NOT:
1. Modify SSOT without P4 approval
2. Modify API without P4 approval
3. Modify FFI without P4 + P7 approval
4. Modify benchmark targets without P8 approval
5. Modify security model without P7 approval
6. Modify storage format without P4 + P5 approval
7. Modify protocol without P4 approval
8. Modify data model without P4 + P5 approval
9. Modify documentation hierarchy without P1 approval
10. Modify dependency architecture without P5 approval
11. Create duplicate implementations
12. Create duplicate specifications
13. Produce documentation conflicting with SSOT
14. Skip quality gates
15. Use placeholder implementations

### 21.3 Output Standards

All AI output must be:
- Deterministic
- Reproducible
- Consistent
- SSOT-compliant
- Well-documented
- Fully traceable

## 22. Conflict Resolution

### 22.1 Skill Conflicts

| Scenario | Resolution |
|----------|-----------|
| Two skills disagree | Higher priority wins |
| Same priority, different domain | Domain authority wins |
| Same priority, same domain | Engineering Orchestrator decides |
| Security vs Performance | Security wins (P7 > P8) |
| Security vs Functionality | Security wins (P7 > any feature) |
| Performance vs Correctness | Correctness wins (per philosophy) |

### 22.2 Escalation Rules

| Level | Escalation Path |
|-------|----------------|
| Level 1 | Skill internally resolves |
| Level 2 | Higher priority skill resolves |
| Level 3 | Engineering Orchestrator (P1) resolves |
| Level 4 | SSOT.md is the final authority |

## 23. Quality Gates

### 23.1 CI Pipeline

```
Format Check → Clippy Lint → Build → Unit Tests → Integration Tests
  → Property Tests → Security Tests → SSOT Validation → Quality Gate
```

### 23.2 Merge Requirements

- All CI jobs pass
- At least 1 reviewer approval
- No unresolved conflicts
- SSOT traceability documented
- Tests pass
- Documentation updated

### 23.3 Release Requirements

- All merge requirements
- No performance regressions > 5%
- All benchmarks within baseline
- Documentation complete
- Changelog updated
- Version bumped

## 24. Enforcement Rules

### 24.1 Automated Enforcement

- CI pipeline enforces format, lint, build, test
- Documentation validator enforces required files
- SSOT validator enforces alignment
- Coverage validator enforces 100%
- Drift detector enforces code-doc sync

### 24.2 Manual Enforcement

- Code owners review all changes
- P10 (Code Quality) runs first
- P13 (Code Review) runs after P10
- P12 (Release) gates all releases
- P1 (Orchestrator) coordinates all skills

## 25. Skill Governance

### 25.1 Skill Registry

| Priority | Skill | Authority |
|----------|-------|-----------|
| P1 | kcm-engineering-orchestrator | Master coordinator — overrides all |
| P2 | kcm-task-planner | Can block implementation without plan |
| P3 | kcm-change-impact-analysis | Can block changes with unassessed impact |
| P4 | kcm-specification-lock | Can VETO format/API/FFI changes |
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

### 25.2 Authority Types

| Type | Skills | Description |
|------|--------|-------------|
| Override | P1 | Can override any skill decision |
| Veto | P4 | Can block contract changes |
| Block | P2,P3,P5,P6,P7,P8,P9,P10,P11,P12 | Can block specific categories |
| Feedback | P13,P14,P15,P16 | Advisory only, no blocking power |

### 25.3 Skill Execution Order

```
1. Repository Intelligence (P16) — understand codebase
2. Task Planner (P2) — plan implementation
3. Change Impact Analysis (P3) — assess impact
4. Specification Lock (P4) — validate contracts
5. Architecture Guardian (P5) — validate architecture
6. Domain Specialist (P6/P7/P8) — implement
7. Code Quality Guardian (P10) — quality check
8. Testing Verification (P9) — test validation
9. Performance Engineer (P8) — benchmark (if needed)
10. Documentation Guardian (P11) — doc update
11. Code Review Auditor (P13) — review
12. Release Readiness (P12) — release gate
13. Engineering Orchestrator (P1) — final coordination
```

## 26. References

- [SSOT.md](SSOT.md) — Single Source of Truth
- [KCM_SPECIFICATION.md](KCM_SPECIFICATION.md) — Technical constitution
- [docs/handbook/repository-structure.md](docs/handbook/repository-structure.md) — Repository structure
- [CONTRIBUTING.md](CONTRIBUTING.md) — Contribution guidelines
- [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) — Community guidelines
- [SECURITY.md](SECURITY.md) — Security policy
- [docs/governance/documentation-governance.md](docs/governance/documentation-governance.md) — Documentation governance
- [skills/](skills/) — AI engineering skills