# KCM Engineering Orchestrator Engine

> Document ID: KCM-ENG-ENGINE-001 | Version: 1.0.0 | Status: Active

## Overview

The Engineering Orchestrator Engine is the master specification for the autonomous engineering execution system. It defines how tasks are analyzed, planned, executed, validated, and completed.

## Core Principles

1. **Deterministic Execution** — Same input always produces same output
2. **SSOT Compliance** — Every decision traces to SSOT
3. **Authority Respect** — No skill overrides another without authority
4. **Full Auditability** — Every decision is recorded
5. **Quality Gates** — No skip, no bypass

## Engine Components

| Component | File | Purpose |
|-----------|------|---------|
| Skill Router | orchestrator/routing.md | Selects skills based on change type |
| Execution Engine | orchestrator/execution-engine.md | Executes engineering workflow |
| Planning Engine | orchestrator/planning-engine.md | Creates execution plans |
| Approval Engine | orchestrator/approval-engine.md | Manages approval chains |
| Conflict Engine | orchestrator/conflict-engine.md | Resolves skill conflicts |
| Escalation Engine | orchestrator/escalation-engine.md | Handles escalations |
| Quality Engine | orchestrator/quality-engine.md | Enforces quality gates |
| Reporting Engine | orchestrator/reporting-engine.md | Generates reports |
| State Machine | orchestrator/state-machine.md | Manages task states |

## Input Processing

### Input Types

| Input | Source | Processing |
|-------|--------|-----------|
| Natural Language | User prompt | Parse → Classify → Route |
| Git Diff | `git diff` | Analyze → Impact → Route |
| Issue | GitHub Issue | Parse → Classify → Route |
| PR | GitHub PR | Analyze → Validate → Approve |
| Bug Report | User report | Classify → Debug → Fix |
| Feature Request | User request | Plan → Implement → Release |
| Spec Change | SSOT update | Validate → Propagate → Update |

### Processing Pipeline

```
Input → Task Analyzer → Skill Router → Planning Engine
  → Approval Engine → Execution Engine → Quality Engine
  → Reporting Engine → Output
```

## Task Classification

### Task Types

| Type | Description | Pipeline |
|------|-------------|----------|
| Feature | New functionality | feature.md |
| Bug Fix | Correct defect | bugfix.md |
| Optimization | Performance improvement | optimization.md |
| Security Patch | Security fix | emergency.md |
| Documentation | Doc update | documentation.md |
| Refactoring | Code restructuring | refactor.md |
| Release | Version release | release.md |
| Emergency | Critical fix | emergency.md |
| Breaking Change | API/format change | feature.md (extended) |
| Research | Investigation | standard.md |
| Prototype | Proof of concept | standard.md |

### Risk Assessment

| Risk Level | Criteria | Approval Required |
|-----------|---------|-------------------|
| Low | Internal, no API change | P10 + P9 |
| Medium | API change, non-breaking | P4 + P5 + P9 + P11 |
| High | Breaking change, security | P4 + P5 + P7 + P9 + P11 + P12 |
| Critical | Production impact | All skills + P1 |

## Execution Model

### Standard Execution

```
1. P16 Repository Intelligence → Understand codebase
2. P2 Task Planner → Create plan
3. P3 Change Impact Analysis → Assess impact
4. P4 Specification Lock → Validate contracts
5. P5 Architecture Guardian → Validate architecture
6. Domain Specialist → Implement
7. P10 Code Quality Guardian → Quality check
8. P9 Testing Verification → Test validation
9. P8 Performance Engineer → Benchmark (if needed)
10. P11 Documentation Guardian → Update docs
11. P13 Code Review Auditor → Review
12. P12 Release Readiness → Release gate
13. P1 Engineering Orchestrator → Final approval
```

### Emergency Execution

```
1. P14 Debugging Root Cause → Root cause
2. P10 Code Quality Guardian → Implement fix
3. P9 Testing Verification → Regression test
4. P12 Release Readiness → Release
5. P1 Engineering Orchestrator → Approval
```

