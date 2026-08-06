# KCM Decision Matrix

> Document ID: KCM-DECISION-001 | Version: 2.0.0 | Status: Active

## Overview

The Decision Matrix defines which skills are involved in each type of change, their roles, and the approval requirements.

## Change Type Decision Matrices

### Storage Changes

```
Storage Change
  → P5 Architecture Guardian (validate architecture)
  → P4 Specification Lock (validate contracts)
  → P6 Database Specialist (implement)
  → P7 Security Engineer (validate security)
  → P10 Code Quality Guardian (validate quality)
  → P9 Testing Verification (validate tests)
  → P8 Performance Engineer (validate performance)
  → P11 Documentation Guardian (update docs)
  → P12 Release Readiness (validate release)
  → P1 Engineering Orchestrator (final approval)
```

### API Changes

```
API Change
  → P4 Specification Lock (validate contracts)
  → P5 Architecture Guardian (validate architecture)
  → P11 Documentation Guardian (update specs)
  → P9 Testing Verification (validate tests)
  → P12 Release Readiness (validate release)
  → P1 Engineering Orchestrator (final approval)
```

### FFI Changes

```
FFI Change
  → P4 Specification Lock (validate contracts)
  → P7 Security Engineer (validate security)
  → P5 Architecture Guardian (validate architecture)
  → P10 Code Quality Guardian (validate quality)
  → P9 Testing Verification (validate tests)
  → P11 Documentation Guardian (update docs)
  → P12 Release Readiness (validate release)
  → P1 Engineering Orchestrator (final approval)
```

### Security Changes

```
Security Change
  → P7 Security Engineer (implement)
  → P4 Specification Lock (validate contracts)
  → P5 Architecture Guardian (validate architecture)
  → P10 Code Quality Guardian (validate quality)
  → P9 Testing Verification (validate tests)
  → P11 Documentation Guardian (update docs)
  → P12 Release Readiness (validate release)
  → P1 Engineering Orchestrator (final approval)
```

### Performance Changes

```
Performance Change
  → P8 Performance Engineer (implement)
  → P6 Database Specialist (validate storage)
  → P10 Code Quality Guardian (validate quality)
  → P9 Testing Verification (validate tests)
  → P11 Documentation Guardian (update docs)
  → P12 Release Readiness (validate release)
  → P1 Engineering Orchestrator (final approval)
```

### Bug Fixes

```
Bug Fix
  → P14 Debugging Root Cause (root cause analysis)
  → P10 Code Quality Guardian (implement fix)
  → P9 Testing Verification (regression test)
  → P12 Release Readiness (validate release)
  → P1 Engineering Orchestrator (final approval)
```

### New Features

```
New Feature
  → P2 Task Planner (plan implementation)
  → P3 Change Impact Analysis (assess impact)
  → P4 Specification Lock (validate contracts)
  → P5 Architecture Guardian (validate architecture)
  → Domain Specialist (implement)
  → P10 Code Quality Guardian (validate quality)
  → P9 Testing Verification (validate tests)
  → P8 Performance Engineer (benchmark)
  → P11 Documentation Guardian (update docs)
  → P13 Code Review Auditor (review)
  → P12 Release Readiness (validate release)
  → P1 Engineering Orchestrator (final approval)
```

### Documentation Changes

```
Documentation Change
  → P11 Documentation Guardian (implement)
  → P4 Specification Lock (validate SSOT alignment)
  → P12 Release Readiness (validate completeness)
  → P1 Engineering Orchestrator (final approval)
```

### Release

```
Release
  → P12 Release Readiness (validate all gates)
  → P1 Engineering Orchestrator (final approval)
  → Version bump
  → Changelog update
  → Git tag
```
