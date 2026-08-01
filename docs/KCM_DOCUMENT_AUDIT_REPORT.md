# KCM Engineering Report: Benchmark OOM Fix

**Date:** 2026-08-01
**Scope:** `inference_full_engine/1000` OutOfMemory panic investigation and resolution

---

## 1. Root Cause

### Exact Failure Location
`crates/kcm-runtime/benches/micro.rs:380` — `engine.infer_forward_chaining(&mut fixture.schema).unwrap()` returned `Err(KcmError::OutOfMemory)`.

### Call Stack
```
bench_inference_full_engine (micro.rs:380)
  → InferenceEngine::infer_forward_chaining (inference.rs:125)
    → InferenceEngine::infer_with_stats (inference.rs:56)
      → Schema::append_fact (column.rs:225)
        → Column::append → DenseVec::push (vec.rs:65)
          → if self.len >= self.capacity → Err(KcmError::OutOfMemory)
```

### Root Cause
`infer_forward_chaining` **mutates the schema by appending derived facts**. The benchmark reused the same schema across all Criterion iterations. With `predicate_range=5`, approximately 1/5 of facts (200 for size=1000) match the rule `subject_predicate_object(None, PredicateID(0), None)`. Each Criterion iteration appended ~200 derived facts. Criterion's default `measurement_time=5s` with fast iterations (~0.3ms) ran approximately 16,000+ iterations, accumulating:
```
1,000 (initial) + 16,000 × 200 (derived) = 3,201,000 facts
```
The schema capacity was `max(1000 × 200, 1,000,000) = 1,000,000`. After ~4,500 iterations (1,000 + 4,500 × 200 = 901,000), the schema filled, and `DenseVec::push` returned `OutOfMemory`.

### Why Capacity Was Insufficient
`SchemaFixture::new` used the formula `capacity = max(fact_count × 200, 1_000_000)`. This assumed at most 1,000 iterations. Criterion's actual iteration count depends on measurement_time and iteration speed — for fast operations, it can run tens of thousands of iterations.

### Why Only 1,000 Facts Exhausted Memory
The memory exhaustion was not about the initial dataset size but about **cumulative schema mutation across benchmark iterations**. The schema grew unboundedly because each `b.iter()` call appended derived facts that persisted across iterations.

---

## 2. Allocation Audit

### Schema Allocation (SchemaFixture::new)
| Component | Type | Capacity | Size | Justified |
|-----------|------|----------|------|-----------|
| subject_col | DenseVec\<u32\> | 1,000,000 | 4.0 MB | Over-allocated |
| predicate_col | DenseVec\<u8\> | 1,000,000 | 1.0 MB | Over-allocated |
| object_col | DenseVec\<u32\> | 1,000,000 | 4.0 MB | Over-allocated |
| confidence_col | DenseVec\<f64\> | 1,000,000 | 8.0 MB | Over-allocated |
| evidence_col | DenseVec\<u8\> | 1,000,000 | 1.0 MB | Over-allocated |
| timestamp_col | DenseVec\<i64\> | 1,000,000 | 8.0 MB | Over-allocated |
| context_col | DenseVec\<u8\> | 1,000,000 | 1.0 MB | Over-allocated |
| version_col | DenseVec\<i32\> | 1,000,000 | 4.0 MB | Over-allocated |
| priority_col | DenseVec\<i8\> | 1,000,000 | 1.0 MB | Over-allocated |
| owner_col | DenseVec\<u16\> | 1,000,000 | 2.0 MB | Over-allocated |
| tombstones | Bitmap | 1,000,000 | 0.12 MB | Over-allocated |
| **Total** | | | **~34 MB** | **Should be ~34 KB** |

For 1,000 facts, the schema needed only 34 KB. The `max(..., 1_000_000)` formula wasted ~34 MB.

### Inference Engine Allocations (per iteration)
| Allocation | Size | Justified |
|------------|------|-----------|
| matches Vec | ~200 × (8 + 8 + 24) = ~8 KB | Per-iteration, temporary |
| new_facts Vec | ~200 × 64 = ~12.8 KB | Per-iteration, temporary |
| vec![c] per match | 200 × 24 = ~4.8 KB | Per-iteration, temporary |
| Fact append (schema) | 200 × 34 = ~6.8 KB | Accumulates — root cause |

