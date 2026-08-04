# Extension System

| Field | Value |
|-------|-------|
| **Document ID** | KCM-ECO-006 |
| **Title** | Extension System |
| **Version** | 1.0.0 |
| **Date** | 2026-08-03 |
| **Status** | Authoritative |
| **Authority** | Engineering Orchestrator (P1) |

---

## 1. Overview

Extensions enhance KCM's query language (KQL) and runtime capabilities without requiring full plugin development.

## 2. Extension vs Plugin

| Aspect | Extension | Plugin |
|--------|-----------|--------|
| Scope | KQL language features | Engine capabilities |
| Loading | Compile-time or runtime | Runtime only |
| Complexity | Low | Medium-High |
| Security | Limited | Full access |

## 3. KQL Extensions

### Custom Functions

```sql
-- Register custom function
CREATE FUNCTION geo_distance(lat1, lon1, lat2, lon2) RETURNS FLOAT;

-- Use in query
SELECT * FROM facts WHERE geo_distance(lat, lon, 40.7128, -74.0060) < 100;
```

### Custom Aggregations

```sql
-- Register custom aggregation
CREATE AGGREGATION weighted_avg(value, weight) RETURNS FLOAT;

-- Use in query
SELECT weighted_avg(confidence, priority) FROM facts GROUP BY subject;
```

## 4. Extension API

```rust
pub trait KqlExtension: Send + Sync {
    fn name(&self) -> &str;
    fn register_functions(&self, registry: &mut FunctionRegistry);
    fn register_aggregations(&self, registry: &mut AggregationRegistry);
}
```

## 5. Extension Distribution

- Extensions are distributed as Rust crates
- Feature-gated in Cargo.toml
- Documentation in crate README

## 6. Testing Extensions

Extensions must include:
- Unit tests for each function
- Integration tests with KQL parser
- Property tests for edge cases
