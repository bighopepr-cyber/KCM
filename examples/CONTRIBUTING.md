# Contributing to examples/

> For core engine contribution rules, refer to the repository [CONTRIBUTING.md](../CONTRIBUTING.md).

## Overview

This document describes how to contribute code examples to the KCM `examples/` directory. Examples serve as educational references for users learning to integrate KCM into their projects. Each language subdirectory (`rust/`, `python/`, `go/`, `java/`, `javascript/`) is an independent module with its own build system and conventions.

## Before Contributing

1. Review existing examples in `examples/rust/` for style and structure.
2. Check the [Examples README](README.md) for current status and planned examples.
3. Open an issue describing the example you intend to add.
4. Ensure the example demonstrates a real KCM use case (not synthetic/fake functionality).
5. Confirm the example compiles and runs on a clean environment.

## Coding Standards

### Idiomatic Code

- Write idiomatic code for the target language.
- Rust examples must follow `rustfmt` conventions.
- Python examples must follow PEP 8.
- JavaScript examples must follow project ESLint rules.
- Go examples must follow `gofmt` conventions.
- Java examples must follow Google Java Style Guide.

### Clear Comments

- Every example must include a brief description at the top explaining what it demonstrates.
- Use doc comments for public functions/types.
- Do not over-comment obvious code.
- Comments must be accurate — no stale or misleading comments.

### Realistic Data

- Use realistic but synthetic data (e.g., sample names, addresses, product IDs).
- Do not use placeholder data like `foo`, `bar`, `test123`.
- Data should illustrate real-world KCM use cases (knowledge graphs, reasoning, transactions).

## Module Architecture Rules

| Rule | Description |
|------|-------------|
| Independence | Each language example is independent and self-contained |
| No cross-language deps | Examples in one language must not depend on examples in another |
| Single responsibility | Each example demonstrates one concept clearly |
| Build isolation | Each language subdirectory has its own build configuration |
| No shared runtime | Examples do not share a common runtime or framework |

## Documentation Rules

| Rule | Requirement |
|------|-------------|
| README required | Every example directory must have a `README.md` |
| Prerequisites | README must list all prerequisites (Rust version, Python version, etc.) |
| Run instructions | README must include step-by-step instructions to build and run |
| Expected output | README must show expected output |
| Security warning | README must include a note that examples are not for production use |

## Testing Requirements

| Requirement | Standard |
|-------------|----------|
| Compilation | Every example must compile without errors |
| Execution | Every example must run to completion |
| Error handling | Examples must handle errors gracefully |
| No panics | Rust examples must not panic (use `Result`, not `unwrap()`) |
| CI integration | Examples must be buildable in CI (currently Rust only) |

### Running Examples

```bash
# Rust
cd examples/rust
cargo run --example basic_usage

# Python (when available)
cd examples/python
python basic_usage.py

# JavaScript (when available)
cd examples/javascript
node basic_usage.js

# Go (when available)
cd examples/go
go run basic_usage.go

# Java (when available)
cd examples/java
javac BasicUsage.java && java BasicUsage
```

## Performance Rules

- Examples must complete within 30 seconds.
- Examples must not allocate unbounded memory.
- Examples must not create unbounded thread pools.
- Benchmarks are not included in examples (use `benches/` instead).

## Review Checklist

Before submitting a PR for an example:

- [ ] Code compiles and runs without errors
- [ ] Code follows language-specific style guidelines
- [ ] README.md is present with prerequisites, run instructions, and expected output
- [ ] No hardcoded paths, secrets, or credentials
- [ ] Error handling is demonstrated (no `unwrap()` in Rust)
- [ ] Data is realistic and synthetic
- [ ] Example has a single, clear purpose
- [ ] No external network calls
- [ ] No new external dependencies (or justified in PR description)
- [ ] Passes CI checks (format, clippy, build, test)

## Pull Request Requirements

| Requirement | Description |
|-------------|-------------|
| Title format | `examples: add <language> <concept> example` |
| Description | What the example demonstrates and why it's useful |
| Language | Which language subdirectory is affected |
| Prerequisites | Any new tools or versions required |
| Screenshot/output | Expected terminal output included in PR description |
| Checklist | All review checklist items addressed |

## References

- [Examples README](README.md)
- [Repository CONTRIBUTING.md](../CONTRIBUTING.md)
- [Examples SECURITY.md](SECURITY.md)
- [KCM AGENTS.md](../AGENTS.md)
