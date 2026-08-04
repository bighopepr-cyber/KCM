# kcm-ml

Machine learning integration for KCM: learned indexes, confidence learners, and rule discovery.

## Purpose

Enhances KCM's query performance and knowledge inference with lightweight ML models that learn from data patterns.

## Modules

| Module | Purpose |
|--------|---------|
| `learned_index` | Learned index (linear regression) for key lookups |
| `confidence_learner` | Learns confidence score distributions |
| `rule_discovery` | Automated rule discovery from data patterns |

## Dependencies

| Dependency | Purpose |
|------------|---------|
| `kcm-core` | Core types |
| `kcm-reasoning` | Rule engine integration |

## Learned Index

Replaces B-tree lookups with a trained linear model:
- Training: O(n) pass over sorted keys
- Lookup: O(1) model inference + small verification window
- Accuracy: >99% within 1 element of true position

```rust
use kcm_ml::learned_index::LearnedIndex;

let mut index = LearnedIndex::new();
index.train(&sorted_keys);
let position = index.lookup(&query_key);
```

## Confidence Learner

Learns distribution of confidence scores per predicate:
- Tracks mean and variance per predicate
- Predicts expected confidence for new facts
- Flags anomalies (unexpected confidence values)

## Rule Discovery

Extracts association rules from fact patterns:
- Support threshold (minimum co-occurrence)
- Confidence threshold (conditional probability)
- Lift threshold (correlation strength)

Output: prioritized list of discovered rules ready for the reasoning engine.
