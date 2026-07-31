# KCM Performance Specification

**Document ID:** KCM-PERF-001  
**Version:** 1.0.0  
**Depends on:** KCM-SPEC-001

---

## 1. Purpose

Defines performance targets, benchmark methodology, and metrics for KCM.

---

## 2. Performance Targets

**See KCM_SPECIFICATION (Section 4.1) for authoritative performance targets.** This section defines measurement methodology for those targets.

| Metric | Target (from KCM_SPECIFICATION) | Measurement |
|--------|--------------------------------|-------------|
| Column sequential scan | > 100M ops/sec | Criterion benchmark |
| Bitmap set/get | > 8M ops/sec | Criterion benchmark |
| Dictionary lookup | < 100ns | Criterion benchmark |
| Insert throughput | > 50K facts/sec | Load test |
| Query latency P99 (1M facts) | < 100ms | Load test |
| Memory per fact | < 100 bytes | Static calculation |
| Compression ratio | > 5x | Compressed / Uncompressed |

---

## 3. Benchmark Suite

### 3.1 Column Operations

| Benchmark | Sizes | Measures |
|-----------|-------|----------|
| column_sequential_scan | 1K, 10K, 100K, 1M | Sum iteration throughput |
| column_random_access | 1K, 10K, 100K, 1M | Non-sequential access pattern |
| column_simd_filter | 10K, 100K, 1M | SIMD-accelerated filtering |

### 3.2 Bitmap Operations

| Benchmark | Sizes | Measures |
|-----------|-------|----------|
| bitmap_set | 10K, 100K, 1M | Set bit throughput |
| bitmap_count | 10K, 100K, 1M | count_ones throughput |
| bitmap_bitwise | 100K, 1M | AND operation throughput |

### 3.3 Dictionary Operations

| Benchmark | Sizes | Measures |
|-----------|-------|----------|
| dictionary_insert | 1K, 10K, 100K | Insert throughput |
| dictionary_lookup | 1K, 10K, 100K | Lookup throughput |

### 3.4 Database Operations

| Benchmark | Sizes | Measures |
|-----------|-------|----------|
| database_insert | 100, 1K, 10K | Batch insert throughput |
| database_query | 1K, 10K, 100K | Predicate query latency |
| inference_pattern_matching | 1K, 10K, 100K | Pattern scan throughput |

---

## 4. Benchmark Methodology

### 4.1 Tool

Criterion.rs with `criterion_group!` and `criterion_main!`.

### 4.2 Configuration

- Minimum 10 iterations per sample
- 95% confidence interval
- Warmup: automatic (Criterion default)

### 4.3 Measurement

| Parameter | Value |
|-----------|-------|
| Platform | Linux x86_64 |
| CPU | ≥ 8 cores, ≥ 3.0 GHz |
| RAM | ≥ 16 GB |
| Kernel | Linux 5.x+ |
| Profile | bench (inherits release with LTO) |

### 4.4 Regression Detection

Criterion auto-detects performance regressions against stored baselines in `target/criterion/`.

---

## 5. Memory Metrics

### 5.1 Per-Column Memory

| Type | Size/Element | 1M Elements |
|------|-------------|-------------|
| u8 | 1 byte | 1 MB |
| u16 | 2 bytes | 2 MB |
| u32 | 4 bytes | 4 MB |
| u64/i64 | 8 bytes | 8 MB |
| f64 | 8 bytes | 8 MB |
| DenseVec overhead | ~64 bytes | 64 bytes |

### 5.2 Per-Fact Memory (uncompressed)

```
Subject (u32):     4 bytes
Predicate (u8):    1 byte
Object (u32):      4 bytes
Confidence (f64):  8 bytes
Evidence (u8):     1 byte
Timestamp (i64):   8 bytes
Context (u8):      1 byte
Version (i32):     4 bytes
Priority (i8):     1 byte
Owner (u16):       2 bytes
─────────────────────────
Total:            34 bytes per fact
+ alignment:      ~34 bytes per fact
```

### 5.3 Bitmap Memory

```
1M bits = 128 KB (16 × 8-byte words per 1024 bits)
```

---

## 6. Load Test Scenarios

| Scenario | Users | Ops/User | Initial Facts | Expected QPS | Max P99 Latency |
|----------|-------|----------|---------------|--------------|-----------------|
| Light | 10 | 100 | 100K | 5,000 | 10ms |
| Medium | 50 | 200 | 1M | 15,000 | 20ms |
| Heavy | 100 | 500 | 5M | 25,000 | 50ms |
| Spike | 200 | — | 10M | 40,000 | 100ms |
| Read-Heavy | 100 | — | 10M | 50,000 | 5ms |
| Write-Heavy | 50 | — | 1M | 10,000 | 30ms |

---

## 7. Stress Test Scenarios

| Scenario | Max Users | Ramp-Up | Hold | Ramp-Down | Max Failure Rate | Memory Limit |
|----------|-----------|---------|------|-----------|-----------------|--------------|
| Gradually Increasing | 1000 | 1hr | 5min | 5min | 5% | 16GB |
| Sudden Spike | 5000 | 10s | 60s | 30s | 10% | 16GB |
| Sustained Maximum | 500 | 5min | 1hr | 5min | 1% | 16GB |
| Memory Exhaustion | 100 | 10min | 30min | 5min | 50% | 16GB |

---

## 8. Comparison Baselines

| System | Type | Strengths |
|--------|------|-----------|
| DuckDB | Columnar OLAP | Vectorized execution |
| ClickHouse | Columnar OLAP | Distributed analytics |
| Apache Arrow | Columnar format | Zero-copy IPC |
| Apache Parquet | Columnar file | Compression ratio |
| Neo4j | Graph DB | Cypher query language |

KCM differentiates by combining columnar storage with knowledge representation semantics (confidence, evidence, provenance) and built-in inference.

---

## 9. Constraints

| Constraint | Rationale |
|------------|-----------|
| Benchmarks use release profile | Reflects production performance |
| Load tests use configurable durations | CI vs full run |
| Memory metrics are static | No runtime allocation tracking needed |

---

## 10. References

- **Depends on:** KCM_SPECIFICATION (KCM_SPECIFICATION)
- **Parent specs:** KCM_SPECIFICATION (KCM_SPECIFICATION)
- **Related:** KCM_BENCHMARK_REPORTING_SPEC (KCM_BENCHMARK_REPORTING_SPEC), KCM_ENGINEERING_RULES (KCM_ENGINEERING_RULES)
