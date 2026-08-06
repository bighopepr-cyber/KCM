# kcm-optimizer Technical Specification

---

## Overview

The `kcm-optimizer` crate implements the cost-based query optimizer for the KCM knowledge engine. It transforms logical query plans into efficient physical execution plans through cost estimation, rule-based rewriting, and adaptive runtime adjustment. The optimizer is the bridge between query parsing and query execution, ensuring that every query runs with minimum resource consumption.

**SSOT Reference**: PRD2.md §16 — Optimizer Specification

## Scope

The optimizer covers:

- Cost-based optimization with pluggable cost models
- Volcano-style top-down query planning
- Column-level statistics collection and maintenance
- Rule-based plan rewriting (filter pushdown, column pruning, join reordering, index selection)
- Adaptive execution with runtime plan adjustment

The optimizer does **not** cover:

- Query parsing (handled by `kcm-interface`)
- Physical execution (handled by `kcm-compute`)
- Transaction management (handled by `kcm-runtime`)
- Security enforcement (handled by `kcm-security`)

## Responsibilities

| Responsibility | Description | Owner |
|----------------|-------------|-------|
| Cost-based optimization | Estimate cost of alternative plans and select the cheapest | `CostModel`, `Planner` |
| Query planning | Transform logical plans into physical execution plans | `Planner` |
| Statistics | Collect and maintain column-level statistics for cost estimation | `Statistics` |
| Plan rewriting | Apply algebraic transformations to reduce plan cost | `RuleOptimizer`, `OptimizerPipeline` |
| Adaptive execution | Adjust plans at runtime based on actual data distribution | `AdaptiveOptimizer` |

## Technical Specification

### CostModel — Operator Cost Estimation

The `CostModel` provides cost estimation for individual operators and complete plans.

**Responsibilities**:
- Estimate I/O cost for scan operations
- Estimate CPU cost for filter and projection operations
- Estimate memory cost for join operations
- Estimate network cost for distributed operations (future)
- Aggregate operator costs into total plan cost

**Properties**:
- All cost values are non-negative `f64` values
- Cost accumulation uses saturating arithmetic
- Cost estimates are deterministic for identical inputs
- Cost model parameters are configurable at initialization

**Cost Formula**:

```
TotalCost = Σ (OperatorCost_i × Card_i) + MemoryCost + NetworkCost
```

Where:
- `OperatorCost_i` = per-row cost of operator i
- `Card_i` = estimated cardinality at operator i
- `MemoryCost` = memory allocation cost for materialization
- `NetworkCost` = data transfer cost (0 for single-node)

**SSOT Reference**: PRD2.md §16.1 — Cost Model

### Planner — Volcano-style Top-down Planning

The `Planner` implements a Volcano-style top-down query planner.

**Responsibilities**:
- Enumerate valid plan alternatives for each logical operator
- Apply cost model to compare alternatives
- Select minimum-cost plan
- Enforce resource constraints (memory limits, time limits)
- Support optimizer hints for manual plan selection

**Planning Algorithm**:

1. Receive logical plan from query parser
2. For each logical operator, enumerate physical alternatives
3. Estimate cost of each alternative using `CostModel`
4. Recursively plan child operators
5. Select minimum-cost physical plan
6. Return optimized physical plan

**Planning Constraints**:

| Constraint | Default | Configurable |
|------------|---------|-------------|
| Maximum plan depth | 64 | Yes |
| Maximum plan nodes | 1024 | Yes |
| Planning timeout | 10s | Yes |
| Maximum memory for planning | 64MB | Yes |

**SSOT Reference**: PRD2.md §16.2 — Planner

### Statistics — Column-level Statistics Collection

The `Statistics` component collects and maintains column-level statistics.

**Responsibilities**:
- Track cardinality per column (distinct value count)
- Track selectivity per predicate
- Track data distribution histograms
- Track freshness of statistics
- Provide default statistics when actual statistics are unavailable

**Statistics Model**:

| Statistic | Type | Range | Description |
|-----------|------|-------|-------------|
| `cardinality` | `u64` | [0, MAX] | Number of distinct values |
| `row_count` | `u64` | [0, MAX] | Total number of rows |
| `selectivity` | `f64` | [0.0, 1.0] | Fraction of rows matching predicate |
| `null_count` | `u64` | [0, MAX] | Number of null values |
| `min_value` | `Value` | Varies | Minimum value in column |
| `max_value` | `Value` | Varies | Maximum value in column |
| `histogram` | `Vec<f64>` | Varies | Value distribution buckets |
| `last_updated` | `Instant` | — | Statistics freshness timestamp |

