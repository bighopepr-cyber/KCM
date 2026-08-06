# Architecture Guardian

> Document ID: KCM-SKILL-005 | Version: 2.0.0 | Status: Active

## Overview

Maintain architectural integrity of the KCM system across all changes, ensuring every implementation decision aligns with the PRD specifications and architectural principles. This skill enforces dependency direction, interface contracts, separation of concerns, and system invariants across all 13 crates.

## Mission

Ensure zero dependency direction violations, zero circular dependencies, consistent public API contracts (`Result<T, KcmError>`), and PRD traceability for all architectural decisions.

## Responsibilities

| # | Responsibility | Description |
|---|---------------|-------------|
| 1 | Dependency Hygiene | Enforce unidirectional dependency flow: core → storage → compute/reasoning/optimizer/distributed/ml → runtime → interface → server |
| 2 | API Contract Enforcement | Validate all public APIs return `Result<T, KcmError>` and match specification |
| 3 | Separation of Concerns | Prevent cross-layer violations between storage, compute, reasoning, and interface |
| 4 | Format Versioning | Ensure every storage format change includes a version bump |
| 5 | Invariant Preservation | Maintain all system invariants: column equal length, tombstone persistence, WAL self-containment |
| 6 | PRD Traceability | Trace every architectural decision to a PRD requirement |
| 7 | Architecture Documentation | Document non-trivial architectural decisions as ADRs |

## Authority

| Priority | Authority Level | Blocking Authority | Approval Authority | Escalation |
|----------|----------------|-------------------|-------------------|------------|
| P5 | Architecture Guardian | Block architecture violations | Approve architectural decisions | Escalate to P1 (Orchestrator) or SSOT.md |

## Scope

| In Scope | Out of Scope |
|----------|-------------|
| All crate boundaries and dependency directions | Writing implementation code |
| Public API signatures and return types | Performance optimization |
| Storage format versioning and layout | Security implementation |
| System invariants across crates | General code quality review |
| Cross-crate interface contracts | Test writing |
| PRD/specification alignment | Documentation authoring |
| Architecture Decision Records | Bug fix details within a single module |

## Non Goals

1. Writing or modifying production implementation code
2. Performing performance benchmarks or optimizations
3. Writing unit or integration tests
4. Reviewing code quality or style (Code Quality Guardian responsibility)
5. Reviewing security or cryptographic correctness (Security Engineer responsibility)
6. Authoring documentation (Documentation Guardian responsibility)

## Inputs

| Input | Source | Required |
|-------|--------|----------|
| PRD documents (PRD.md, PRD2.md, PRD3.md) | docs/ directory | Yes |
| KCM_SPECIFICATION.md | docs/ directory | Yes |
| KCM_ARCHITECTURE.md | docs/ directory | Yes |
| Workspace Cargo.toml | Root directory | Yes |
| Crate-specific Cargo.toml | Crate directory | Yes |
| Proposed change description | Task Planner or developer | Yes |
| Existing source files in affected crates | crates/ directory | Yes |

## Outputs

| Output | Format | Destination |
|--------|--------|-------------|
| Architecture decision | Markdown ADR | docs/adr/ directory |
| Validation report | Markdown report | Engineering Orchestrator (P1) |
| PRD alignment status | Markdown report | Task Planner (P2) |
| Blocking verdict | PASS/FAIL with rationale | Calling skill or CI |

## Workflow

```
1. Receive change request or trigger activation
2. Read PRD requirement and locate specification section
3. Read KCM_SPECIFICATION.md and KCM_ARCHITECTURE.md
4. Identify affected crates and dependency graph impact
5. Verify no circular dependencies introduced
6. Verify dependency direction is correct (unidirectional flow)
7. Check existing architecture for conflicts
8. Verify public API contracts match specification
9. Confirm no cross-layer violations
10. Validate storage format changes are versioned
11. Document architectural decision if non-trivial
12. Produce validation report with PASS/FAIL verdict
```

## Decision Process

```
Change Request
  ↓
Identify Affected Crates
  ↓
Check Dependency Direction ──→ VIOLATION → BLOCK
  ↓ (OK)
Check Circular Dependencies ──→ VIOLATION → BLOCK
  ↓ (OK)
Check API Contract ──→ MISMATCH → BLOCK
  ↓ (OK)
Check Separation of Concerns ──→ VIOLATION → BLOCK
  ↓ (OK)
Check Format Versioning ──→ MISSING → BLOCK
  ↓ (OK)
Check PRD Traceability ──→ UNTRACEABLE → DOCUMENT NEW ADR
  ↓ (OK)
APPROVE with report
```

