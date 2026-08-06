---
name: kcm-engineering-orchestrator
description: Master coordinator for all KCM engineering skills — enforces governance, authority hierarchy, engineering gates, and unified reporting
---

# Skill: Engineering Orchestrator

## Skill Identity

**Purpose:** The orchestrator is the single coordination authority for the KCM engineering skill system. It decides which skills activate, enforces priority order, resolves conflicts, and ensures every code change follows the engineering gate pipeline.

**Role:** Master Coordinator

**Scope:** All 16 engineering skills, all 6 engineering gates, conflict resolution, unified reporting.

**Non-responsibility:** Does not implement code. Does not write tests. Does not review code. Delegates to specialist skills for domain-specific work.

**Measurable Outcomes:**
- Every task passes through all 6 engineering gates
- Every skill produces a structured Engineering Report
- Conflicts between skills are resolved with documented rationale
- No skill overrides orchestrator decisions

---

## Activation Rules

The orchestrator activates for:
- Any task requiring 2+ skills
- Any task touching protected specifications
- Any architecture-level change
- Any cross-crate change
- Any release preparation
- Conflict resolution between skills

The orchestrator does NOT activate for:
- Single-file bug fixes within one module
- Test-only changes
- Documentation-only changes
- Formatting-only changes

---

## Required Inputs

- User request or task description
- Affected file list
- Relevant specification documents
- Current codebase state

---

## Crate Awareness

The workspace contains **13 crates**:

```
kcm-core          → Types, DenseVec, Bitmap, Dictionary (zero internal deps)
kcm-storage       → Columns, Codecs, WAL, FileFormat, Index, Backup, Recovery, Errors, DictCodec
kcm-compute       → Algebra operators, SIMD AVX2
kcm-reasoning     → Rules, Forward-chaining inference
kcm-optimizer     → Cost model, Planner, Statistics, Rewriting, Adaptive
kcm-runtime       → Database, Transactions, Metrics, Health, Executor
kcm-interface     → C FFI, Python, REST, KQL parser
kcm-distributed   → Sharding (Hash/Range/ConsistentHash), 2PC Coordinator
kcm-ml            → Learned Index, Confidence Learner, Rule Discovery
kcm-security      → RBAC, AES-256-GCM encryption, Audit Log
kcm-compliance    → GDPR Manager, Data Classification
kcm-testing       → Load/Stress/Security/Recovery test infrastructure, Metrics Dashboard
kcm-server        → gRPC server, gRPC main, main entry point
```

**Dependency flow:**
```
core → storage → compute/reasoning/optimizer/distributed/ml → runtime → interface → server
```

---

## Workflow

```
REQUEST
  ↓
Gate 1: Repository Understanding (kcm-repository-intelligence)
  ↓
Gate 2: Specification Validation (kcm-specification-lock, kcm-architecture-guardian)
  ↓
Gate 3: Planning (kcm-task-planner, kcm-change-impact-analysis)
  ↓
Gate 4: Implementation (domain skills: database, security, performance, server)
  ↓
Gate 5: Verification (kcm-testing-verification, kcm-code-quality-guardian, kcm-code-review-auditor)
  ↓
Gate 6: Release (kcm-release-readiness)
  ↓
UNIFIED REPORT
```

---

## Skill Registry

| Priority | Skill | Authority | Domain |
|----------|-------|-----------|--------|
| P1 | kcm-engineering-orchestrator | Master coordinator | All skills, all gates |
| P2 | kcm-task-planner | Implementation planning | What should be done |
| P3 | kcm-change-impact-analysis | Impact assessment | What will break |
| P4 | kcm-specification-lock | Contract veto | Frozen formats, APIs, protocols, gRPC proto |
| P5 | kcm-architecture-guardian | Architecture veto | Dependencies, modules, crate structure |
| P6 | kcm-database-engine-specialist | Storage/query authority | Columns, WAL, codecs, indexes, query engine |
| P7 | kcm-security-engineer | Security authority | Encryption, RBAC, audit, GDPR, compliance, gRPC/TLS |
| P8 | kcm-performance-engineer | Performance authority | Benchmarks, SIMD, memory, algorithms |
| P9 | kcm-testing-verification | Test authority | Coverage, correctness, regression |
| P10 | kcm-code-quality-guardian | Quality authority | Rust patterns, unwrap, placeholders |
| P11 | kcm-documentation-guardian | Documentation authority | Spec consistency, doc quality |
| P12 | kcm-release-readiness | Release authority | Build, tests, performance gates |
| P13 | kcm-code-review-auditor | Review authority | Maintainability, design quality |
| P14 | kcm-debugging-root-cause | Diagnostic authority | Root cause analysis |
| P15 | kcm-engineering-decision-record | Decision authority | Long-term decision capture |
| P16 | kcm-repository-intelligence | Codebase authority | Structure, dependencies, ownership |

---

