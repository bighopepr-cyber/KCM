# kcm-compute Technical Specification

## Overview

`kcm-compute` is the compute engine of the KCM (Knowledge Columnar Model) system. It implements relational algebra operators using Volcano-style pull-based execution and provides SIMD AVX2 acceleration for column scan, filter, and aggregate operations. The crate is the execution layer that transforms logical query plans into materialized results.

## Scope

This specification covers the `kcm-compute` crate only. It does not cover core types, storage, optimization, reasoning, or any higher-level functionality.

## Responsibilities

| Responsibility | Description |
|---------------|-------------|
| Relational algebra | `ScanOp`, `FilterOp`, `ProjectOp`, `JoinOp`, `AggregateOp` — standard relational operators |
| SIMD acceleration | AVX2-accelerated column scan, filter, and count operations for u8, u32, and f64 data types |
| Cost estimation | `estimated_rows()` on every operator for query planner integration |

## Technical Specification

### Relational Algebra Operators

All operators implement the `Operator` trait:

```rust
pub trait Operator: Send + Sync {
    fn execute(&self) -> Result<Vec<usize>, KcmError>;
    fn estimated_rows(&self) -> usize;
}
```

Operators return `Vec<usize>` — a list of row IDs. This is the Volcano iterator model: each operator pulls row IDs from its child operator and filters or transforms them.

#### ScanOp

Full-table scan with optional context and confidence filters.

```rust
pub struct ScanOp<'a> {
    schema: &'a Schema,
    context_filter: Option<u8>,
    confidence_filter: Option<f64>,
}
```

| Operation | Complexity | Description |
|-----------|-----------|-------------|
| `new(schema)` | O(1) | Create scan operator |
| `with_context(ctx)` | O(1) | Set context filter (builder pattern) |
| `with_confidence(conf)` | O(1) | Set confidence filter (builder pattern) |
| `execute()` | O(n) | Scan all rows, apply filters, return matching row IDs |
| `estimated_rows()` | O(1) | Heuristic: 10% for context filter, 30% for confidence, 100% for unfiltered |

**Semantics:** Iterates all rows in the schema. Deleted rows (`is_deleted(idx) == true`) are skipped. When `context_filter` is set, only rows matching the context are included. When `confidence_filter` is set, only rows with confidence ≥ threshold are included.

#### FilterOp

Predicate-based row filtering.

```rust
pub enum FilterPredicate {
    EqualSubject(u32),
    EqualPredicate(u8),
    EqualObject(u32),
    EqualContext(u8),
    InSet(std::collections::HashSet<u32>),
    RangeTimestamp(i64, i64),
}

pub struct FilterOp<'a> {
    rowids: Vec<usize>,
    schema: &'a Schema,
    predicate: FilterPredicate,
}
```

| Operation | Complexity | Description |
|-----------|-----------|-------------|
| `new(rowids, schema, predicate)` | O(1) | Create filter operator |
| `execute()` | O(m) | Filter input row IDs by predicate, m = input rows |
| `estimated_rows()` | O(1) | Heuristic selectivity per predicate type |

**Selectivity estimates:**

| Predicate | Selectivity |
|-----------|------------|
| EqualSubject | 5% |
| EqualPredicate | 15% |
| EqualObject | 5% |
| EqualContext | 20% |
| InSet | set_size / 255, clamped [1%, 50%] |
| RangeTimestamp | 30% |

#### ProjectOp

Column projection with materialization.

```rust
pub struct ProjectOp<'a> {
    rowids: Vec<usize>,
    schema: &'a Schema,
    columns: Vec<ColumnID>,
}
```

| Operation | Complexity | Description |
|-----------|-----------|-------------|
| `new(rowids, schema, columns)` | O(1) | Create projection operator |
| `execute()` | O(m) | Pass-through row IDs (pipeline compatible) |
| `execute_projection()` | O(m × k) | Materialize k columns for m rows as `Vec<Vec<u64>>` |
| `estimated_rows()` | O(1) | Returns input row count (no filtering) |

**Semantics:** `execute()` passes row IDs through unchanged for pipeline compatibility. `execute_projection()` extracts the selected columns and returns materialized values. Each value is cast to `u64` (or `f64::to_bits()` for Confidence).

#### JoinOp

Hash-based inner join.

```rust
pub struct JoinOp<'a> {
    left_rowids: Vec<usize>,
    right_rowids: Vec<usize>,
    schema: &'a Schema,
    join_column: ColumnID,
}
```

