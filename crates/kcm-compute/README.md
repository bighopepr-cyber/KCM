# kcm-compute

Relational algebra compute engine with SIMD AVX2 acceleration for KCM.

## Purpose

Implements Volcano-style pull-based query execution operators for filtering, projection, joins, and aggregation over columnar data. Includes SIMD-accelerated kernels for hot paths.

## Modules

| Module | Purpose |
|--------|---------|
| `algebra` | Relational algebra operators (Scan, Filter, Project, Join, Aggregate) |
| `simd` | AVX2-accelerated vectorized operations |

## Dependencies

| Dependency | Purpose |
|------------|---------|
| `kcm-core` | Core types |
| `kcm-storage` | Columnar data access |

## Operators

| Operator | Description |
|----------|-------------|
| ScanOp | Reads rows from columnar storage |
| FilterOp | Applies predicate filtering |
| ProjectOp | Selects specific columns |
| JoinOp | Equi-joins on column values |
| AggregateOp | Group-by aggregation (COUNT, SUM, AVG, MIN, MAX) |

## Execution Model

Pull-based Volcano iterator model:
```
next() call flows: Aggregate -> Join -> Filter -> Scan
```

Each operator implements `Operator` trait:

```rust
pub trait Operator {
    fn next(&mut self) -> Option<Result<Vec<Row>, KcmError>>;
    fn reset(&mut self);
}
```

## SIMD Acceleration

AVX2 kernels are used for:
- Bitmap rank/select
- Dictionary lookup
- Filter predicate evaluation
- Aggregate accumulation

Falls back to scalar code on non-AVX2 platforms.

## Usage

```rust
use kcm_compute::algebra::*;

let scan = ScanOp::new(&store, columns)?;
let filter = FilterOp::new(scan, predicate)?;
let project = ProjectOp::new(filter, output_columns)?;

for row in project {
    println!("{:?}", row?);
}
```
