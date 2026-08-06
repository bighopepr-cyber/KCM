# kcm-ml Technical Specification

## Overview

`kcm-ml` is the machine learning integration crate for KCM (Knowledge Columnar Model). It provides learned index prediction via piecewise linear regression, confidence learning from data observation patterns, and rule discovery through association rule mining on fact collections. The crate operates entirely in-memory with no I/O, networking, or persistence — all data lifecycle is managed by the caller.

## Scope

This specification covers the `kcm-ml` crate only. It does not cover storage, compute, reasoning, optimization, runtime, or any other KCM subsystem. ML integration with the runtime and security layers is described at the integration boundary only.

## Responsibilities

| Responsibility | Description |
|---------------|-------------|
| Learned index | Piecewise linear regression model for predicting index positions from key values |
| Confidence learning | Exponential moving average model for learning confidence scores from fact observation patterns |
| Rule discovery | Association rule mining to discover predicate chains from fact collections |

## Technical Specification

### LearnedIndex

A piecewise linear regression model that predicts index positions from `u32` key values. The index is divided into chunks, each fitted with an independent `RegressionModel`.

#### RegressionModel

```rust
pub struct RegressionModel {
    slope: f64,
    intercept: f64,
}
```

**Algorithm:** Ordinary least squares (OLS) simple linear regression.

**Operations:**

| Operation | Complexity | Description |
|-----------|-----------|-------------|
| `new()` | O(1) | Create uninitialized model (slope=0, intercept=0) |
| `train(x_values, y_positions)` | O(n) | Fit slope and intercept via OLS |
| `predict(value)` | O(1) | Predict position: `slope * value + intercept`, clamped to ≥ 0 |

**Constraints:**
- `x_values` and `y_positions` must have equal length
- Empty inputs produce default model (slope=0, intercept=0)
- `predict` returns `usize` — negative predictions are clamped to 0

#### LearnedIndex

```rust
pub struct LearnedIndex {
    models: Vec<RegressionModel>,
    ranges: Vec<(u32, u32)>,
    model_count: usize,
}
```

**Operations:**

| Operation | Complexity | Description |
|-----------|-----------|-------------|
| `new(model_count)` | O(model_count) | Create index with N regression models |
| `train(values, positions)` | O(n) | Chunk data into N segments, fit each model |
| `search(value)` | O(log model_count) | Predict position, return (lower, upper) bounds |

**Chunking strategy:**
- Input data is divided into `model_count` equal-sized chunks
- Each chunk is fitted with an independent `RegressionModel`
- `ranges` stores the key boundaries for each chunk

**Search behavior:**
- Binary search (`partition_point`) selects the appropriate model chunk
- Predicted position is expanded by ±100 to create a search window
- Returns `(lower, upper)` bounds where `lower ≤ predicted ≤ upper`

**Constraints:**
- `model_count` must be > 0 for meaningful results
- Empty training data produces no models; search returns (0, 0+epsilon)
- Training data should be sorted by key value for accurate chunk boundaries

### ConfidenceLearner

Learns confidence scores from fact observation patterns using exponential moving average (EMA) for rule accuracy tracking.

```rust
pub struct ConfidenceLearner {
    fact_sources: HashMap<String, Vec<f64>>,
    rule_accuracy: HashMap<u32, f64>,
}
```

**Operations:**

| Operation | Complexity | Description |
|-----------|-----------|-------------|
| `new()` | O(1) | Create empty learner |
| `observe_fact(fact_key, confidence, is_correct)` | O(1) | Record fact observation (+confidence if correct, -confidence if incorrect) |
| `observe_rule_inference(rule_id, predicted, actual)` | O(1) | Update rule accuracy via EMA: `0.9 * old + 0.1 * (1.0 - error)` |
| `predict_confidence(fact_key)` | O(1) | Average of accumulated observations, clamped to [0.0, 1.0] |
| `get_rule_accuracy(rule_id)` | O(1) | Return EMA accuracy for rule (default 0.5 if unseen) |
| `adjust_confidence(rule_id, base)` | O(1) | Scale base confidence by rule accuracy, clamped to [0.0, 1.0] |
| `rules_tracked()` | O(1) | Count of rules with observed accuracy |