| Operation | Complexity | Description |
|-----------|-----------|-------------|
| `new(left, right, schema, join_column)` | O(1) | Create join operator |
| `execute()` | O(L + R) | Hash join on specified column |
| `estimated_rows()` | O(1) | Heuristic: L × R / distinct_right |

**Semantics:** Builds a hash table from the right relation on the join column. Probes with the left relation. Result is interleaved row ID pairs `[left_id, right_id, ...]`. Only columns that can be cast to `u32` are valid join keys (Subject, Object, Predicate, Context, Evidence, Owner).

#### AggregateOp

Aggregation with optional grouping.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AggregateFunc {
    Count,
    Sum,
    Avg,
    Min,
    Max,
}

pub struct AggregateOp<'a> {
    rowids: Vec<usize>,
    schema: &'a Schema,
    group_by: Option<ColumnID>,
    agg_func: AggregateFunc,
}
```

| Operation | Complexity | Description |
|-----------|-----------|-------------|
| `new(rowids, schema, group_by, agg_func)` | O(1) | Create aggregate operator |
| `execute()` | O(m) | Pass-through row IDs (pipeline compatible) |
| `execute_aggregate()` | O(m) | Compute aggregate over confidence values |
| `execute_grouped()` | O(m × g) | Compute aggregate grouped by column, g = groups |
| `estimated_rows()` | O(1) | 1 for ungrouped, distinct group count for grouped |

**Semantics:** Aggregation operates on the `confidence_col` of the schema. `Count` returns the number of non-null confidence values. `Sum` sums all confidence values. `Avg` computes the arithmetic mean. `Min` and `Max` find the extremes. For grouped aggregation, the group key is extracted from the specified `group_by` column.

### SIMD Acceleration

```rust
pub trait SimdOps<T: Copy> {
    fn simd_filter_eq(&self, value: T) -> Vec<bool>;
    fn simd_filter_ge(&self, value: T) -> Vec<bool>;
    fn simd_count(&self) -> usize;
}
```

Implemented for `[u8]`, `[u32]`, and `[f64]`.

#### AVX2 Operations (x86_64 only)

| Function | Target | Chunk Size | Description |
|----------|--------|-----------|-------------|
| `avx2_filter_eq_u8` | `[u8]` | 32 bytes | Element-wise equality test using `_mm256_cmpeq_epi8` |
| `avx2_filter_ge_u8` | `[u8]` | 32 bytes | Element-wise ≥ test using `_mm256_max_epu8` |
| `avx2_count_nonzero_u8` | `[u8]` | 32 bytes | Count non-zero elements using `_mm256_cmpeq_epi8` |
| `avx2_filter_eq_u32` | `[u32]` | 8 elements | Element-wise equality test using `_mm256_cmpeq_epi32` |
| `avx2_filter_ge_u32` | `[u32]` | 8 elements | Element-wise ≥ test using `_mm256_max_epu32` |

**Fallback:** Every SIMD function has a scalar fallback for non-x86_64 platforms and when AVX2 is not detected at runtime.

**Safety:** All SIMD functions are gated behind `is_x86_feature_detected!("avx2")` runtime checks. The `unsafe` block is only entered after the feature is confirmed present.

## Architecture

```
kcm-compute
  ├── algebra.rs      → Relational algebra operators (Scan, Filter, Project, Join, Aggregate)
  ├── simd.rs         → AVX2-accelerated column operations
  └── lib.rs          → Module declarations
```

## Internal Components

### algebra.rs

Implements the `Operator` trait and five operator structs. Each operator holds a reference to the schema and input row IDs. The `Operator::execute()` method returns filtered or transformed row IDs. `estimated_rows()` provides cardinality estimates for the query planner.

### simd.rs

Implements the `SimdOps<T>` trait with AVX2-accelerated functions for `u8` and `u32` slices. Functions process data in 32-byte (u8) or 8-element (u32) chunks using AVX2 intrinsics. Remainder elements are processed with scalar fallback. The `f64` implementation uses scalar operations only (no AVX2 for f64 comparisons).

## Data Model

### Operator Pipeline

```
Input: Schema (columns stored in DenseVec<T>)

ScanOp    → filter rows by context/confidence → Vec<RowID>
FilterOp  → apply predicate to row IDs        → Vec<RowID>
ProjectOp → extract selected columns           → Vec<Vec<u64>>
JoinOp    → hash join two row sets             → Vec<RowID> (interleaved pairs)
AggregateOp → compute aggregate values          → f64 or Vec<(u32, f64)>
```

### SIMD Data Flow

```
Input: &[u8] or &[u32] column slice
  → is_x86_feature_detected!("avx2")
  → Yes: process in 32/8-element AVX2 chunks
  → No:  process element-by-element
