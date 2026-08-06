---
name: kcm-debugging-root-cause
description: Perform systematic debugging to find root causes of issues, ensuring fixes are minimal, correct, and prevent recurrence
---

# Skill: Debugging and Root Cause Analysis

> Document ID: KCM-SKILL-014 | Version: 2.0.0 | Status: Active

## Overview

Perform systematic debugging to find root causes of issues, ensuring fixes are minimal, correct, and prevent recurrence. Senior Debugging Engineer role covering bug investigation, root cause analysis, crash analysis, data corruption investigation, and performance regression diagnosis across all 13 crates.

## Mission

Every bug has a documented root cause, every fix is minimal (smallest change that fixes the issue), every fix has a regression test, every fix is verified against all existing tests. Fixes bring implementation back to SSOT compliance.

## Responsibilities

| # | Responsibility | Description |
|---|---------------|-------------|
| 1 | Bug Investigation | Collect evidence, reproduce issues, understand symptoms |
| 2 | Root Cause Analysis | Identify fundamental cause, not just symptoms |
| 3 | Crash Analysis | Investigate panics, segfaults, and unexpected termination |
| 4 | Data Corruption Investigation | Trace data integrity issues to root cause |
| 5 | Performance Regression Diagnosis | Identify cause of performance degradation |
| 6 | Minimal Fix Implementation | Make smallest change that fixes the root cause |
| 7 | Regression Test Writing | Write test that prevents recurrence |
| 8 | Fix Verification | Confirm fix resolves issue without introducing new problems |

## Authority

| Priority | Authority Level | Blocking Authority | Approval Authority | Escalation |
|----------|----------------|-------------------|-------------------|------------|
| P14 | Diagnostic Authority | Advisory only (no blocking) | Root cause analysis decisions | P1 (Orchestrator) |

## Scope

| In Scope | Out of Scope |
|----------|-------------|
| Bug investigation across all 13 crates | Writing new features |
| Root cause analysis | Architecture review (P5) |
| Crash and panic analysis | Performance optimization (P8) |
| Data corruption investigation | Security implementation (P7) |
| Performance regression diagnosis | Code quality review (P10) |
| Minimal fix implementation | Test coverage analysis (P9) |
| Regression test writing | Documentation updates (P11) |

## Non Goals

1. Write new features
2. Review architecture
3. Optimize performance
4. Implement security features
5. Review code quality patterns
6. Update documentation

## Inputs

| Input | Source | Required |
|-------|--------|----------|
| Bug report or failure description | User/CI | Yes |
| Relevant source code | Codebase | Yes |
| Stack trace or error message | CI/logs | Yes |
| Steps to reproduce | Bug report | Yes |
| Expected vs actual behavior | Bug report | Yes |

## Outputs

| Output | Format | Destination |
|--------|--------|-------------|
| Root cause analysis | Markdown | Engineering Report |
| Minimal fix | Code change | Codebase |
| Regression test | Test code | Codebase |
| Prevention recommendations | Markdown | Engineering Report |

## Workflow

```
1. Understand the symptom
   a. What is the expected behavior?
   b. What is the actual behavior?
   c. When does it occur?

2. Collect evidence
   a. Error messages and stack traces
   b. Log output
   c. Reproduction steps
   d. Recent code changes

3. Form hypothesis
   a. What could cause this symptom?
   b. What is the most likely cause?

4. Test hypothesis
   a. Add diagnostic output
   b. Use debugger
   c. Check specific code paths
   d. Narrow down systematically

5. Identify root cause
   a. What is the fundamental issue?
   b. Why did it happen?

6. Implement minimal fix
   a. Fix the root cause, not the symptom
   b. Make the smallest change that fixes the issue
   c. Don't refactor while fixing bugs

7. Verify fix
   a. Confirm issue resolved
   b. Run all existing tests
   c. No new issues introduced

8. Write regression test
   a. Test that prevents recurrence
   b. Test validates SSOT compliance

9. Document
   a. What was the issue?
   b. What was the root cause?
   c. What was the fix?
   d. How to prevent recurrence?
```

