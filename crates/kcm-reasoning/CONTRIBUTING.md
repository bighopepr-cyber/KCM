# Contributing to kcm-reasoning

Contribution guidelines specific to the `kcm-reasoning` crate.

> For core engine contribution rules, refer to the repository [CONTRIBUTING.md](../../CONTRIBUTING.md).

## Overview

`kcm-reasoning` is the inference and reasoning engine for KCM. It implements rule definitions and a forward-chaining inference engine that derives new knowledge from existing facts. Changes here directly affect knowledge correctness — incorrect rules or flawed inference logic can silently corrupt derived facts.

## Before Contributing

1. Read the [root CONTRIBUTING.md](../../CONTRIBUTING.md)
2. Read the [kcm-reasoning technical specification](../../docs/kcm-reasoning/spesifikasi.md)
3. Verify your change does not break the public API without an SSOT-approved reason
4. Check [existing issues](https://github.com/bighopepr-cyber/KCM/issues) for related work
5. Understand the dependency constraints — `kcm-reasoning` may only depend on `kcm-core` and `kcm-storage`

## Coding Standards

### Rust Requirements

- Edition 2021
- All public APIs return `Result<T, KcmError>`
- No `unwrap()` in production code
- No `panic!()` in production code
- No `TODO`/`FIXME`/`HACK` markers
- All `unsafe` blocks must have `// SAFETY:` comments

### Inference Engine Rules

- `InferenceEngine` must enforce `max_iterations` and `timeout_secs` on every call
- `find_pattern_matches` must skip deleted rows (`is_deleted`)
- Derived facts must be deduplicated via `derived_set` before appending to schema
- Confidence thresholds must be applied before `Fact::new` construction
- Rule priority must be clamped to `i8` range when applied to derived facts

### Naming Conventions

| Element | Convention | Example |
|---------|-----------|---------|
| Types | PascalCase | `InferenceEngine`, `RulePattern` |
| Functions | snake_case | `infer_forward_chaining`, `find_pattern_matches` |
| Constants | SCREAMING_SNAKE_CASE | `DEFAULT_MAX_ITERATIONS` |
| Modules | snake_case | `inference`, `rule` |

## Module Architecture Rules

- `kcm-reasoning` depends on **only** `kcm-core` and `kcm-storage`
- No other crate dependencies are permitted without SSOT-approved justification
- No I/O, networking, or file system operations
- No async code — inference is synchronous
- All modules must be declared in `lib.rs`
- `RuleRegistry` is the single registration point for all rules
- `InferenceEngine` is the single entry point for inference execution

### Module Responsibilities

| Module | Responsibility |
|--------|---------------|
| `rule` | `Rule`, `RulePattern`, `RuleRegistry`, `ConfidenceFormula` type definitions |
| `inference` | `InferenceEngine`, `Derivation`, `InferenceStats` — forward-chaining execution |

## Documentation Rules

- Every public function must have a `///` doc comment
- Every public type must have a `///` doc comment
- Doc comments must include at least one code example for public APIs
- Module-level documentation must explain the module's purpose
- `Rule` descriptions must document the knowledge the rule encodes

## Testing Requirements

### Inference Correctness

- Rules must produce correct derived facts for known input schemas
- Multi-iteration inference must terminate within `max_iterations`
- Inference with no matching rules must return empty results
- Inference with no rules registered must return empty results
- Disabled rules must not participate in inference

### Rule Application

- `RuleRegistry` must reject duplicate `RuleID` values
- `RuleRegistry::all_enabled` must return only enabled rules
- `RulePattern::And`, `Or`, `Not` combinators must match correctly
- `ConfidenceFormula` closures must be applied to matched confidence values

### Provenance Tracking

- Each `Derivation` must carry the originating `RuleID`
- `InferenceStats` must report accurate iteration count, facts derived, and rules applied
- Backward compatibility between `infer_forward_chaining` and `infer_with_stats` must be maintained

### Running Tests

```bash
cargo test -p kcm-reasoning
```

## Performance Rules

- Inference throughput target: ≥ 10,000 facts/second for simple single-predicate rules
- Pattern matching must be O(n) per rule per iteration (n = schema row count)
- `RuleRegistry::all_enabled` must not allocate on repeated calls in steady state
- Benchmark regressions >5% require justification
- No unnecessary allocations in the inference hot loop

## Review Checklist

- [ ] All public APIs return `Result<T, KcmError>`
- [ ] No `unwrap()` in production code
- [ ] No `panic!()` in production code
- [ ] `max_iterations` and `timeout_secs` are enforced
- [ ] Derived facts are deduplicated
- [ ] Confidence thresholds are applied
- [ ] All public functions have doc comments
- [ ] All tests pass
- [ ] No clippy warnings
- [ ] SSOT traceability documented

## Pull Request Requirements

- Reference the SSOT requirement being addressed (PRD.md §6)
- Include test coverage for new/changed inference logic
- Include benchmarks if performance-sensitive
- Do not break backward compatibility without SSOT approval
- Document any changes to inference semantics or termination guarantees

## References

- [CONTRIBUTING.md](../../CONTRIBUTING.md) — Repository-wide contribution guidelines
- [CODE_OF_CONDUCT.md](../../CODE_OF_CONDUCT.md) — Community guidelines
- [docs/kcm-reasoning/spesifikasi.md](../../docs/kcm-reasoning/spesifikasi.md) — Technical specification
- [AGENTS.md](../../AGENTS.md) — Engineering constitution
