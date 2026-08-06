# Quality Engine

> Document ID: KCM-QUALITY-001 | Version: 2.0.0 | Status: Active

## Overview

The Quality Engine enforces quality gates throughout the engineering workflow. No gate can be skipped or bypassed.

## Quality Gate Pipeline

```
Implementation → Format → Lint → Build → Unit Tests → Integration Tests
  → Property Tests → Security Audit → SSOT Validation → Doc Coverage
  → Doc Validation → [Benchmark] → Release Gate
```

## Mandatory Gates

| # | Gate | Validator | Pass Criteria | Blocking | Phase |
|---|------|-----------|--------------|----------|-------|
| 1 | Format | `cargo fmt --check` | Zero diff | Yes | Quality |
| 2 | Lint | `cargo clippy -- -D warnings` | Zero warnings | Yes | Quality |
| 3 | Build | `cargo build --workspace` | Success | Yes | Quality |
| 4 | Unit Tests | `cargo test --lib` | 100% pass | Yes | Testing |
| 5 | Integration | `cargo test --test` | 100% pass | Yes | Testing |
| 6 | Property Tests | `cargo test property` | 100% pass | Yes | Testing |
| 7 | Security Audit | `cargo audit` | Zero vulnerabilities | Yes | Security |
| 8 | SSOT Validation | `bash scripts/validate-ssot.sh` | Pass | Yes | Validation |
| 9 | Doc Coverage | `bash tools/doc-coverage/calculate.sh` | 100% | Yes | Documentation |
| 10 | Doc Validation | `bash tools/doc-validator/validate-docs.sh` | Pass | Yes | Documentation |

## Conditional Gates

| # | Gate | Condition | Validator | Pass Criteria | Blocking |
|---|------|-----------|-----------|--------------|----------|
| 11 | Benchmark | Performance change | `bench-compare.py` | < 5% regression | Yes |
| 12 | FFI Safety | FFI change | Manual review | All checks pass | Yes |
| 13 | SDK Consistency | SDK change | `validate-sdk-api.sh` | All SDKs pass | Yes |
| 14 | Storage Format | Format change | Roundtrip test | Pass | Yes |
| 15 | Version Sync | Version change | `verify-version.sh` | Pass | Yes |

## Release Gates

| # | Gate | Validator | Pass Criteria | Blocking |
|---|------|-----------|--------------|----------|
| 16 | All CI | CI pipeline | All green | Yes |
| 17 | No Regression | `bench-compare.py` | < 5% | Yes |
| 18 | SSOT Aligned | `validate-ssot.sh` | Pass | Yes |
| 19 | Docs Complete | `validate-docs.sh` | 100% | Yes |
| 20 | Version Bumped | `git log` | Yes | Yes |
| 21 | Changelog | `CHANGELOG.md` | Updated | Yes |

## Gate Execution

```
For each gate:
1. Run validator
2. Capture output
3. Check pass criteria
4. If pass → next gate
5. If fail → record failure
6. If blocking → halt execution
7. If non-blocking → continue with warning
8. Record result in quality report
```

## Gate Results

| Result | Action |
|--------|--------|
| PASS | Continue to next gate |
| FAIL (blocking) | Halt execution, record failure, notify skill |
| FAIL (non-blocking) | Continue with warning, record warning |
| SKIP (condition not met) | Record skip reason |
| ERROR | Halt execution, record error, escalate |

## Quality Report Format

```markdown
# Quality Report

**Task ID:** {{TASK_ID}}
**Date:** {{DATE}}

## Gate Results
| # | Gate | Status | Details | Duration |
|---|------|--------|---------|----------|
| 1 | Format | PASS | — | 2s |
| 2 | Lint | PASS | — | 15s |
| 3 | Build | PASS | — | 45s |
| 4 | Unit Tests | PASS | 142/142 | 30s |
| 5 | Integration | PASS | 47/47 | 120s |
| 6 | Property | PASS | 8/8 | 60s |
| 7 | Security | PASS | 0 vulns | 10s |
| 8 | SSOT | PASS | — | 5s |
| 9 | Doc Coverage | PASS | 100% | 3s |
| 10 | Doc Validation | PASS | — | 2s |

## Summary
- **Total Gates:** {{TOTAL}}
- **Passed:** {{PASSED}}
- **Failed:** {{FAILED}}
- **Skipped:** {{SKIPPED}}
- **Status:** {{STATUS}}
```

## Regression Thresholds

| Regression | Action | Blocking |
|-----------|--------|----------|
| < 5% | Acceptable | No |
| 5-10% | Warning, requires justification | Yes |
| > 10% | Failure, blocks merge | Yes |
