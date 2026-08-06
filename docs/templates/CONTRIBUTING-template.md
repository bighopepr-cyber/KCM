# Contributing to {{CRATE_NAME}}

Contribution guidelines specific to the `{{CRATE_NAME}}` crate.

> For core engine contribution rules, refer to the repository [CONTRIBUTING.md](../../CONTRIBUTING.md).

## Overview

{{CONTRIBUTING_OVERVIEW}}

## Before Contributing

1. Read the [root CONTRIBUTING.md](../../CONTRIBUTING.md)
2. Read the [{{CRATE_NAME}} technical specification](../../docs/{{CRATE_NAME}}/spesifikasi.md)
3. Verify your change does not break the public API

## Coding Standards

### Rust Requirements

- Edition 2024
- All public APIs return `Result<T, KcmError>`
- No `unwrap()` in production code
- No `panic!()` in production code
- Use `parking_lot` for synchronization

### Naming Conventions

| Element | Convention | Example |
|---------|-----------|---------|
| Types | PascalCase | `TypeName` |
| Functions | snake_case | `function_name` |
| Constants | SCREAMING_SNAKE_CASE | `CONSTANT_NAME` |

## Module Architecture Rules

{{MODULE_RULES}}

## Documentation Rules

- Every public function must have a `///` doc comment
- Every public type must have a `///` doc comment
- Doc comments must include code examples

## Testing Requirements

- Every public function must have at least one unit test
- Run: `cargo test -p {{CRATE_NAME}}`

## Performance Rules

{{PERFORMANCE_RULES}}

## Review Checklist

- [ ] All public APIs return `Result<T, KcmError>`
- [ ] No `unwrap()` in production code
- [ ] All tests pass
- [ ] No clippy warnings
- [ ] SSOT traceability documented

## Pull Request Requirements

- Reference the SSOT requirement being addressed
- Include test coverage for new/changed APIs
- Do not break backward compatibility without approval

## References

- [CONTRIBUTING.md](../../CONTRIBUTING.md) — Repository-wide contribution guidelines
- [CODE_OF_CONDUCT.md](../../CODE_OF_CONDUCT.md) — Community guidelines
- [AGENTS.md](../../AGENTS.md) — Engineering constitution