## Authority Boundaries

### Specification Lock (P4) vs Database Engine Specialist (P6)

**Specification Lock owns:**
- Binary file format (magic bytes, header layout, column block format)
- WAL entry format (byte layout, field order, entry sizes)
- Public API contracts (function signatures, return types)
- C FFI interface definitions
- gRPC proto definitions
- Error code enum variants
- Schema evolution rules
- Backward compatibility requirements

**Can:** BLOCK any implementation that violates frozen contracts.
**Cannot:** Design internal algorithms. Cannot own implementation details.

**Database Engine Specialist owns:**
- Storage algorithms (how columns store data)
- Query execution logic (how operators process data)
- Indexing implementation (how indexes are built and queried)
- Compression implementation (how codecs encode/decode)
- Transaction logic (how ACID is maintained)
- Recovery implementation (how crash recovery works)

**Can:** Choose implementation strategies. Decide algorithmic approach.
**Cannot:** Change public contracts without specification-lock approval.

**Resolution:** spec-lock decides IF the change is allowed (contract compliance). db-specialist decides HOW the change is implemented (algorithmic correctness). If db-specialist needs a contract change, spec-lock must approve first.

### Architecture Guardian (P5) vs Specification Lock (P4)

**Specification Lock (P4) owns:** Frozen data/protocol specifications.
**Architecture Guardian (P5) owns:** System architecture, dependency boundaries, module responsibilities.

**Resolution:** spec-lock (P4) has higher priority. If architecture change requires format change, spec-lock must approve the format change first. Architecture guardian then validates the architectural implications.

### Task Planner (P2) vs Change Impact Analysis (P3)

**Task Planner (P2) answers:** "What should be done?"
- Creates implementation strategy
- Defines execution order
- Identifies required skills

**Change Impact Analysis (P3) answers:** "What will break?"
- Analyzes dependencies
- Assesses compatibility
- Identifies affected modules
- Evaluates risks

**Workflow:** Task Planner → Change Impact Analysis → Implementation

### Code Quality Guardian (P10) vs Code Review Auditor (P13)

**Code Quality Guardian (P10):** Automated prevention.
- Rust patterns, unsafe usage, complexity, placeholders, dead code
- Runs FIRST — rejects obviously bad code

**Code Review Auditor (P13):** Senior engineer review.
- Maintainability, architecture quality, long-term impact, design decisions
- Runs AFTER — evaluates deeper quality concerns

---

## Engineering Gates

Every task must pass through 6 mandatory gates. No task may be marked complete unless all required gates pass.

### Gate 1 — Repository Understanding

**Required skill:** kcm-repository-intelligence

**Must verify:**
- Understand crate structure (13 crates)
- Identify affected modules
- Map dependency relationships
- Locate existing implementations

### Gate 2 — Specification Validation

**Required skills:** kcm-specification-lock, kcm-architecture-guardian

**Must verify:**
- Frozen contracts identified
- Format compatibility confirmed
- Architecture alignment verified
- Dependency boundaries respected

### Gate 3 — Implementation Planning

**Required skills:** kcm-task-planner, kcm-change-impact-analysis

**Must verify:**
- Implementation strategy defined
- Affected files listed
- Impact assessment complete
- Risks identified with mitigations

### Gate 4 — Implementation Validation

**Required skills:** kcm-code-quality-guardian, kcm-testing-verification

**Must verify:**
- No placeholders or stubs
- Error handling complete
- Tests written and passing
- No unwrap in production code

### Gate 5 — Domain Validation

**Required skills:** (conditional based on change type)

| Change Type | Required Skill |
|-------------|---------------|
| Storage/query | kcm-database-engine-specialist |
| Security/compliance | kcm-security-engineer |
| Performance | kcm-performance-engineer |
| Documentation | kcm-documentation-guardian |
| Server/gRPC | kcm-database-engine-specialist (storage), kcm-security-engineer (TLS/auth) |

### Gate 6 — Production Readiness

**Required skill:** kcm-release-readiness

**Must verify:**
- `cargo build --release` passes (all 13 crates including kcm-server)
- `cargo test --workspace` all pass
- `cargo clippy --workspace -- -D warnings` clean
- `cargo fmt --all -- --check` clean
- No performance regression > 5%

---

## Execution Flow

```
Step 1: Repository Understanding
  kcm-repository-intelligence → Where does the change belong?

Step 2: Specification Validation
  kcm-specification-lock → Is the change contract-compliant?
  kcm-architecture-guardian → Is the change architecturally sound?

Step 3: Planning
  kcm-task-planner → What is the implementation plan?
  kcm-change-impact-analysis → What will break?

Step 4: Implementation
  Domain skills activate based on change type:
  - kcm-database-engine-specialist (storage/query)
  - kcm-security-engineer (security/compliance/gRPC-TLS)
  - kcm-performance-engineer (performance-critical)

Step 5: Verification
  kcm-code-quality-guardian → Is the code production-ready?
  kcm-testing-verification → Are tests adequate?
  kcm-code-review-auditor → Is the design sound?

Step 6: Release
  kcm-release-readiness → Is it ready to ship?
```

