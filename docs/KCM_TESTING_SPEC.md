# KCM Testing Specification

**Document ID:** KCM-TEST-001  
**Version:** 1.0.0  
**Depends on:** KCM-SPEC-001

---

## 1. Purpose

Defines testing standards, coverage requirements, test categories, and validation criteria.

---

## 2. Testing Pyramid

```
                    /\
                   /  \         E2E Tests (5-10%)
                  /    \
                 /______\
                /        \      Integration Tests (20-30%)
               /          \
              /____________\
             /              \    Unit Tests (60-75%)
            /                \
           /__________________\
```

---

## 3. Test Categories

### 3.1 Unit Tests

| Property | Requirement |
|----------|-------------|
| Scope | Single function or module |
| Speed | < 100ms each |
| Count target | 200+ |
| Frequency | Every commit |
| Framework | #[test] with assert!/assert_eq! |
| Coverage target | ≥ 95% line coverage |

**Current count: 82 unit tests** across kcm-core (43), kcm-compute (15+7), kcm-storage (codec/compress/index inline).

### 3.2 Integration Tests

| Property | Requirement |
|----------|-------------|
| Scope | Multiple components |
| Speed | 100ms - 5s each |
| Count target | 50+ |
| Frequency | Every commit |
| Location | `crates/*/tests/` |
| Coverage target | ≥ 80% |

**Current count: 64 integration tests** across kcm-storage (18), kcm-runtime (12+11), kcm-storage/persistence (8), kcm-compute (15), kcm-reasoning (17), kcm-optimizer (14+8).

### 3.3 Property-Based Tests

| Property | Requirement |
|----------|-------------|
| Scope | Invariant verification |
| Speed | 100ms - 5s each |
| Count | 100K+ iterations per test |
| Frequency | Every commit |
| Framework | proptest |
| Location | `crates/kcm-core/tests/property_tests.rs` |

**Current: 8 proptest tests** covering Confidence arithmetic, Fact creation, Bitmap operations, RowID arithmetic.

### 3.4 Security Tests

| Property | Requirement |
|----------|-------------|
| Scope | Security scenarios |
| Speed | < 1s each |
| Count | 20+ |
| Frequency | Every commit |

**Current count: 29 security tests** across kcm-security (11) and kcm-testing/security_tests (18).

### 3.5 Load Tests

| Property | Requirement |
|----------|-------------|
| Scope | Concurrent throughput |
| Duration | 1-5 minutes |
| Count | 3 scenarios |
| Frequency | Pre-release |
| Location | `kcm-testing/src/load_tests.rs` |

**Current: 4 load tests** (light, medium, concurrent inserts, concurrent queries).

### 3.6 Stress Tests

| Property | Requirement |
|----------|-------------|
| Scope | Breaking point |
| Duration | 1-10 seconds |
| Count | 2 scenarios |
| Frequency | Monthly |
| Location | `kcm-testing/src/stress_tests.rs` |

**Current: 3 stress tests** (sustained, spike, zero users).

### 3.7 Regression Tests

| Property | Requirement |
|----------|-------------|
| Scope | Performance regression detection |
| Threshold | 5% from baseline |
| Frequency | Every commit |
| Location | `kcm-testing/src/regression_detector.rs` |

**Current: 9 regression tests**.

---

## 4. Test Matrix

| Crate | Unit | Integration | Property | Security | Total |
|-------|------|-------------|----------|----------|-------|
| kcm-core | 43 | 10 | 8 | — | 61 |
| kcm-storage | 14 | 18 | — | — | 32 |
| kcm-compute | 22 | — | — | — | 22 |
| kcm-reasoning | — | 17 | — | — | 17 |
| kcm-optimizer | 8 | 14 | — | — | 22 |
| kcm-runtime | — | 23 | — | — | 23 |
| kcm-interface | 3 | — | — | — | 3 |
| kcm-distributed | — | 10 | — | — | 10 |
| kcm-ml | — | 9 | — | — | 9 |
| kcm-security | — | — | — | 11 | 11 |
| kcm-compliance | — | 7 | — | — | 7 |
| kcm-testing | — | — | — | 18 | 18 |
| **TOTAL** | **90** | **108** | **8** | **29** | **235+** |

**Actual total: 313 tests** (includes inline module tests).

---

## 5. Acceptance Criteria

| Criterion | Threshold | Measurement |
|-----------|-----------|-------------|
| Test pass rate | 100% | `cargo test --workspace` |
| Clippy warnings | 0 (style excluded) | `cargo clippy --workspace` |
| Format compliance | 100% | `cargo fmt --check` |
| Coverage | ≥ 95% | cargo-tarpaulin |
| No unwrap() in production | 0 occurrences | Manual review |
| No unsafe in public API | 0 occurrences | Manual review |
| Property test iterations | ≥ 100K | proptest config |
| Security test scenarios | ≥ 20 | Test count |

---

## 6. CI Pipeline Jobs

| Job | Trigger | Timeout |
|-----|---------|---------|
| Build & Format | Every push/PR | 5 min |
| Clippy Linting | Every push/PR | 5 min |
| Unit Tests | Every push/PR | 5 min |
| Integration Tests | Every push/PR | 10 min |
| Property Tests | Every push/PR | 5 min |
| Security Tests | Every push/PR | 5 min |
| Load Tests | After unit tests pass | 10 min |
| Stress Tests | After unit tests pass | 5 min |
| Benchmarks | After unit tests pass | 10 min |
| Regression Tests | Every push/PR | 5 min |
| Metrics Dashboard | After unit/integration/security | 5 min |
| Quality Gate | All above pass | — |

---

## 7. Constraints

| Constraint | Rationale |
|------------|-----------|
| All tests runnable via `cargo test` | Developer ergonomics |
| No external service dependencies | Self-contained testing |
| Deterministic test results | Reproducible validation |
| Property tests use fixed seed | Reproducible debugging |
