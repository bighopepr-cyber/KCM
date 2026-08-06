# Engineering Orchestrator

> Document ID: KCM-SKILL-001 | Version: 2.0.0 | Status: Active

## Overview

The Engineering Orchestrator is the single coordination authority for the KCM engineering skill system. It decides which skills activate, enforces priority order, resolves conflicts between skills, and ensures every code change follows the 6-gate engineering pipeline. It does not implement code, write tests, or review code — it delegates to specialist skills for domain-specific work.

## Mission

Coordinate all 16 engineering skills through 6 mandatory gates, enforce governance rules, resolve skill conflicts with documented rationale, and produce unified engineering reports for every task.

## Responsibilities

| # | Responsibility | Description |
|---|---------------|-------------|
| 1 | Gate Enforcement | Ensure every task passes through all 6 engineering gates |
| 2 | Skill Activation | Decide which skills activate based on change type |
| 3 | Conflict Resolution | Resolve conflicts between skills with documented rationale |
| 4 | Authority Enforcement | Prevent skills from overriding orchestrator decisions |
| 5 | Unified Reporting | Produce structured Engineering Reports for every task |
| 6 | SSOT Protocol | Enforce SSOT-First development across all skills |
| 7 | Pre-Implementation Analysis | Require requirements traceability, architecture impact, dependency analysis before coding |
| 8 | Post-Implementation Verification | Verify compilation, tests, clippy, fmt, SSOT compliance after coding |

## Authority

| Attribute | Value |
|-----------|-------|
| Priority | P1 |
| Authority Level | Override |
| Blocking Authority | Can override any skill decision; final arbiter in all conflicts |
| Approval Authority | Can approve or reject any engineering task |
| Escalation | SSOT.md is the final authority |

## Scope

| In Scope | Out of Scope |
|----------|-------------|
| All 16 engineering skills | Implementing code |
| All 6 engineering gates | Writing tests |
| Conflict resolution between skills | Reviewing code quality |
| Unified engineering reports | Designing internal algorithms |
| SSOT-First protocol enforcement | Domain-specific decisions |
| Cross-crate coordination | Single-file bug fixes (no orchestrator needed) |

## Non Goals

1. Implementing code or writing tests — domain specialists handle this
2. Reviewing code quality — Code Quality Guardian (P10) handles this
3. Designing algorithms — Database Engine Specialist (P6) handles this
4. Making architecture decisions — Architecture Guardian (P5) handles this
5. Assessing change impact — Change Impact Analysis (P3) handles this

## Inputs

| Input | Source | Required |
|-------|--------|----------|
| User request / task description | User | Yes |
| Affected file list | Repository Intelligence (P16) | Yes |
| Relevant specification documents | Specification Lock (P4) | Yes |
| Current codebase state | Repository Intelligence (P16) | Yes |
| Engineering gate reports | Specialist skills | Yes |

## Outputs

| Output | Format | Destination |
|--------|--------|-------------|
| Engineering Report | Structured markdown | User / Reviewers |
| Gate pass/fail status | Checklist | CI pipeline |
| Conflict resolution decisions | Documented rationale | Engineering Record |
| Skill activation decisions | Skill registry | All skills |

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

## Decision Process

```
Task Received → Skill Registry Check → Gate Assignment → Skill Activation → Conflict Resolution → Unified Report
```

## Validation

| Check | Method | Pass Criteria |
|-------|--------|--------------|
| All 16 skills registered | Registry table | All entries present and authoritative |
| All 13 crates recognized | Crate map | Count correct, dependency flow mapped |
| Authority boundaries clear | Boundary matrix | No overlapping veto power |
| Engineering gates enforced | Gate checklist | Every task passes all required gates |
| Unified report produced | Report template | Every task generates complete report |
| Conflict resolution works | Conflict log | No unresolved conflicting recommendations |

## Quality Gates

- [ ] All 6 engineering gates passed for every task
- [ ] Every skill produces a structured Engineering Report
- [ ] Conflicts resolved with documented rationale
- [ ] No skill overrides orchestrator decisions
- [ ] SSOT-First protocol enforced
- [ ] Pre-implementation analysis completed
- [ ] Post-implementation verification completed

## Dependencies

| Skill | Dependency Type | Description |
|-------|----------------|-------------|
| kcm-repository-intelligence (P16) | Upstream | Provides codebase understanding for Gate 1 |
| kcm-specification-lock (P4) | Upstream | Validates contracts for Gate 2 |
| kcm-architecture-guardian (P5) | Upstream | Validates architecture for Gate 2 |
| kcm-task-planner (P2) | Upstream | Plans implementation for Gate 3 |
| kcm-change-impact-analysis (P3) | Upstream | Assesses impact for Gate 3 |
| kcm-code-quality-guardian (P10) | Upstream | Validates code quality for Gate 5 |
| kcm-testing-verification (P9) | Upstream | Validates tests for Gate 5 |
| kcm-release-readiness (P12) | Upstream | Validates release readiness for Gate 6 |

## Related Skills

| Skill | Relationship |
|-------|-------------|
| kcm-task-planner (P2) | Delegates planning to this skill |
| kcm-change-impact-analysis (P3) | Delegates impact assessment to this skill |
| kcm-specification-lock (P4) | Delegates contract validation to this skill |
| kcm-architecture-guardian (P5) | Delegates architecture validation to this skill |
| kcm-engineering-decision-record (P15) | Delegates decision capture to this skill |

## SSOT References