## Quality Gates

### Mandatory Gates

| Gate | Validator | Pass Criteria |
|------|-----------|--------------|
| Format | cargo fmt | Zero diff |
| Lint | cargo clippy | Zero warnings |
| Build | cargo build | Success |
| Unit Tests | cargo test --lib | 100% pass |
| Integration | cargo test --test | 100% pass |
| Property | cargo test property | 100% pass |
| Security | cargo audit | Zero vulnerabilities |
| SSOT | validate-ssot.sh | Pass |
| Coverage | doc-coverage | 100% |
| Documentation | validate-docs.sh | Pass |

### Release Gates

| Gate | Validator | Pass Criteria |
|------|-----------|--------------|
| All CI | CI pipeline | All green |
| No Regression | bench-compare | < 5% |
| SSOT Aligned | ssot-check | Pass |
| Docs Complete | validate-docs | 100% |
| Version Bumped | git log | Yes |
| Changelog Updated | CHANGELOG.md | Updated |

## State Machine

### States

| State | Description | Next States |
|-------|-------------|-------------|
| NEW | Task identified | PLANNED, REJECTED |
| PLANNED | Plan created | ANALYZED, BLOCKED |
| ANALYZED | Impact analyzed | APPROVED, BLOCKED |
| APPROVED | All approvals received | IMPLEMENTING |
| IMPLEMENTING | Code being written | TESTING, BLOCKED |
| TESTING | Tests running | BENCHMARKING, DOCUMENTING |
| BENCHMARKING | Benchmarks running | DOCUMENTING |
| DOCUMENTING | Docs being updated | VALIDATING |
| VALIDATING | Quality gates running | READY, BLOCKED |
| READY | All gates passed | COMPLETED |
| COMPLETED | Task done | — |
| BLOCKED | Blocked by skill | — (with reason) |
| REJECTED | Rejected by skill | — (with reason) |

### State Transitions

```
NEW → PLANNED (P2 creates plan)
PLANNED → ANALYZED (P3 completes impact)
ANALYZED → APPROVED (all approvals received)
APPROVED → IMPLEMENTING (implementation starts)
IMPLEMENTING → TESTING (implementation complete)
TESTING → BENCHMARKING (if performance-related)
TESTING → DOCUMENTING (if no benchmark needed)
BENCHMARKING → DOCUMENTING (benchmark complete)
DOCUMENTING → VALIDATING (docs updated)
VALIDATING → READY (all gates pass)
READY → COMPLETED (merge approved)
任何 → BLOCKED (skill blocks)
任何 → REJECTED (skill rejects)
```

## Reporting

### Report Types

| Report | Trigger | Content |
|--------|---------|---------|
| Execution Report | Task completion | Full execution summary |
| Impact Report | P3 analysis | Change impact matrix |
| Approval Report | Approval received | Approval chain status |
| Quality Report | Gate check | Gate pass/fail status |
| Validation Report | Validation complete | Validation results |
| Completion Report | Task done | Final summary |

### Report Format

```markdown
# KCM Engineering Report

**Task:** {{TASK}}
**Date:** {{DATE}}
**Status:** {{STATUS}}

## Executive Summary
{{SUMMARY}}

## Task Classification
{{CLASSIFICATION}}

## Impact Analysis
{{IMPACT}}

## Execution Pipeline
{{PIPELINE}}

## Quality Gates
{{GATES}}

## Completion Criteria
{{CRITERIA}}
```

## References

- [AGENTS.md](../../AGENTS.md) — Engineering Constitution
- [skills/AUTHORITY-SYSTEM.md](../../skills/AUTHORITY-SYSTEM.md) — Authority system
- [skills/DECISION-MATRIX.md](../../skills/DECISION-MATRIX.md) — Decision matrix
- [skills/WORKFLOW.md](../../skills/WORKFLOW.md) — Workflow definitions
- [SSOT.md](../../SSOT.md) — Single Source of Truth
