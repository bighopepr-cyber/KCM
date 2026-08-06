# Quality Report Template

> Document ID: KCM-TPL-QUALITY-001 | Version: 1.0.0

---

# Quality Report

**Task ID:** {{TASK_ID}}
**Date:** {{DATE}}
**Validator:** Quality Engine

## Gate Results

| # | Gate | Status | Details | Duration |
|---|------|--------|---------|----------|
| 1 | Format (cargo fmt) | {{STATUS}} | {{DETAILS}} | {{DURATION}} |
| 2 | Lint (cargo clippy) | {{STATUS}} | {{DETAILS}} | {{DURATION}} |
| 3 | Build (cargo build) | {{STATUS}} | {{DETAILS}} | {{DURATION}} |
| 4 | Unit Tests | {{STATUS}} | {{DETAILS}} | {{DURATION}} |
| 5 | Integration Tests | {{STATUS}} | {{DETAILS}} | {{DURATION}} |
| 6 | Property Tests | {{STATUS}} | {{DETAILS}} | {{DURATION}} |
| 7 | Security Audit | {{STATUS}} | {{DETAILS}} | {{DURATION}} |
| 8 | SSOT Validation | {{STATUS}} | {{DETAILS}} | {{DURATION}} |
| 9 | Doc Coverage | {{STATUS}} | {{DETAILS}} | {{DURATION}} |
| 10 | Doc Validation | {{STATUS}} | {{DETAILS}} | {{DURATION}} |

## Conditional Gates

| Gate | Condition | Status | Details | Duration |
|------|-----------|--------|---------|----------|
| Benchmark | {{CONDITION}} | {{STATUS}} | {{DETAILS}} | {{DURATION}} |
| FFI Safety | {{CONDITION}} | {{STATUS}} | {{DETAILS}} | {{DURATION}} |
| SDK Consistency | {{CONDITION}} | {{STATUS}} | {{DETAILS}} | {{DURATION}} |
| Storage Format | {{CONDITION}} | {{STATUS}} | {{DETAILS}} | {{DURATION}} |
| Version Sync | {{CONDITION}} | {{STATUS}} | {{DETAILS}} | {{DURATION}} |

## Summary

| Metric | Value |
|--------|-------|
| Total Gates | {{TOTAL}} |
| Passed | {{PASSED}} |
| Failed | {{FAILED}} |
| Skipped | {{SKIPPED}} |
| Status | {{STATUS}} |
