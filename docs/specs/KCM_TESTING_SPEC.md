# KCM Testing Specification

**Document ID:** KCM-TEST-002
**Version:** 1.0.0
**Status:** Active
**Owner:** Specification Lock (P4)
**Authority:** P1 (PRD-TESTING-AND-BENCHMARK.md)

---

## 1. Purpose

Defines KCM's testing standards: test pyramid, quality gates, test distribution, and testing rules.

## 2. Test Philosophy

Testing proves correctness, not just absence of failures. Every test must validate documented behavior. Every feature must have corresponding tests. Passing tests alone is insufficient — tests must cover edge cases, error paths, and invariants.

## 3. Test Pyramid

```
          /\
         /  \      Security Tests (29+)
        /    \     Attack surface validation
       /______\
      /        \   Property Tests (8+)
     /          \  Invariant verification
    /____________\
   /              \  Integration Tests (470+)
  /                \ Cross-component correctness
 /__________________\
/                    \  Unit Tests (89+)
\                    / Single function correctness
 \__________________/
```

## 4. Test Categories

### 4.1 Unit Tests

| Attribute | Value |
|-----------|-------|
| Scope | Single function/module |
| Speed | < 100ms |
| Count Target | 89+ |
| Framework | `#[test]` |
| Location | `src/` modules or `tests/` |

### 4.2 Integration Tests

| Attribute | Value |
|-----------|-------|
| Scope | Cross-crate |
| Speed | 1s-5s |
| Count Target | 470+ |
| Framework | `#[test]` |
| Location | `tests/` directories |

### 4.3 Property Tests

| Attribute | Value |
|-----------|-------|
| Scope | Invariant verification |
| Speed | 1-5min |
| Count Target | 8+ |
| Framework | proptest |
| Location | `tests/` directories |

### 4.4 Security Tests

| Attribute | Value |
|-----------|-------|
| Scope | Attack surface validation |
| Speed | Varies |
| Count Target | 29+ |
| Framework | `#[test]` |
| Location | `kcm-testing` crate |

### 4.5 Load Tests

| Attribute | Value |
|-----------|-------|
| Scope | Concurrency/throughput |
| Speed | 5min+ |
| Count Target | 6 scenarios |
| Framework | Custom |
| Location | `kcm-testing` crate |

### 4.6 Stress Tests

| Attribute | Value |
|-----------|-------|
| Scope | Breaking point |
| Speed | 1hr+ |
| Count Target | 4 scenarios |
| Framework | Custom |
| Location | `kcm-testing` crate |

### 4.7 Recovery Tests

| Attribute | Value |
|-----------|-------|
| Scope | Crash/fault tolerance |
| Speed | Varies |
| Count Target | 5+ |
| Framework | Custom |
| Location | `kcm-testing` crate |

## 5. Test Distribution by Crate

| Crate | src/ | tests/ | Total |
|-------|------|--------|-------|
| kcm-core | 6 | 59 | 65 |
| kcm-storage | 19 | 80 | 99 |
| kcm-compute | 7 | 23 | 30 |
| kcm-reasoning | 0 | 21 | 21 |
| kcm-optimizer | 8 | 16 | 24 |
| kcm-runtime | 0 | 62 | 62 |
| kcm-interface | 6 | 61 | 67 |
| kcm-distributed | 0 | 18 | 18 |
| kcm-ml | 0 | 14 | 14 |
| kcm-security | 0 | 22 | 22 |
| kcm-compliance | 0 | 7 | 7 |
| kcm-testing | 43 | 87 | 130 |
| kcm-server | 0 | 0 | 0 |
| **Total** | **89** | **470** | **559** |

## 6. Quality Gates

| Gate | Metric | Threshold | Enforcement |
|------|--------|-----------|-------------|
| Test Pass Rate | tests_passed / tests_total | = 100% | CI blocks merge |
| Code Coverage | lines_covered / lines_total | ≥ 95% | CI warning |
| Clippy Warnings | warning_count | = 0 | CI blocks merge |
| Formatting | diff_count | = 0 | CI blocks merge |
| unwrap() Count | unwrap_in_production | = 0 | CI blocks merge |
| Performance Regression | (baseline - current) / baseline | < 5% | CI warning |
| Critical Regression | (baseline - current) / baseline | < 10% | CI blocks merge |

## 7. Testing Rules

| Rule | Description |
|------|-------------|
| TR-001 | Every PR must pass `cargo test --workspace` |
| TR-002 | Every PR must pass `cargo clippy --workspace` |
| TR-003 | Every PR must pass `cargo fmt --check` |
| TR-004 | New code must have ≥ 95% test coverage |
| TR-005 | Security code must have security tests |
| TR-006 | Performance-critical code must have benchmarks |
| TR-007 | Property tests required for arithmetic operations |

## 8. Property-Based Testing

| Property | Invariant | Cases |
|----------|-----------|-------|
| Dictionary idempotence | insert(x) == insert(x) | 1000 |
| Dictionary bijection | insert(x) → unique_id | 5000 |
| Dictionary retrieval | get(insert(x)) == x | 5000 |
| Confidence bounds | 0 ≤ result ≤ 1 | 10000 |
| Confidence commutativity | multiply(a,b) == multiply(b,a) | 5000 |
| Confidence identity | multiply(x, 1) == x | 1000 |
| Confidence absorption | multiply(x, 0) == 0 | 1000 |
| Fact creation | Valid input → valid fact | 10000 |
| Bitmap set/get | set(i); assert(get(i)) | 10000 |
| Bitmap clear | set(i); clear(i); !get(i) | 10000 |
| Bitmap AND | Intersection correctness | 5000 |
| Bitmap OR | Union correctness | 5000 |

## 9. CI Pipeline

```
push/PR → build → fmt → clippy → unit-tests → integration-tests
                    ↓
             property-tests → security-tests → benchmarks
                    ↓
             quality-gate → deploy (main branch only)
```

## 10. References

- **Implements:** PRD-TESTING-AND-BENCHMARK.md §3 (Test Pyramid)
- **Depends on:** KCM_DATA_MODEL_SPEC
- **Related:** KCM_PERFORMANCE_SPEC, KCM_ENGINEERING_RULES
