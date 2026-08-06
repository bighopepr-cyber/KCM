# Contributing to kcm-ml

Contribution guidelines specific to the `kcm-ml` crate.

> For core engine contribution rules, refer to the repository [CONTRIBUTING.md](../../CONTRIBUTING.md).

## Overview

`kcm-ml` is the machine learning integration crate for KCM. It provides learned index prediction, confidence learning, and rule discovery. Changes here directly affect the accuracy of index predictions, the reliability of confidence scores, and the quality of discovered rules. Incorrect ML logic can silently produce unreliable predictions or generate misleading rules.

## Before Contributing

1. Read the [root CONTRIBUTING.md](../../CONTRIBUTING.md)
2. Read the [kcm-ml technical specification](../../docs/kcm-ml/spesifikasi.md)
3. Verify your change does not break the public API without an SSOT-approved reason
4. Check [existing issues](https://github.com/bighopepr-cyber/KCM/issues) for related work
5. Understand the dependency constraints — `kcm-ml` may only depend on `kcm-core` and `kcm-reasoning`

## Coding Standards

### Rust Requirements

- Edition 2024
- All public APIs return `Result<T, KcmError>`
- No `unwrap()` in production code
- No `panic!()` in production code
- No `TODO`/`FIXME`/`HACK` markers
- All `unsafe` blocks must have `// SAFETY:` comments

### ML Module Rules

- `LearnedIndex::train` must validate training data before fitting models
- `ConfidenceLearner` must clamp all confidence outputs to [0.0, 1.0]
- `RuleDiscoveryEngine` must enforce `min_support` and `min_confidence` thresholds
- Exponential moving average in `ConfidenceLearner` must use bounded alpha (0.0 to 1.0)
- No external ML library dependencies — all models are self-contained
- No randomness in prediction paths — all outputs must be deterministic given identical inputs

### Naming Conventions

| Element | Convention | Example |
|---------|-----------|---------|
| Types | PascalCase | `LearnedIndex`, `ConfidenceLearner` |
| Functions | snake_case | `predict_confidence`, `discover_patterns` |
| Constants | SCREAMING_SNAKE_CASE | `DEFAULT_MIN_SUPPORT` |
| Modules | snake_case | `learned_index`, `confidence_learner` |

## Module Architecture Rules

- `kcm-ml` depends on **only** `kcm-core` and `kcm-reasoning`
- No other crate dependencies are permitted without SSOT-approved justification
- No I/O, networking, or file system operations
- No async code — all ML operations are synchronous
- All modules must be declared in `lib.rs`
- Each module has a single public struct with clearly defined responsibilities

### Module Responsibilities

| Module | Responsibility |
|--------|---------------|
| `learned_index` | `RegressionModel`, `LearnedIndex` — piecewise linear regression for index position prediction |
| `confidence_learner` | `ConfidenceLearner` — learns confidence scores from fact observation patterns |
| `rule_discovery` | `RuleDiscoveryEngine` — mines association rules from fact collections |

## Documentation Rules

- Every public function must have a `///` doc comment
- Every public type must have a `///` doc comment
- Doc comments must include at least one code example for public APIs
- Module-level documentation must explain the module's purpose
- ML model documentation must describe the algorithm, expected input ranges, and output guarantees

## Testing Requirements

### Model Accuracy Tests

- `RegressionModel::train` must produce correct slope/intercept for known linear data
- `LearnedIndex::search` must return bounds that contain the queried value for trained data
- Empty training data must not panic; predictions must return safe defaults
- Non-linear data must produce reasonable approximations within expected error bounds

### Confidence Learning Tests

- `predict_confidence` must return values clamped to [0.0, 1.0]
- `observe_fact` must correctly accumulate positive and negative confidence signals
- `observe_rule_inference` must update rule accuracy via exponential moving average
- `adjust_confidence` must scale base confidence by rule accuracy

### Discovery Tests

- `discover_patterns` must find predicate chains that meet `min_support` threshold
- `patterns_to_rules` must filter patterns below `min_confidence` threshold
- Empty fact collections must return empty results without panicking
- Single-fact collections must not produce spurious patterns

### Running Tests

```bash
cargo test -p kcm-ml
```

## Performance Rules

- Learned index search must be O(log n) for model selection + O(1) for prediction
- Confidence prediction must be O(1) per fact key lookup
- Rule discovery pattern mining must be O(n * m) where n = facts, m = average predicates per subject
- Benchmark regressions >5% require justification
- No unnecessary allocations in prediction hot paths

## Review Checklist

- [ ] All public APIs return `Result<T, KcmError>`
- [ ] No `unwrap()` in production code
- [ ] No `panic!()` in production code
- [ ] Confidence values are clamped to [0.0, 1.0]
- [ ] Model training validates input data
- [ ] Rule discovery enforces threshold parameters
- [ ] All public functions have doc comments
- [ ] All tests pass
- [ ] No clippy warnings
- [ ] SSOT traceability documented

## Pull Request Requirements

- Reference the SSOT requirement being addressed (PRD3.md §29)
- Include test coverage for new/changed ML logic
- Include benchmarks if performance-sensitive
- Do not break backward compatibility without SSOT approval
- Document any changes to model algorithms or prediction guarantees

## References

- [CONTRIBUTING.md](../../CONTRIBUTING.md) — Repository-wide contribution guidelines
- [CODE_OF_CONDUCT.md](../../CODE_OF_CONDUCT.md) — Community guidelines
- [docs/kcm-ml/spesifikasi.md](../../docs/kcm-ml/spesifikasi.md) — Technical specification
- [AGENTS.md](../../AGENTS.md) — Engineering constitution