| Document | Section | Relevance |
|----------|---------|-----------|
| SSOT.md | — | Single Source of Truth |
| AGENTS.md | Section 9 | Decision Hierarchy |
| AGENTS.md | Section 10 | Change Management |
| AGENTS.md | Section 11 | Engineering Workflow |
| AGENTS.md | Section 21 | AI Agent Behaviour |
| AGENTS.md | Section 25 | Skill Governance |

## Failure Conditions

| Condition | Impact | Escalation |
|-----------|--------|------------|
| Skill conflict unresolved | Task blocked | Engineering Orchestrator decides (P1) |
| Gate bypassed | Quality risk | Orchestrator rejects task completion |
| Skill overrides orchestrator | Authority violation | SSOT.md is final authority |
| Incorrect skill activation | Domain mismatch | Orchestrator reassigns skill |
| Report not produced | Audit gap | Task cannot be marked complete |

## Escalation

| Level | Path | SLA |
|-------|------|-----|
| 1 | Skill internal | 1 hour |
| 2 | Higher priority skill | 4 hours |
| 3 | Engineering Orchestrator (P1) | 24 hours |
| 4 | SSOT.md | Final authority |

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

## Engineering Gates

Every task must pass through 6 mandatory gates:

### Gate 1 — Repository Understanding
**Required skill:** kcm-repository-intelligence
- Understand crate structure (13 crates)
- Identify affected modules
- Map dependency relationships

### Gate 2 — Specification Validation
**Required skills:** kcm-specification-lock, kcm-architecture-guardian
- Frozen contracts identified
- Format compatibility confirmed
- Architecture alignment verified

### Gate 3 — Implementation Planning
**Required skills:** kcm-task-planner, kcm-change-impact-analysis
- Implementation strategy defined
- Impact assessment complete
- Risks identified with mitigations

### Gate 4 — Implementation Validation
**Required skills:** kcm-code-quality-guardian, kcm-testing-verification
- No placeholders or stubs
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

### Gate 6 — Production Readiness
**Required skill:** kcm-release-readiness
- `cargo build --release` passes
- `cargo test --workspace` all pass
- `cargo clippy --workspace -- -D warnings` clean
- `cargo fmt --all -- --check` clean
- No performance regression > 5%

## Authority Boundaries

### Specification Lock (P4) vs Database Engine Specialist (P6)
- **P4 owns:** Binary format, WAL format, API contracts, FFI, gRPC proto, error codes, schema evolution
- **P6 owns:** Storage algorithms, query execution, indexing, compression, transactions, recovery
- **Resolution:** P4 decides IF (contract compliance). P6 decides HOW (algorithmic correctness). P6 needs P4 approval for contract changes.

### Architecture Guardian (P5) vs Specification Lock (P4)
- **P4 owns:** Frozen data/protocol specifications
- **P5 owns:** System architecture, dependency boundaries, module responsibilities
- **Resolution:** P4 has higher priority. Format changes require P4 approval first.

### Code Quality Guardian (P10) vs Code Review Auditor (P13)
- **P10:** Automated prevention — runs FIRST
- **P13:** Senior engineer review — runs AFTER

## SSOT-First Development Protocol

Every engineering task MUST follow:

1. **Requirement Discovery** — Find the SSOT requirement that mandates this change
2. **Specification Check** — Verify the specification exists and is current
3. **Implementation Planning** — Plan the implementation matching the specification
4. **Code Implementation** — Write code that exactly matches the specification
5. **Test Validation** — Write tests that validate against the specification
6. **SSOT Verification** — Run `bash scripts/validate-ssot.sh`
7. **Documentation Update** — Update SSOT if implementation reveals spec gaps

## Pre-Implementation Analysis Requirements

1. Requirements Traceability — Map change to SSOT requirement ID
2. Architecture Impact — Assess impact on system architecture
3. Dependency Analysis — Map affected dependencies
4. Backward Compatibility — Assess breaking change potential
5. Test Strategy — Define test approach matching specification
6. Benchmark Strategy — Define performance validation approach
7. Risk Assessment — Identify and mitigate risks
8. Rollback Plan — Define how to revert if issues arise

## Post-Implementation Verification Requirements

1. SSOT Compliance — `bash scripts/validate-ssot.sh` passes
2. Compilation — `cargo build --workspace` succeeds
3. Tests — `cargo test --workspace` all pass
4. Clippy — `cargo clippy --workspace -- -D warnings` clean
5. Format — `cargo fmt --all -- --check` clean
6. No Stubs — No placeholder implementations introduced
7. No unwrap — No new unwrap() in production code
8. No TODO — No new TODO/FIXME markers
9. Documentation — SSOT updated if behavior changed
10. Benchmark — Performance within 5% of baseline

## Conflict Resolution

When skills disagree:

1. **Higher priority wins** — P4 overrides P6
2. **Domain authority wins** — Within same priority, domain expertise wins
3. **Engineering priority wins** — Correctness > Specification > Data Integrity > Security > Reliability > Performance > Maintainability > Speed
4. **Orchestrator is final** — If unresolved, orchestrator decides

## Forbidden Actions

- Never skip engineering gates
- Never allow a skill to override orchestrator
- Never allow performance to override correctness
- Never allow speed to override testing
- Never allow conflicting recommendations without resolution
- Never activate irrelevant skills

## Examples

See [examples/](examples/) for usage examples.

## Checklist

See [checklists/](checklists/) for validation checklists.

## References

- [SSOT.md](../../SSOT.md)
- [AGENTS.md](../../AGENTS.md)
- [CONTRIBUTING.md](../../CONTRIBUTING.md)
- [SECURITY.md](../../SECURITY.md)
