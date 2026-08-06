# kcm-reasoning Technical Specification

## Overview

`kcm-reasoning` is the inference and reasoning engine for KCM. It implements rule-based knowledge derivation through forward-chaining inference, enabling the system to derive new facts from existing knowledge using configurable rules with confidence propagation.

## Scope

This specification covers the reasoning engine's public API, internal architecture, execution model, and integration with the KCM storage layer. It does not cover the query optimizer (`kcm-optimizer`) or runtime orchestration (`kcm-runtime`), which consume the reasoning engine's output.

## Responsibilities

| Responsibility | Description |
|---------------|-------------|
| Rule definitions | Define inference rules as pattern-matching conditions with confidence formulas |
| Forward-chaining inference | Iteratively match rules against the knowledge base and derive new facts |
| Provenance tracking | Record which rule produced each derived fact for auditability |
| Confidence propagation | Compute derived confidence values from matched facts using configurable formulas |
| Inference safety | Enforce iteration limits, timeouts, and deduplication to prevent resource exhaustion |

## Technical Specification

### Rule

A `Rule` is a named inference unit that derives a new fact from existing knowledge:

```
Rule {
    id: RuleID                          // Unique identifier (u32)
    name: String                        // Human-readable name
    description: String                 // Documentation of the rule's purpose
    pattern: RulePattern                // Conditions to match in the schema
    consequent_predicate: PredicateID   // Predicate of the derived fact
    confidence_formula: ConfidenceFormula // Closure: &[f64] -> f64
    enabled: bool                       // Whether the rule participates in inference
    priority: i32                       // Higher priority rules fire first
}
```

**Confidence propagation**: The `confidence_formula` receives a slice of confidence values extracted from matched facts and computes a single confidence for the derived fact. The result is filtered by `confidence_threshold` (default 0.3) before fact creation. `Fact::new` enforces the [0.0, 1.0] range.

### RulePattern

Patterns define the conditions a rule matches against:

| Variant | Description | Matching Behavior |
|---------|-------------|-------------------|
| `Triple(Option<SubjectID>, PredicateID, Option<ObjectID>)` | Single triple pattern | Matches rows where predicate equals `PredicateID`; subject/object filters are optional |
| `And(left, right)` | Conjunction | Matches when both sub-patterns match with shared entity bindings |
| `Or(left, right)` | Disjunction | Matches when either sub-pattern matches; deduplicates by (subject, object) |
| `Not(inner)` | Negation | Matches rows where the inner pattern does NOT match |

### InferenceEngine

The `InferenceEngine` executes forward-chaining inference:

```
InferenceEngine {
    rule_registry: RuleRegistry         // All registered rules
    max_iterations: usize              // Termination limit (default: 1000)
    confidence_threshold: f64          // Minimum confidence for derivation (default: 0.3)
    timeout_secs: u64                  // Wall-clock timeout (default: 60)
}
```

**Configuration**:

| Parameter | Default | Description |
|-----------|---------|-------------|
| `max_iterations` | 1000 | Maximum forward-chaining iterations before forced termination |
| `confidence_threshold` | 0.3 | Minimum confidence for a derived fact to be accepted |
| `timeout_secs` | 60 | Maximum wall-clock seconds before forced termination |

### Provenance

Each derived fact is wrapped in a `Derivation`:

```
Derivation {
    derived_fact: Fact                  // The derived knowledge triple
    rule_id: RuleID                     // Which rule produced this fact
    confidence_formula_result: f64      // Raw confidence formula output
}
```

`InferenceStats` provides aggregate provenance:

