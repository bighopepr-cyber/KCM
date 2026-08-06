# kcm-reasoning Security Policy

Security considerations specific to the `kcm-reasoning` crate.

> For project-wide security policies, refer to the [SECURITY.md](../../SECURITY.md) located in the repository root.

## Overview

`kcm-reasoning` implements the inference and reasoning engine for KCM. It provides rule definitions (`Rule`, `RulePattern`, `RuleRegistry`) and a forward-chaining inference engine (`InferenceEngine`) that derives new facts from existing knowledge. Because the inference engine mutates the schema by appending derived facts, any security flaw here can corrupt knowledge integrity, cause resource exhaustion, or enable rule injection attacks.

## Security Scope

| Asset | Risk Level | Description |
|-------|-----------|-------------|
| `Rule` struct | High | Rule definitions control inference behavior — injected or malformed rules can corrupt derived facts |
| `InferenceEngine` | High | Forward-chaining loop mutates schema — infinite loops or unbounded expansion cause resource exhaustion |
| `RulePattern` | Medium | Pattern matching against schema data — complex nested patterns can cause quadratic blowup |
| `ConfidenceFormula` | Medium | Arbitrary closure computes confidence — must be bounded to [0.0, 1.0] |
| `RuleRegistry` | Medium | Stores all registered rules — duplicate IDs cause `KcmError::Conflict` |
| `Derivation` | Low | Derived fact output — incorrect provenance tracking weakens auditability |

## Threat Model

| Threat | Vector | Mitigation |
|--------|--------|------------|
| Rule injection | Attacker registers malicious rules via `register_rule` | Validate rule structure; enforce unique `RuleID`; restrict registration to authorized callers via `kcm-security` RBAC |
| Infinite inference loops | Cyclic rules derive facts that re-trigger themselves | Enforce `max_iterations` limit (default 1000); terminate on timeout (default 60s); deduplicate derived facts via `derived_set` |
| Resource exhaustion | Excessive rule count or complex nested `RulePattern` | Bound `max_iterations` and `timeout_secs`; pattern matching scans bounded by `schema.len()` |
| Confidence manipulation | Malicious `ConfidenceFormula` returns values outside [0.0, 1.0] | Apply `confidence_threshold` filter; validate derived `Fact::new` which enforces confidence bounds |
| Denial-of-service via schema growth | Derived facts appended unboundedly | Each iteration appends only new facts (deduplicated); `max_iterations` caps total growth |
| Priority inversion | Maliciously high priority causes rule ordering manipulation | Priority is clamped to `i8` range when applied to derived facts |

## Access Control

`kcm-reasoning` does not enforce access control directly. Rule registration and inference invocation must be gated by `kcm-security` RBAC in production deployments. The crate trusts its caller to provide valid rules and schema references.

## RBAC Integration

| Operation | Minimum Permission | Enforcement |
|-----------|-------------------|-------------|
| `register_rule` | `WRITE` on knowledge base | Caller responsibility (enforced by `kcm-runtime`) |
| `infer_forward_chaining` | `READ` + `WRITE` on knowledge base | Caller responsibility (enforced by `kcm-runtime`) |
| `infer_with_stats` | `READ` + `WRITE` on knowledge base | Caller responsibility (enforced by `kcm-runtime`) |

## Sensitive Assets

- `Rule.consequent_predicate` — Determines what predicate is derived. Misuse can overwrite or shadow existing predicates.
- `Rule.confidence_formula` — Arbitrary closure with access to all matched confidence values. Must not leak side effects.
- `Schema` (mutated) — Derived facts are appended directly to the schema. Incorrect inference corrupts the knowledge base.

## Secret Management

No secrets are stored or managed in `kcm-reasoning`. The crate has no I/O, networking, or file system access. Rule definitions and confidence formulas are in-memory only.

## Secure Development Rules

1. **Inference loop limits** — `InferenceEngine` must enforce `max_iterations` (default 1000) and `timeout_secs` (default 60) to prevent infinite loops
2. **Rule validation** — `RuleRegistry::register` must reject duplicate `RuleID` values with `KcmError::Conflict`
3. **Confidence propagation bounds** — Derived confidence values must be filtered by `confidence_threshold` before fact creation; `Fact::new` enforces [0.0, 1.0] range
4. **Deduplication** — The `derived_set` must track `(RuleID, SubjectID, ObjectID)` tuples to prevent duplicate derivations
5. **Priority clamping** — Rule priority must be clamped to `i8` range when applied to derived facts (`i8::MIN` to `i8::MAX`)
6. **No unwrap** — Zero `unwrap()` in production code paths; all errors return `KcmError`
7. **Pattern match safety** — `find_pattern_matches` must skip deleted rows (`is_deleted`) and handle `None` column values gracefully

## Audit Logging

| Event | Log Level | Details |
|-------|-----------|---------|
| Rule registered | INFO | `RuleID`, rule name |
| Inference started | INFO | `max_iterations`, `timeout_secs` |
| Inference completed | INFO | `InferenceStats` (iterations, facts derived, rules applied, duration) |
| Inference timeout | WARN | Elapsed time, iteration count |
| Inference iteration limit | WARN | Iteration count reached `max_iterations` |

## Validation Checklist

- [ ] `max_iterations` is enforced and cannot be set to 0
- [ ] `timeout_secs` is enforced and cannot be set to 0
- [ ] `RuleRegistry` rejects duplicate `RuleID` values
- [ ] `confidence_threshold` filters low-confidence derivations
- [ ] `derived_set` prevents duplicate derivations per iteration
- [ ] `find_pattern_matches` skips deleted schema rows
- [ ] Priority is clamped to `i8` range in derived facts
- [ ] No `unwrap()` in production code paths
- [ ] No `panic!()` in production code paths
- [ ] All public APIs return `Result<T, KcmError>`
- [ ] `ConfidenceFormula` closures produce values in [0.0, 1.0]

## References

- [SECURITY.md](../../SECURITY.md) — Project-wide security policy
- [AGENTS.md](../../AGENTS.md) — Engineering constitution
- [SSOT.md](../../SSOT.md) — Single Source of Truth
- [docs/kcm-reasoning/spesifikasi.md](../../docs/kcm-reasoning/spesifikasi.md) — Technical specification
