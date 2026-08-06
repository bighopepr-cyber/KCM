# KCM Engineering Orchestrator Engine

> Document ID: KCM-ENG-ENGINE-001 | Version: 2.0.0 | Status: Active

## Overview

The Engineering Orchestrator Engine is the master specification for the autonomous engineering execution system. It defines how tasks are analyzed, planned, executed, validated, and completed — deterministically, auditably, and in compliance with AGENTS.md.

## Core Principles

| # | Principle | Description |
|---|-----------|-------------|
| 1 | **Deterministic Execution** | Same input always produces same output |
| 2 | **SSOT Compliance** | Every decision traces to SSOT |
| 3 | **Authority Respect** | No skill overrides another without authority |
| 4 | **Full Auditability** | Every decision is recorded |
| 5 | **Quality Gates** | No skip, no bypass |
| 6 | **Zero Trust** | Every input validated, every output verified |
| 7 | **Minimal Dependencies** | Every dependency justified |
| 8 | **Backward Compatibility** | Breaking changes require approval |

## Engine Components

| Component | File | Purpose | Authority |
|-----------|------|---------|-----------|
| Skill Router | `orchestrator/routing.md` | Selects skills based on change type | — |
| Execution Engine | `orchestrator/execution-engine.md` | Executes engineering workflow | — |
| Planning Engine | `orchestrator/planning-engine.md` | Creates execution plans | P2 |
| Approval Engine | `orchestrator/approval-engine.md` | Manages approval chains | P1 |
| Conflict Engine | `orchestrator/conflict-engine.md` | Resolves skill conflicts | P1 |
| Escalation Engine | `orchestrator/escalation-engine.md` | Handles escalations | P1 |
| Quality Engine | `orchestrator/quality-engine.md` | Enforces quality gates | P10 |
| Reporting Engine | `orchestrator/reporting-engine.md` | Generates reports | — |
| State Machine | `orchestrator/state-machine.md` | Manages task states | — |
| Documentation Engine | `orchestrator/documentation-engine.md` | Manages doc updates | P11 |

## Input Processing

### Input Types

| Input | Source | Processing Pipeline |
|-------|--------|-------------------|
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

| Type | Description | Pipeline | Duration |
|------|-------------|----------|----------|
| Feature | New functionality | `feature.md` | 4-12 hours |
| Bug Fix | Correct defect | `bugfix.md` | 1-4 hours |
| Optimization | Performance improvement | `optimization.md` | 2-8 hours |
| Security Patch | Security fix | `emergency.md` | 2-8 hours |
| Documentation | Doc update | `documentation.md` | 1-2 hours |
| Refactoring | Code restructuring | `refactor.md` | 2-8 hours |
| Release | Version release | `release.md` | 1-2 hours |
| Emergency | Critical fix | `emergency.md` | 1-4 hours |
| Breaking Change | API/format change | `feature.md` (extended) | 8-24 hours |
| Research | Investigation | `standard.md` | 2-8 hours |
| Prototype | Proof of concept | `standard.md` | 4-16 hours |

### Risk Assessment

| Risk Level | Criteria | Approval Required | SLA |
|-----------|---------|-------------------|-----|
| Low | Internal, no API change | P10 + P9 | 12 hours |
| Medium | API change, non-breaking | P4 + P5 + P9 + P11 | 24 hours |
| High | Breaking change, security | P4 + P5 + P7 + P9 + P11 + P12 | 24 hours |
| Critical | Production impact | All skills + P1 | 4 hours |

## Execution Model

### Standard Execution (13 phases)

```
1.  P16 Repository Intelligence → Understand codebase
2.  P2 Task Planner → Create plan
3.  P3 Change Impact Analysis → Assess impact
4.  P4 Specification Lock → Validate contracts
5.  P5 Architecture Guardian → Validate architecture
6.  Domain Specialist → Implement
7.  P10 Code Quality Guardian → Quality check
8.  P9 Testing Verification → Test validation
9.  P8 Performance Engineer → Benchmark (if needed)
10. P11 Documentation Guardian → Update docs
11. P13 Code Review Auditor → Review
12. P12 Release Readiness → Release gate
13. P1 Engineering Orchestrator → Final approval
```

### Emergency Execution (5 phases)

```
1. P14 Debugging Root Cause → Root cause
2. P10 Code Quality Guardian → Implement fix
3. P9 Testing Verification → Regression test
4. P12 Release Readiness → Release
5. P1 Engineering Orchestrator → Approval
```

### Security Execution (6 phases)

```
1. P7 Security Engineer → Assess severity
2. P4 Specification Lock → Validate contracts
3. Implementation → Implement fix
4. Security Testing → Validate fix
5. P12 Release Readiness → Release
6. P1 Engineering Orchestrator → Approval
```