```
InferenceStats {
    iterations: usize                   // Number of forward-chaining iterations executed
    facts_derived: usize                // Total facts derived
    rules_applied: usize                // Total rule applications across all iterations
    duration_ms: u64                    // Wall-clock duration of inference
}
```

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                  kcm-reasoning                       │
│                                                     │
│  ┌──────────────┐    ┌──────────────────────────┐   │
│  │   rule.rs     │    │     inference.rs          │   │
│  │              │    │                          │   │
│  │  Rule        │───▶│  InferenceEngine          │   │
│  │  RulePattern │    │    .register_rule()       │   │
│  │  RuleRegistry│    │    .infer_with_stats()    │   │
│  │  Confidence  │    │    .infer_forward_chain() │   │
│  │    Formula   │    │    .find_pattern_matches()│   │
│  └──────────────┘    └──────────┬───────────────┘   │
│                                 │                    │
└─────────────────────────────────┼────────────────────┘
                                  │
                    ┌─────────────┴──────────────┐
                    │                             │
              ┌─────▼──────┐             ┌────────▼───────┐
              │  kcm-core   │             │  kcm-storage   │
              │             │             │                │
              │  Fact        │             │  Schema         │
              │  SubjectID   │             │  subject_col    │
              │  ObjectID    │             │  predicate_col  │
              │  PredicateID │             │  object_col     │
              │  KcmError    │             │  confidence_col │
              └─────────────┘             └────────────────┘
```

## Internal Components

### rule.rs

Defines the type system for inference rules:

| Type | Purpose |
|------|---------|
| `RuleID` | Type alias for `u32` — unique rule identifier |
| `RulePattern` | Enum representing pattern matching conditions (Triple, And, Or, Not) |
| `ConfidenceFormula` | Type alias for `Box<dyn Fn(&[f64]) -> f64 + Send + Sync>` |
| `Rule` | Complete rule definition with pattern, consequent, and confidence formula |
| `RuleRegistry` | HashMap-based storage for registered rules with duplicate detection |

**`Rule` construction** uses a builder pattern:

```rust
Rule::new(id, name, pattern, consequent_predicate, confidence_formula)
    .with_description("...".to_string())
    .with_priority(10)
```

### inference.rs

Implements the forward-chaining inference loop:

| Type/Method | Purpose |
|-------------|---------|
| `InferenceEngine::new()` | Creates engine with default configuration (1000 iterations, 0.3 threshold, 60s timeout) |
| `InferenceEngine::with_max_iterations()` | Builder method to set iteration limit |
| `InferenceEngine::with_confidence_threshold()` | Builder method to set minimum confidence |
| `InferenceEngine::with_timeout_secs()` | Builder method to set timeout |
| `InferenceEngine::register_rule()` | Add a rule to the registry; rejects duplicate IDs |
| `InferenceEngine::infer_with_stats()` | Execute inference and return derivations with statistics |
| `InferenceEngine::infer_forward_chaining()` | Execute inference and return simplified (Fact, RuleID) pairs |
| `InferenceEngine::find_pattern_matches()` | Pattern-match a `RulePattern` against the schema |
| `Derivation` | Output type wrapping a derived fact with provenance |
| `InferenceStats` | Aggregate statistics from an inference run |

## Data Model

### Rule Pattern Matching

Pattern matching operates on the `Schema` columnar storage:

| Column | Used By | Matching Logic |
|--------|---------|---------------|
| `subject_col` | `RulePattern::Triple` | Optional subject filter |
| `predicate_col` | `RulePattern::Triple` | Required predicate match |
| `object_col` | `RulePattern::Triple` | Optional object filter |
| `confidence_col` | All patterns | Extracted for confidence formula input |

### Deduplication

Derived facts are deduplicated by `(RuleID, SubjectID, ObjectID)` tuples within a single `infer_with_stats` call. The same fact may be re-derived across separate inference calls.

### Confidence Propagation

```
matched_confidences = [c1, c2, ..., cn]   // from pattern-matched facts
raw_confidence = confidence_formula(&matched_confidences)
accepted = raw_confidence >= confidence_threshold
derived_fact.confidence = clamp(raw_confidence, 0.0, 1.0)
```

## Execution Flow

```
1. Initialize derivation set (empty HashSet)
2. FOR each iteration (0..max_iterations):
   a. Check timeout → break if exceeded
   b. Collect all enabled rules
   c. Sort rules by priority (descending)
   d. FOR each enabled rule:
      i.   Match rule.pattern against schema → list of (SubjectID, ObjectID, confidences)
      ii.  For each match: compute confidence = confidence_formula(&confidences)
      iii. Skip if confidence < confidence_threshold
      iv.  Skip if (rule_id, subject, object) already in derivation set
      v.   Create Fact via Fact::new(subject, consequent_predicate, object, confidence)
      vi.  Clamp priority to i8 range
      vii. Record derivation (fact, rule_id, confidence_result)
      viii.Add to derivation set
   e. If no new facts derived → break (fixed point reached)
   f. Append all new derived facts to schema