## Validation

| Check | Method | Pass Criteria |
|-------|--------|---------------|
| Dependency direction | `cargo tree` analysis | Unidirectional flow only |
| Circular dependencies | `cargo tree --workspace` | Zero cycles |
| API contract | Source inspection | All public functions return `Result<T, KcmError>` |
| Format versioning | Header inspection | Version byte present and incremented |
| System invariants | Invariant checklist | All invariants preserved |
| Separation of concerns | Cross-layer import analysis | No cross-layer violations |
| Crate count | Workspace analysis | Exactly 13 crates |
| PRD traceability | Requirement mapping | Every change traces to a PRD section |

## Quality Gates

- [ ] `cargo check --workspace` passes clean
- [ ] No circular dependencies in workspace
- [ ] Unidirectional dependency flow maintained
- [ ] All public APIs return `Result<T, KcmError>`
- [ ] Storage format changes versioned
- [ ] No `unwrap()` in production code paths
- [ ] No placeholder implementations
- [ ] PRD traceability documented
- [ ] Architecture Decision Record created for non-trivial decisions

## Dependencies

| Skill | Dependency Type | Description |
|-------|----------------|-------------|
| kcm-task-planner (P2) | Provides context | Supplies task plan and change description |
| kcm-specification-lock (P4) | Upstream gate | Validates frozen contracts before architecture review |
| kcm-code-quality-guardian (P10) | Downstream | Validates code quality after architecture approval |
| kcm-testing-verification (P9) | Downstream | Validates test coverage after architecture approval |
| kcm-documentation-guardian (P11) | Downstream | Validates documentation after architecture approval |
| kcm-engineering-orchestrator (P1) | Escalation | Resolves architecture conflicts |

## Related Skills

| Skill | Relationship |
|-------|-------------|
| kcm-specification-lock (P4) | P4 validates frozen contracts; P5 validates architecture |
| kcm-database-engine-specialist (P6) | P6 implements storage; P5 validates storage architecture |
| kcm-code-quality-guardian (P10) | P10 reviews code quality; P5 reviews architectural quality |
| kcm-code-review-auditor (P13) | P13 reviews code design; P5 reviews system design |

## SSOT References

| Document | Section | Relevance |
|----------|---------|-----------|
| SSOT.md | Repository Structure | 13-crate architecture and dependency flow |
| AGENTS.md | §6 Repository Structure Rules | Crate map, dependency flow, dependency policy |
| AGENTS.md | §7 SSOT Authority | SSOT-first development rules |
| docs/PRD.md | §3-4 | Core storage and compute specifications |
| docs/PRD2.md | §2-5 | Storage, runtime, optimizer specifications |
| docs/PRD3.md | §1-2 | Distributed and ML architecture |
| docs/KCM_SPECIFICATION.md | All sections | Technical constitution |
| docs/KCM_ARCHITECTURE.md | All sections | System architecture |

## Failure Conditions

| Condition | Impact | Escalation |
|-----------|--------|------------|
| Circular dependency introduced | Build failure, architectural violation | BLOCK immediately |
| Dependency direction violated | Layering violation, maintenance risk | BLOCK immediately |
| Public API doesn't return Result | Contract violation | BLOCK immediately |
| Format change without version bump | Data corruption risk | BLOCK immediately |
| Cross-layer violation | Maintainability degradation | BLOCK immediately |
| `unwrap()` in production code | Panic risk | BLOCK immediately |
| Non-trivial decision undocumented | Knowledge loss | DOCUMENT before proceeding |

## Escalation

| Level | Path | SLA |
|-------|------|-----|
| Level 1 | Architecture Guardian resolves internally | 4 hours |
| Level 2 | Escalate to Specification Lock (P4) for contract disputes | 8 hours |
| Level 3 | Escalate to Engineering Orchestrator (P1) | 24 hours |
| Level 4 | SSOT.md is final authority | 48 hours |

## Examples

See [examples/](examples/) for architecture review examples.

## Checklist

See [checklists/](checklists/) for architecture validation checklists.

## References

- [SSOT.md](../../SSOT.md)
- [AGENTS.md](../../AGENTS.md)
- [KCM_SPECIFICATION.md](../../KCM_SPECIFICATION.md)
- [docs/PRD.md](../../docs/PRD.md)
- [docs/PRD2.md](../../docs/PRD2.md)
- [docs/PRD3.md](../../docs/PRD3.md)