## Quality Gates

### Mandatory Gates (10)

| Gate | Validator | Pass Criteria | Blocking |
|------|-----------|--------------|----------|
| Format | `cargo fmt --check` | Zero diff | Yes |
| Lint | `cargo clippy -- -D warnings` | Zero warnings | Yes |
| Build | `cargo build --workspace` | Success | Yes |
| Unit Tests | `cargo test --lib` | 100% pass | Yes |
| Integration | `cargo test --test` | 100% pass | Yes |
| Property Tests | `cargo test property` | 100% pass | Yes |
| Security Audit | `cargo audit` | Zero vulnerabilities | Yes |
| SSOT Validation | `validate-ssot.sh` | Pass | Yes |
| Doc Coverage | `calculate-coverage.sh` | 100% | Yes |
| Doc Validation | `validate-docs.sh` | Pass | Yes |

### Conditional Gates (5)

| Gate | Condition | Validator | Pass Criteria |
|------|-----------|-----------|--------------|
| Benchmark | Performance change | `bench-compare.py` | < 5% regression |
| FFI Safety | FFI change | Manual review | All checks pass |
| SDK Consistency | SDK change | `validate-sdk-api.sh` | All SDKs pass |
| Storage Format | Format change | Roundtrip test | Pass |
| Version Sync | Version change | `verify-version.sh` | Pass |

### Release Gates (6)

| Gate | Validator | Pass Criteria |
|------|-----------|--------------|
| All CI | CI pipeline | All green |
| No Regression | `bench-compare.py` | < 5% |
| SSOT Aligned | `validate-ssot.sh` | Pass |
| Docs Complete | `validate-docs.sh` | 100% |
| Version Bumped | `git log` | Yes |
| Changelog | `CHANGELOG.md` | Updated |

## State Machine

### States (13)

| State | Description | Next States |
|-------|-------------|-------------|
| NEW | Task identified | PLANNED, REJECTED |
| PLANNED | Plan created | ANALYZED, BLOCKED |
| ANALYZED | Impact analyzed | APPROVED, BLOCKED |
| APPROVED | All approvals | IMPLEMENTING |
| IMPLEMENTING | Code being written | TESTING, BLOCKED |
| TESTING | Tests running | BENCHMARKING, DOCUMENTING |
| BENCHMARKING | Benchmarks running | DOCUMENTING |
| DOCUMENTING | Docs being updated | VALIDATING |
| VALIDATING | Quality gates running | READY, BLOCKED |
| READY | All gates passed | COMPLETED |
| COMPLETED | Task done | — |
| BLOCKED | Blocked by skill | — (with reason) |
| REJECTED | Rejected by skill | — (with reason) |

### Valid Transitions

```
NEW → PLANNED (P2 creates plan)
NEW → REJECTED (skill rejects)
PLANNED → ANALYZED (P3 completes impact)
PLANNED → BLOCKED (skill blocks)
ANALYZED → APPROVED (all approvals)
ANALYZED → BLOCKED (approval denied)
APPROVED → IMPLEMENTING (implementation starts)
IMPLEMENTING → TESTING (implementation complete)
IMPLEMENTING → BLOCKED (implementation blocked)
TESTING → BENCHMARKING (if performance-related)
TESTING → DOCUMENTING (if no benchmark needed)
BENCHMARKING → DOCUMENTING (benchmark complete)
DOCUMENTING → VALIDATING (docs updated)
VALIDATING → READY (all gates pass)
VALIDATING → BLOCKED (gate failed)
READY → COMPLETED (merge approved)
```

## Reporting

### Report Types

| Report | Trigger | Content |
|--------|---------|---------|
| Executive Summary | Task completion | Full execution summary |
| Task Classification | Task start | Type, risk, pipeline, skills |
| Impact Analysis | P3 analysis | Change impact matrix |
| Approval Report | Approval received | Approval chain status |
| Quality Report | Gate check | Gate pass/fail status |
| Completion Report | Task done | Final summary |

### Report Storage

All reports stored in `.engineering/examples/` with naming:
```
report-{{TASK_ID}}-{{TYPE}}-{{DATE}}.md
```

## References

- [AGENTS.md](../../AGENTS.md) — Engineering Constitution
- [skills/AUTHORITY-SYSTEM.md](../../skills/AUTHORITY-SYSTEM.md) — Authority system
- [skills/DECISION-MATRIX.md](../../skills/DECISION-MATRIX.md) — Decision matrix
- [skills/WORKFLOW.md](../../skills/WORKFLOW.md) — Workflow definitions
- [SSOT.md](../../SSOT.md) — Single Source of Truth
