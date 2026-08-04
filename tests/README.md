# KCM Integration Tests

Cross-crate integration tests that validate correctness across module boundaries.

## Test Categories

| Category | Location | Purpose |
|----------|----------|---------|
| Unit tests | crates/*/tests/ | Per-crate correctness |
| Integration tests | crates/*/tests/ | Cross-module interaction |
| Property tests | crates/*/tests/*_property.rs | Invariant verification |
| Security tests | crates/kcm-testing/src/security_tests.rs | Attack surface validation |
| Stress tests | crates/kcm-testing/tests/test_stress_scale.rs | Performance under load |
| Recovery tests | crates/kcm-testing/tests/test_recovery.rs | Crash recovery |
| Concurrent tests | crates/kcm-testing/tests/test_concurrent_access.rs | Thread safety |

## Running Tests

```bash
# All tests
cargo test --workspace

# Unit tests only
cargo test --lib --workspace

# Integration tests only
cargo test --test '*' --workspace

# Property tests
cargo test property_tests --workspace -- --nocapture

# Security tests
cargo test security_tests --workspace -- --nocapture

# Stress tests
cargo test stress_tests --workspace -- --nocapture

# Recovery tests
cargo test recovery --workspace -- --nocapture
```

## Test Counts

- Unit tests: 534+
- Property tests: 8+ (with 1000+ cases each)
- Security tests: 29+
- Integration tests: 108+

## Quality Gates

All tests must pass before merge. CI enforces:
- 100% test pass rate
- >=95% code coverage
- 0 clippy warnings
- 0 formatting diff
