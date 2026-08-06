# Contributing to kcm-optimizer

> For core engine contribution rules, refer to the repository [CONTRIBUTING.md](../../CONTRIBUTING.md).

---

## Overview

This document provides contribution guidelines specific to the `kcm-optimizer` crate. The optimizer is responsible for cost-based query optimization, query planning, statistics collection, plan rewriting, and adaptive execution. Contributions must maintain correctness guarantees, performance targets, and architectural integrity.

## Before Contributing

1. Read the project-wide [CONTRIBUTING.md](../../CONTRIBUTING.md) for general rules
2. Review the optimizer specification in [PRD2.md §16](../../docs/PRD2.md)
3. Understand the crate dependency boundary: `kcm-optimizer` depends on `kcm-core` and `kcm-storage` only
4. Check existing issues and pull requests to avoid duplication
5. Run `cargo test --workspace` and `cargo clippy --workspace -- -D warnings` before submitting

## Coding Standards

| Rule | Standard | Enforcement |
|------|----------|-------------|
| Error handling | All public APIs return `Result<T, KcmError>` | Compiler + clippy |
| No unwrap | Zero `unwrap()` in production code | CI gate |
| No panic | Zero `panic!()` in production code | CI gate |
| No TODO/FIXME | Zero markers in production code | CI gate |
| No placeholders | Every function has real implementation | Code review |
| Thread safety | All shared types are `Send + Sync` | Compiler |
| Memory safety | No unsafe without documented justification | Code review |
| Format | `cargo fmt --all -- --check` must pass | CI gate |
| Lint | `cargo clippy --workspace -- -D warnings` must pass | CI gate |

## Module Architecture Rules

The `kcm-optimizer` crate has strict dependency boundaries:

| Allowed Dependency | Reason |
|--------------------|--------|
| `kcm-core` | Core types (`KcmError`, `Fact`, `RowID`, `SubjectID`, `Confidence`) |
| `kcm-storage` | Schema, columns, dictionary, and index access |

| Prohibited Dependency | Reason |
|----------------------|--------|
| `kcm-compute` | Separation of concerns — optimizer plans, compute executes |
| `kcm-reasoning` | Reasoning engine is a consumer of optimizer output |
| `kcm-runtime` | Runtime orchestrates optimizer; optimizer must not depend upward |
| `kcm-interface` | Interface layer is above optimizer in dependency graph |
| Any external crate not in approved list | Justification required per AGENTS.md |

### Adding Dependencies

External dependencies require explicit justification:

1. Document the necessity in the pull request description
2. Verify the dependency is not already provided transitively through `kcm-core` or `kcm-storage`
3. Confirm the dependency is `no_std` compatible or justified for `std` usage
4. Add to the crate-level dependency table in this document

## Documentation Rules

| Requirement | Standard |
|-------------|----------|
| Public API docs | Every public function, struct, enum, and trait must have doc comments |
| Module docs | Every module must have a module-level doc comment |
| Algorithm docs | Cost model algorithms must reference their mathematical basis |
| Plan node docs | Each `PlanNode` variant must document its semantic behavior |
| Error docs | Every error variant must document when it occurs |

## Testing Requirements

### Optimizer Plan Tests

Every optimizer rule must have tests validating:

- Correct plan transformation for valid input
- No transformation for already-optimal input
- Correctness of the resulting plan (same result as unoptimized)
- Performance improvement (cost reduction or node reduction)

```rust
// Example test pattern
#[test]
fn test_filter_pushdown_across_join() {
    let plan = create_plan_with_filter_above_join();
    let optimized = FilterPushdownOptimizer::new().optimize(plan);
    assert!(optimized.filter_depth() < original_depth());
}
```

### Cost Model Tests

Cost model tests must validate:

- Monotonicity: larger inputs produce larger costs
- Bounds: costs are always non-negative and finite
- Consistency: identical inputs produce identical costs
- Sensitivity: cost differences reflect actual performance differences

### Statistics Tests

Statistics tests must validate:

- Correct cardinality estimation for known distributions
- Selectivity bounds `[0.0, 1.0]` are maintained
- Freshness validation rejects stale statistics
- Default statistics provide reasonable fallbacks

## Performance Rules

| Metric | Target | Measurement |
|--------|--------|-------------|
| Optimizer overhead | < 5% of total query time | Criterion benchmarks |
| Planning time (simple query) | < 1ms | Criterion benchmarks |
| Planning time (complex query) | < 10ms | Criterion benchmarks |
| Statistics collection overhead | < 1% of query time | Criterion benchmarks |
| Memory overhead per plan | < 64KB | Memory profiling |

Performance regressions exceeding 5% from baseline are blocking. Run benchmarks before submitting:

```bash
cargo bench --workspace --bench optimizer_bench
```

## Review Checklist

Before approving a pull request to `kcm-optimizer`:

- [ ] All public APIs return `Result<T, KcmError>`
- [ ] No `unwrap()` in production code
- [ ] No `panic!()` in production code
- [ ] No TODO/FIXME markers
- [ ] No placeholder implementations
- [ ] Tests pass for all affected optimizer rules
- [ ] Cost model tests pass
- [ ] Statistics tests pass
- [ ] Benchmarks within 5% of baseline
- [ ] Clippy clean (`cargo clippy --workspace -- -D warnings`)
- [ ] Format clean (`cargo fmt --all -- --check`)
- [ ] Documentation updated for public API changes
- [ ] No new external dependencies without justification
- [ ] Dependency boundary (core + storage only) respected
- [ ] Plan verification handles edge cases
- [ ] Security considerations reviewed for cost model changes

## Pull Request Requirements

| Requirement | Description |
|-------------|-------------|
| Title format | `feat(optimizer): <description>`, `fix(optimizer): <description>`, `refactor(optimizer): <description>` |
| Description | What changed, why, how it was tested |
| Benchmark results | Include before/after for performance-relevant changes |
| Test coverage | New code must include tests; coverage must not decrease |
| SSOT alignment | Changes must align with PRD2.md §16 |

## References

| Document | Scope |
|----------|-------|
| [Project CONTRIBUTING.md](../../CONTRIBUTING.md) | General contribution rules |
| [AGENTS.md](../../AGENTS.md) | Engineering constitution |
| [PRD2.md §16](../../docs/PRD2.md) | Optimizer specification |
| [PRD-TESTING](../../docs/PRD-TESTING%26%20BRACHMARCK.md) | Testing and benchmarking strategy |