3. Return (derivations, stats)
```

### Termination Conditions

| Condition | Default | Behavior |
|-----------|---------|----------|
| Fixed point | — | No new facts derived in an iteration → stop |
| Iteration limit | 1000 | Maximum iterations reached → stop |
| Timeout | 60s | Wall-clock time exceeded → stop |

## Public API

### InferenceEngine

```rust
impl InferenceEngine {
    pub fn new() -> Self;
    pub fn with_max_iterations(self, max: usize) -> Self;
    pub fn with_confidence_threshold(self, threshold: f64) -> Self;
    pub fn with_timeout_secs(self, secs: u64) -> Self;
    pub fn register_rule(&mut self, rule: Rule) -> Result<(), KcmError>;
    pub fn infer_with_stats(&self, schema: &mut Schema) -> Result<(Vec<Derivation>, InferenceStats), KcmError>;
    pub fn infer_forward_chaining(&self, schema: &mut Schema) -> Result<Vec<(Fact, RuleID)>, KcmError>;
}
```

### Rule

```rust
impl Rule {
    pub fn new(id: RuleID, name: String, pattern: RulePattern, consequent_predicate: PredicateID, confidence_formula: ConfidenceFormula) -> Self;
    pub fn with_description(self, desc: String) -> Self;
    pub fn with_priority(self, priority: i32) -> Self;
    pub fn disabled(self) -> Self;
}
```

### RulePattern

```rust
impl RulePattern {
    pub fn subject_predicate_object(s: Option<SubjectID>, p: PredicateID, o: Option<ObjectID>) -> Self;
    pub fn and(left: RulePattern, right: RulePattern) -> Self;
    pub fn or(left: RulePattern, right: RulePattern) -> Self;
    pub fn not(pattern: RulePattern) -> Self;
}
```

### RuleRegistry

```rust
impl RuleRegistry {
    pub fn new() -> Self;
    pub fn register(&mut self, rule: Rule) -> Result<(), KcmError>;
    pub fn get(&self, id: RuleID) -> Option<&Rule>;
    pub fn all_enabled(&self) -> Vec<&Rule>;
    pub fn all(&self) -> Vec<&Rule>;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
}
```

## Configuration

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| `max_iterations` | `usize` | 1000 | ≥ 1 | Maximum forward-chaining iterations |
| `confidence_threshold` | `f64` | 0.3 | [0.0, 1.0] | Minimum confidence for derived facts |
| `timeout_secs` | `u64` | 60 | ≥ 1 | Maximum wall-clock inference time |

## Dependencies

| Crate | Relationship | Used For |
|-------|-------------|----------|
| `kcm-core` | Core dependency | `Fact`, `SubjectID`, `ObjectID`, `PredicateID`, `KcmError` types |
| `kcm-storage` | Storage dependency | `Schema` — columnar storage for facts being matched against |

No other crate dependencies are permitted without SSOT-approved justification.

## Error Handling

| Error Source | KcmError Variant | Trigger |
|-------------|------------------|---------|
| Duplicate rule registration | `Conflict` | `RuleID` already exists in `RuleRegistry` |
| Schema append failure | `Io` | Failed to write derived fact to schema |
| Fact construction failure | `InvalidArgument` | Confidence out of [0.0, 1.0] range (should not occur due to threshold filter) |

All public APIs return `Result<T, KcmError>`. No `unwrap()` or `panic!()` in production code paths.

## Performance Characteristics

| Operation | Time Complexity | Notes |
|-----------|----------------|-------|
| `register_rule` | O(1) amortized | HashMap insert |
| `all_enabled` | O(r) | r = total registered rules |
| `find_pattern_matches` (Triple) | O(n) | n = schema row count |
| `find_pattern_matches` (And) | O(n²) worst case | Cross-product of left × right matches |
| `find_pattern_matches` (Or) | O(n) | Union with deduplication |
| `find_pattern_matches` (Not) | O(n²) worst case | Scan all rows, exclude matched |
| `infer_with_stats` | O(i × r × n) | i = iterations, r = rules, n = schema size |
| `infer_forward_chaining` | O(i × r × n) | Same as `infer_with_stats` |

**Throughput target**: ≥ 10,000 facts/second for simple single-predicate rules on schemas with ≤ 100,000 rows.

## Security Considerations

| Concern | Mitigation |
|---------|-----------|
| Rule injection | `RuleRegistry` enforces unique IDs; RBAC enforced by `kcm-runtime` |
| Infinite loops | `max_iterations` and `timeout_secs` enforce termination |
| Resource exhaustion | Deduplication prevents unbounded schema growth |
| Confidence manipulation | Threshold filter + `Fact::new` range validation |
| Priority abuse | Priority clamped to `i8` range on derived facts |

See [SECURITY.md](../../crates/kcm-reasoning/SECURITY.md) for the complete security policy.

## Integration

### With kcm-core

- Imports `Fact`, `SubjectID`, `ObjectID`, `PredicateID`, `KcmError`
- Creates derived facts via `Fact::new`
- Uses `KcmError::Conflict` for duplicate rule detection

### With kcm-storage

- Reads fact data from `Schema` columns (`subject_col`, `predicate_col`, `object_col`, `confidence_col`)
- Skips deleted rows via `schema.is_deleted(idx)`
- Appends derived facts via `schema.append_fact()`
- Uses `schema.len()` for iteration bounds

### With kcm-runtime

- `KnowledgeDatabase` exposes inference capabilities through the `InferenceEngine`
- RBAC permissions gate rule registration and inference invocation

### With kcm-security

- Rule registration requires `WRITE` permission on the knowledge base
- Inference execution requires `READ` + `WRITE` permissions

## Sequence Diagram

```
┌──────────┐     ┌──────────────────┐     ┌─────────────┐     ┌────────────┐
│  Caller   │     │  InferenceEngine │     │ RuleRegistry │     │   Schema   │
└─────┬─────┘     └────────┬─────────┘     └──────┬──────┘     └─────┬──────┘
      │                    │                      │                   │
      │  register_rule()   │                      │                   │
      │───────────────────▶│  register(rule)      │                   │
      │                    │─────────────────────▶│                   │
      │                    │  Ok/Conflict         │                   │
      │                    │◀─────────────────────│                   │
      │                    │                      │                   │
      │  infer_with_stats()│                      │                   │
      │───────────────────▶│                      │                   │
      │                    │  all_enabled()       │                   │
      │                    │─────────────────────▶│                   │
      │                    │  Vec<&Rule>          │                   │
      │                    │◀─────────────────────│                   │
      │                    │                      │                   │
      │                    │  find_pattern_matches(pattern, schema)   │
      │                    │─────────────────────────────────────────▶│
      │                    │  Vec<(SubjectID, ObjectID, confidences)> │
      │                    │◀─────────────────────────────────────────│
      │                    │                      │                   │
      │                    │  [compute confidence]                    │
      │                    │  [check threshold]                       │
      │                    │  [deduplicate]                           │
      │                    │                      │                   │
      │                    │  append_fact(derived)                    │
      │                    │─────────────────────────────────────────▶│
      │                    │                      │                   │
      │  (derivations,     │                      │                   │
      │   stats)           │                      │                   │
      │◀───────────────────│                      │                   │
