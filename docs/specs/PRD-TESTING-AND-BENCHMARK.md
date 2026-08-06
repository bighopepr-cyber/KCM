# KCM Testing & Benchmark Specification

**Document ID:** KCM-TEST-001
**Version:** 2.0.0
**Status:** Authoritative
**Owner:** Specification Lock (P4)
**Authority:** P1 (Highest — overrides all on validation methodology)

---

## 1. Purpose

This document defines KCM's testing strategy, benchmark methodology, quality gates, and performance targets. As the highest-priority document, it governs all validation activities.

## 2. Testing Philosophy

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
       /              \  Integration Tests (108+)
      /                \ Cross-component correctness
     /__________________\
    /                    \  Unit Tests (90+)
   /                      \ Single function correctness
  /________________________\
```

### 3.1 Test Categories

| Category | Scope | Speed | Count Target | Framework |
|----------|-------|-------|-------------|-----------|
| Unit | Single function/module | < 100ms | 89+ | #[test] |
| Integration | Cross-crate | 1s-5s | 470+ | #[test] |
| Property | Invariant verification | 1-5min | 8+ | proptest |
| Security | Attack surface | varies | 29+ | #[test] |
| Load | Concurrency/throughput | 5min+ | 6 scenarios | custom |
| Stress | Breaking point | 1hr+ | 4 scenarios | custom |
| Recovery | Crash/fault tolerance | varies | 5+ | custom |

### 3.2 Test Distribution by Crate

Actual `#[test]` annotation counts (as of 2026-08-04):

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

## 4. Quality Gates

Every build must pass all gates:

| Gate | Metric | Threshold | Enforcement |
|------|--------|-----------|-------------|
| Test Pass Rate | tests_passed / tests_total | = 100% | CI blocks merge |
| Code Coverage | lines_covered / lines_total | ≥ 95% | CI warning |
| Clippy Warnings | warning_count | = 0 | CI blocks merge |
| Formatting | diff_count | = 0 | CI blocks merge |
| unwrap() Count | unwrap_in_production | = 0 | CI blocks merge |
| Performance Regression | (baseline - current) / baseline | < 5% | CI warning |
| Critical Regression | (baseline - current) / baseline | < 10% | CI blocks merge |

## 5. Benchmark Suite

### 5.1 Benchmark Inventory

| # | Benchmark | Category | Target |
|---|-----------|----------|--------|
| 1 | column_sequential_scan_1k | Column | < 1ms |
| 2 | column_sequential_scan_10k | Column | < 10ms |
| 3 | column_sequential_scan_100k | Column | < 100ms |
| 4 | column_sequential_scan_1m | Column | < 1s |
| 5 | column_random_access_1k | Column | < 1ms |
| 6 | column_random_access_10k | Column | < 10ms |
| 7 | column_random_access_100k | Column | < 100ms |
| 8 | column_random_access_1m | Column | < 1s |
| 9 | column_simd_filter_10k | Column/SIMD | < 5ms |
| 10 | column_simd_filter_100k | Column/SIMD | < 50ms |
| 11 | column_simd_filter_1m | Column/SIMD | < 500ms |
| 12 | bitmap_set_10k | Bitmap | < 1ms |
| 13 | bitmap_set_100k | Bitmap | < 10ms |
| 14 | bitmap_set_1m | Bitmap | < 100ms |
| 15 | bitmap_count_ones_10k | Bitmap | < 1ms |
| 16 | bitmap_count_ones_100k | Bitmap | < 10ms |
| 17 | bitmap_count_ones_1m | Bitmap | < 100ms |
| 18 | bitmap_bitwise_100k | Bitmap | < 10ms |
| 19 | bitmap_bitwise_1m | Bitmap | < 100ms |
| 20 | dictionary_insert_1k | Dictionary | < 10ms |
| 21 | dictionary_insert_10k | Dictionary | < 100ms |
| 22 | dictionary_insert_100k | Dictionary | < 1s |
| 23 | dictionary_lookup_1k | Dictionary | < 1ms |
| 24 | dictionary_lookup_10k | Dictionary | < 10ms |
| 25 | dictionary_lookup_100k | Dictionary | < 100ms |
| 26 | database_insert_100 | Database | < 1ms |
| 27 | database_insert_1k | Database | < 10ms |
| 28 | database_insert_10k | Database | < 100ms |
| 29 | database_query_1k | Database | < 1ms |
| 30 | database_query_10k | Database | < 10ms |
| 31 | database_query_100k | Database | < 100ms |
| 32 | inference_pattern_1k | Inference | < 1ms |
| 33 | inference_pattern_10k | Inference | < 10ms |
| 34 | inference_pattern_100k | Inference | < 100ms |

