# Contributing to kcm-testing

> **Note:** This document is crate-specific. The project-wide contribution guidelines are in the repository root at [`CONTRIBUTING.md`](../../CONTRIBUTING.md).

## Overview

kcm-testing is the testing infrastructure crate for KCM. It provides load tests, stress tests, security tests, chaos engineering, regression detection, metrics dashboard, and benchmark fixtures. Contributions to this crate must follow the same engineering rigor as production code, with additional attention to test determinism, reproducibility, and resource management.

## Before Contributing

1. Read the root [`CONTRIBUTING.md`](../../CONTRIBUTING.md) for project-wide conventions.
2. Read [`AGENTS.md`](../../AGENTS.md) for the engineering constitution, quality gates, and non-negotiable rules.
3. Understand the crate's dependency boundaries: kcm-testing depends on kcm-core, kcm-storage, kcm-runtime, kcm-reasoning, kcm-security.
4. Verify your change does not introduce new external dependencies without justification per the Dependency Policy in AGENTS.md.
5. Run the full validation suite before submitting:

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check
```

## Coding Standards

### General Rules

- All public functions must return `Result<T, KcmError>` (except fixture constructors which may panic with descriptive messages).
- No `unwrap()` or `panic!()` in non-test public code paths.
- No `TODO`, `FIXME`, or `HACK` markers in production code.
- Thread safety: shared state must use `Arc`, atomics, or `parking_lot` mutexes.
- Determinism: test data must be generated deterministically from `DatasetConfig`, never from timestamps or external sources.

### Test Code Exception

The crate-level attribute `#[allow(clippy::unwrap_used, clippy::panic)]` permits `unwrap()` and `panic!()` in test code. This is standard practice for the testing crate — tests assert expected behavior and panics provide clear failure messages.

## Module Architecture Rules

kcm-testing depends on:

| Dependency | Usage |
|------------|-------|
| kcm-core | Types (`Fact`, `SubjectID`, `RowID`), `DenseVec`, `Bitmap`, `Dictionary` |
| kcm-storage | `Schema`, `WriteAheadLog`, `DatabaseFile`, `Compressor` |
| kcm-runtime | `KnowledgeDatabase` |
| kcm-reasoning | `RulePattern` (for rule fixtures) |
| kcm-security | `ACLManager` (for RBAC security tests) |
| parking_lot | `Mutex` for shared test state |
| tempfile | `TempDir` for auto-cleaned temporary files |
| getrandom | CSPRNG for chaos fault injection probability |

Additional dev-dependencies for integration tests: `kcm-distributed`, `kcm-compliance`.

### Dependency Rules

- Do not add new dependencies without AGENTS.md Dependency Policy justification.
- Do not depend on crates outside the kcm-* family without explicit approval.
- All `kcm-*` dependencies must be path dependencies (no version-pinned crates.io).

## Documentation Rules

- Every public struct must have a doc comment explaining its purpose.
- Every public function must have a doc comment with a brief description.
- Fixtures must document their invariants (determinism, value ranges, capacity).
- Configuration structs must document valid ranges for all fields.
- Module-level documentation should explain the module's role in the testing infrastructure.

## Testing Requirements

kcm-testing IS the testing crate — it must test itself thoroughly.

### Self-Test Standards

- Every public function must have at least one corresponding `#[test]`.
- Tests must be deterministic: same inputs always produce same results.
- Tests must clean up resources: use `TempDir`, avoid persistent file artifacts.
- Concurrent tests must validate final state (e.g., `fact_count() == expected`).
- Edge cases must be tested: zero users, maximum values, empty baselines, invalid configs.

### Test Naming Convention

```rust
#[test]
fn test_<module>_<scenario>() { ... }
```

Examples:
- `test_stress_sustained_load`
- `test_chaos_monkey_lifecycle`
- `test_metrics_collector`
- `test_no_regression`

## Performance Rules

Test execution time targets:

| Test Category | Target | Max |
|--------------|--------|-----|
| Unit tests | < 100ms | 500ms |
| Integration tests | 1s-5s | 30s |
| Load tests (light/medium) | < 5s | 15s |
| Stress tests | < 10s | 30s |
| Security tests | < 2s | 10s |

- Avoid tests that run indefinitely. All stress tests must have bounded duration.
- Chaos injection must be explicit (activate/deactivate), never automatic.
- Benchmark fixtures must validate pre-conditions outside `b.iter()`.

## Review Checklist

Before submitting a PR to kcm-testing:

- [ ] All new functions return `Result<T, KcmError>` (or panic with descriptive message in fixtures)
- [ ] All new public structs have doc comments
- [ ] All new functions have corresponding tests
- [ ] Tests are deterministic (no timestamps, no external dependencies)
- [ ] Temporary files use `TempDir`
- [ ] Concurrent tests use proper synchronization
- [ ] No new external dependencies added
- [ ] No `TODO`/`FIXME`/`HACK` markers
- [ ] `cargo clippy --workspace -- -D warnings` passes
- [ ] `cargo fmt --all -- --check` passes
- [ ] `cargo test --workspace` passes
- [ ] SSOT requirements traced (if applicable)

## Pull Request Requirements

1. **Title:** `<type>(kcm-testing): <description>` — e.g., `feat(kcm-testing): add chaos latency injection test`
2. **Description:** What changed, why, and how it was tested.
3. **Scope:** Only kcm-testing files unless cross-crate changes are required.
4. **Tests:** All new code must include tests. PRs without tests will be rejected.
5. **Performance:** No regression in test execution time. Include benchmark results if applicable.
6. **SSOT:** If the change affects public API or behavior, update the relevant specification document.

## References

- [Root CONTRIBUTING.md](../../CONTRIBUTING.md)
- [AGENTS.md](../../AGENTS.md)
- [PRD-TESTING — Testing Strategy](../PRD-TESTING%26%20BRACHMARCK.md)
- [Root CODE_OF_CONDUCT.md](../../CODE_OF_CONDUCT.md)