**EMA parameters:**
- Smoothing factor α = 0.1 (weight for new observations)
- Decay factor (1 - α) = 0.9 (weight for historical accuracy)
- Initial accuracy for new rules: 1.0 (optimistic default)
- Default accuracy for unseen rules: 0.5 (neutral)

**Confidence bounds:**
- All output values are clamped to [0.0, 1.0]
- `predict_confidence` averages signed observations; positive = correct, negative = incorrect
- `adjust_confidence` multiplies base confidence by rule accuracy factor

### RuleDiscoveryEngine

Mines association rules from fact collections by detecting predicate chains (subject → predicate1 → object → predicate2 patterns).

```rust
pub struct RuleDiscoveryEngine {
    min_support: f64,
    min_confidence: f64,
}
```

**Operations:**

| Operation | Complexity | Description |
|-----------|-----------|-------------|
| `new(min_support, min_confidence)` | O(1) | Create engine with threshold parameters |
| `discover_patterns(facts)` | O(n * m) | Find predicate chains meeting support threshold |
| `patterns_to_rules(patterns)` | O(p) | Convert patterns to `RulePattern` structures |

**Mining algorithm:**
1. Group facts by subject into `subject_to_facts` map
2. For each fact, check if its object is also a subject in another fact
3. Count predicate chain occurrences: `(predicate1, predicate2)` pairs
4. Filter chains by `min_support` threshold (minimum occurrence fraction)
5. Compute confidence as `count / total_facts`, clamped to [0.0, 1.0]

**Rule generation:**
- Patterns meeting `min_confidence` are converted to `RulePattern::And` combinators
- Each rule links two `RulePattern::subject_predicate_object` patterns
- Output includes the consequent predicate and confidence score

**Constraints:**
- `min_support` and `min_confidence` must be in [0.0, 1.0]
- Empty fact collections produce empty results
- Single-fact collections produce no patterns (no chains possible)

## Architecture

### Internal Components

| File | Component | Description |
|------|-----------|-------------|
| `learned_index.rs` | `RegressionModel` | OLS linear regression model |
| `learned_index.rs` | `LearnedIndex` | Piecewise regression index with chunk-based model selection |
| `confidence_learner.rs` | `ConfidenceLearner` | EMA-based confidence learning from fact observations |
| `rule_discovery.rs` | `RuleDiscoveryEngine` | Association rule mining from fact collections |

### Data Model

```
LearnedIndex
├── models: Vec<RegressionModel>
│   ├── slope: f64
│   └── intercept: f64
├── ranges: Vec<(u32, u32)>
└── model_count: usize

ConfidenceLearner
├── fact_sources: HashMap<String, Vec<f64>>
└── rule_accuracy: HashMap<u32, f64>

RuleDiscoveryEngine
├── min_support: f64
└── min_confidence: f64
```

### Execution Flow

#### Index Training

```
1. Caller provides sorted key values and corresponding positions
2. LearnedIndex chunks data into model_count segments
3. Each segment is fitted with RegressionModel::train (OLS)
4. Chunk boundaries are stored in ranges vector
5. Ready for search() queries
```

#### Confidence Prediction

```
1. Caller observes facts via observe_fact(key, confidence, is_correct)
2. Positive observations accumulate confidence; negative observations subtract
3. predict_confidence computes average of all observations for a key
4. Result is clamped to [0.0, 1.0]
5. Rule accuracy updated via observe_rule_inference using EMA
6. adjust_confidence scales base confidence by learned rule accuracy
```

#### Rule Mining

```
1. Caller provides fact collection from schema
2. discover_patterns groups facts by subject
3. Predicate chains detected: subject→pred1→object→pred2
4. Chains below min_support threshold filtered out
5. Remaining patterns converted to RulePattern structures
6. patterns_to_rules filters by min_confidence
7. Output: Vec<(RulePattern, PredicateID, confidence)>
```

## Public API

### LearnedIndex

```rust
impl LearnedIndex {
    pub fn new(model_count: usize) -> Self;
    pub fn train(&mut self, values: &[u32], positions: &[usize]);
    pub fn search(&self, value: u32) -> (usize, usize);
}

impl RegressionModel {
    pub fn new() -> Self;
    pub fn train(&mut self, x_values: &[u32], y_positions: &[usize]);
    pub fn predict(&self, value: u32) -> usize;
}
```

