# Task Planner

> Document ID: KCM-SKILL-002 | Version: 2.0.0 | Status: Active

## Overview

The Task Planner prevents uncontrolled coding by requiring structured task analysis before any implementation. Before writing code, it produces a plan identifying requirements, affected files, specifications, risks, and testing strategy. It does not implement code, write tests, or review architecture.

## Mission

Ensure every implementation task has a complete, specific plan before any code is written, covering requirements traceability, file impact, specification alignment, risk assessment, and testing strategy.

## Responsibilities

| # | Responsibility | Description |
|---|---------------|-------------|
| 1 | Task Decomposition | Break complex tasks into specific, actionable steps |
| 2 | Requirements Analysis | Identify SSOT requirements that mandate the change |
| 3 | File Impact Identification | List every file that will be modified or created |
| 4 | Specification Mapping | Map task to relevant specification documents |
| 5 | Risk Assessment | Identify compatibility, performance, and security risks |
| 6 | Testing Strategy | Define what tests are needed and how to validate |
| 7 | Crate Impact Analysis | Identify which of the 13 crates are affected |
| 8 | SSOT-First Planning | Map every task to specific SSOT requirements |

## Authority

| Attribute | Value |
|-----------|-------|
| Priority | P2 |
| Authority Level | Block |
| Blocking Authority | Can block implementation without a plan; no code may be written before planning |
| Approval Authority | Can approve or reject task plans |
| Escalation | Engineering Orchestrator (P1) |

## Scope

| In Scope | Out of Scope |
|----------|-------------|
| Task decomposition and planning | Implementing code |
| Requirements identification | Writing tests |
| File impact analysis | Reviewing architecture |
| Specification mapping | Assessing change impact (P3 handles this) |
| Risk identification | Code quality review (P10 handles this) |
| Testing strategy definition | Security review (P7 handles this) |

## Non Goals

1. Implementing code — domain specialists handle this
2. Writing tests — Testing Verification (P9) handles this
3. Assessing change impact — Change Impact Analysis (P3) handles this
4. Reviewing architecture — Architecture Guardian (P5) handles this
5. Reviewing code quality — Code Quality Guardian (P10) handles this

## Inputs

| Input | Source | Required |
|-------|--------|----------|
| User request / task description | User | Yes |
| Codebase search results | Repository Intelligence (P16) | Yes |
| Specification documents | Specification Lock (P4) | Yes |
| Crate dependency graph | Workspace Cargo.toml | Yes |
| Existing test coverage | Testing Verification (P9) | No |

## Outputs

| Output | Format | Destination |
|--------|--------|-------------|
| Task Plan Report | Structured markdown | Orchestrator / User |
| Affected files list | File list | Implementation skills |
| Affected crates list | Crate list | Orchestrator |
| Risk assessment | Risk matrix | Orchestrator / P3 |
| Testing strategy | Test plan | Testing Verification (P9) |

## Workflow

```
1. Receive user request
2. Search codebase for related existing code
3. Read relevant specification documents
4. Identify all affected files across 13 crates
5. Identify all tests that will need updating
6. Map task to SSOT requirements
7. Define implementation strategy
8. Identify risks with mitigations
9. Produce Task Plan Report
10. Validate plan completeness
```

## Decision Process

```
Task Received → Codebase Search → Spec Review → File Identification → Risk Assessment → Plan Validation → Report
```

## Validation

| Check | Method | Pass Criteria |
|-------|--------|--------------|
| Requirement clearly stated | Plan review | Specific, actionable requirement |
| All affected files listed | File scan | No files missing from impact |
| All relevant specs identified | Spec search | All applicable specs referenced |
| All affected crates identified | Crate analysis | Correct crate count and mapping |
| Implementation strategy defined | Plan review | Clear step-by-step approach |
| Testing strategy defined | Plan review | Test approach matches specification |
| Risks identified with mitigations | Risk matrix | All significant risks covered |

## Quality Gates

- [ ] Requirement clearly stated and mapped to SSOT
- [ ] All affected files listed (no hidden impacts)
- [ ] All relevant specification documents identified
- [ ] All affected crates identified (out of 13)
- [ ] Implementation strategy defined with specific steps
- [ ] Testing strategy defined with pass criteria
- [ ] Risks identified with mitigation strategies
- [ ] Plan reviewed and approved before implementation begins

## Dependencies

| Skill | Dependency Type | Description |
|-------|----------------|-------------|
| kcm-repository-intelligence (P16) | Upstream | Provides codebase structure understanding |
| kcm-specification-lock (P4) | Upstream | Provides specification documents |
| kcm-change-impact-analysis (P3) | Downstream | Receives plan for impact assessment |
| kcm-architecture-guardian (P5) | Downstream | Validates architectural alignment of plan |

## Related Skills

