# Contributing to KCM

Thank you for your interest in contributing to KCM!

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

### 3. Make Changes

Follow the coding standards:
- All public APIs return `Result<T, KcmError>`
- No `unwrap()` in production code
- No `panic!()` in production code
- No TODO/FIXME/HACK in production code
- Write tests for new functionality
- Update documentation

### 4. Run Checks

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check
```

### 5. Submit PR

- Fill out the PR template
- Reference any related issues
- Include test results

## Development Setup

### Prerequisites

- Rust 1.85+ (see rust-toolchain.toml)
- Docker (for deployment testing)

### Building

```bash
cargo build --workspace
```

### Testing

```bash
cargo test --workspace
```

## Code Review Process

1. PR submitted
2. CI passes (build, test, clippy, fmt)
3. Code review by CODEOWNERS
4. Approval and merge

## Reporting Issues

Use GitHub Issues with the provided templates:
- Bug Report: For reporting bugs
- Feature Request: For suggesting features

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
