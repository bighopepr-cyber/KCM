# Contributing to KCM

Thank you for your interest in contributing to KCM! This document provides guidelines and instructions for contributing.

## Code of Conduct

This project adheres to the [Microsoft Open Source Code of Conduct](https://opensource.microsoft.com/codeofconduct/). By participating, you are expected to uphold this code.

## Getting Started

### Prerequisites

- Rust 1.85+ (edition 2024)
- Cargo (comes with Rust)
- Git

### Development Setup

```bash
# Clone the repository
git clone https://github.com/bighopepr-cyber/KCM.git
cd KCM

# Build the workspace
cargo build --workspace

# Run all tests
cargo test --workspace

# Run clippy
cargo clippy --workspace -- -D warnings

# Check formatting
cargo fmt --all -- --check
```

## How to Contribute

### Reporting Bugs

1. Check [existing issues](https://github.com/bighopepr-cyber/KCM/issues) to avoid duplicates
2. Use the **Bug Report** issue template
3. Include reproduction steps, expected behavior, and actual behavior
4. Include your Rust version (`rustc --version`) and OS

### Suggesting Features

1. Check [existing issues](https://github.com/bighopepr-cyber/KCM/issues) to avoid duplicates
2. Use the **Feature Request** issue template
3. Explain the use case and proposed solution
4. Reference relevant SSOT requirements if applicable

### Submitting Changes

1. Fork the repository
2. Create a feature branch from `main`
3. Make your changes following the coding standards below
4. Add or update tests for your changes
5. Ensure all quality gates pass
6. Submit a pull request

## Coding Standards

### Rust Edition

All code must use **Rust edition 2024**. Do not use edition 2021 patterns.

### Non-Negotiable Rules

| Rule | Enforcement |
|------|-------------|
| All public APIs return `Result<T, KcmError>` | Compiler |
| No `unwrap()` in production code | CI gate |
| No `panic!()` in production code | CI gate |
| No TODO/FIXME/HACK in production code | CI gate |
| No placeholder implementations | Code review |
| All tests pass before commit | CI gate |
| All clippy warnings resolved | CI gate |

### Code Style

- Use `parking_lot` for mutexes/rwlocks (not std)
- Use `Send + Sync` bounds on all shared types
- All `unsafe` blocks must have `// SAFETY:` comment
- All public types implement `Debug`
- Prefer `is_some_and` over `map_or(false, ...)`
- Prefer `div_ceil` over manual ceiling division
- Prefer `clamp` over chained `min/max`
- Use `or_default()` over `or_insert_with(Vec::new)`

### Dependencies

- Use `workspace.dependencies` for all shared dependencies
- Do not add new dependencies without justification
- All dependencies must be auditable

### Testing

- Every new feature must have tests
- Every bug fix must have a regression test
- Property tests for arithmetic operations
- Security code must have security tests
- Performance code must have benchmarks

## Quality Gates

All pull requests must pass these checks before merge:

```bash
# Format check
cargo fmt --all -- --check

# Clippy lint
cargo clippy --workspace -- -D warnings

# Build
cargo build --workspace

# Unit tests
cargo test --lib --all

# Integration tests
cargo test --test '*' --all

# Property tests
cargo test property_tests --all

# SSOT validation
bash scripts/validate-ssot.sh
```

## SSOT Traceability

Every change must trace back to a requirement in the SSOT or its authoritative sources:

```
SSOT Requirement → Specification Document → Implementation File → Test File → Benchmark
```

Reference the SSOT requirement in your PR description.

## Pull Request Process

1. Fill out the PR template completely
2. Reference any related issues
3. Describe the SSOT traceability
4. Ensure all CI checks pass
5. Request review from the appropriate code owners
6. Address review feedback

## Commit Messages

Follow conventional commit format:

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`, `ci`

Examples:
```
feat(kcm-storage): add dictionary codec compression
fix(kcm-core): handle NaN confidence validation
docs(kcm-interface): add FFI safety documentation
test(kcm-runtime): add transaction rollback property tests
```

## License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).
