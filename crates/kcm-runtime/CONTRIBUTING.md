# Contributing to kcm-runtime

> This document covers contribution guidelines specific to the `kcm-runtime` crate. For the project-wide contribution guide, see the root `CONTRIBUTING.md`.

## Overview

`kcm-runtime` provides the runtime and transaction layer for KCM, including `KnowledgeDatabase`, `Transaction`, `Metrics`, `Health`, `Executor`, and `AsyncExecutor`. Contributions to this crate affect data integrity, concurrency correctness, and performance of the entire system.

## Before Contributing

1. Read `AGENTS.md` — the engineering constitution is binding on all contributions.
2. Read `docs/PRD2.md §18` — the runtime specification is the source of truth for behavior.
3. Check existing issues and PRs to avoid duplicate work.
4. Run `cargo clippy --workspace -- -D warnings` and `cargo test --workspace` to verify baseline.

## Coding Standards

- All public APIs return `Result<T, KcmError>`
- No `unwrap()` in production code paths
- No `panic!()` in production code
- No `TODO` / `FIXME` / `HACK` markers
- Use `parking_lot` for locks (not `std::sync`)
- Use `AtomicU64` for metric counters
- Use `rayon` for synchronous parallel execution
- Use `tokio` for asynchronous execution
- Follow existing code style in the crate

## Module Architecture Rules

| Dependency | Allowed Usage |
|---|---|
| `kcm-core` | Types (`Fact`, `RowID`, `SubjectID`, `Confidence`, `KcmError`), `DenseVec`, `Bitmap`, `Dictionary` |
| `kcm-storage` | Column storage, codecs, WAL, file format, index, backup, recovery |
| `kcm-optimizer` | Query planning, cost model, plan rewriting |
| `parking_lot` | `RwLock`, `Mutex` for shared state synchronization |
| `rayon` | Thread pool for synchronous parallel operations |
| `tokio` | Async runtime for asynchronous operations |

Do not add new dependencies without justification per the dependency policy in `AGENTS.md`.

## Documentation Rules

- Every public function must have a doc comment explaining purpose, parameters, and return value
- Module-level doc comments must explain the module's responsibility
- Complex algorithms must include inline comments explaining the approach
- All behavior changes must be reflected in `docs/PRD2.md §18` or a derived spec document

## Testing Requirements

### Transaction Tests

- Test `begin` / `commit` / `rollback` lifecycle
- Test concurrent transaction isolation
- Test transaction state machine transitions
- Test rollback on error paths

### Concurrent Access Tests

- Test `KnowledgeDatabase` under concurrent read/write load
- Test lock contention scenarios with rayon thread pool
- Test async executor concurrency with tokio
- Test metric counter accuracy under concurrent increments

### Metric Tests

- Test all 14 counters increment correctly
- Test metric reset behavior
- Test metric snapshot consistency

### General Test Standards

- All tests must pass before merge
- Tests must be deterministic
- No sleep-based timing in tests (use barriers or channels)
- Property tests with `proptest` for invariant verification

## Performance Rules

| Operation | Target |
|---|---|
| Insert | > 50,000 facts/sec |
| Query | < 100ms P99 latency |
| Transaction commit | < 10ms P99 latency |
| Metric increment | < 100ns per operation |

Run `cargo bench --workspace` before and after performance-sensitive changes. Regressions beyond 5% require justification and SSOT alignment.

## Review Checklist

Before requesting review:

- [ ] `cargo build --workspace` passes
- [ ] `cargo test --workspace` passes
- [ ] `cargo clippy --workspace -- -D warnings` clean
- [ ] `cargo fmt --all -- --check` clean
- [ ] All public APIs return `Result<T, KcmError>`
- [ ] No `unwrap()` in production code
- [ ] No new dependencies added (or justified per policy)
- [ ] Doc comments on all public functions
- [ ] Tests cover new/changed behavior
- [ ] SSOT alignment verified

## Pull Request Requirements

1. PR description explains **what** and **why** (not just how)
2. Reference the SSOT requirement being addressed
3. All CI checks pass
4. At least one approval from a domain-appropriate reviewer
5. No merge conflicts with target branch
6. Benchmark results included for performance changes

## References

- `AGENTS.md` — Engineering constitution
- `CONTRIBUTING.md` (root) — Project-wide contribution guide
- `docs/PRD2.md §18` — Runtime specification
- `docs/PRD-TESTING& BRACHMARCK.md` — Testing strategy and benchmark targets