```

## Architecture Diagram

```
                        ┌─────────────────────────────────┐
                        │         kcm-reasoning             │
                        │                                   │
   Rule Definitions     │  ┌─────────────────────────────┐ │
   ────────────────────▶│  │        rule.rs                │ │
   Pattern Matching     │  │                              │ │
   Confidence Formulas  │  │  Rule ─── RulePattern         │ │
                        │  │   │         ├─ Triple         │ │
                        │  │   │         ├─ And            │ │
                        │  │   │         ├─ Or             │ │
                        │  │   │         └─ Not            │ │
                        │  │   │                           │ │
                        │  │   ├─ ConfidenceFormula        │ │
                        │  │   └─ enabled, priority        │ │
                        │  │                              │ │
                        │  │  RuleRegistry                 │ │
                        │  │   ├─ register(rule)           │ │
                        │  │   ├─ get(id)                  │ │
                        │  │   ├─ all_enabled()            │ │
                        │  │   └─ all()                    │ │
                        │  └──────────────┬──────────────┘ │
                        │                 │                 │
                        │                 ▼                 │
                        │  ┌─────────────────────────────┐ │
                        │  │      inference.rs             │ │
                        │  │                              │ │
   Schema (mutable)     │  │  InferenceEngine              │
   ◀───────────────────│  │   ├─ new()                    │ │
   Derived Facts        │  │   ├─ with_max_iterations()    │ │
                        │  │   ├─ with_confidence_thresh() │ │
                        │  │   ├─ with_timeout_secs()      │ │
                        │  │   ├─ register_rule()          │ │
                        │  │   ├─ infer_with_stats()       │ │
                        │  │   ├─ infer_forward_chaining() │ │
                        │  │   └─ find_pattern_matches()   │ │
                        │  │                              │ │
                        │  │  Derivation                  │ │
                        │  │  InferenceStats              │ │
                        │  └─────────────────────────────┘ │
                        └─────────────────────────────────┘
                                    │         │
                          ┌─────────┘         └──────────┐
                          ▼                              ▼
                 ┌─────────────────┐          ┌─────────────────┐
                 │    kcm-core      │          │   kcm-storage    │
                 │                  │          │                  │
                 │  Fact            │          │  Schema           │
                 │  SubjectID       │          │  subject_col      │
                 │  ObjectID        │          │  predicate_col    │
                 │  PredicateID     │          │  object_col       │
                 │  KcmError        │          │  confidence_col   │
                 └─────────────────┘          └─────────────────┘