### 5.2 Benchmark Configuration

```rust
Criterion::default()
    .measurement_time(Duration::from_secs(10))
    .measurement_batch_size(100)
    .warm_up_time(Duration::from_secs(3))
```

### 5.3 Benchmark Reporting

Results stored in `benchmark-results/` with metadata:
- `metadata/benchmark-version.json` — spec version
- `metadata/environment.json` — CPU, RAM, OS, Rust version
- `metadata/git.json` — commit hash, branch, dirty flag

## 6. Load Test Scenarios

| Scenario | Duration | Users | Initial Facts | Insert% | Query% | Target QPS | P99 Latency |
|----------|----------|-------|---------------|---------|--------|------------|-------------|
| Light | 5min | 10 | 100K | 20% | 80% | 5K | 10ms |
| Medium | 10min | 50 | 1M | 30% | 70% | 15K | 20ms |
| Heavy | 15min | 100 | 5M | 40% | 60% | 25K | 50ms |
| Spike | 5min | 200 | 10M | 50% | 50% | 40K | 100ms |
| Read-Heavy | 10min | 100 | 10M | 5% | 95% | 50K | 5ms |
| Write-Heavy | 10min | 50 | 1M | 90% | 10% | 10K | 30ms |

## 7. Stress Test Scenarios

| Scenario | Ramp-Up | Hold | Max Users | Max Failure Rate |
|----------|---------|------|-----------|-----------------|
| Gradual Increase | 1hr | 5min | 1000 | 5% |
| Sudden Spike | 10s | 1min | 5000 | 10% |
| Sustained Max | 5min | 1hr | 500 | 1% |
| Memory Exhaustion | 10min | 30min | 100 | 50% |

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

## 9. Security Test Matrix

| # | Test | Attack Vector | Expected |
|---|------|--------------|----------|
| 1 | Injection prevention | Malicious dictionary input | Stored safely |
| 2 | Buffer overflow | DenseVec capacity exceeded | Error returned |
| 3 | Integer overflow | Max ID values handled | No wrap |
| 4 | RBAC enforcement | Unauthorized access denied | Permission denied |
| 5 | Timing attack | Constant-time operations | No timing leak |
| 6 | Memory safety | No use-after-free | Clean access |
| 7 | Concurrent safety | Race condition prevention | No data corruption |
| 8 | Confidence boundary | NaN/Infinity rejected | Error returned |
| 9 | Context isolation | Cross-context data leakage | Isolation maintained |
| 10 | Audit integrity | Hash chain verification | Chain valid |

## 10. Regression Detection

Baseline comparison with severity classification:

| Change | Severity | Action |
|--------|----------|--------|
| < 2% | Low | Informational |
| 2-5% | Medium | CI warning |
| 5-10% | High | CI failure |
| > 10% | Critical | CI failure + alert |

## 11. CI Pipeline

```
push/PR → build → fmt → clippy → unit-tests → integration-tests
                     ↓
              property-tests → security-tests → benchmarks
                     ↓
              quality-gate → deploy (main branch only)
```

Jobs:
1. **Build & Format** — cargo build, fmt check, clippy
2. **Unit Tests** — cargo test --lib --all
3. **Integration Tests** — cargo test --test '*' --all
4. **Benchmarks** — cargo bench (informational, regresses > 10% blocks)
5. **Property Tests** — proptest suite
6. **Security Tests** — attack surface validation
7. **Quality Gate** — aggregate all results
8. **Deploy** — Docker build + K8s rollout (main only)

## 12. References

- **Depends on:** PRD.md (P4 — types), PRD2.md (P3 — runtime)
- **Parent specs:** AGENTS.md
- **Related:** KCM_PERFORMANCE_SPEC, KCM_BENCHMARK_REPORTING_SPEC, KCM_ENGINEERING_RULES
