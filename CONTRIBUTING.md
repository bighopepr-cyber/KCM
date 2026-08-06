# Contributing to KCM

Thank you for your interest in contributing to KCM! This document provides guidelines and instructions for contributing.

## Code of Conduct

This project adheres to the [Microsoft Open Source Code of Conduct](https://opensource.microsoft.com/codeofconduct/). By participating, you are expected to uphold this code.

## Getting Started

### Prerequisites

- Rust 1.85+ (edition 2021)
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
5. For SDK bugs, include the SDK language and version

### Suggesting Features

1. Check [existing issues](https://github.com/bighopepr-cyber/KCM/issues) to avoid duplicates
2. Use the **Feature Request** issue template
3. Explain the use case and proposed solution
4. Reference relevant SSOT requirements if applicable
5. For SDK features, include proposed API examples for affected languages

### Submitting Changes

1. Fork the repository
2. Create a feature branch from `main`
3. Make your changes following the coding standards below
4. Add or update tests for your changes
5. Ensure all quality gates pass
6. Submit a pull request

## Contributing to SDKs

### SDK Directory Structure

Each SDK follows a consistent structure:

```
sdk/<language>/
├── src/            # Source code
├── tests/          # Test suite
├── examples/       # Usage examples
├── README.md       # SDK documentation
└── <build-file>    # Language-specific build config
```

### SDK Contribution Rules

1. **API Consistency**: All SDKs must expose the same core API surface defined in `sdk/README.md`
2. **Examples Required**: Every public API must have at least one runnable example
3. **Tests Required**: Every public API must have test coverage
4. **Documentation Required**: Every public API must have docstrings/comments
5. **SSOT Compliance**: SDK changes must trace to SSOT requirements

### SDK-Specific Guidelines

| Language | Build File | Linter | Type Checker | Test Framework |
|----------|-----------|--------|--------------|----------------|
| Rust | Cargo.toml | clippy | rustc | cargo test |
| Python | pyproject.toml | ruff | mypy | pytest |
| JavaScript | package.json | eslint | — | jest |
| TypeScript | package.json | eslint | tsc | jest |
| Go | go.mod | go vet | go vet | go test |
| Java | pom.xml | — | — | mvn test |
| .NET | *.csproj | — | — | dotnet test |
| C | Makefile | — | — | make test |
| C++ | CMakeLists.txt | — | — | ctest |

### Cross-SDK API Surface

All SDKs implement the same core operations:

| Operation | Description |
|-----------|-------------|
| Database(path) | Open or create a database |
| insert(fact) | Insert a knowledge fact |
| query(kql) | Execute a KQL query |
| delete(row_id) | Delete a fact by ID |
| update(fact) | Update an existing fact |
| fact_count() | Get total fact count |
| active_count() | Get active fact count |
| begin_transaction() | Start a transaction |
| commit(txn) | Commit a transaction |
| rollback(txn) | Rollback a transaction |
| save(path) | Save database to file |
| load(path) | Load database from file |
| verify() | Verify database integrity |
| close() | Close database |

## Coding Standards

### Rust Edition

All code must use **Rust edition 2021**. Do not use older edition patterns.

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

### Code Style — Rust

- Use `parking_lot` for mutexes/rwlocks (not std)
- Use `Send + Sync` bounds on all shared types
- All `unsafe` blocks must have `// SAFETY:` comment
- All public types implement `Debug`
- Prefer `is_some_and` over `map_or(false, ...)`
- Prefer `div_ceil` over manual ceiling division
- Prefer `clamp` over chained `min/max`
- Use `or_default()` over `or_insert_with(Vec::new)`

### Code Style — Python

- Follow PEP 8 style
- Use type hints for all public functions
- Use docstrings for all public APIs (Google style)
- Prefer `pathlib` over `os.path`
- Use `ruff` for linting
- Use `mypy` for type checking

### Code Style — JavaScript/TypeScript

- Use ESLint with recommended config
- Use JSDoc comments for public APIs
- TypeScript: enable strict mode
- Use `const` by default, `let` when reassignment is needed
- Never use `var`

### Code Style — Go

- Follow `gofmt` conventions
- Use `golint` / `go vet` for linting
- Write Go doc comments for all exported types
- Handle all errors explicitly
- Use context.Context for cancellation

### Code Style — Java

- Follow Google Java Style Guide
- Use Javadoc for all public APIs
- Prefer immutability
- Use `Optional` instead of null returns

### Code Style — .NET

- Follow .NET coding conventions
- Use XML doc comments for all public APIs
- Use `async/await` for async operations
- Prefer `IAsyncEnumerable` over `Task<List<T>>`

### Code Style — C

- Follow Linux kernel style (with adjustments)
- All functions have `/// ` doc comments
- Use `KCM_` prefix for all public symbols
- Always check return values
- Never use `malloc` without null check

### Code Style — C++

- Follow C++ Core Guidelines
- Use RAII for resource management
- Use `std::string_view` for read-only string parameters
- Prefer smart pointers over raw pointers
- Use namespaces for organization

### Dependencies

- Use `workspace.dependencies` for all shared dependencies (Rust)
- Do not add new dependencies without justification
- All dependencies must be auditable

### Testing

- Every new feature must have tests
- Every bug fix must have a regression test
- Property tests for arithmetic operations
- Security code must have security tests
- Performance code must have benchmarks
- SDKs must have integration tests against the core engine

## Quality Gates

All pull requests must pass these checks before merge:

### Core Engine

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

### SDKs

```bash
# Validate all SDKs
bash scripts/validate-sdk-api.sh

# Per-SDK checks (examples)
cd sdk/python && ruff check src/ tests/ && mypy src/ && pytest
cd sdk/javascript && npm run lint && npm test
cd sdk/typescript && npx tsc --noEmit && npm run lint && npm test
cd sdk/go && go vet ./... && go test ./...
cd sdk/java && mvn test
cd sdk/dotnet && dotnet test
cd sdk/c && make test
cd sdk/cpp && cmake --build build && ctest --test-dir build
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
feat(sdk-python): add async query support
fix(sdk-go): handle null pointer in FFI calls
```

## License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).