**Freshness Policy**:

| Condition | Action |
|-----------|--------|
| Statistics age < 1 hour | Use cached statistics |
| Statistics age 1–24 hours | Use cached with warning |
| Statistics age > 24 hours | Trigger background refresh |
| Statistics unavailable | Use default fallback statistics |

**SSOT Reference**: PRD2.md §16.3 — Statistics

### Rewriting — Rule-based Plan Transformation

The `RuleOptimizer` applies algebraic transformations to reduce plan cost.

**Rewriting Rules**:

#### Filter Pushdown (`FilterPushdownOptimizer`)

Pushes filter operations closer to data sources to reduce intermediate result size.

**Preconditions**:
- Filter predicate is evaluable at the scan level
- Filter does not depend on data from child operators
- Pushdown does not violate join semantics

**Transformation**:

```
Before:  Join(Scan(A), Filter(Scan(B), pred))
After:   Join(Scan(A), Scan(B, pred))
```

**SSOT Reference**: PRD2.md §16.4.1 — Filter Pushdown

#### Column Pruning (`ColumnPruningOptimizer`)

Removes unused columns from scan and projection operations.

**Preconditions**:
- Column usage is statically determinable
- Pruned columns are not required by downstream operators
- Pruning does not affect correctness of expressions

**Transformation**:

```
Before:  Project(Scan(A, [a,b,c,d]), [a,c])
After:   Project(Scan(A, [a,c]), [a,c])
```

**SSOT Reference**: PRD2.md §16.4.2 — Column Pruning

#### Join Reordering (`JoinOrderingOptimizer`)

Reorders join operations to minimize intermediate result sizes.

**Preconditions**:
- Join is associative (for multi-way joins)
- Join is commutative (for binary joins)
- Statistics are available for join cardinality estimation

**Transformation**:

```
Before:  Join(Join(A, B, pred1), C, pred2)
After:   Join(Join(A, C, pred2), B, pred1)  -- if cheaper
```

**SSOT Reference**: PRD2.md §16.4.3 — Join Reordering

#### Index Selection (`IndexSelectionOptimizer`)

Selects optimal index for scan operations based on predicates and statistics.

**Preconditions**:
- Index exists for the target column
- Index selectivity is better than full scan
- Index is not corrupted or stale

**Transformation**:

```
Before:  Scan(table, pred)
After:   IndexScan(index, pred)
```

**SSOT Reference**: PRD2.md §16.4.4 — Index Selection

### Adaptive — Runtime Plan Adjustment

The `AdaptiveOptimizer` adjusts execution plans at runtime based on observed data distribution.

**Responsibilities**:
- Monitor actual vs. estimated cardinalities during execution
- Trigger re-optimization when estimates deviate beyond threshold
- Switch between plan alternatives without restarting execution
- Collect runtime statistics for future optimization

**Adaptation Triggers**:

| Trigger | Threshold | Action |
|---------|-----------|--------|
| Cardinality deviation | > 2x estimated | Re-optimize subtree |
| Selectivity shift | > 50% change | Re-evaluate join order |
| Memory pressure | > 80% of limit | Switch to streaming plan |
| Latency spike | > 3x expected | Switch to alternative index |

**SSOT Reference**: PRD2.md §16.5 — Adaptive Execution

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      kcm-optimizer                          │
│                                                             │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  │
│  │  CostModel│  │ Planner  │  │Statistics│  │ Adaptive │  │
│  │          │  │          │  │          │  │Optimizer │  │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘  │
│       │              │              │              │        │
│  ┌────┴──────────────┴──────────────┴──────────────┴────┐  │
│  │              OptimizerPipeline                        │  │
│  └──────────────────────────────────────────────────────┘  │
│                                                             │
│  ┌──────────────────────────────────────────────────────┐  │
│  │              RuleOptimizer (Pipeline)                 │  │
│  │  FilterPushdown → ColumnPruning → JoinOrdering       │  │
│  │                 → IndexSelection                     │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
         │                                    │
         ▼                                    ▼
