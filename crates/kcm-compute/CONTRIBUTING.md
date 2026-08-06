# Contributing to kcm-compute

Contribution guidelines specific to the `kcm-compute` crate.

> For core engine contribution rules, refer to the repository [CONTRIBUTING.md](../../CONTRIBUTING.md).

## Overview

`kcm-compute` implements the relational algebra operators and SIMD-accelerated column operations for the KCM query engine. It depends on `kcm-core` and `kcm-storage` only. Changes here affect query correctness and performance for all consumers.

## Before Contributing

1. Read the [root CONTRIBUTING.md](../../CONTRIBUTING.md)
2. Read the [kcm-compute technical specification](../../docs/kcm-compute/spesifikasi.md)
3. Verify your change does not break the public API without an SSOT-approved reason
4. Check [existing issues](https://github.com/bighopepr-cyber/KCM/issues) for related work

## Coding Standards

### Rust Requirements

- Edition 2024
- All public APIs return `Result<T, KcmError>`
- No `unwrap()` in production code
- No `panic!()` in production code
- No `TODO`/`FIXME`/`HACK` markers
- All `unsafe` blocks must have `// SAFETY:` comments

### Type Design Rules

- All operators must implement `Operator` trait (`execute()` + `estimated_rows()`)
- SIMD operations must have scalar fallback paths
- Aggregation functions must handle empty input gracefully
- Filter predicates must use exhaustive pattern matching

### Naming Conventions

| Element | Convention | Example |
|---------|-----------|---------|
| Types | PascalCase | `ScanOp`, `FilterOp`, `AggregateOp` |
| Functions | snake_case | `execute`, `simd_filter_eq` |
| Enums | PascalCase | `FilterPredicate`, `AggregateFunc` |
| Modules | snake_case | `algebra`, `simd` |

## Module Architecture Rules

- `kcm-compute` depends on **kcm-core** and **kcm-storage** only
- No other KCM crate dependencies are permitted
- No I/O, networking, or file system operations
- No async code — this is a synchronous compute crate
- All modules must be declared in `lib.rs`
- SIMD implementations must be architecture-gated (`#[cfg(target_arch = "x86_64")]`)

## Documentation Rules

- Every public function must have a `///` doc comment
- Every public type must have a `///` doc comment
- Doc comments must include at least one code example for public APIs
- Module-level documentation must explain the module's purpose
- SIMD functions must document their safety preconditions

## Testing Requirements

### Algebra Operator Tests

- Every operator must have at least one unit test for `execute()`
- Every operator must have at least one test for `estimated_rows()`
- Filter predicates must be tested for all enum variants
- Join must be tested for empty inputs, single matches, and multiple matches
- Aggregate must be tested for Count, Sum, Avg, Min, Max
- Grouped aggregation must be tested with multiple groups

### SIMD Correctness Tests

- Every SIMD function must have a scalar correctness test
- SIMD functions must be tested with empty input
- SIMD functions must be tested with data smaller than one chunk
- SIMD functions must be tested with data exactly one chunk size
- SIMD functions must be tested with data larger than one chunk
- Boundary values (0, MAX, MIN) must be tested
- Run: `cargo test -p kcm-compute`

## Performance Rules

- SIMD acceleration must achieve ≥4x speedup for batch column operations
- Filter operations must process ≥100M elements/second with AVX2
- Aggregate operations must use SIMD where data type permits
- No unnecessary allocations in hot paths
- Benchmark regressions >5% require justification

## Review Checklist

- [ ] All public APIs return `Result<T, KcmError>`
- [ ] No `unwrap()` in production code
- [ ] No `panic!()` in production code
- [ ] All `unsafe` blocks have `// SAFETY:` comments
- [ ] SIMD functions have scalar fallback paths
- [ ] Aggregation handles empty input
- [ ] All public functions have doc comments
- [ ] All tests pass
- [ ] No clippy warnings
- [ ] SSOT traceability documented

## Pull Request Requirements

- Reference the SSOT requirement being addressed
- Include test coverage for new/changed operators or SIMD functions
- Include benchmarks if performance-sensitive
- Do not break backward compatibility without SSOT approval
- Document any SIMD target feature requirements

## References

- [CONTRIBUTING.md](../../CONTRIBUTING.md) — Repository-wide contribution guidelines
- [CODE_OF_CONDUCT.md](../../CODE_OF_CONDUCT.md) — Community guidelines
- [docs/kcm-compute/spesifikasi.md](../../docs/kcm-compute/spesifikasi.md) — Technical specification
- [AGENTS.md](../../AGENTS.md) — Engineering constitution
