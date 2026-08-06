# KCM Query Execution Specification

**Document ID:** KCM-QUERY-001
**Version:** 1.0.0
**Status:** Active
**Owner:** Specification Lock (P4)
**Authority:** P4 (PRD.md §5)

---

## 1. Purpose

Defines KCM's query execution model: operator trait, query operators, query lifecycle, and query builder API.

## 2. Execution Model

### 2.1 Volcano-Style Pull-Based Execution

KCM uses Volcano-style execution where each operator implements a `next()` method that returns one row at a time. Operators compose via row ID passing.

### 2.2 Operator Trait

```rust
pub trait Operator: Send + Sync {
    fn execute(&self) -> Result<Vec<usize>, KcmError>;
    fn estimated_rows(&self) -> usize;
}
```

- `execute()` returns a list of row IDs matching the operator's criteria
- `estimated_rows()` provides cardinality estimates for cost-based optimization
- All operators are `Send + Sync` for thread safety

## 3. Query Operators

### 3.1 ScanOp

**Purpose:** Full table scan with optional filtering.

```rust
pub struct ScanOp<'a> {
    schema: &'a Schema,
    context_filter: Option<u8>,
    confidence_filter: Option<f64>,
}
```

**Behavior:**
- Iterates all rows in schema
- Skips tombstoned (deleted) rows
- Optionally filters by context (exact match)
- Optionally filters by confidence (>= threshold)

**Complexity:** O(n) where n = total rows

**Cardinality Estimates:**
- No filter: n
- Context filter: ceil(n × 0.1)
- Confidence filter: ceil(n × 0.3)

### 3.2 FilterOp

**Purpose:** Predicate evaluation on a set of row IDs.

```rust
pub enum FilterPredicate {
    EqualSubject(u32),
    EqualPredicate(u8),
    EqualObject(u32),
    EqualContext(u8),
    InSet(HashSet<u32>),
    RangeTimestamp(i64, i64),
}
```

**Behavior:**
- Takes input row IDs from upstream operator
- Evaluates predicate for each row
- Returns matching row IDs

**Complexity:** O(m) where m = input rows

**Selectivity Estimates:**
| Predicate | Selectivity |
|-----------|------------|
| EqualSubject | 5% |
| EqualPredicate | 15% |
| EqualObject | 5% |
| EqualContext | 20% |
| InSet | set_size / 255 (clamped 1%-50%) |
| RangeTimestamp | 30% |

### 3.3 ProjectOp

**Purpose:** Column selection (pass-through for row IDs).

```rust
pub struct ProjectOp<'a> {
    rowids: Vec<usize>,
    schema: &'a Schema,
    columns: Vec<ColumnID>,
}
```

**Behavior:**
- `execute()` returns input row IDs unchanged
- `execute_projection()` extracts actual column values as `Vec<Vec<u64>>`

**Complexity:** O(1) for row ID pass-through, O(m × k) for projection where k = number of selected columns

### 3.4 JoinOp

**Purpose:** Hash join on a specified column.

```rust
pub struct JoinOp<'a> {
    left_rowids: Vec<usize>,
    right_rowids: Vec<usize>,
    schema: &'a Schema,
    join_column: ColumnID,
}
```

**Behavior:**
- Builds hash table from right row IDs keyed by join column value
- Probes hash table with left row IDs
- Returns interleaved left-right matched row IDs

**Complexity:** O(n + m) where n = left rows, m = right rows

**Supported Join Columns:** Subject, Object, Predicate, Context, Evidence, Owner

### 3.5 AggregateOp

**Purpose:** Aggregate functions with optional group-by.

```rust
pub enum AggregateFunc {
    Count,
    Sum,
    Avg,
    Min,
    Max,
}
```

**Behavior:**
- `execute()` returns input row IDs (pipeline compatible)
- `execute_aggregate()` computes single aggregate value over confidence column
- `execute_grouped()` computes per-group aggregates grouped by specified column

**Complexity:** O(n) where n = input rows

## 4. Query Lifecycle

```
1. User constructs QueryBuilder
2. QueryBuilder builds operator tree
3. Optimizer applies transformations:
   a. Filter pushdown — move filters closer to scan
   b. Column pruning — remove unused columns
   c. Join reordering — smallest relations first
   d. Index selection — choose best index per predicate
4. Executor executes operator tree
5. Results collected as Vec<Fact>
```

## 5. QueryBuilder API

```rust
// Fluent query construction
let results = kb.query()
    .with_subject(SubjectID(1))
    .with_predicate(PredicateID(5))
    .with_confidence(0.8)
    .execute()?;

// Returns Vec<Fact>
```

### 5.1 Methods

| Method | Description |
|--------|-------------|
| `with_subject(id)` | Filter by subject ID |
| `with_predicate(id)` | Filter by predicate ID |
| `with_object(id)` | Filter by object ID |
| `with_confidence(min)` | Filter by minimum confidence |
| `with_context(ctx)` | Filter by context ID |
| `with_timestamp_range(low, high)` | Filter by timestamp range |
| `execute()` | Execute query and return facts |

## 6. KQL (Knowledge Query Language)

### 6.1 Syntax

```sql
SELECT subject, object FROM facts
WHERE predicate = 0 AND confidence >= 0.8
ORDER BY timestamp DESC
LIMIT 100
```

### 6.2 Tokens

28 token variants including: SELECT, FROM, WHERE, AND, OR, ORDER, BY, DESC, ASC, LIMIT, JOIN, ON, INSERT, DELETE, UPDATE, SET, VALUES, INTO, CREATE, DROP, etc.

### 6.3 Parser Output

Produces `SelectQuery` AST with:
- `columns: Vec<ColumnID>` — selected columns
- `where_clause: Option<WhereClause>` — filter conditions
- `order_by: Option<(ColumnID, SortDirection)>` — sort specification
- `limit: Option<usize>` — result count limit

## 7. Cost Model

```rust
pub struct OperatorCost {
    pub cpu_cost: f64,
    pub io_cost: f64,
    pub memory_cost: f64,
    pub estimated_rows: usize,
}
```

**Cost Weights:** `total = cpu × 1.0 + io × 10.0 + memory × 0.1`

## 8. Adaptive Execution

- Tracks prediction vs actual cardinality per operator
- Re-optimizes when prediction error > 50%
- Updates statistics for future query planning

## 9. References

- **Implements:** PRD.md §5 (Compute Engine)
- **Depends on:** KCM_DATA_MODEL_SPEC, KCM_COLUMNAR_FORMAT_SPEC
- **Related:** KCM_INDEXING_SPEC, KCM_PERFORMANCE_SPEC
