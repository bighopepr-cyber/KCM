# State Machine

> Document ID: KCM-STATE-001 | Version: 2.0.0 | Status: Active

## Overview

The State Machine manages task lifecycle states and transitions. It ensures deterministic state progression and prevents invalid state transitions.

## States

| State | Code | Description | Color | Blocking |
|-------|------|-------------|-------|----------|
| NEW | NEW | Task identified | Blue | No |
| PLANNED | PLANNED | Plan created | Cyan | No |
| ANALYZED | ANALYZED | Impact analyzed | Yellow | No |
| APPROVED | APPROVED | All approvals received | Green | No |
| IMPLEMENTING | IMPLEMENTING | Code being written | Orange | No |
| TESTING | TESTING | Tests running | Purple | No |
| BENCHMARKING | BENCHMARKING | Benchmarks running | Magenta | No |
| DOCUMENTING | DOCUMENTING | Docs being updated | Teal | No |
| VALIDATING | VALIDATING | Quality gates running | Yellow | No |
| READY | READY | All gates passed | Green | No |
| COMPLETED | COMPLETED | Task done | Bold Green | No |
| BLOCKED | BLOCKED | Blocked by skill | Red | Yes |
| REJECTED | REJECTED | Rejected by skill | Dark Red | Yes |

## Valid Transitions

| From | To | Trigger | Actor | SLA |
|------|-----|---------|-------|-----|
| NEW | PLANNED | Plan created | P2 | 30 min |
| NEW | REJECTED | Task rejected | P1/P4/P5/P7 | — |
| PLANNED | ANALYZED | Impact analyzed | P3 | 30 min |
| PLANNED | BLOCKED | Plan blocked | P4/P5 | — |
| ANALYZED | APPROVED | All approvals received | All required | 24 hr |
| ANALYZED | BLOCKED | Approval denied | Any approver | — |
| APPROVED | IMPLEMENTING | Implementation starts | Domain | — |
| IMPLEMENTING | TESTING | Implementation complete | Domain | 2-8 hr |
| IMPLEMENTING | BLOCKED | Implementation blocked | P10/P9 | — |
| TESTING | BENCHMARKING | Performance-related | P9 | 30 min |
| TESTING | DOCUMENTING | No benchmark needed | P9 | — |
| BENCHMARKING | DOCUMENTING | Benchmark complete | P8 | 30 min |
| DOCUMENTING | VALIDATING | Docs updated | P11 | 30 min |
| VALIDATING | READY | All gates pass | Quality Engine | 15 min |
| VALIDATING | BLOCKED | Gate failed | Quality Engine | — |
| READY | COMPLETED | Merge approved | P12/P1 | — |
| BLOCKED | PLANNED | Block resolved | P2 | — |
| BLOCKED | REJECTED | Block cannot be resolved | P1 | — |
| REJECTED | NEW | Task redefined | P2 | — |

## Invalid Transitions

The following transitions are INVALID and must NOT occur:

| From | To | Reason |
|------|-----|--------|
| NEW | IMPLEMENTING | Must be planned first |
| NEW | TESTING | Must be implemented first |
| PLANNED | IMPLEMENTING | Must be analyzed first |
| ANALYZED | IMPLEMENTING | Must be approved first |
| IMPLEMENTING | READY | Must be tested first |
| TESTING | COMPLETED | Must be validated first |
| Any | COMPLETED | Must go through READY |

## State Machine Diagram

```
                    ┌─────────┐
                    │   NEW   │
                    └────┬────┘
                         │
              ┌──────────┼──────────┐
              ↓          ↓          ↓
        ┌─────────┐ ┌─────────┐ ┌──────────┐
        │PLANNED  │ │ REJECTED│ │ BLOCKED  │
        └────┬────┘ └─────────┘ └──────────┘
             │                    ↑
             ↓                    │
        ┌─────────┐              │
        │ANALYZED │──────────────┘
        └────┬────┘
             │
        ┌────┼────┐
        ↓         ↓
  ┌──────────┐ ┌──────────┐
  │ APPROVED │ │ BLOCKED  │
  └────┬─────┘ └──────────┘
       │
       ↓
  ┌──────────────┐
  │IMPLEMENTING  │
  └────┬─────────┘
       │
  ┌────┼────┐
  ↓         ↓
┌────────┐ ┌──────────────┐
│TESTING │ │  BLOCKED     │
└───┬────┘ └──────────────┘
    │
    ├──────────────┐
    ↓              ↓
┌────────────┐ ┌──────────────┐
│BENCHMARKING│ │ DOCUMENTING  │
└─────┬──────┘ └──────┬───────┘
      │               │
      └───────┬───────┘
              ↓
        ┌──────────┐
        │VALIDATING│
        └────┬─────┘
             │
        ┌────┼────┐
        ↓         ↓
  ┌──────────┐ ┌──────────┐
  │  READY   │ │ BLOCKED  │
  └────┬─────┘ └──────────┘
       │
       ↓
  ┌──────────┐
  │COMPLETED │
  └──────────┘
```

## State Record Format

```markdown
# State Record

**Task ID:** {{TASK_ID}}
**Task:** {{TASK}}

## State History
| # | State | Entry Time | Exit Time | Duration | Actor | Notes |
|---|-------|-----------|-----------|----------|-------|-------|
| 1 | NEW | {{TIME}} | {{TIME}} | {{DURATION}} | {{ACTOR}} | {{NOTES}} |
| 2 | PLANNED | {{TIME}} | {{TIME}} | {{DURATION}} | {{ACTOR}} | {{NOTES}} |

## Current State
- **State:** {{STATE}}
- **Entered:** {{TIME}}
- **Duration:** {{DURATION}}
- **Actor:** {{ACTOR}}
```

## State Validation

| Check | Method | Pass Criteria |
|-------|--------|--------------|
| Valid transition | Transition table | Allowed transition |
| Required actor | Actor table | Correct skill |
| SLA met | Timestamp | Within SLA |
| State consistency | State machine | No invalid states |
| No cycles | Graph analysis | Acyclic path |
| Terminal state reached | State check | COMPLETED or REJECTED |
