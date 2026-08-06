# KCM Performance Specification

**Document ID:** KCM-PERF-001
**Version:** 1.0.0
**Status:** Active
**Owner:** Specification Lock (P4)
**Authority:** P1 (PRD-TESTING-AND-BENCHMARK.md)

---

## 1. Purpose

Defines KCM's performance targets, benchmark methodology, and regression detection.

## 2. Performance Targets

### 2.1 Core Engine

| Metric | Target | Measurement |
|--------|--------|-------------|
| Column scan throughput | > 100M ops/sec | Criterion benchmark |
| Bitmap operations | > 8M ops/sec | Criterion benchmark |
| Dictionary lookup | < 100ns | Criterion benchmark |
| Insert throughput | > 50K facts/sec | Load test |
| Query latency (1M facts) | P99 < 100ms | Load test |
| Memory per fact | < 100 bytes | Memory profiling |
| Compression ratio | > 5x | Storage measurement |

### 2.2 Column Operations

| Operation | Target | Complexity |
|-----------|--------|-----------|
| Sequential scan 1K | < 1ms | O(n) |
| Sequential scan 10K | < 10ms | O(n) |
| Sequential scan 100K | < 100ms | O(n) |
| Sequential scan 1M | < 1s | O(n) |
| Random access 1K | < 1ms | O(1) |
| Random access 10K | < 10ms | O(1) |
| Random access 100K | < 100ms | O(1) |
| Random access 1M | < 1s | O(1) |
| SIMD filter 10K | < 5ms | O(n/32) |
| SIMD filter 100K | < 50ms | O(n/32) |
| SIMD filter 1M | < 500ms | O(n/32) |

### 2.3 Bitmap Operations

| Operation | Target | Complexity |
|-----------|--------|-----------|
| Set 10K bits | < 1ms | O(n/64) |
| Set 100K bits | < 10ms | O(n/64) |
| Set 1M bits | < 100ms | O(n/64) |
| Count ones 10K | < 1ms | O(n/64) |
| Count ones 100K | < 10ms | O(n/64) |
| Count ones 1M | < 100ms | O(n/64) |
| Bitwise AND 100K | < 10ms | O(n/64) |
| Bitwise AND 1M | < 100ms | O(n/64) |

### 2.4 Dictionary Operations

| Operation | Target | Complexity |
|-----------|--------|-----------|
| Insert 1K | < 10ms | O(1) amortized |
| Insert 10K | < 100ms | O(1) amortized |
| Insert 100K | < 1s | O(1) amortized |
| Lookup 1K | < 1ms | O(1) |
| Lookup 10K | < 10ms | O(1) |
| Lookup 100K | < 100ms | O(1) |

### 2.5 Database Operations

| Operation | Target | Complexity |
|-----------|--------|-----------|
| Insert 100 | < 1ms | O(n) |
| Insert 1K | < 10ms | O(n) |
| Insert 10K | < 100ms | O(n) |
| Query 1K | < 1ms | O(n) |
| Query 10K | < 10ms | O(n) |
| Query 100K | < 100ms | O(n) |

### 2.6 Inference Operations

| Operation | Target | Complexity |
|-----------|--------|-----------|
| Pattern match 1K | < 1ms | O(n) |
| Pattern match 10K | < 10ms | O(n) |
| Pattern match 100K | < 100ms | O(n) |

## 3. Benchmark Suite

### 3.1 Benchmark Inventory (34 benchmarks)

| # | Benchmark | Category | Target |
|---|-----------|----------|--------|
| 1-4 | column_sequential_scan | Column | < 1ms-1s |
| 5-8 | column_random_access | Column | < 1ms-1s |
| 9-11 | column_simd_filter | Column/SIMD | < 5ms-500ms |
| 12-14 | bitmap_set | Bitmap | < 1ms-100ms |
| 15-17 | bitmap_count_ones | Bitmap | < 1ms-100ms |
| 18-19 | bitmap_bitwise | Bitmap | < 10ms-100ms |
| 20-22 | dictionary_insert | Dictionary | < 10ms-1s |
| 23-25 | dictionary_lookup | Dictionary | < 1ms-100ms |
| 26-28 | database_insert | Database | < 1ms-100ms |
| 29-31 | database_query | Database | < 1ms-100ms |
| 32-34 | inference_pattern | Inference | < 1ms-100ms |

### 3.2 Criterion Configuration

```rust
Criterion::default()
    .measurement_time(Duration::from_secs(10))
    .measurement_batch_size(100)
    .warm_up_time(Duration::from_secs(3))
```

### 3.3 Benchmark Reporting

Results stored in `benchmark-results/` with metadata:
- `metadata/benchmark-version.json` — spec version
- `metadata/environment.json` — CPU, RAM, OS, Rust version
- `metadata/git.json` — commit hash, branch, dirty flag

## 4. Regression Detection

### 4.1 Severity Classification

| Change | Severity | Action |
|--------|----------|--------|
| < 2% | Low | Informational |
| 2-5% | Medium | CI warning |
| 5-10% | High | CI failure |
| > 10% | Critical | CI failure + alert |

### 4.2 Comparison Method

```bash
python3 scripts/bench-compare.py --baseline baseline.json --current current.json
```

## 5. Memory Budget

| Component | Budget | Measurement |
|-----------|--------|-------------|
| Per fact | < 100 bytes | Memory profiling |
| DenseVec | 64-byte aligned | Layout verification |
| Bitmap | n/8 bytes | Size calculation |
| Dictionary | ~50 bytes/entry | Memory profiling |
| BloomFilter | ~1.25 bytes/element | Formula |

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

## 8. References

- **Implements:** PRD-TESTING-AND-BENCHMARK.md §4 (Benchmarks)
- **Depends on:** KCM_DATA_MODEL_SPEC, KCM_COLUMNAR_FORMAT_SPEC
- **Related:** KCM_TESTING_SPEC, KCM_QUERY_EXECUTION_SPEC
