---
name: kcm-repository-intelligence
description: Help AI agents understand the complete repository structure before making changes
---

# Skill: Repository Intelligence

> Document ID: KCM-SKILL-016 | Version: 2.0.0 | Status: Active

## Overview

Before any code change, the agent must understand where the change belongs in the repository. This skill provides structured analysis of the KCM codebase to prevent misplaced code, duplicated implementations, and incorrect dependency usage. Codebase Intelligence Analyst role covering repository structure analysis, dependency graph understanding, module ownership identification, existing implementation discovery, and test location mapping.

## Mission

Every change targets the correct crate, no duplicated implementations exist, dependencies flow in correct direction, tests are in the correct location, all 13 crates and their files are accurately mapped.

## Responsibilities

| # | Responsibility | Description |
|---|---------------|-------------|
| 1 | Workspace Structure Analysis | Read root Cargo.toml and map all 13 crates |
| 2 | Crate Map Generation | Identify purpose, public API, and test locations per crate |
| 3 | Dependency Graph Validation | Verify dependency flow: core → storage → compute/reasoning/optimizer/distributed/ml → runtime → interface → server |
| 4 | Module Ownership Mapping | Map every module to its owning crate and responsibility |
| 5 | Existing Implementation Discovery | Search for existing implementations before creating new code |
| 6 | Test Location Mapping | Identify correct test location for each crate |
| 7 | Duplicate Detection | Prevent duplicated implementations across crates |
| 8 | Dependency Direction Enforcement | Ensure dependencies flow in correct direction |

## Authority

| Priority | Authority Level | Blocking Authority | Approval Authority | Escalation |
|----------|----------------|-------------------|-------------------|------------|
| P16 | Codebase Authority | Advisory only (no blocking) | Codebase structure decisions | P1 (Orchestrator) |

## Scope

| In Scope | Out of Scope |
|----------|-------------|
| Repository structure analysis | Writing production code |
| Dependency graph understanding | Writing test code |
| Module ownership identification | Architecture review (P5) |
| Existing implementation discovery | Code quality review (P10) |
| Test location mapping | Performance optimization (P8) |
| Duplicate detection | Security review (P7) |

## Non Goals

1. Implement code
2. Write tests
3. Review architecture
4. Review code quality
5. Optimize performance
6. Review security

## Inputs

| Input | Source | Required |
|-------|--------|----------|
| Root Cargo.toml | Workspace root | Yes |
| Crate lib.rs files | `crates/*/src/lib.rs` | Yes |
| Crate Cargo.toml files | `crates/*/Cargo.toml` | Yes |
| Change description | User request | Yes |

## Outputs

| Output | Format | Destination |
|--------|--------|-------------|
| Repository intelligence report | Markdown | Engineering Report |
| Target crate and module | String | Engineering Report |
| Dependency validation | Boolean | Engineering Report |
| Duplicate detection | List | Engineering Report |
| Test location | File path | Engineering Report |

## Workflow

```
1. Read root Cargo.toml — identify all 13 crates
2. For each crate, identify:
   a. Purpose (from lib.rs module declarations)
   b. Public API surface (from pub exports)
   c. Test locations (tests/ directory or #[cfg(test)] modules)
3. Map dependency graph:
   core → storage → compute/reasoning/optimizer/distributed/ml → runtime → interface → server
4. Map module ownership (every module to its crate)
5. Search for existing implementations before creating new code
6. Identify correct test location for the crate
7. Verify no duplication exists
8. Verify dependency direction is correct
9. Produce repository intelligence report
```

## Decision Process