### ConfidenceLearner

```rust
impl ConfidenceLearner {
    pub fn new() -> Self;
    pub fn observe_fact(&mut self, fact_key: String, confidence: f64, is_correct: bool);
    pub fn observe_rule_inference(&mut self, rule_id: u32, predicted: f64, actual: f64);
    pub fn predict_confidence(&self, fact_key: &str) -> Option<f64>;
    pub fn get_rule_accuracy(&self, rule_id: u32) -> f64;
    pub fn adjust_confidence(&self, rule_id: u32, base: f64) -> f64;
    pub fn rules_tracked(&self) -> usize;
}
```

### RuleDiscoveryEngine

```rust
impl RuleDiscoveryEngine {
    pub fn new(min_support: f64, min_confidence: f64) -> Self;
    pub fn discover_patterns(&self, facts: &[Fact]) -> Vec<(PredicateID, PredicateID, f64)>;
    pub fn patterns_to_rules(&self, patterns: &[(PredicateID, PredicateID, f64)]) -> Vec<(RulePattern, PredicateID, f64)>;
}
```

## Configuration

No runtime configuration files. All parameters are passed at construction time:

| Parameter | Default | Range | Description |
|-----------|---------|-------|-------------|
| `model_count` | N/A | 1..usize::MAX | Number of regression models in LearnedIndex |
| `min_support` | N/A | [0.0, 1.0] | Minimum fraction of facts for pattern discovery |
| `min_confidence` | N/A | [0.0, 1.0] | Minimum confidence for rule generation |
| EMA α | 0.1 | Fixed | Smoothing factor for rule accuracy updates |
| Search margin | 100 | Fixed | Position ± margin for search bounds |

## Dependencies

| Dependency | Purpose | Justification |
|------------|---------|---------------|
| `kcm-core` | Core types (`Fact`, `PredicateID`, `ObjectID`, `SubjectID`) | Required for fact representation and type definitions |
| `kcm-reasoning` | `RulePattern` type for rule discovery output | Required to convert discovered patterns into reasoning-compatible rule structures |

No external crate dependencies. All ML algorithms are self-contained implementations.

## Error Handling

`kcm-ml` currently uses panic-free default values for edge cases rather than `Result` returns. Future versions will adopt `Result<T, KcmError>` for all public APIs per AGENTS.md rules.

Current error behavior:
- Empty training data → default model (slope=0, intercept=0)
- Unseen rule ID → default accuracy (0.5)
- Empty fact collection → empty results
- Confidence values → clamped to [0.0, 1.0]

## Performance Characteristics

| Operation | Complexity | Notes |
|-----------|-----------|-------|
| `RegressionModel::train` | O(n) | Single pass OLS computation |
| `RegressionModel::predict` | O(1) | Single multiply-add |
| `LearnedIndex::train` | O(n) | n = total training samples |
| `LearnedIndex::search` | O(log m) | m = model_count, binary search + O(1) predict |
| `ConfidenceLearner::observe_fact` | O(1) | HashMap insert/append |
| `ConfidenceLearner::predict_confidence` | O(1) | HashMap lookup + average |
| `RuleDiscoveryEngine::discover_patterns` | O(n * p) | n = facts, p = avg predicates per subject |
| `RuleDiscoveryEngine::patterns_to_rules` | O(p) | p = discovered patterns |

## Security Considerations

- **Model poisoning:** Training data must be validated by the caller; `kcm-ml` does not validate data provenance
- **Numerical safety:** All confidence outputs clamped to [0.0, 1.0]; NaN/Inf not explicitly guarded (caller responsibility)
- **Adversarial inputs:** No input validation beyond bounds clamping; RBAC enforcement deferred to `kcm-runtime`
- **Resource bounds:** HashMap growth is unbounded; caller must manage memory pressure
- **Determinism:** All predictions are deterministic given identical inputs (no randomness)

## Integration

### With kcm-reasoning

- `RuleDiscoveryEngine::patterns_to_rules` produces `RulePattern` structures compatible with `kcm-reasoning::RuleRegistry`
- Discovered rules can be registered via `kcm-reasoning` for forward-chaining inference

