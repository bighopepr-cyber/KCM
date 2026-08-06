# Contributing to kcm-distributed

> This document covers contribution guidelines specific to the `kcm-distributed` crate. For project-wide contribution guidelines, see the root `CONTRIBUTING.md`.

---

## Overview

The `kcm-distributed` crate implements distributed coordination, sharding, replication, and transport for KCM. Contributions to this crate must maintain strict correctness guarantees, as errors in distributed systems code can cause data loss, split-brain scenarios, or cluster-wide failures. All changes are subject to the engineering gates defined in `AGENTS.md`.

## Before Contributing

1. Read `AGENTS.md` — particularly the Engineering Gates and Non-Negotiable Rules.
2. Read `docs/PRD3.md` §27 — the authoritative specification for distributed architecture.
3. Read the source files in `crates/kcm-distributed/src/` to understand the current implementation.
4. Verify that your proposed change has a corresponding SSOT requirement. If not, the SSOT must be updated first per SSOT-07.
5. Run `cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace -- -D warnings`, and `cargo fmt --all -- --check` before submitting.

## Coding Standards

- All public APIs return `Result<T, KcmError>`.
- No `unwrap()` in production code paths.
- No `panic!()` in production code.
- No `todo!()`, `unimplemented!()`, `FIXME`, `TODO`, or `HACK` markers.
- No placeholder implementations or stubs.
- Follow existing code style and conventions within the crate.
- All error variants must map to the `KcmError` hierarchy.
- Thread-safe types must be `Send + Sync`.

## Module Architecture Rules

| Rule | Description |
|------|-------------|
| Dependencies | `kcm-distributed` may only depend on `kcm-core` and `kcm-security` (plus parking_lot, rayon) |
| No upward dependencies | `kcm-distributed` must NOT depend on `kcm-runtime`, `kcm-compute`, or any higher-level crate |
| Single responsibility | Each source file has exactly one responsibility |
| No cross-module coupling | Sharding, coordination, replication, and transport are independent modules |

## Documentation Rules

- All public types, functions, and modules must have doc comments.
- Doc comments must accurately describe current behavior — not intended behavior.
- Internal implementation details must be documented where non-obvious.
- Security-relevant code must include comments explaining the security rationale.
- No documentation describing behavior that doesn't exist (per SSOT rule 12).

## Testing Requirements

### Distributed Coordination Tests

- 2PC prepare/commit/abort flow correctness
- 2PC coordinator failure recovery
- Transaction timeout handling
- Concurrent transaction isolation
- Node failure during commit phase
- Split-brain detection and prevention

### Transport Tests

- Message serialization/deserialization correctness
- Connection failure and reconnection
- Message ordering guarantees
- Large message handling
- TLS handshake and certificate validation
- Message authentication verification

### Sharding Tests

- Hash sharding distribution uniformity
- Range sharding boundary correctness
- Consistent hash ring rebalancing
- Shard migration during node join/leave
- Shard map serialization/deserialization
- Edge cases: single node, empty cluster, maximum nodes

### Replication Tests

- Sync replication round-trip correctness
- Async replication eventual consistency
- Replication lag measurement
- Conflict resolution correctness
- Replication stream recovery after failure
- Replica promotion after primary failure

## Performance Rules

- Sharding decisions must execute in O(1) for hash and consistent hash strategies.
- Shard map lookups must not allocate on the hot path.
- Transport message serialization should minimize allocations.
- Replication should not block primary transaction processing.
- Benchmark any change that affects the hot path.

## Review Checklist

- [ ] All tests pass: `cargo test --workspace`
- [ ] No clippy warnings: `cargo clippy --workspace -- -D warnings`
- [ ] Format correct: `cargo fmt --all -- --check`
- [ ] SSOT validation passes: `bash scripts/validate-ssot.sh`
- [ ] No `unwrap()` in production code
- [ ] No `todo!()` or `unimplemented!()` markers
- [ ] Public APIs return `Result<T, KcmError>`
- [ ] Error handling is complete for all code paths
- [ ] Tests cover the change adequately
- [ ] Documentation matches implementation
- [ ] No dependency boundary violations (kcm-core, kcm-security only)
- [ ] Thread safety verified for shared types

## Pull Request Requirements

1. PR title and description reference the SSOT requirement being addressed.
2. All CI checks pass (format, clippy, build, tests, SSOT validation).
3. PR includes tests for new functionality or bug fixes.
4. PR includes documentation updates if public API changed.
5. PR does not introduce new external dependencies without justification.
6. PR is reviewed by at least one domain expert (kcm-database-engine-specialist or kcm-security-engineer as appropriate).
7. PR does not break backward compatibility without explicit approval.

## References

- `AGENTS.md` — Engineering gates and non-negotiable rules
- `docs/PRD3.md` §27 — Distributed architecture specification
- Root `CONTRIBUTING.md` — Project-wide contribution guidelines
- `crates/kcm-distributed/src/` — Source implementation
- `docs/PRD-TESTING& BRACHMARCK.md` — Testing strategy and quality gates