## Decision Process

```
Bug Reported
  ↓
Symptom → Evidence Collection → Hypothesis → Root Cause
  ↓
Root cause identified?
  ├── No → Collect more evidence, form new hypothesis
  └── Yes ↓
Implement minimal fix
  ↓
Verify fix:
  ├── Fix resolves issue? → No → Re-investigate
  └── Fix resolves issue? → Yes ↓
Run all existing tests
  ├── Tests fail → Fix introduced regression → Re-investigate
  └── Tests pass ↓
Write regression test
  ↓
Document root cause, fix, and prevention
```

## Validation

| Check | Method | Pass Criteria |
|-------|--------|--------------|
| Root cause identified | Analysis | Fundamental issue documented |
| Fix is minimal | Code review | Smallest change that fixes issue |
| Fix resolves issue | Reproduction test | Issue no longer occurs |
| No new issues introduced | All tests pass | 100% test pass rate |
| Regression test added | Test review | Test prevents recurrence |
| SSOT compliance | Spec comparison | Fix brings implementation back to spec |

## Quality Gates

- [ ] Root cause documented with evidence
- [ ] Fix is minimal (smallest change that fixes the issue)
- [ ] Fix resolves the reported issue
- [ ] All existing tests still pass
- [ ] Regression test added for the specific scenario
- [ ] Fix verified against SSOT specification
- [ ] Prevention recommendations documented

## Dependencies

| Skill | Dependency Type | Description |
|-------|----------------|-------------|
| kcm-testing-verification (P9) | Coordinate | P9 validates fix with regression test |
| kcm-code-quality-guardian (P10) | Coordinate | P10 ensures fix quality |
| kcm-architecture-guardian (P5) | Escalate | Architecture questions escalated |
| kcm-specification-lock (P4) | Escalate | Specification questions escalated |

## Related Skills

| Skill | Relationship |
|-------|-------------|
| kcm-testing-verification (P9) | P14 finds root cause; P9 writes regression test |
| kcm-code-quality-guardian (P10) | P14 fixes bug; P10 validates fix quality |
| kcm-release-readiness (P12) | P12 gates release; P14 ensures bug is fixed |
| kcm-engineering-orchestrator (P1) | P1 coordinates; P14 provides diagnostic analysis |

## SSOT References

| Document | Section | Relevance |
|----------|---------|-----------|
| AGENTS.md | §11.2 Emergency Workflow | Emergency debugging process |
| SSOT.md | Implementation Specification | Expected behavior reference |
| SSOT.md | Error Code Enum | Error handling validation |
| docs/KCM_ENGINEERING_RULES.md | Code Standards | Fix quality requirements |

## Failure Conditions

| Condition | Impact | Escalation |
|-----------|--------|------------|
| Root cause not identified | Cannot fix | Collect more evidence |
| Fix introduces regression | Blocks merge | Re-investigate root cause |
| Fix is not minimal | Blocks merge | Simplify fix |
| Regression test missing | Blocks merge | Write regression test |
| Fix deviates from SSOT | Blocks merge | Align fix with specification |

## Escalation

| Level | Path | SLA |
|-------|------|-----|
| Level 1 | Investigate internally | Immediate |
| Level 2 | Escalate to domain specialist (P6/P7/P8) | 4 hours |
| Level 3 | Escalate to Engineering Orchestrator (P1) | 24 hours |
| Level 4 | SSOT.md is the final authority | As needed |

## Examples

See [examples/](./examples/) for debugging implementation examples.

## Checklist

See [checklists/](./checklists/) for debugging validation checklists.

## References

- [AGENTS.md](../../../AGENTS.md)
- [SSOT.md](../../../SSOT.md)
- [CONTRIBUTING.md](../../../CONTRIBUTING.md)
- [SECURITY.md](../../../SECURITY.md)
