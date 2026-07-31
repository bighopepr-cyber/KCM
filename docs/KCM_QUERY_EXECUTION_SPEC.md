# KCM Query Execution Specification

**Document ID:** KCM-QUERY-001  
**Version:** 1.0.0  
**Depends on:** KCM-DATA-001, KCM_ARCHITECTURE-001

---

## 1. Purpose

Defines the query execution model, operator specifications, and optimization pipeline.

---

## 2. Query Lifecycle

```
┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐
│  KQL /   │───►│  Parser  │───►│  Planner │───►│ Executor │───►│  Result  │
│  API     │    │  (AST)   │    │  (Plan)  │    │  (Ops)   │    │  (Facts) │
└──────────┘    └──────────┘    └──────────┘    └──────────┘    └──────────┘
```

### 2.1 Execution Model

KCM uses a **pull-based volcano model** for query execution:
- Each operator implements the `Operator` trait
- `execute()` returns `Result<Vec<usize>, KcmError>` (row ID set)
- Operators compose by passing row ID sets downstream

### 2.2 Operator Trait

```rust
trait Operator: Send + Sync {
    fn execute(&self) -> Result<Vec<usize>, KcmError>;
    fn estimated_rows(&self) -> usize;
}
```

---

## 3. Operators

### 3.1 ScanOp

| Field | Type | Description |
|-------|------|-------------|
| schema | &Schema | Reference to columnar schema |
| context_filter | Option<u8> | Optional context ID filter |
| confidence_filter | Option<f64> | Optional minimum confidence |

**Behavior:**
1. Iterate all rows (0..schema.len())
2. Skip tombstone-deleted rows
3. Apply context filter (if set): skip rows where context != filter
4. Apply confidence filter (if set): skip rows where confidence < filter
5. Return surviving row indices

**Complexity:** O(n) where n = row count

### 3.2 FilterOp

| Field | Type | Description |
|-------|------|-------------|
| rowids | Vec<usize> | Input row set |
| schema | &Schema | Columnar schema |
| predicate | FilterPredicate | Filter condition |

**FilterPredicate variants:**
- `EqualSubject(u32)` — Match subject column
- `EqualPredicate(u8)` — Match predicate column
- `EqualObject(u32)` — Match object column
- `EqualContext(u8)` — Match context column
- `InSet(HashSet<u32>)` — Match against value set (O(1) per lookup)
- `RangeTimestamp(i64, i64)` — Match timestamp range

**Behavior:** Filter input rows by predicate.

### 3.3 ProjectOp

| Field | Type | Description |
|-------|------|-------------|
| rowids | Vec<usize> | Input row set |
| schema | &Schema | Columnar schema |
| columns | Vec<ColumnID> | Columns to extract |

**Behavior:** `execute()` passes through row IDs. `execute_projection()` extracts actual column values as `Vec<Vec<u64>>`.

### 3.4 JoinOp

| Field | Type | Description |
|-------|------|-------------|
| left_rowids | Vec<usize> | Left input |
| right_rowids | Vec<usize> | Right input |
| schema | &Schema | Columnar schema |
| join_column | ColumnID | Column for equi-join key |

**Algorithm:** Hash join
1. Build hash table from right side on join_column
2. Probe with left side
3. Return concatenated matched row ID pairs

### 3.5 AggregateOp

| Field | Type | Description |
|-------|------|-------------|
| rowids | Vec<usize> | Input row set |
| schema | &Schema | Columnar schema |
| group_by | Option<ColumnID> | Grouping column |
| agg_func | AggregateFunc | Aggregation function |

**AggregateFunc variants:**
- `Count` — Count matching rows
- `Sum` — Sum confidence values
- `Avg` — Average confidence values
- `Min` — Minimum confidence value
- `Max` — Maximum confidence value

**Methods:**
- `execute_aggregate() -> Result<f64>` — Global aggregate
- `execute_grouped() -> Result<Vec<(u32, f64)>>` — Grouped aggregate

---

## 4. Optimization Pipeline

### 4.1 CostModel

Estimates operator cost based on:
- Row count
- Selectivity
- CPU cost (per-row processing)
- I/O cost (storage access)
- Memory cost (intermediate result size)

### 4.2 Optimizer Rules

| Rule | Behavior |
|------|----------|
| FilterPushdown | Push filters closer to data source |
| JoinReorder | Join smaller relations first |
| IndexSelection | Choose best index for predicates |
| ColumnPruning | Read only required columns |
| ConstantFolding | Evaluate constant expressions at plan time |

### 4.3 Adaptive Execution

The AdaptiveExecutor tracks prediction vs actual execution:
- Records row count estimates and actuals
- Computes cardinality correction factor
- Triggers re-optimization when error exceeds threshold (50%)

---

## 5. KQL (Knowledge Query Language)

### 5.1 Grammar

```sql
SELECT <columns> | *
FROM <entity>
WHERE <condition> [AND|OR <condition>]*
[JOIN <entity> ON <col> = <col>]
[ORDER BY <column> [ASC|DESC]]
[LIMIT <n>]
```

### 5.2 Condition Operators

| Operator | Token | Example |
|----------|-------|---------|
| Equal | `=` | `subject = 1` |
| Not Equal | `!=` | `predicate != 0` |
| Less Than | `<` | `confidence < 0.5` |
| Greater Than | `>` | `confidence > 0.5` |
| Less or Equal | `<=` | `confidence <= 0.5` |
| Greater or Equal | `>=` | `confidence >= 0.5` |

### 5.3 Reserved Keywords

`SELECT`, `FROM`, `WHERE`, `AND`, `OR`, `NOT`, `LIMIT`, `ORDER`, `BY`, `ASC`, `DESC`, `JOIN`, `ON`

---

## 6. Constraints

| Constraint | Rationale |
|------------|-----------|
| All operators skip tombstoned rows | Soft-delete consistency |
| Join uses hash algorithm | O(n+m) vs O(n×m) nested loop |
| SIMD path requires runtime detection | Portable across CPU architectures |
| Optimizer must be idempotent | Repeated optimization must not change result |