Output: Vec<bool> (filter) or usize (count)
```

## Execution Flow

### Operator Pipeline: Scan → Filter → Project → Join → Aggregate

```
1. ScanOp.execute()
   → Iterate schema rows, skip deleted
   → Apply context_filter, confidence_filter
   → Return Vec<RowID>

2. FilterOp.execute(input_rowids)
   → For each row ID, evaluate FilterPredicate
   → Return matching row IDs

3. ProjectOp.execute(filtered_rowids)
   → Pass through row IDs (pipeline compatible)

4. JoinOp.execute(left_rowids, right_rowids)
   → Build hash table from right relation
   → Probe with left relation
   → Return interleaved [left_id, right_id, ...] pairs

5. AggregateOp.execute(joined_rowids)
   → Group by column (if specified)
   → Apply aggregate function to confidence values
   → Return aggregate result
```

### SIMD Filter Execution

```
1. Receive column slice &[u8] or &[u32]
2. Check: is_x86_feature_detected!("avx2")
   → Yes: call avx2_filter_eq_u8 / avx2_filter_ge_u8 / etc.
   → No:  call scalar fallback
3. Process 32 bytes (u8) or 8 elements (u32) per chunk
4. Process remainder elements scalar
5. Return Vec<bool> — one bool per element
```

## Public API

| API | Description |
|-----|-------------|
| `Operator` trait | Common interface for all operators: `execute()`, `estimated_rows()` |
| `ScanOp::new(schema)` | Create a scan operator with optional filters |
| `ScanOp::with_context(self, ctx)` | Builder: set context filter |
| `ScanOp::with_confidence(self, conf)` | Builder: set confidence filter |
| `FilterOp::new(rowids, schema, predicate)` | Create a filter operator |
| `ProjectOp::new(rowids, schema, columns)` | Create a projection operator |
| `ProjectOp::execute_projection()` | Materialize selected columns as `Vec<Vec<u64>>` |
| `JoinOp::new(left, right, schema, column)` | Create a hash join operator |
| `AggregateOp::new(rowids, schema, group_by, func)` | Create an aggregate operator |
| `AggregateOp::execute_aggregate()` | Compute ungrouped aggregate as `f64` |
| `AggregateOp::execute_grouped()` | Compute grouped aggregate as `Vec<(u32, f64)>` |
| `SimdOps<T>` trait | SIMD operations: `simd_filter_eq`, `simd_filter_ge`, `simd_count` |
| `FilterPredicate` enum | Six predicate variants for filter operators |
| `AggregateFunc` enum | Five aggregate functions: Count, Sum, Avg, Min, Max |

## Configuration

No configuration options. `kcm-compute` is a stateless computation library. Operator behavior is determined entirely at construction time.

## Dependencies

| Dependency | Type | Justification |
|-----------|------|---------------|
| `kcm-core` | Runtime | Core types (`Fact`, `KcmError`, `ColumnID`, `DenseVec`) |
| `kcm-storage` | Runtime | `Schema` and column storage types |

## Error Handling

All public APIs return `Result<T, KcmError>`. Error variants:

| Variant | Usage in kcm-compute |
|---------|---------------------|
| `InvalidArgument` | No group_by column specified for grouped aggregation |
| `NotFound` | Reserved for future use |
| Other variants | Propagated from upstream or reserved for future use |

## Performance Characteristics

| Operation | Target | Measurement |
|-----------|--------|-------------|
| `ScanOp::execute` (unfiltered) | O(n) | Linear scan |
| `FilterOp::execute` | O(m) | Linear filter on input rows |
| `JoinOp::execute` | O(L + R) | Hash build + probe |
| `AggregateOp::execute_aggregate` | O(m) | Linear aggregation |
| `simd_filter_eq_u8` | ≥4x vs scalar | Criterion benchmark |
| `simd_filter_ge_u8` | ≥4x vs scalar | Criterion benchmark |
| `simd_count_nonzero_u8` | ≥4x vs scalar | Criterion benchmark |
| `simd_filter_eq_u32` | ≥4x vs scalar | Criterion benchmark |
| `simd_filter_ge_u32` | ≥4x vs scalar | Criterion benchmark |

**SIMD speedup targets:**
- u8 operations: ≥4x throughput improvement over scalar on AVX2 hardware
- u32 operations: ≥4x throughput improvement over scalar on AVX2 hardware
- f64 operations: Scalar only (no AVX2 implementation), no speedup target

## Security Considerations

- All SIMD functions are gated behind runtime AVX2 detection
- All SIMD `unsafe` blocks have `// SAFETY:` comments
- SIMD chunk processing uses `chunks_exact` — no buffer overruns possible
- Scalar remainder fallback handles non-chunk-aligned data
- Aggregation handles empty input without panic
- No `unwrap()` or `panic!()` in production code paths

