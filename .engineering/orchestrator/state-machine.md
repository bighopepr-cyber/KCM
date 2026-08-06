# State Machine

> Document ID: KCM-STATE-001 | Version: 1.0.0

## Overview

The State Machine manages task lifecycle states and transitions.

## States

| State | Code | Description | Color |
|-------|------|-------------|-------|
| NEW | NEW | Task identified | Blue |
| PLANNED | PLANNED | Plan created | Cyan |
| ANALYZED | ANALYZED | Impact analyzed | Yellow |
| APPROVED | APPROVED | All approvals | Green |
| IMPLEMENTING | IMPLEMENTING | Code being written | Orange |
| TESTING | TESTING | Tests running | Purple |
| BENCHMARKING | BENCHMARKING | Benchmarks running | Magenta |
| DOCUMENTING | DOCUMENTING | Docs being updated | Teal |
| VALIDATING | VALIDATING | Quality gates running | Yellow |
| READY | READY | All gates passed | Green |
| COMPLETED | COMPLETED | Task done | Bold Green |
| BLOCKED | BLOCKED | Blocked by skill | Red |
| REJECTED | REJECTED | Rejected by skill | Dark Red |

## Transition Rules

| From | To | Trigger | Actor |
|------|-----|---------|-------|
| NEW | PLANNED | Plan created | P2 |
| NEW | REJECTED | Task rejected | P1/P4/P5/P7 |
| PLANNED | ANALYZED | Impact analyzed | P3 |
| PLANNED | BLOCKED | Plan blocked | P4/P5 |
| ANALYZED | APPROVED | All approvals | All required |
| ANALYZED | BLOCKED | Approval denied | Any approver |
| APPROVED | IMPLEMENTING | Implementation starts | Domain |
| IMPLEMENTING | TESTING | Implementation complete | Domain |
| IMPLEMENTING | BLOCKED | Implementation blocked | P10/P9 |
| TESTING | BENCHMARKING | Performance-related | P9 |
| TESTING | DOCUMENTING | No benchmark needed | P9 |
| BENCHMARKING | DOCUMENTING | Benchmark complete | P8 |
| DOCUMENTING | VALIDATING | Docs updated | P11 |
| VALIDATING | READY | All gates pass | Quality Engine |
| VALIDATING | BLOCKED | Gate failed | Quality Engine |
| READY | COMPLETED | Merge approved | P12/P1 |
| 任何 | BLOCKED | Skill blocks | Any skill |
| 任何 | REJECTED | Skill rejects | Any skill |

## State Record

```markdown
# State Record

**Task:** {{TASK}}

| State | Entry Time | Exit Time | Duration | Actor | Notes |
|-------|-----------|-----------|----------|-------|-------|
| {{STATE}} | {{ENTRY}} | {{EXIT}} | {{DURATION}} | {{ACTOR}} | {{NOTES}} |
```

## State Validation

| Check | Method | Pass Criteria |
|-------|--------|--------------|
| Valid transition | Transition table | Allowed transition |
| Required actor | Actor table | Correct skill |
| SLA met | Timestamp | Within SLA |
| State consistency | State machine | No invalid states |