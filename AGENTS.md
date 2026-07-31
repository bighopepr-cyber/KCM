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

Higher priority skills can block changes recommended by lower priority skills.

| Priority | Skill | Authority |
|----------|-------|-----------|
| 1 | kcm-engineering-orchestrator | Can override any skill, coordinates all |
| 2 | kcm-task-planner | Can block implementation without plan |
| 3 | kcm-change-impact-analysis | Can block changes with unassessed impact |
| 4 | kcm-specification-lock | Can veto format/API/FFI changes |
| 5 | kcm-architecture-guardian | Can block architecture violations |
| 6 | kcm-database-engine-specialist | Can block storage/query changes |
| 7 | kcm-security-engineer | Can block security violations |
| 8 | kcm-performance-engineer | Can block performance regressions |
| 9 | kcm-testing-verification | Can block changes without tests |
| 10 | kcm-code-quality-guardian | Can reject code quality issues |
| 11 | kcm-documentation-guardian | Can block undocumented changes |
| 12 | kcm-release-readiness | Can block releases |
| 13 | kcm-code-review-auditor | Provides review feedback |
| 14 | kcm-debugging-root-cause | Provides diagnostic analysis |
| 15 | kcm-engineering-decision-record | Documents decisions |
| 16 | kcm-repository-intelligence | Provides codebase understanding |

### Conflict Resolution

When skills disagree:
1. Higher priority skill wins
2. If same priority, the skill with domain authority wins
3. If still ambiguous, escalate to orchestrator

### Responsibility Boundaries

Each skill has exclusive authority over its domain. No skill may override another skill's domain decision without escalation.

## Engineering Gates

Every task must pass through 5 mandatory gates.

### Gate 1 — Understanding

Before any code change:
- [ ] Analyze user request completely
- [ ] Identify affected components
- [ ] Read related specifications
- [ ] Identify risks
- [ ] Activate kcm-task-planner skill

### Gate 2 — Design Validation

Before implementation:
- [ ] Architecture compatibility verified (kcm-architecture-guardian)
- [ ] Dependency correctness verified (kcm-repository-intelligence)
- [ ] Data format impact assessed (kcm-specification-lock)
- [ ] API impact assessed (kcm-specification-lock)
- [ ] Security impact assessed (kcm-security-engineer)

### Gate 3 — Implementation

During implementation:
- [ ] Production-ready code (no placeholders)
- [ ] Complete logic (no stubs)
- [ ] Error handling (Result<T, KcmError>)
- [ ] Validation (input checks)
- [ ] Tests (unit + integration)

### Gate 4 — Verification

After implementation:
- [ ] `cargo build --release` — passes
- [ ] `cargo test --workspace` — all pass
- [ ] `cargo clippy --workspace -- -D warnings` — zero warnings
- [ ] `cargo fmt --all -- --check` — clean

### Gate 5 — Final Engineering Review

Before completion:
- [ ] Unified Engineering Report generated
- [ ] All gates passed
- [ ] No outstanding issues

## Unified Engineering Report Format

Every task must produce this report before completion:

```
## Engineering Report

Task: [description]

Skills Activated: [list of skills used]

Specifications Reviewed: [list of spec documents read]

Architecture Impact: [none/minor/major + description]

Files Changed:
- [file path]: [change description]

Implementation Status: COMPLETE / PARTIAL / BLOCKED

Tests Added: [count and description]

Benchmark Impact: [none/improved/regressed + numbers]

Security Impact: [none/positive/negative + description]

Compatibility Impact: [backward compatible/breaking + description]

Known Risks: [list or none]

Final Decision: COMPLETE / BLOCKED / NEEDS REVIEW
```

## Non-Negotiable Rules

- All public APIs return `Result<T, KcmError>`
- No `unwrap()` in production code paths
- No `panic!()` in production code
- No TODO/FIXME/HACK comments in production code
- No placeholder implementations
- No fake success responses
- No incomplete modules
- All tests must pass before any change is committed
- All clippy warnings must be resolved (`-D warnings`)

## Development Workflow

```
REQUEST → Gate 1 (Understanding) → Gate 2 (Design) → Gate 3 (Implementation) → Gate 4 (Verification) → Gate 5 (Report)
```

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
kcm-security      → RBAC, AES-256-GCM encryption, Audit Log
kcm-compliance    → GDPR Manager, Data Classification
kcm-testing       → Load/Stress/Security/Recovery test infrastructure
```

Dependency flow: `core → storage → compute/reasoning/optimizer/distributed/ml → runtime → interface/testing`

No circular dependencies. kcm-core has zero internal dependencies.

## Build and Test Commands

```bash
cargo build --workspace
cargo build --release --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check
cargo bench --workspace
```