┌────────────────┐                  ┌────────────────┐
│   kcm-core     │                  │  kcm-storage   │
│  (Types, BTree)│                  │ (Schema, Index) │
└────────────────┘                  └────────────────┘
```

## Internal Components

### cost_model.rs

| Item | Type | Description |
|------|------|-------------|
| `CostModel` | Struct | Main cost estimation engine |
| `OperatorCost` | Struct | Cost breakdown for a single operator |
| `CostParameters` | Struct | Configurable cost model parameters |

### planner.rs

| Item | Type | Description |
|------|------|-------------|
| `Planner` | Struct | Volcano-style top-down query planner |
| `QueryPlan` | Struct | Optimized physical execution plan |
| `PlanNode` | Enum | Logical/physical plan node representation |

### statistics.rs

| Item | Type | Description |
|------|------|-------------|
| `Statistics` | Struct | Column-level statistics collection and management |
| `ColumnStatistics` | Struct | Per-column statistics |
| `Histogram` | Struct | Value distribution representation |

### rewriting.rs

| Item | Type | Description |
|------|------|-------------|
| `RuleOptimizer` | Trait | Interface for plan rewriting rules |
| `FilterPushdownOptimizer` | Struct | Filter pushdown rule |
| `ColumnPruningOptimizer` | Struct | Column pruning rule |
| `JoinOrderingOptimizer` | Struct | Join reordering rule |
| `IndexSelectionOptimizer` | Struct | Index selection rule |

### adaptive.rs

| Item | Type | Description |
|------|------|-------------|
| `AdaptiveOptimizer` | Struct | Runtime plan adjustment engine |
| `AdaptationTrigger` | Enum | Events that trigger plan re-optimization |
| `RuntimeStatistics` | Struct | Observed execution statistics |

## Data Model

### PlanNode Enum

```rust
pub enum PlanNode {
    Scan {
        table: SubjectID,
        columns: Vec<u32>,
        predicate: Option<Expr>,
    },
    IndexScan {
        index_id: u32,
        predicate: Expr,
    },
    Filter {
        input: Box<PlanNode>,
        predicate: Expr,
    },
    Project {
        input: Box<PlanNode>,
        expressions: Vec<Expr>,
    },
    Join {
        left: Box<PlanNode>,
        right: Box<PlanNode>,
        predicate: Expr,
        join_type: JoinType,
    },
    Aggregate {
        input: Box<PlanNode>,
        group_by: Vec<Expr>,
        aggregates: Vec<AggregateExpr>,
    },
    Sort {
        input: Box<PlanNode>,
        order_by: Vec<SortKey>,
    },
    Limit {
        input: Box<PlanNode>,
        count: u64,
    },
}
```

### CostModel Struct

```rust
pub struct CostModel {
    io_cost_per_page: f64,
    cpu_cost_per_row: f64,
    memory_cost_per_byte: f64,
    network_cost_per_byte: f64,
}
```

### Statistics Struct

```rust
pub struct Statistics {
    columns: HashMap<ColumnID, ColumnStatistics>,
    row_count: u64,
    last_updated: Instant,
}
```

## Execution Flow

```
Query Input
    │
    ▼
┌──────────────┐
│ Query Parse  │  (kcm-interface)
└──────┬───────┘
       │
       ▼
┌──────────────┐
│  Optimize    │  (kcm-optimizer)
│  ├─ Collect  │  Statistics
│  ├─ Plan     │  Planner + CostModel
│  ├─ Rewrite  │  RuleOptimizer
│  └─ Verify   │  Plan verification
└──────┬───────┘
       │
       ▼
┌──────────────┐
│  Execute     │  (kcm-compute)
│  ├─ Adaptive │  AdaptiveOptimizer
│  └─ Monitor  │  RuntimeStatistics
└──────┬───────┘
       │
       ▼
    Results
```

## Public API

### OptimizerPipeline

```rust
pub struct OptimizerPipeline {
    rules: Vec<Box<dyn RuleOptimizer>>,
}

impl OptimizerPipeline {
    pub fn new() -> Self;
    pub fn add_rule(&mut self, rule: Box<dyn RuleOptimizer>);
    pub fn optimize(&self, plan: PlanNode) -> Result<PlanNode, KcmError>;
}
```

### Planner

```rust
pub struct Planner {
    cost_model: CostModel,
    statistics: Arc<RwLock<Statistics>>,
}

impl Planner {
    pub fn new(cost_model: CostModel, statistics: Arc<RwLock<Statistics>>) -> Self;
    pub fn plan(&self, logical_plan: PlanNode) -> Result<QueryPlan, KcmError>;
    pub fn explain(&self, plan: &PlanNode) -> String;
}
```

### CostModel

```rust
pub struct CostModel { /* ... */ }