```
Change Requested
  ↓
Identify target crate:
  ├── Types/DenseVec/Bitmap/Dictionary → kcm-core
  ├── Columns/Codecs/WAL/FileFormat/Index/Backup/Recovery → kcm-storage
  ├── Algebra/SIMD → kcm-compute
  ├── Rules/Inference → kcm-reasoning
  ├── CostModel/Planner/Statistics → kcm-optimizer
  ├── Database/Transactions/Metrics → kcm-runtime
  ├── FFI/REST/KQL/Python → kcm-interface
  ├── Sharding/Coordinator → kcm-distributed
  ├── LearnedIndex/ConfidenceLearner → kcm-ml
  ├── RBAC/Encryption/Audit → kcm-security
  ├── GDPR/Classification → kcm-compliance
  ├── Test infrastructure → kcm-testing
  └── Server binaries → kcm-server
  ↓
Verify dependency direction:
  └── Is the dependency flow correct? → core → storage → ... → server
  ↓
Search for existing implementations
  ├── Found → Use existing implementation
  └── Not found → Implement in correct crate
  ↓
Identify test location:
  └── Tests go in the crate that owns the code
  ↓
Verify no duplication
  ↓
Produce report
```

## Validation

| Check | Method | Pass Criteria |
|-------|--------|--------------|
| Correct crate | Module ownership map | Change targets correct crate |
| No duplication | Search existing implementations | No duplicate found |
| Correct dependencies | Dependency graph check | Dependencies flow in correct direction |
| Public API used correctly | API surface check | Existing APIs used instead of internals |
| Tests located | Test location map | Tests in correct location |
| 13 crates recognized | Workspace analysis | All crates identified |

## Quality Gates

- [ ] All 13 crates recognized and mapped
- [ ] Dependency flow verified: core → storage → compute/reasoning/optimizer/distributed/ml → runtime → interface → server
- [ ] No duplicated implementations exist
- [ ] Public API used correctly (no reaching into internals)
- [ ] Tests in correct location for the crate
- [ ] Existing implementations discovered before creating new code
- [ ] Module ownership mapped for all modules

## Dependencies

| Skill | Dependency Type | Description |
|-------|----------------|-------------|
| kcm-architecture-guardian (P5) | Escalate | Architecture questions escalated |
| kcm-code-quality-guardian (P10) | Coordinate | Code quality validated after placement |
| kcm-engineering-orchestrator (P1) | Escalate | Complex decisions escalated |

## Related Skills

| Skill | Relationship |
|-------|-------------|
| kcm-architecture-guardian (P5) | P5 validates architecture; P16 provides structure |
| kcm-code-quality-guardian (P10) | P16 identifies crate; P10 validates quality |
| kcm-engineering-orchestrator (P1) | P1 coordinates; P16 provides intelligence |
| kcm-testing-verification (P9) | P16 locates test target; P9 writes tests |

## SSOT References

| Document | Section | Relevance |
|----------|---------|-----------|
| AGENTS.md | §6 Repository Structure Rules | Crate map and dependency flow |
| AGENTS.md | §6.2 Dependency Flow | Dependency direction requirements |
| AGENTS.md | §6.3 Dependency Policy | External dependency justification |
| SSOT.md | Crate Structure | Repository structure specification |

## Failure Conditions

| Condition | Impact | Escalation |
|-----------|--------|------------|
| Change targets wrong crate | Architectural violation | Re-identify correct crate |
| Duplicate implementation created | Maintenance burden | Use existing implementation |
| Dependency direction violated | Architecture violation | Fix dependency flow |
| Tests in wrong location | Test discoverability | Move tests to correct crate |
| Existing implementation missed | Code duplication | Search and reuse existing code |

## Escalation

| Level | Path | SLA |
|-------|------|-----|
| Level 1 | Provide intelligence report | Immediate |
| Level 2 | Escalate to arch-guardian (P5) | 4 hours |
| Level 3 | Escalate to Engineering Orchestrator (P1) | 24 hours |
| Level 4 | SSOT.md is the final authority | As needed |

## Examples

See [examples/](./examples/) for repository intelligence examples.

## Checklist

See [checklists/](./checklists/) for repository intelligence checklists.

## References

- [AGENTS.md](../../../AGENTS.md)
- [SSOT.md](../../../SSOT.md)
- [CONTRIBUTING.md](../../../CONTRIBUTING.md)
- [SECURITY.md](../../../SECURITY.md)
