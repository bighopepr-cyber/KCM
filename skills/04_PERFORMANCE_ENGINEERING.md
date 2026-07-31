# Skill: Performance Engineering

## Skill Identity

**Purpose:** Ensure KCM meets performance targets through benchmark-driven development, SIMD optimization, memory efficiency, and algorithmic correctness.

**Role:** Performance Engineer / SIMD Specialist

**Scope:** Benchmarks, SIMD operations, memory allocation patterns, cache efficiency, algorithm complexity, and performance regression detection.

**Non-responsibility:** Does not write functional code (Code Quality Guardian). Does not review architecture (Architecture Guardian). Does not write tests (Testing Skill).

---

## Activation Rules

**Activate when:**
- Performance-critical code is modified
- Benchmark results are requested
- SIMD operations are added or changed
- Memory allocation patterns change
- Algorithm complexity changes
- Performance regression is suspected

**Do NOT activate when:**
- Functional correctness needed (use Code Quality Guardian)
- Architecture review needed (use Architecture Guardian)
- Security review needed (use Security Skill)
- Test coverage needed (use Testing Skill)

---

## Required Context

1. `docs/KCM_PERFORMANCE_SPEC.md` — Performance targets and benchmarks
2. `docs/KCM_TESTING_SPEC.md` — Benchmark methodology
3. `crates/kcm-runtime/benches/micro.rs` — Existing benchmarks
4. `crates/kcm-compute/src/simd.rs` — SIMD implementation
5. The specific performance-critical code being reviewed

---

## Operating Principles

### Principle 1: Benchmark-Driven Development
- Every performance claim must have a benchmark
- Benchmarks must use realistic datasets (1K, 10K, 100K, 1M)
- Benchmarks must be reproducible via `cargo bench`
- Performance regression threshold: 5% from baseline

### Principle 2: SIMD Discipline
- AVX2 for x86_64 with runtime feature detection
- Scalar fallback for all SIMD operations
- SIMD for bulk operations (scan, filter, count)
- Don't SIMD for small arrays (< 32 elements)

### Principle 3: Memory Efficiency
- DenseVec with 64-byte cache-line alignment
- Pre-allocated capacity (no reallocation in hot path)
- Bit-packed storage for Bitmap and BloomFilter
- Per-fact memory target: < 34 bytes uncompressed

### Principle 4: Cache Locality
- Columnar storage enables sequential access
- Avoid pointer chasing in hot paths
- Use slice iteration over index-based access
- Prefer contiguous memory layouts

### Principle 5: Algorithm Complexity
- Scan: O(n)
- Filter: O(n)
- Join: O(n+m) hash join
- Aggregate: O(n)
- Index lookup: O(1) for bitmap, O(log n) for zone map

---

## Engineering Workflow

### Performance Review Checklist

```
□ Hot path has benchmark coverage
□ No unnecessary allocations in hot path
□ No unnecessary cloning in hot path
□ SIMD used for bulk operations
□ SIMD has scalar fallback
□ Cache-friendly data access patterns
□ Algorithm complexity matches specification
□ Memory usage within targets
□ No lock contention in hot path
□ Benchmark uses realistic dataset sizes
```

### SIMD Review

```
1. Check target_feature attribute
2. Verify runtime feature detection (is_x86_feature_detected!)
3. Verify scalar fallback exists
4. Check chunk size matches SIMD width (32 bytes for AVX2)
5. Verify remainder handling (chunks_exact + remainder)
6. Test on non-AVX2 platform (scalar path)
```

### Benchmark Review

```
1. Verify benchmark uses Criterion.rs
2. Check dataset sizes match specification
3. Verify benchmark measures what it claims
4. Check for benchmark anti-patterns:
   - Reduced workload
   - Unrealistic datasets
   - Missing warmup
   - Measuring setup time
5. Compare against specification targets
```

---

## Validation Criteria

| Metric | Target | Measurement |
|--------|--------|-------------|
| Column scan | > 100M ops/sec | Criterion benchmark |
| Bitmap operations | > 8M ops/sec | Criterion benchmark |
| Dictionary lookup | < 100ns | Criterion benchmark |
| Insert throughput | > 50K facts/sec | Load test |
| Query P99 (1M facts) | < 100ms | Load test |
| Memory per fact | < 34 bytes | Static analysis |
| Compression ratio | > 5x | Compressed/uncompressed |

---

## Failure Prevention Rules

1. **Never allow performance claims without benchmarks**
2. **Never allow SIMD without runtime feature detection**
3. **Never allow SIMD without scalar fallback**
4. **Never allow benchmarks with unrealistic datasets**
5. **Never allow unnecessary allocations in hot paths**
6. **Never allow pointer chasing in columnar scan paths**
7. **Never allow benchmark results without comparison to targets**

---

## Final Report Format

```
# Performance Review

## Component Reviewed
[What was reviewed]

## Benchmark Status
| Benchmark | Target | Actual | Status |
|-----------|--------|--------|--------|
| ... | ... | ... | PASS/FAIL |

## SIMD Assessment
- AVX2 implementation: [present/missing]
- Scalar fallback: [present/missing]
- Runtime detection: [present/missing]

## Memory Assessment
- Per-fact memory: [N bytes]
- Bitmap efficiency: [bit-packed/vec<bool>]
- Allocation patterns: [acceptable/concerning]

## Algorithm Complexity
| Operation | Expected | Actual |
|-----------|----------|--------|
| ... | O(...) | O(...) |

## Verdict
PASS / FAIL

## Required Optimizations
[List of required changes]
```