impl CostModel {
    pub fn new() -> Self;
    pub fn estimate_operator_cost(&self, node: &PlanNode) -> Result<OperatorCost, KcmError>;
    pub fn estimate_total_cost(&self, plan: &PlanNode) -> Result<f64, KcmError>;
}
```

### Statistics

```rust
pub struct Statistics { /* ... */ }

impl Statistics {
    pub fn new() -> Self;
    pub fn collect(&mut self, schema: &Schema) -> Result<(), KcmError>;
    pub fn column_statistics(&self, column_id: ColumnID) -> Option<&ColumnStatistics>;
    pub fn is_stale(&self) -> bool;
}
```

## Configuration

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `max_plan_depth` | `usize` | 64 | Maximum plan tree depth |
| `max_plan_nodes` | `usize` | 1024 | Maximum plan node count |
| `planning_timeout_ms` | `u64` | 10000 | Planning timeout in milliseconds |
| `statistics_staleness_threshold_secs` | `u64` | 3600 | Statistics freshness threshold |
| `adaptive_cardinality_deviation` | `f64` | 2.0 | Cardinality deviation threshold for re-optimization |
| `adaptive_selectivity_threshold` | `f64` | 0.5 | Selectivity change threshold |
| `cost_model_io_weight` | `f64` | 1.0 | I/O cost weight |
| `cost_model_cpu_weight` | `f64` | 1.0 | CPU cost weight |
| `cost_model_memory_weight` | `f64` | 1.0 | Memory cost weight |

## Dependencies

| Dependency | Type | Justification |
|------------|------|---------------|
| `kcm-core` | Internal | Core types (`KcmError`, `Fact`, `RowID`, `SubjectID`, `Confidence`) |
| `kcm-storage` | Internal | Schema access, column metadata, dictionary, index information |
| `parking_lot` | External | 3-5x faster synchronization than `std::sync` |

**Prohibited Dependencies**:
- `kcm-compute` — optimizer plans, compute executes
- `kcm-reasoning` — reasoning consumes optimizer output
- `kcm-runtime` — runtime orchestrates optimizer
- `kcm-interface` — interface layer is above optimizer

## Error Handling

All public APIs return `Result<T, KcmError>`. Error variants used by the optimizer:

| Variant | Usage |
|---------|-------|
| `KcmError::InvalidArgument(String)` | Invalid plan structure or cost model parameters |
| `KcmError::NotFound(String)` | Missing statistics or index for optimization |
| `KcmError::OutOfMemory` | Planning exceeds memory limit |
| `KcmError::Conflict(String)` | Concurrent statistics update conflict |

**Error Propagation Pattern**:

```rust
pub fn optimize(&self, plan: PlanNode) -> Result<PlanNode, KcmError> {
    let stats = self.statistics.read()
        .map_err(|e| KcmError::Conflict(format!("Statistics lock: {}", e)))?;
    
    let cost = self.cost_model.estimate_total_cost(&plan)?;
    // ...
    Ok(optimized_plan)
}
```

## Performance Characteristics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Optimizer overhead | < 5% of total query time | Criterion benchmarks |
| Simple query planning | < 1ms | Criterion benchmarks |
| Complex query planning | < 10ms | Criterion benchmarks |
| Statistics collection | < 1% of query time | Criterion benchmarks |
| Memory overhead per plan | < 64KB | Memory profiling |
| Rule application (per rule) | < 100μs | Criterion benchmarks |

## Security Considerations

- Cost model inputs are validated at the API boundary
- Statistics values are bounded to prevent overflow
- Plan trees have maximum depth limits
- No `unwrap()` in production code paths
- Security context is propagated through plan nodes
- No sensitive data is logged or serialized

See [kcm-optimizer SECURITY.md](../../crates/kcm-optimizer/SECURITY.md) for details.

## Integration

The optimizer integrates with:

| Component | Direction | Interface |
|-----------|-----------|-----------|
| `kcm-interface` | Receives from | Parsed logical plans |
| `kcm-compute` | Sends to | Optimized physical plans |
| `kcm-runtime` | Receives from | Configuration, statistics refresh triggers |
| `kcm-storage` | Reads from | Schema, columns, indexes, dictionaries |
| `kcm-core` | Uses | Core types and error model |
| `kcm-security` | Indirect | Security context propagation |

## Sequence Diagram

```
┌─────────┐    ┌───────────┐    ┌──────────┐    ┌───────────┐    ┌──────────┐
│ Interface│    │ Optimizer │    │Statistics│    │CostModel  │    │ Compute  │
└────┬─────┘    └─────┬─────┘    └────┬─────┘    └─────┬─────┘    └────┬─────┘
     │                │               │                 │               │
     │  LogicalPlan   │               │                 │               │
     │───────────────>│               │                 │               │
     │                │  GetStats     │                 │               │
     │                │──────────────>│                 │               │
     │                │  Statistics   │                 │               │
     │                │<──────────────│                 │               │
     │                │               │                 │               │
     │                │  EstimateCost │                 │               │
     │                │───────────────────────────────>│               │
     │                │  OperatorCost │                 │               │
     │                │<───────────────────────────────│               │
     │                │               │                 │               │
     │                │  ApplyRules   │                 │               │
     │                │──────────────>│                 │               │
     │                │  Transformed  │                 │               │
     │                │<──────────────│                 │               │
     │                │               │                 │               │
     │  PhysicalPlan  │               │                 │               │
     │<───────────────│               │                 │               │
     │                │               │                 │               │
     │  ExecutePlan   │               │                 │               │
     │────────────────────────────────────────────────────────────────>│
     │                │               │                 │               │
     │                │  AdaptNotify  │                 │               │
     │                │<──────────────────────────────────────────────│
     │                │               │                 │               │
     │                │  Re-optimize  │                 │               │
     │                │───────────────────────────────>│               │
     │                │  NewPlan      │                 │               │
     │                │<───────────────────────────────│               │