---

## 3. Changes Made

### 3.1 `bench_inference_full_engine` (micro.rs:356-394)
**Before:** Schema created once outside `b.iter()`, mutated by `infer_forward_chaining` on every iteration. After thousands of iterations, capacity exhausted.

**After:** Facts pre-built as `Vec<Fact>` outside `b.iter()`. Inside each iteration, a fresh schema is created with `capacity = size + size/predicate_range`, facts are inserted, and inference runs. Schema is dropped after each iteration — no mutation accumulates.

**Key design decisions:**
1. Schema capacity = `size + derived_budget` where `derived_budget = size / predicate_range`. This provides exactly enough room for one inference pass.
2. Schema creation + fact insertion happen inside `b.iter()` — this adds setup overhead but ensures deterministic state. For size=1000, setup is ~5µs vs inference ~32µs (setup is ~16% overhead).
3. All `unwrap()` replaced with `expect()` containing diagnostic context.

### 3.2 `SchemaFixture::new` (bench_fixtures.rs:192-224)
**Before:** `capacity = max(fact_count × 200, 1_000_000)` — allocated up to 34 MB for 1,000 facts.

**After:** `capacity = fact_count.max(1)` — allocates exactly what's needed. Benchmarks that need mutating operations must rebuild schemas internally.

### 3.3 All `unwrap()` in benchmark code
**Before:** 28 bare `unwrap()` calls with no diagnostic context.

**After:** All replaced with `expect("descriptive message")` including operation name and benchmark context.

---

## 4. Memory Usage

| Metric | Before | After |
|--------|--------|-------|
| SchemaFixture capacity (size=1000) | 1,000,000 | 1,000 |
| SchemaFixture memory (size=1000) | ~34 MB | ~34 KB |
| SchemaFixture capacity (size=100000) | 20,000,000 | 100,000 |
| SchemaFixture memory (size=100000) | ~680 MB | ~3.4 MB |
| Per-iteration schema (inference bench) | N/A (shared, grew) | ~34 KB (size=1000) |
| Peak memory during inference bench | ~34 MB + accumulated | ~34 KB per iteration |

**Memory reduction: 99.9%** for size=1000, **99.5%** for size=100000.

---

## 5. Benchmark Correctness Verification

| Benchmark | Size | Time | Status |
|-----------|------|------|--------|
| inference_full_engine/1000 | 1K | ~32 µs | ✓ PASS |
| inference_full_engine/10000 | 10K | ~350 µs | ✓ PASS |
| inference_full_engine/100000 | 100K | ~4.0 ms | ✓ PASS |

All three sizes complete without panics. The benchmark now measures inference performance on a fresh, deterministic schema for each iteration.

### Measurement Methodology
Each `b.iter()` call:
1. Creates a fresh `Schema` with capacity = facts + derived budget
2. Inserts all pre-built facts (setup cost included in measurement)
3. Runs `infer_forward_chaining` (pure inference cost)
4. Drops the schema

The measurement includes schema setup overhead (~16% for size=1000, ~5% for size=100000). This is the correct trade-off: deterministic state > zero-overhead measurement for correctness-first benchmarks.

---

## 6. Workspace Validation

| Check | Result |
|-------|--------|
| `cargo fmt --all -- --check` | ✓ CLEAN |
| `cargo clippy --workspace -- -D warnings` | ✓ CLEAN |
| `cargo test --workspace` | ✓ 534 passed, 0 failed |
| `cargo bench -p kcm-runtime --bench micro --no-run` | ✓ Compiles |
| `cargo bench -- "inference_full_engine"` | ✓ All 3 sizes complete |
| Remaining `unwrap()` in benchmarks | 0 |

---

## 7. Architectural Changes Summary

1. **Benchmark isolation:** Inference benchmark now creates a fresh schema per iteration, eliminating cross-iteration state accumulation.
2. **Memory efficiency:** `SchemaFixture` allocates exactly `fact_count` capacity instead of `max(fact_count × 200, 1_000_000)`.
3. **Error handling:** All 28 bare `unwrap()` calls replaced with `expect()` containing diagnostic context.
4. **Correctness guarantee:** Each benchmark iteration starts from a known, deterministic state. No hidden state leaks between iterations.
