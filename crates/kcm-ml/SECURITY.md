# kcm-ml Security Policy

Security considerations specific to the `kcm-ml` crate.

> For project-wide security policies, refer to the [SECURITY.md](../../SECURITY.md) located in the repository root.

## Overview

`kcm-ml` is the machine learning integration crate for KCM. It provides learned index prediction via regression models, confidence learning from data patterns, and rule discovery through association rule mining. Because ML models operate on knowledge data and produce predictions that influence query execution and inference outcomes, security flaws here can lead to incorrect predictions, model poisoning, or adversarial manipulation of learned behaviors.

## Security Scope

| Asset | Risk Level | Description |
|-------|-----------|-------------|
| `LearnedIndex` | Medium | Regression-based index prediction — poisoned training data leads to incorrect position estimates |
| `ConfidenceLearner` | Medium | Learns confidence scores from fact patterns — adversarial observations corrupt confidence predictions |
| `RuleDiscoveryEngine` | Medium | Mines association rules from fact collections — malicious facts generate spurious rules |

## Threat Model

| Threat | Vector | Mitigation |
|--------|--------|------------|
| Model poisoning | Attacker supplies crafted training data to `LearnedIndex::train` or `ConfidenceLearner::observe_fact` | Validate training inputs are within expected ranges; bound confidence values to [0.0, 1.0]; detect statistical outliers before training |
| Adversarial inputs | Crafted `u32` values or `f64` confidence scores exploit numerical edge cases | Clamp all confidence outputs to [0.0, 1.0]; handle NaN/Inf via `clamp` and `max` guards |
| Overfitting | Insufficient training data or excessive model complexity produces unreliable predictions | `LearnedIndex` uses piecewise linear regression bounded by chunk size; `ConfidenceLearner` applies exponential moving average (EMA) to smooth predictions |
| Data leakage | Training data or rule discovery patterns expose sensitive knowledge relationships | `kcm-ml` operates in-memory only; no persistence layer; no network I/O; data lifecycle managed by caller |

## Security Risks

| Risk | Description | Severity |
|------|-------------|----------|
| Incorrect index prediction | Poisoned `LearnedIndex` returns wrong search bounds, causing queries to miss results or scan excess data | Medium |
| Confidence corruption | Adversarial `observe_fact` calls skew `ConfidenceLearner` predictions, causing the system to trust incorrect inferences | Medium |
| Spurious rule generation | Malicious facts fed to `RuleDiscoveryEngine` produce rules that incorrectly associate unrelated predicates | Medium |
| Resource exhaustion | Unbounded `learned_models` or `fact_sources` HashMaps grow without limit | Low |

## Access Control

`kcm-ml` does not enforce access control directly. Model training, confidence observation, and rule discovery must be gated by `kcm-security` RBAC in production deployments. The crate trusts its caller to provide valid training data and fact collections.

## RBAC Integration

| Operation | Minimum Permission | Enforcement |
|-----------|-------------------|-------------|
| `LearnedIndex::train` | `WRITE` on knowledge base | Caller responsibility (enforced by `kcm-runtime`) |
| `ConfidenceLearner::observe_fact` | `WRITE` on knowledge base | Caller responsibility (enforced by `kcm-runtime`) |
| `RuleDiscoveryEngine::discover_patterns` | `READ` on knowledge base | Caller responsibility (enforced by `kcm-runtime`) |
| `RuleDiscoveryEngine::patterns_to_rules` | `READ` on knowledge base | Caller responsibility (enforced by `kcm-runtime`) |

## Sensitive Assets

- `RegressionModel.slope` / `RegressionModel.intercept` — Learned model parameters that determine index predictions. Compromised values lead to incorrect search bounds.
- `ConfidenceLearner.fact_sources` — Maps fact keys to observed confidence trajectories. Contains historical accuracy data for confidence prediction.
- `ConfidenceLearner.rule_accuracy` — Stores per-rule accuracy scores via EMA. Compromised values lead to incorrect confidence adjustments.
- `RuleDiscoveryEngine` output patterns — Discovered predicate associations. Spurious patterns can be injected as misleading rules.

## Secret Management

No secrets are stored or managed in `kcm-ml`. The crate has no I/O, networking, or file system access. All data structures are in-memory only. Training data and fact collections are provided by the caller and not persisted by this crate.

## Secure Development Rules

1. **Model validation** — `LearnedIndex::train` must validate that training data is non-empty and positions are monotonically non-decreasing; reject degenerate inputs
2. **Input sanitization** — All `f64` inputs to `observe_fact` and `observe_rule_inference` must be finite (no NaN, no Inf); reject non-finite values
3. **Confidence bounds** — All confidence outputs from `predict_confidence` and `adjust_confidence` must be clamped to [0.0, 1.0]
4. **Rule safety** — `RuleDiscoveryEngine::discover_patterns` must enforce `min_support` and `min_confidence` thresholds; patterns below thresholds must be filtered
5. **No unwrap** — Zero `unwrap()` in production code paths; all errors return `Result<T, KcmError>`
6. **Result return** — All public APIs must return `Result<T, KcmError>` to propagate errors to the caller

## Audit Logging

| Event | Log Level | Details |
|-------|-----------|---------|
| Model trained | INFO | Model count, training data size |
| Confidence observation | DEBUG | Fact key, confidence value, correctness |
| Rule inference observed | DEBUG | Rule ID, predicted vs actual accuracy |
| Pattern discovered | INFO | Predicate pair, support count, confidence |
| Pattern converted to rule | INFO | Predicate ID, confidence threshold met |

## Validation Checklist

- [ ] `LearnedIndex::train` validates non-empty training data
- [ ] `ConfidenceLearner::observe_fact` rejects NaN/Inf confidence values
- [ ] `ConfidenceLearner::predict_confidence` returns values clamped to [0.0, 1.0]
- [ ] `ConfidenceLearner::adjust_confidence` returns values clamped to [0.0, 1.0]
- [ ] `RuleDiscoveryEngine` enforces `min_support` and `min_confidence` thresholds
- [ ] No `unwrap()` in production code paths
- [ ] No `panic!()` in production code paths
- [ ] All public APIs return `Result<T, KcmError>`
- [ ] No NaN/Inf values leak from model predictions

## References

- [SECURITY.md](../../SECURITY.md) — Project-wide security policy
- [AGENTS.md](../../AGENTS.md) — Engineering constitution
- [SSOT.md](../../SSOT.md) — Single Source of Truth
- [docs/kcm-ml/spesifikasi.md](../../docs/kcm-ml/spesifikasi.md) — Technical specification
- [PRD3.md §29](../../docs/PRD3.md) — ML integration requirements