```

## References

- [PRD.md §6](../../docs/specs/PRD.md) — Reasoning engine specification
- [AGENTS.md](../../AGENTS.md) — Engineering constitution
- [SSOT.md](../../SSOT.md) — Single Source of Truth

## SSOT Alignment

| Requirement | Source | Implementation |
|-------------|--------|---------------|
| Forward-chaining inference engine | PRD.md §6 | `InferenceEngine::infer_with_stats()` in `inference.rs` |
| Rule definitions with conditions and conclusions | PRD.md §6 | `Rule`, `RulePattern`, `ConfidenceFormula` in `rule.rs` |
| Confidence propagation | PRD.md §6 | `confidence_formula` closure applied to matched confidences |
| Provenance tracking | PRD.md §6 | `Derivation` struct with `rule_id` and `confidence_formula_result` |
| Termination guarantees | PRD.md §6 | `max_iterations`, `timeout_secs`, fixed-point detection |
| Deduplication | PRD.md §6 | `derived_set: HashSet<(RuleID, u32, u32)>` |
| Rule registry with unique IDs | PRD.md §6 | `RuleRegistry` with `KcmError::Conflict` on duplicates |
| Pattern matching (And/Or/Not) | PRD.md §6 | `RulePattern` enum with recursive matching |
| Error handling via KcmError | AGENTS.md | All public APIs return `Result<T, KcmError>` |
| No unwrap in production | AGENTS.md | Zero `unwrap()` in non-test code |
