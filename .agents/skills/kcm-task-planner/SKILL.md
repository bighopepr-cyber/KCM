---
name: kcm-task-planner
description: Prevent uncontrolled coding by requiring structured task analysis before implementation
---

# Skill: Task Planner

## Skill Identity

**Purpose:** Prevent uncontrolled coding. Before any implementation, the agent must produce a structured task plan that identifies requirements, affected files, specifications, risks, and testing strategy.

**Role:** Implementation Planner

**Scope:** Task decomposition, requirement analysis, file impact identification, specification mapping, risk assessment, testing strategy.

**Non-responsibility:** Does not implement code. Does not write tests. Does not review architecture (Architecture Guardian). Does not review code quality (Code Quality Guardian). Does not assess impact (Change Impact Analysis).

**Measurable Outcomes:**
- Every task has a structured plan before implementation
- Every plan lists all affected files
- Every plan identifies all relevant specifications
- Every plan has a testing strategy
- Every plan has risk mitigations

---

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

---

## Required Context

Before producing a plan:
1. Read the user's request completely
2. Search the codebase for related existing code
3. Read relevant specification documents
4. Identify all files that will be affected
5. Identify all tests that will need updating

---

## Crate Awareness

The workspace contains **13 crates**. Plans must identify which crates are affected:

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

---

## Operating Rules

1. **No code before plan** — Never write implementation code before producing a task plan
2. **Plan must be specific** — Vague plans like "fix the bug" are not acceptable
3. **Plan must identify files** — List every file that will be modified
4. **Plan must identify specs** — List every specification document relevant to the change
5. **Plan must identify tests** — List every test that needs to be added or modified
6. **Plan must identify risks** — List every risk (compatibility, performance, security)
7. **Plan must identify affected crates** — List every crate in the workspace that will be affected

---

## Validation Checklist

- [ ] Requirement clearly stated
- [ ] All affected files listed
- [ ] All relevant specs identified
- [ ] All affected crates identified (out of 13)
- [ ] Implementation strategy defined
- [ ] Testing strategy defined
- [ ] Risks identified with mitigations

---

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

1. **Identify SSOT Requirements**: Map task to specific SSOT requirements
2. **Verify Specification**: Ensure specification exists and is current
3. **Assess Impact**: Use change impact analysis for affected components
4. **Define Success Criteria**: SSOT compliance is the success criterion
5. **Plan Validation**: Include SSOT validation in the task checklist
