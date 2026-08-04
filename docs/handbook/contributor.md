# Contributor Handbook

Welcome to the KCM contributor community!

## Getting Started

### Prerequisites

- Rust 1.85+ (see rust-toolchain.toml)
- Git
- GitHub account

### First Contribution

1. Fork the repository
2. Clone your fork
3. Create a branch
4. Make changes
5. Run tests
6. Submit PR

### Development Setup

```bash
git clone https://github.com/your-username/KCM.git
cd KCM
cargo build --workspace
cargo test --workspace
```

## Coding Standards

### Rust Style

- Follow `rustfmt` defaults
- Use `clippy` warnings as errors
- No `unwrap()` in production code
- No `panic!()` in production code
- All public APIs return `Result<T, KcmError>`

### Documentation

- Write doc comments for all public items
- Include examples in doc comments
- Update README when adding features
- Add ADR for architectural decisions

### Testing

- Write unit tests for new functions
- Write integration tests for new features
- Maintain test coverage >= 95%
- Property tests for algorithms

## Pull Request Process

1. Fill out PR template
2. Reference related issues
3. Include test results
4. Update documentation
5. Get review from CODEOWNERS
6. Address feedback
7. Merge after approval

## Code Review

### What We Look For

- Correctness
- Performance
- Security
- Documentation
- Test coverage
- Code style

### Review Timeline

- Initial review: Within 48 hours
- Follow-up: Within 24 hours

## Getting Help

- GitHub Discussions: Ask questions
- Discord: Real-time chat
- Issues: Report bugs