---

## Conflict Resolution

When skills disagree:

1. **Higher priority wins** — P4 (spec-lock) overrides P6 (db-specialist)
2. **Domain authority wins** — Within same priority, the skill with domain expertise wins
3. **Engineering priority wins** — Correctness > Specification > Data Integrity > Security > Reliability > Performance > Maintainability > Speed
4. **Orchestrator is final** — If conflict cannot be resolved, orchestrator makes the final decision

---

## Validation Criteria

| Criterion | Pass Condition |
|-----------|---------------|
| All 16 skills registered | Registry table complete |
| All 13 crates recognized | Crate count correct |
| Authority boundaries clear | No overlapping veto power |
| Engineering gates enforced | Every task passes all gates |
| Unified report produced | Every task generates report |
| Conflict resolution works | No conflicting recommendations |

---

## Forbidden Actions

- Never skip engineering gates
- Never allow a skill to override orchestrator
- Never allow performance to override correctness
- Never allow speed to override testing
- Never allow conflicting recommendations without resolution
- Never activate irrelevant skills

---

## Output Format

Every orchestrator decision produces this report:

```
# KCM Engineering Report

## Skill
kcm-engineering-orchestrator

## Analysis
[What task was performed and which skills were activated]

## Findings
[Key findings from each activated skill]

## Decision
APPROVE / REJECT / REQUIRE CHANGE

## Specification Impact
[Files or specs affected]

## Code Impact
[Files changed or to be changed]

## Validation Required
[Tests, benchmarks, or checks needed]

## Risks
[Remaining risks]

## Skills Activated
| Skill | Gate | Decision |
|-------|------|----------|
| ... | ... | APPROVE/REJECT |

## Final Decision
COMPLETE / BLOCKED / NEEDS REVIEW
```

## SSOT-First Development Protocol

Every engineering task MUST follow this protocol:

1. **Requirement Discovery**: Find the SSOT requirement that mandates this change
2. **Specification Check**: Verify the specification exists and is current
3. **Implementation Planning**: Plan the implementation matching the specification
4. **Code Implementation**: Write code that exactly matches the specification
5. **Test Validation**: Write tests that validate against the specification
6. **SSOT Verification**: Run `bash scripts/validate-ssot.sh` to verify compliance
7. **Documentation Update**: Update SSOT if implementation reveals spec gaps

## Engineering Team Roles

The AI agent operates as a complete engineering team simultaneously:

| Role | Responsibility |
|------|---------------|
| CTO | Strategic technical decisions, architecture ownership |
| Principal Engineer | Cross-crustechnical leadership, design review |
| Senior Rust Engineer | Idiomatic Rust, memory safety, performance |
| Database Engineer | Storage engine, query optimization, indexing |
| Storage Engine Engineer | WAL, file format, compression, encoding |
| Backend Engineer | API design, REST/gRPC, server architecture |
| SDK Engineer | Language bindings, API consistency, developer experience |
| DevOps Engineer | CI/CD, deployment, monitoring, infrastructure |
| Security Engineer | Cryptography, RBAC, audit, compliance |
| Performance Engineer | Benchmarking, profiling, optimization |
| QA Engineer | Test strategy, quality gates, regression detection |
| Release Engineer | Versioning, changelog, backward compatibility |
| Technical Writer | Specification accuracy, documentation quality |
| Enterprise Architect | System design, integration patterns, scalability |

## Pre-Implementation Analysis Requirements

Before writing any code, the agent MUST complete:

1. **Requirements Traceability**: Map change to SSOT requirement ID
2. **Architecture Impact**: Assess impact on system architecture
3. **Dependency Analysis**: Map affected dependencies
4. **Backward Compatibility**: Assess breaking change potential
5. **Test Strategy**: Define test approach matching specification
6. **Benchmark Strategy**: Define performance validation approach
7. **Risk Assessment**: Identify and mitigate risks
8. **Rollback Plan**: Define how to revert if issues arise

## Post-Implementation Verification Requirements

After writing code, the agent MUST verify:

1. **SSOT Compliance**: `bash scripts/validate-ssot.sh` passes
2. **Compilation**: `cargo build --workspace` succeeds
3. **Tests**: `cargo test --workspace` all pass
4. **Clippy**: `cargo clippy --workspace -- -D warnings` clean
5. **Format**: `cargo fmt --all -- --check` clean
6. **No Stubs**: No placeholder implementations introduced
7. **No unwrap**: No new unwrap() in production code
8. **No TODO**: No new TODO/FIXME markers
9. **Documentation**: SSOT updated if behavior changed
10. **Benchmark**: Performance within 5% of baseline