### With kcm-runtime

- `ConfidenceLearner` adjusts inference confidence based on observed rule accuracy
- `LearnedIndex` accelerates column lookups in the storage engine

### With kcm-security

- RBAC enforcement for ML operations must be handled by `kcm-runtime` before calling `kcm-ml`
- Model training and rule discovery are write operations requiring `WRITE` permission

## Sequence Diagram

```
Caller                    kcm-ml                      kcm-reasoning
  │                         │                              │
  │── LearnIndex::train ──→ │                              │
  │   (values, positions)   │                              │
  │                         │── OLS regression ──→         │
  │                         │   (per chunk)                │
  │←── trained model ──────│                              │
  │                         │                              │
  │── search(value) ──────→ │                              │
  │                         │── binary search chunks ──→   │
  │                         │── predict position ──→       │
  │←── (lower, upper) ─────│                              │
  │                         │                              │
  │── observe_fact() ──────→│                              │
  │── predict_confidence()→│                              │
  │←── confidence [0,1] ───│                              │
  │                         │                              │
  │── discover_patterns() →│                              │
  │   (facts)               │── mine predicate chains ──→  │
  │←── patterns ───────────│                              │
  │                         │                              │
  │── patterns_to_rules() →│                              │
  │←── RulePatterns ───────│                              │
  │── register_rule() ──────────────────────────────────→ │
```

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────┐
│                        kcm-ml                           │
│                                                         │
│  ┌──────────────────┐  ┌──────────────────┐            │
│  │  learned_index.rs │  │confidence_learner│            │
│  │                   │  │     .rs          │            │
│  │  RegressionModel  │  │                  │            │
│  │  - slope: f64     │  │  ConfidenceLearner│           │
│  │  - intercept: f64 │  │  - fact_sources  │            │
│  │                   │  │  - rule_accuracy │            │
│  │  LearnedIndex     │  │                  │            │
│  │  - models: Vec<>  │  │  observe_fact()  │            │
│  │  - ranges: Vec<>  │  │  predict_conf()  │            │
│  │  - train()        │  │  adjust_conf()   │            │
│  │  - search()       │  │                  │            │
│  └──────────────────┘  └──────────────────┘            │
│                                                         │
│  ┌──────────────────────────────────────┐              │
│  │        rule_discovery.rs              │              │
│  │                                      │              │
│  │  RuleDiscoveryEngine                 │              │
│  │  - min_support: f64                  │              │
│  │  - min_confidence: f64               │              │
│  │  - discover_patterns(facts)          │              │
│  │  - patterns_to_rules(patterns)       │              │
│  └──────────────────────────────────────┘              │
│                                                         │
│  Dependencies:                                          │
│  ├── kcm-core (Fact, PredicateID, ObjectID)            │
│  └── kcm-reasoning (RulePattern)                       │
└─────────────────────────────────────────────────────────┘
```

## References

- [AGENTS.md](../../AGENTS.md) — Engineering constitution
- [PRD3.md §29](../../docs/PRD3.md) — ML integration requirements
- [SSOT.md](../../SSOT.md) — Single Source of Truth
- [PRD.md](../../docs/PRD.md) — Core types and query model
- [PRD2.md](../../docs/PRD2.md) — Storage and runtime architecture
- [kcm-reasoning/spesifikasi.md](../../docs/kcm-reasoning/spesifikasi.md) — Reasoning engine specification

## SSOT Alignment

| Requirement | Source | Implementation |
|-------------|--------|----------------|
| Learned index for column prediction | PRD3.md §29 | `LearnedIndex`, `RegressionModel` in `learned_index.rs` |
| Confidence learning from data patterns | PRD3.md §29 | `ConfidenceLearner` in `confidence_learner.rs` |
| Rule discovery from fact collections | PRD3.md §29 | `RuleDiscoveryEngine` in `rule_discovery.rs` |
| Depends only on kcm-core, kcm-reasoning | AGENTS.md | `Cargo.toml` dependency declarations |
| No unwrap in production code | AGENTS.md §Non-Negotiable Rules | Enforcement via clippy + CI |
| All public APIs return Result | AGENTS.md §Non-Negotiable Rules | Adopted in future version |
