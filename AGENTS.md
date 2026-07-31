# KCM Engineering Agent Configuration

## Project Identity

**KCM — Knowledge Columnar Model** is a production-grade columnar knowledge representation, storage, and reasoning engine implemented in Rust.

- 12 crates, ~12,000 lines of Rust
- 372 tests, 0 failures
- 17 specification documents
- 4 PRD source documents

## Single Source of Truth (SSOT)

### Priority Order (highest to lowest)

1. `PRD-TESTING&BRACHMARCK.md` — Performance targets, validation methodology
2. `PRD3.md` — Distributed architecture, ML, security, compliance
3. `PRD2.md` — Persistence, optimizer, monitoring
4. `PRD.md` — Core types, storage, query, reasoning
5. `docs/*.md` — Derived technical specifications

When documents conflict, the higher-priority document wins.

## Skill Governance System

### Skill Priority Order

The orchestrator (P1) is the single coordination authority. No skill may override orchestrator decisions.

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

Skill Priority ≠ Execution Order. Skills execute in this order:

```
1. Repository Understanding    → kcm-repository-intelligence (P16)
2. Specification Validation    → kcm-specification-lock (P4), kcm-architecture-guardian (P5)
3. Planning                    → kcm-task-planner (P2), kcm-change-impact-analysis (P3)
4. Implementation              → Domain skills (P6, P7, P8)
5. Verification                → kcm-testing-verification (P9), kcm-code-quality-guardian (P10), kcm-code-review-auditor (P13)
6. Release                     → kcm-release-readiness (P12)
```

### Authority Boundaries

**Specification Lock (P4)** owns frozen contracts (formats, APIs, protocols). Can VETO.
**Database Engine Specialist (P6)** owns implementation (algorithms, execution, indexes). Cannot change contracts.

**Resolution:** spec-lock decides IF the change is allowed. db-specialist decides HOW to implement.

**Task Planner (P2)** answers "What should be done?"
**Change Impact Analysis (P3)** answers "What will break?"
**Workflow:** Task Planner → Change Impact Analysis → Implementation

**Code Quality Guardian (P10)** = automated prevention (Rust patterns, placeholders)
**Code Review Auditor (P13)** = senior review (design quality, maintainability)
**Workflow:** CQG runs first → CRA runs after

## Engineering Gates

Every task must pass through 6 mandatory gates.

### Gate 1 — Repository Understanding
- [ ] Crate structure understood
- [ ] Affected modules identified
- [ ] Dependencies mapped
- [ ] Existing implementations located

### Gate 2 — Specification Validation
- [ ] Frozen contracts identified
- [ ] Format compatibility confirmed
- [ ] Architecture alignment verified
- [ ] Dependency boundaries respected

### Gate 3 — Implementation Planning
- [ ] Implementation strategy defined
- [ ] Affected files listed
- [ ] Impact assessment complete
- [ ] Risks identified

### Gate 4 — Implementation Validation
- [ ] No placeholders or stubs
- [ ] Error handling complete
- [ ] Tests written and passing
- [ ] No unwrap in production code

### Gate 5 — Domain Validation
- [ ] Storage/query changes reviewed by db-specialist (if applicable)
- [ ] Security changes reviewed by security-engineer (if applicable)
- [ ] Performance changes benchmarked (if applicable)

### Gate 6 — Production Readiness
- [ ] `cargo build --release` passes
- [ ] `cargo test --workspace` all pass
- [ ] `cargo clippy --workspace -- -D warnings` clean
- [ ] `cargo fmt --all -- --check` clean

## Unified Engineering Report

Every task must produce this report:

```
# KCM Engineering Report

## Skill
[name]

## Analysis
[summary]

## Findings
[list]

## Decision
APPROVE / REJECT / REQUIRE CHANGE

## Specification Impact
[files]

## Code Impact
[files]

## Validation Required
[tests/benchmarks]

## Risks
[list]
```

## Non-Negotiable Rules

- All public APIs return `Result<T, KcmError>`
- No `unwrap()` in production code paths
- No `panic!()` in production code
- No TODO/FIXME/HACK in production code
- No placeholder implementations
- No fake success responses
- All tests must pass before commit
- All clippy warnings must be resolved

## Crate Architecture

```
kcm-core          → Types, DenseVec, Bitmap, Dictionary (zero internal deps)
kcm-storage       → Columns, Codecs, WAL, FileFormat, Index, Backup
kcm-compute       → Algebra operators, SIMD AVX2
kcm-reasoning     → Rules, Forward-chaining inference, Confidence calculus
kcm-optimizer     → Cost model, Planner, Statistics, Rewriting, Adaptive
kcm-runtime       → Database, Transactions, Metrics, Health, Executor
kcm-interface     → C FFI, Python, REST, KQL parser
kcm-distributed   → Sharding (Hash/Range/ConsistentHash), 2PC Coordinator
kcm-ml            → Learned Index, Confidence Learner, Rule Discovery
kcm-security      → RBAC, AES-256-GCM encryption, Audit Log, Compliance
kcm-compliance    → GDPR Manager, Data Classification
kcm-testing       → Load/Stress/Security/Recovery test infrastructure
```

Dependency flow: `core → storage → compute/reasoning/optimizer/distributed/ml → runtime → interface/testing`

## Build and Test Commands

```bash
cargo build --workspace
cargo build --release --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check
cargo bench --workspace
```
