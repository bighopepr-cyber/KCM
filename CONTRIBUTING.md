# Contributing to KCM

Thank you for your interest in contributing to KCM!

## SSOT-First Development

**All development must follow the SSOT (Single Source of Truth) protocol.** The SSOT documentation is the absolute technical contract for the KCM project. No implementation may deviate from the SSOT without an approved SSOT update.

Before making any code change:
1. Find the SSOT requirement that mandates this change
2. Verify the specification exists and is current
3. Plan the implementation matching the specification
4. Write code that exactly matches the specification
5. Write tests that validate against the specification
6. Run `bash scripts/validate-ssot.sh` to verify compliance

## How to Contribute

### 1. Fork and Clone

```bash
git clone https://github.com/your-username/KCM.git
cd KCM
```

### 2. Create Branch

```bash
git checkout -b feature/your-feature-name
```

### 3. Identify SSOT Requirement

Every change must trace back to an SSOT requirement. Find the requirement in:
- `docs/PRD.md` — Core types, storage, compute, reasoning
- `docs/PRD2.md` — Storage, runtime, interfaces
- `docs/PRD3.md` — Distributed, ML, security, compliance
- `docs/PRD-TESTING& BRACHMARCK.md` — Testing, benchmarks, quality gates

If no requirement exists, create one in the appropriate PRD document before implementing.

### 4. Make Changes

Follow the coding standards:
- All public APIs return `Result<T, KcmError>`
- No `unwrap()` in production code
- No `panic!()` in production code
- No TODO/FIXME/HACK in production code
- No placeholder implementations
- No fake success responses
- Write tests for new functionality
- Update documentation if behavior changes

### 5. Run Checks

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check
bash scripts/validate-ssot.sh
```

### 6. Submit PR

- Fill out the PR template
- Reference the SSOT requirement ID
- Include test results
- Include benchmark results if performance-related
- Document any backward compatibility implications

## Development Setup

### Prerequisites

- Rust 1.85+ (see rust-toolchain.toml)
- Docker (for deployment testing)

### Building

```bash
cargo build --workspace
cargo build --release --workspace
```

### Testing

```bash
cargo test --workspace
cargo test -p kcm-core
cargo test -p kcm-storage
```

### Benchmarks

```bash
cargo bench --workspace
cargo bench --workspace --no-run  # compile only
```

### SSOT Validation

```bash
bash scripts/validate-ssot.sh
```

## Code Review Process

1. PR submitted with SSOT requirement reference
2. CI passes (build, test, clippy, fmt, SSOT validation)
3. Code review by CODEOWNERS
4. SSOT compliance verification
5. Approval and merge

## Quality Gates

Every PR must pass these quality gates:

| Gate | Command | Blocks Merge |
|------|---------|-------------|
| Format | `cargo fmt --all -- --check` | Yes |
| Clippy | `cargo clippy --workspace -- -D warnings` | Yes |
| Build | `cargo build --workspace` | Yes |
| Unit Tests | `cargo test --lib --all` | Yes |
| Integration Tests | `cargo test --test '*' --all` | Yes |
| SSOT Validation | `bash scripts/validate-ssot.sh` | Yes |

## Reporting Issues

Use GitHub Issues with the provided templates:
- Bug Report: For reporting bugs
- Feature Request: For suggesting features

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