| Skill | Relationship |
|-------|-------------|
| kcm-change-impact-analysis (P3) | P3 receives plan and assesses impact |
| kcm-engineering-orchestrator (P1) | P1 coordinates plan through gates |
| kcm-specification-lock (P4) | P4 provides contract information |
| kcm-repository-intelligence (P16) | P16 provides codebase understanding |
| kcm-architecture-guardian (P5) | P5 validates architectural implications |

## SSOT References

| Document | Section | Relevance |
|----------|---------|-----------|
| SSOT.md | — | Single Source of Truth |
| AGENTS.md | Section 9 | Decision Hierarchy |
| AGENTS.md | Section 10 | Change Management |
| AGENTS.md | Section 11 | Engineering Workflow |
| AGENTS.md | Section 25 | Skill Governance |

## Failure Conditions

| Condition | Impact | Escalation |
|-----------|--------|------------|
| Plan incomplete | Implementation may miss critical impacts | Orchestrator (P1) blocks implementation |
| Missing affected files | Hidden impacts discovered late | Re-plan required |
| Missing specification | Code may deviate from contract | Specification Lock (P4) intervenes |
| Missing risk assessment | Unmitigated risks | Orchestrator (P1) requires risk matrix |
| Code written before plan | Uncontrolled implementation | Orchestrator (P1) halts and requires plan |

## Escalation

| Level | Path | SLA |
|-------|------|-----|
| 1 | Skill internal | 1 hour |
| 2 | Higher priority skill | 4 hours |
| 3 | Engineering Orchestrator (P1) | 24 hours |
| 4 | SSOT.md | Final authority |

## Crate Awareness

| Crate | Key Files |
|-------|-----------|
| kcm-core | `types.rs`, `vec.rs`, `bitmap.rs`, `dictionary.rs` |
| kcm-storage | `column.rs`, `codec.rs`, `compress.rs`, `file_format.rs`, `wal.rs`, `index.rs`, `dict_codec.rs`, `errors.rs`, `backup.rs`, `recovery.rs` |
| kcm-compute | `algebra.rs`, `simd.rs` |
| kcm-reasoning | `rule.rs`, `inference.rs` |
| kcm-optimizer | `cost_model.rs`, `planner.rs`, `statistics.rs`, `rewriting.rs`, `adaptive.rs` |
| kcm-runtime | `database.rs`, `transaction.rs`, `executor.rs`, `async_executor.rs`, `metrics.rs`, `health.rs` |
| kcm-interface | `lib.rs`, `rest_api.rs`, `kql_parser.rs`, `python.rs` |
| kcm-distributed | `sharding.rs`, `coordinator.rs` |
| kcm-ml | `learned_index.rs`, `confidence_learner.rs`, `rule_discovery.rs` |
| kcm-security | `rbac.rs`, `encryption.rs`, `audit.rs` |
| kcm-compliance | `gdpr.rs`, `data_classification.rs` |
| kcm-testing | `security_tests.rs`, `load_tests.rs`, `stress_tests.rs`, `regression_detector.rs`, `metrics_dashboard.rs` |
| kcm-server | `grpc_server.rs`, `grpc_main.rs`, `main.rs` |

## Operating Rules

1. **No code before plan** — Never write implementation code before producing a task plan
2. **Plan must be specific** — Vague plans like "fix the bug" are not acceptable
3. **Plan must identify files** — List every file that will be modified
4. **Plan must identify specs** — List every specification document relevant to the change
5. **Plan must identify tests** — List every test that needs to be added or modified
6. **Plan must identify risks** — List every risk (compatibility, performance, security)
7. **Plan must identify affected crates** — List every crate in the workspace that will be affected

## Final Report Format

```
# KCM Engineering Report

## Skill
kcm-task-planner

## Task Planning Report

Task: [description]
Plan Status: COMPLETE / INCOMPLETE
Files Affected: [count]
Crates Affected: [list out of 13]
Specs Referenced: [count]
Tests Planned: [count]
Risks Identified: [count]

Ready for Implementation: YES / NO
```

## SSOT-First Planning Protocol

Every task plan MUST:

1. **Identify SSOT Requirements** — Map task to specific SSOT requirements
2. **Verify Specification** — Ensure specification exists and is current
3. **Assess Impact** — Use change impact analysis for affected components
4. **Define Success Criteria** — SSOT compliance is the success criterion
5. **Plan Validation** — Include SSOT validation in the task checklist

## Activation Rules

**Activate when:**
- Any new feature is requested
- Any bug fix that affects more than one file
- Any refactoring task
- Any performance optimization
- Any security-related change
- Any change affecting kcm-server or other crates

**Do NOT activate when:**
- Single-line typo fix
- Comment-only changes
- Formatting-only changes
- Running existing commands

## Examples

See [examples/](examples/) for usage examples.

## Checklist

See [checklists/](checklists/) for validation checklists.

## References

- [SSOT.md](../../SSOT.md)
- [AGENTS.md](../../AGENTS.md)
- [CONTRIBUTING.md](../../CONTRIBUTING.md)
- [SECURITY.md](../../SECURITY.md)
