# Bug Fix Example

> Document ID: KCM-EX-BUG-001 | Version: 1.0.0

## Task

Fix a bug where `kcm_query()` returns incorrect results when filtering by confidence > 0.9 on an empty database.

## Execution Plan

**Task ID:** KCM-2026-002
**Pipeline:** bugfix.md
**Risk Level:** Low

### Root Cause

In `kcm-compute/src/algebra.rs`, the `filter_confidence()` function does not handle the empty column case correctly. When the column is empty, it returns a non-empty iterator instead of an empty one.

### Affected Components

| Component | Files | Change Type | Risk |
|-----------|-------|-------------|------|
| kcm-compute | `src/algebra.rs` | Bug fix | Low |

### Required Skills

| Skill | Phase | Responsibility |
|-------|-------|---------------|
| P14 | Debugging | Root cause analysis |
| P10 | Quality | Implement fix |
| P9 | Testing | Regression test |
| P12 | Release | Release gate |
| P1 | Orchestrator | Approval |

## Execution Steps

### Step 1: Root Cause Analysis (P14)

```
- Read kcm-compute/src/algebra.rs
- Identify filter_confidence() function
- Find empty column handling bug
- Document root cause
```

### Step 2: Implement Fix (P10)

```rust
// Before (buggy):
fn filter_confidence(column: &[f64], threshold: f64) -> Vec<usize> {
    column.iter().enumerate()
        .filter(|(_, &v)| v > threshold)
        .map(|(i, _)| i)
        .collect()
}

// After (fixed):
fn filter_confidence(column: &[f64], threshold: f64) -> Vec<usize> {
    if column.is_empty() {
        return Vec::new();
    }
    column.iter().enumerate()
        .filter(|(_, &v)| v > threshold)
        .map(|(i, _)| i)
        .collect()
}
```

### Step 3: Regression Test (P9)

```rust
#[test]
fn test_filter_confidence_empty_column() {
    let column: Vec<f64> = vec![];
    let result = filter_confidence(&column, 0.9);
    assert!(result.is_empty());
}
```

### Step 4: Validation

```
- All existing tests pass ✓
- New regression test passes ✓
- No behavior change for non-empty columns ✓
```

## Completion Report

**Status:** COMPLETED
**Duration:** 1 hour
**Files Changed:** 2
**Tests Added:** 1
**Root Cause:** Empty column not handled in filter_confidence()
**Fix:** Added early return for empty column
