# KCM Engineering Rules

**Document ID:** KCM-ENG-001  
**Version:** 2.0.0  
**Status:** Active  
**Owner:** Code Quality Guardian (P10)  
**Authoritative Source:** AGENTS.md §Non-Negotiable Rules

---

## 1. Core Rules (Non-Negotiable)

| # | Rule | Enforcement |
|---|------|-------------|
| 1 | All public APIs return `Result<T, KcmError>` | Compiler |
| 2 | No `unwrap()` in production code | CI gate |
| 3 | No `panic!()` in production code | CI gate |
| 4 | No TODO/FIXME/HACK in production code | CI gate |
| 5 | No placeholder implementations | Code review |
| 6 | No fake success responses | Code review |
| 7 | All tests pass before commit | CI gate |
| 8 | All clippy warnings resolved | CI gate |
| 9 | Every requirement maps to an implementation | SSOT traceability |
| 10 | Every implementation maps to a test | Test coverage |
| 11 | Every benchmark validates a documented requirement | Benchmark suite |
| 12 | No documentation describes non-existent behavior | SSOT validation |

## 2. Rust Conventions

- Use `parking_lot` for mutexes/rwlocks (not std)
- Use `Send + Sync` bounds on all shared types
- All `unsafe` blocks must have `// SAFETY:` comment
- All public types implement `Debug`
- Use `is_some_and` over `map_or(false, ...)`
- Use `div_ceil` instead of manual ceiling division
- Use `clamp` instead of chained `min/max`
- Use `or_default()` instead of `or_insert_with(Vec::new)`

## 3. Architecture Rules

- No circular crate dependencies
- kcm-core has zero internal dependencies
- All inter-crate communication through public API only
- New modules must have corresponding tests
- Feature-gated dependencies must have `#[cfg(feature)]`

## 4. Testing Rules

| Rule | Description |
|------|-------------|
| TR-001 | Every PR passes `cargo test --workspace` |
| TR-002 | Every PR passes `cargo clippy --workspace` |
| TR-003 | Every PR passes `cargo fmt --check` |
| TR-004 | New code has ≥ 95% test coverage |
| TR-005 | Security code has security tests |
| TR-006 | Performance code has benchmarks |
| TR-007 | Property tests for arithmetic operations |

## 5. Performance Rules

- Benchmark regression > 5% triggers WARNING
- Benchmark regression > 10% triggers FAILURE
- Memory usage < 100 bytes/fact
- Compression ratio > 5x
- Query latency P99 < 100ms (1M facts)

## 6. Security Rules

- No hardcoded secrets or keys
- Encryption uses AEAD (AES-256-GCM)
- Key generation uses CSPRNG
- All user input validated
- Audit logging for all write operations

## 7. Process Rules

- All changes via pull request
- CI must pass before merge
- Breaking changes require ADR
- Version bump on every release
