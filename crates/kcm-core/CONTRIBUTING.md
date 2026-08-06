# Contributing to kcm-core

Contribution guidelines specific to the `kcm-core` crate.

> For core engine contribution rules, refer to the repository [CONTRIBUTING.md](../../CONTRIBUTING.md).

## Overview

`kcm-core` is the foundational crate of the KCM engine. All other crates depend on it. Changes here have the widest blast radius and require the highest quality bar.

## Before Contributing

1. Read the [root CONTRIBUTING.md](../../CONTRIBUTING.md)
2. Read the [kcm-core technical specification](../../docs/kcm-core/spesifikasi.md)
3. Verify your change does not break the public API without an SSOT-approved reason
4. Check [existing issues](https://github.com/bighopepr-cyber/KCM/issues) for related work

## Coding Standards

### Rust Requirements

- Edition 2021
- All public APIs return `Result<T, KcmError>`
- No `unwrap()` in production code
- No `panic!()` in production code
- No `TODO`/`FIXME`/`HACK` markers
- All `unsafe` blocks must have `// SAFETY:` comments
- Use `parking_lot` for synchronization (not `std`)

### Type Design Rules

- `Fact` must validate `Confidence` range at construction
- `DenseVec<T>` must use `T: Copy + Default` bounds
- `Bitmap` must validate all bit positions
- `Dictionary` must handle `u32::MAX` overflow
- All public types must implement `Debug`

### Naming Conventions

| Element | Convention | Example |
|---------|-----------|---------|
| Types | PascalCase | `DenseVec`, `Bitmap` |
| Functions | snake_case | `push`, `get`, `set` |
| Constants | SCREAMING_SNAKE_CASE | `DEFAULT_CAPACITY` |
| Modules | snake_case | `bitmap`, `dictionary` |

## Module Architecture Rules

- `kcm-core` must have **zero** internal crate dependencies
- Only `parking_lot` is allowed as an external dependency (plus optional `serde`)
- No I/O, networking, or file system operations
- No async code — this is a synchronous foundational crate
- All modules must be declared in `lib.rs`

## Documentation Rules

- Every public function must have a `///` doc comment
- Every public type must have a `///` doc comment
- Doc comments must include at least one code example for public APIs
- Module-level documentation must explain the module's purpose

## Testing Requirements

- Every public function must have at least one unit test
- `Bitmap` operations must have property tests
- `Dictionary` must have roundtrip tests
- `DenseVec` must have capacity and boundary tests
- `Fact` must have validation tests for all fields
- Run: `cargo test -p kcm-core`

## Performance Rules

- `DenseVec` must use contiguous memory allocation
- `Bitmap` operations must be O(1) for `get`/`set`
- `Dictionary` lookups must be O(1) amortized
- No unnecessary allocations in hot paths
- Benchmark regressions >5% require justification

## Review Checklist

- [ ] All public APIs return `Result<T, KcmError>`
- [ ] No `unwrap()` in production code
- [ ] No `panic!()` in production code
- [ ] All types have `Debug` implementations
- [ ] All public functions have doc comments
- [ ] All tests pass
- [ ] No clippy warnings
- [ ] SSOT traceability documented

## Pull Request Requirements

- Reference the SSOT requirement being addressed
- Include test coverage for new/changed APIs
- Include benchmarks if performance-sensitive
- Do not break backward compatibility without SSOT approval

## References

- [CONTRIBUTING.md](../../CONTRIBUTING.md) — Repository-wide contribution guidelines
- [CODE_OF_CONDUCT.md](../../CODE_OF_CONDUCT.md) — Community guidelines
- [docs/kcm-core/spesifikasi.md](../../docs/kcm-core/spesifikasi.md) — Technical specification
- [AGENTS.md](../../AGENTS.md) — Engineering constitution
