# KCM Engineering Workflow

> Document ID: KCM-WORKFLOW-001 | Version: 2.0.0 | Status: Active

## Overview

The KCM Engineering Workflow defines the standard process for all engineering activities, from task identification to release. Every change must follow this workflow.

## Standard Workflow

```
Task Identified
  ↓
1. Repository Intelligence (P16)
   → Understand codebase
   → Map affected modules
   → Identify existing implementations
  ↓
2. Task Planning (P2)
   → Decompose task
   → Identify required skills
   → Create implementation plan
  ↓
3. Impact Analysis (P3)
   → Assess direct impact
   → Assess indirect impact
   → Identify specification updates
   → Identify test updates
  ↓
4. Specification Validation (P4)
   → Validate no frozen contract violations
   → Validate SSOT alignment
   → Approve spec changes
  ↓
5. Architecture Validation (P5)
   → Validate dependency direction
   → Validate separation of concerns
   → Validate interface stability
  ↓
6. Implementation (Domain Specialist)
   → Implement changes
   → Follow coding standards
   → Write tests
  ↓
7. Code Quality (P10)
   → Validate no unwrap/panic/TODO
   → Validate error handling
   → Validate naming
   → Validate complexity
  ↓
8. Testing (P9)
   → Run all tests
   → Validate coverage
   → Validate test quality
  ↓
9. Benchmark (P8) — if performance-related
   → Run benchmarks
   → Compare against baseline
   → Validate no regression
  ↓
10. Documentation (P11)
    → Update README
    → Update spesifikasi
    → Update SSOT traceability
  ↓
11. Code Review (P13)
    → Review for risks
    → Classify findings
    → Provide recommendations
  ↓
12. Release Readiness (P12)
    → Validate build
    → Validate tests
    → Validate quality
    → Validate security
    → Validate documentation
  ↓
13. Final Coordination (P1)
    → Unified report
    → Final approval
    → Merge
```

## Emergency Workflow (Critical Bugs)

```
Bug Report
  ↓
1. Debugging Root Cause (P14)
   → Root cause analysis
   → Identify minimal fix
  ↓
2. Code Quality (P10)
   → Implement fix
   → Validate quality
  ↓
3. Testing (P9)
   → Regression test
   → Validate all tests pass
  ↓
4. Release Readiness (P12)
   → Validate release
  ↓
5. Engineering Orchestrator (P1)
   → Final approval
   → Emergency release
```

## Security Workflow

```
Security Issue
  ↓
1. Security Engineer (P7)
   → Assess severity
   → Identify fix
  ↓
2. Specification Lock (P4)
   → Validate contract impact
  ↓
3. Implementation
   → Implement fix
  ↓
4. Security Testing
   → Validate fix
   → Test attack surface
  ↓
5. Release Readiness (P12)
   → Validate release
  ↓
6. Engineering Orchestrator (P1)
   → Final approval
   → Security release
```

## Skill Interaction Flow

```
P16 (Intelligence) → P2 (Planning) → P3 (Impact) → P4 (Spec) → P5 (Arch)
  → Domain Specialist → P10 (Quality) → P9 (Testing) → P8 (Performance)
  → P11 (Documentation) → P13 (Review) → P12 (Release) → P1 (Orchestrator)
```
