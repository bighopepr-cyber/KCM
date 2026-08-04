# kcm-optimizer

Cost-based query optimizer with adaptive execution for KCM.

## Purpose

Transforms logical query plans into optimized physical execution plans using cost-based optimization, statistics-driven decisions, and plan rewriting rules.

## Modules

| Module | Purpose |
|--------|---------|
| `planner` | Query planner — converts logical plans to physical plans |
| `cost_model` | Cost estimation (I/O, CPU, memory) |
| `statistics` | Table and column statistics for cost estimation |
| `rewriting` | Plan rewriting rules (filter pushdown, column pruning) |
| `adaptive` | Adaptive execution with runtime plan adjustment |

## Dependencies

| Dependency | Purpose |
|------------|---------|
| `kcm-core` | Core types |
| `kcm-storage` | Column and index metadata |

## Optimization Pipeline

```
Logical Plan
  -> Filter Pushdown
  -> Column Pruning
  -> Join Reordering
  -> Index Selection
  -> Cost-Based Physical Plan Selection
  -> Adaptive Runtime Adjustment
Physical Plan
```

## Cost Model

Factors:
- **I/O cost**: Pages read from storage
- **CPU cost**: Computation per row
- **Memory cost**: Buffer allocation
- **Network cost**: (distributed mode) Shards accessed

## Usage

```rust
use kcm_optimizer::planner::QueryPlanner;
use kcm_optimizer::statistics::Statistics;

let stats = Statistics::from_store(&store)?;
let planner = QueryPlanner::new(stats);
let physical_plan = planner.optimize(&logical_plan)?;
```

## Adaptive Execution

Runtime monitors:
- Actual vs estimated row counts
- Operator selectivity
- Memory pressure

Adjusts plan mid-execution when estimates deviate significantly.