## Integration

`kcm-compute` is consumed by higher-level KCM crates:

```
kcm-core      ← kcm-compute (types, DenseVec)
kcm-storage   ← kcm-compute (Schema, columns)
kcm-compute   ← kcm-optimizer (operator construction)
kcm-compute   ← kcm-runtime (query execution)
```

## Sequence Diagram

### Query Execution Flow

```
Runtime → Construct ScanOp(schema)
         → scan.execute() → Vec<RowID>
         → Construct FilterOp(rowids, schema, predicate)
         → filter.execute() → Vec<RowID>
         → Construct ProjectOp(rowids, schema, columns)
         → project.execute_projection() → Vec<Vec<u64>>
         → Return results
```

### Join Execution Flow

```
Runtime → Construct JoinOp(left_rowids, right_rowids, schema, ColumnID::Subject)
         → join.execute()
         → Build hash table from right_rowids on Subject column
         → Probe hash table with left_rowids
         → Return interleaved [left_id, right_id, ...] pairs
```

### SIMD Filter Flow

```
Column slice → simd_filter_eq(value)
  → is_x86_feature_detected!("avx2")?
  → Yes: avx2_filter_eq_u8(data, value)
          → Process 32-byte chunks with AVX2
          → Process remainder scalar
          → Return Vec<bool>
  → No:  scalar filter
          → Process element-by-element
          → Return Vec<bool>
```

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────┐
│                    kcm-compute                          │
├─────────────────────────────────────────────────────────┤
│  algebra.rs                │  simd.rs                   │
│  ┌──────────────────────┐  │  ┌──────────────────────┐  │
│  │ Operator trait        │  │  │ SimdOps<T> trait     │  │
│  │ ScanOp               │  │  │ avx2_filter_eq_u8    │  │
│  │ FilterOp             │  │  │ avx2_filter_ge_u8    │  │
│  │ ProjectOp            │  │  │ avx2_count_nonzero   │  │
│  │ JoinOp               │  │  │ avx2_filter_eq_u32   │  │
│  │ AggregateOp          │  │  │ avx2_filter_ge_u32   │  │
│  │ FilterPredicate      │  │  │ Scalar fallbacks     │  │
│  │ AggregateFunc        │  │  └──────────────────────┘  │
│  └──────────────────────┘  │                            │
├────────────────────────────┴────────────────────────────┤
│  Dependencies: kcm-core (types), kcm-storage (Schema)  │
└─────────────────────────────────────────────────────────┘
```

## References

- [PRD.md](../specs/PRD.md) §5 — Query engine specification
- [SSOT.md](../../SSOT.md) — Single Source of Truth
- [AGENTS.md](../../AGENTS.md) — Engineering constitution
- [KCM_SPECIFICATION.md](../specs/KCM_SPECIFICATION.md) — Technical constitution
- [README.md](../../crates/kcm-compute/README.md) — Crate overview

## SSOT Alignment

| SSOT Requirement | Specification | Implementation | Test |
|-----------------|---------------|----------------|------|
| R-QUERY-001 | Scan operator with context/confidence filters | `algebra.rs:ScanOp` | `tests/test_algebra.rs` |
| R-QUERY-002 | Filter operator with predicate variants | `algebra.rs:FilterOp` | `tests/test_algebra.rs` |
| R-QUERY-003 | Project operator with column selection | `algebra.rs:ProjectOp` | `tests/test_algebra.rs` |
| R-QUERY-004 | Join operator with hash join strategy | `algebra.rs:JoinOp` | `tests/test_algebra.rs` |
| R-QUERY-005 | Aggregate operator with group-by support | `algebra.rs:AggregateOp` | `tests/test_algebra.rs` |
| R-QUERY-006 | Volcano-style pull-based execution | `Operator` trait | `tests/test_algebra.rs` |
| R-SIMD-001 | AVX2 acceleration for column scan | `simd.rs:avx2_filter_eq_u8` | `simd::tests` |
| R-SIMD-002 | AVX2 acceleration for column filter | `simd.rs:avx2_filter_ge_u8` | `simd::tests` |
| R-SIMD-003 | AVX2 acceleration for column count | `simd.rs:avx2_count_nonzero_u8` | `simd::tests` |
| R-SIMD-004 | Scalar fallback for non-AVX2 platforms | `SimdOps` impls | `simd::tests` |