```

## Architecture Diagram

```
                    ┌─────────────────────────────────────┐
                    │            kcm-optimizer             │
                    │                                     │
                    │  ┌─────────────────────────────┐   │
                    │  │      OptimizerPipeline       │   │
                    │  │                             │   │
                    │  │  ┌───────────────────────┐  │   │
                    │  │  │ FilterPushdownOptimizer│  │   │
                    │  │  └───────────────────────┘  │   │
                    │  │  ┌───────────────────────┐  │   │
                    │  │  │ ColumnPruningOptimizer │  │   │
                    │  │  └───────────────────────┘  │   │
                    │  │  ┌───────────────────────┐  │   │
                    │  │  │JoinOrderingOptimizer  │  │   │
                    │  │  └───────────────────────┘  │   │
                    │  │  ┌───────────────────────┐  │   │
                    │  │  │IndexSelectionOptimizer│  │   │
                    │  │  └───────────────────────┘  │   │
                    │  └─────────────────────────────┘   │
                    │                                     │
                    │  ┌──────────────┐  ┌────────────┐  │
                    │  │   Planner    │  │ CostModel  │  │
                    │  │  (Volcano)   │  │            │  │
                    │  └──────┬───────┘  └─────┬──────┘  │
                    │         │                │          │
                    │  ┌──────┴────────────────┴──────┐  │
                    │  │        Statistics             │  │
                    │  └──────────────────────────────┘  │
                    │                                     │
                    │  ┌──────────────────────────────┐  │
                    │  │     AdaptiveOptimizer        │  │
                    │  └──────────────────────────────┘  │
                    └─────────────────────────────────────┘
                              │                │
                    ┌─────────┴──┐    ┌────────┴────────┐
                    │  kcm-core  │    │  kcm-storage    │
                    │ Types,BTree│    │ Schema, Index   │
                    └────────────┘    └─────────────────┘
```

## References

| Document | Section | Authority |
|----------|---------|-----------|
| `docs/PRD2.md` | §16 | Optimizer specification (authoritative) |
| `docs/PRD.md` | §5 | Query engine operator definitions |
| `AGENTS.md` | — | Engineering constitution |
| `docs/SSOT.md` | — | Single Source of Truth registry |

## SSOT Alignment

This specification is aligned with the following SSOT documents:

| SSOT Requirement | Document | Section | Status |
|------------------|----------|---------|--------|
| Cost model specification | PRD2.md | §16.1 | ✅ Aligned |
| Planner specification | PRD2.md | §16.2 | ✅ Aligned |
| Statistics specification | PRD2.md | §16.3 | ✅ Aligned |
| Rewriting rules | PRD2.md | §16.4 | ✅ Aligned |
| Adaptive execution | PRD2.md | §16.5 | ✅ Aligned |
| Operator trait definitions | PRD.md | §5 | ✅ Aligned |
| Error model | AGENTS.md | Error Model | ✅ Aligned |
| Concurrency model | AGENTS.md | Concurrency Model | ✅ Aligned |
| Testing strategy | PRD-TESTING | §1-8 | ✅ Aligned |
