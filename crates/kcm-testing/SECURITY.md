# kcm-testing Security Policy

> **Note:** This document is crate-specific. The project-wide security policy is in the repository root at [`SECURITY.md`](../../SECURITY.md).

## Overview

kcm-testing is the testing infrastructure crate for KCM. It provides load tests, stress tests, security tests, chaos engineering, regression detection, metrics dashboard, and benchmark fixtures. Because it exercises security-sensitive components (RBAC, injection prevention, timing attacks, memory safety), the testing crate itself must follow strict security discipline to avoid introducing attack surfaces or leaking sensitive information.

## Security Scope

| Component | Risk Level | Description |
|-----------|------------|-------------|
| SecurityTests | High | Validates attack surfaces: injection prevention, buffer overflow, RBAC enforcement, timing attacks, memory safety |
| LoadTests | Medium | Spawns concurrent threads that exercise the database under realistic load |
| ChaosTests | Medium | Injects faults via CSPRNG-driven failure injection |
| StressTests | Medium | Pushes the system beyond normal operating limits to validate graceful degradation |
| RegressionDetector | Low | Compares benchmark metrics against baselines |
| MetricsDashboard | Low | Collects and reports test metrics |
| BenchFixtures | Low | Generates deterministic, non-sensitive test data |

## Threat Model

### Test Data Leakage
- Test fixtures contain synthetic data generated deterministically from `DatasetConfig`. No production data is used.
- Temporary directories created by `WalBenchmarkFixture` and `FileFormatFixture` are auto-cleaned via `tempfile::TempDir`.
- Benchmark results (QPS, latency) are operational metrics, not sensitive data.

### Resource Exhaustion During Tests
- Load and stress tests spawn concurrent threads. Configurations must include bounded iteration counts and duration limits.
- Stress tests must verify graceful degradation (failure rate < 10%) rather than requiring zero failures.
- Chaos tests use `AtomicBool` activation/deactivation to prevent unbounded fault injection.

### Test Environment Escape
- All tests operate on in-memory databases or temporary directories. No production file paths are touched.
- `getrandom` is used only for chaos fault injection probability, not for test data generation.
- Test modules are gated with `#[cfg(test)]` to prevent inclusion in production builds.

## Security Risks

| Risk | Mitigation |
|------|------------|
| Malicious test inputs affecting production | All test data is synthetic; no user input flows into production paths |
| Concurrent test threads causing race conditions | `Arc` + `AtomicU64` for shared counters; `parking_lot::Mutex` for shared collections |
| Temporary file residue | `TempDir` auto-cleanup on drop; `WalBenchmarkFixture` owns its `TempDir` |
| Timing side-channels in tests | Timing attack tests verify bounded ratios, not exact times |
| Uncontrolled chaos injection | `ChaosMonkey` requires explicit `activate()` call; `is_active()` checked before every injection |

## Access Control

- `security_tests` module is `#[cfg(test)]` — only compiled during test builds.
- RBAC tests in `security_tests.rs` exercise `ACLManager` with isolated user/role configurations.
- No test function is `pub` — all are `#[test]` private functions.
- Chaos activation requires explicit API call; no automatic activation.

## RBAC Integration

Security tests validate KCM's RBAC system through:
- `test_rbac_enforcement` — verifies role-based permission checking
- `test_rbac_admin_role` — verifies admin role has all permissions
- `test_context_isolation` — verifies cross-context permission isolation

These tests use `ACLManager::new()` which creates an isolated ACL instance, not connected to any persistent storage.

## Sensitive Assets

| Asset | Sensitivity | Handling |
|-------|-------------|----------|
| Test data | Non-sensitive | Synthetic, deterministic, generated from `DatasetConfig` |
| Benchmark results | Operational | QPS, latency, pass rates — not user data |
| Chaos injection parameters | Low | Configured per-test, no persistence |
| Temp WAL files | Transient | Created in temp directories, auto-cleaned on drop |

## Secret Management

- No secrets, keys, or credentials are used in kcm-testing.
- `getrandom` is used only for chaos fault probability (CSPRNG), not for cryptographic operations.
- No hardcoded paths, API keys, or connection strings.

## Secure Development Rules

1. **Test Isolation:** Every test creates its own `KnowledgeDatabase` or `Schema` instance. Tests never share mutable state.
2. **Temp File Cleanup:** All temporary files use `tempfile::TempDir` which auto-cleans on drop. No manual cleanup code.
3. **Resource Limits:** Load/stress tests use bounded iteration counts and thread counts. Chaos tests require explicit activation.
4. **No Production Data:** All test data is generated deterministically from `DatasetConfig`. No production databases, files, or networks are accessed.
5. **No unwrap in Production Paths:** Public APIs (`run_load_test`, `run_stress_test`, `RegressionDetector::detect`) return `Result<T, KcmError>`. The crate-level `#[allow(clippy::unwrap_used, clippy::panic)]` is for test code only.
6. **Result Return:** All public functions return `Result` types. Panics are restricted to `#[cfg(test)]` blocks and fixture constructors with descriptive messages.

## Audit Logging

Security tests do not produce audit log entries. The tests validate that the `kcm-security` crate's audit logging works correctly, but kcm-testing itself does not emit audit events.

## Validation Checklist

Before any change to kcm-testing:

- [ ] No production data paths referenced
- [ ] Temporary files use `TempDir`
- [ ] Concurrent tests use `Arc` + atomics or `Mutex`
- [ ] Chaos injection requires explicit activation
- [ ] All public functions return `Result<T, KcmError>`
- [ ] `#[cfg(test)]` gate present on security-sensitive test modules
- [ ] No hardcoded secrets, keys, or credentials
- [ ] Stress test failure rate threshold is validated (< 10%)
- [ ] Benchmark fixtures are deterministic (no randomness in data generation)
- [ ] No unwrap in non-test public code paths

## References

- [Root SECURITY.md](../../SECURITY.md)
- [AGENTS.md — Concurrency Model](../../AGENTS.md)
- [PRD3.md §30 — Security](../PRD3.md)
- [PRD-TESTING — Testing Strategy](../PRD-TESTING%26%20BRACHMARCK.md)
