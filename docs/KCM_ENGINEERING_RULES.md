# KCM Engineering Rules

**Document ID:** KCM-ENG-001  
**Version:** 1.0.0

---

## 1. Purpose

Defines mandatory engineering practices for KCM development.

---

## 2. Code Rules

### 2.1 Rust Conventions

| Rule | Description |
|------|-------------|
| ER-001 | All public functions return `Result<T, KcmError>` |
| ER-002 | No `unwrap()` in production code paths |
| ER-003 | All `unsafe` blocks must have a `// SAFETY:` comment explaining correctness |
| ER-004 | All public types must implement `Debug` |
| ER-005 | Use `parking_lot` for mutexes/rwlocks (not std) |
| ER-006 | Use `Send + Sync` bounds on all shared types |
| ER-007 | Prefer `is_some_and` over `map_or(false, ...)` for Option checks |
| ER-008 | Use `div_ceil` instead of manual ceiling division |
| ER-009 | Use `clamp` instead of chained `min/max` |
| ER-010 | Use `or_default()` instead of `or_insert_with(Vec::new)` |

### 2.2 Architecture Rules

| Rule | Description |
|------|-------------|
| ER-011 | No circular crate dependencies |
| ER-012 | kcm-core has zero internal dependencies |
| ER-013 | All inter-crate communication through public API only |
| ER-014 | New modules must have corresponding tests |
| ER-015 | Feature-gated dependencies (e.g., serde, pyo3) must have `#[cfg(feature)]` |

### 2.3 API Rules

| Rule | Description |
|------|-------------|
| ER-016 | No breaking API changes without version bump |
| ER-017 | New API functions must have doc comments |
| ER-018 | C FFI functions must validate null pointers |
| ER-019 | Builder pattern methods consume self |
| ER-020 | Error messages must be descriptive and actionable |

---

## 3. Testing Rules

| Rule | Description |
|------|-------------|
| TR-001 | Every PR must pass `cargo test --workspace` |
| TR-002 | Every PR must pass `cargo clippy --workspace` |
| TR-003 | Every PR must pass `cargo fmt --check` |
| TR-004 | New code must have ≥ 95% test coverage |
| TR-005 | Security-sensitive code must have security tests |
| TR-006 | Performance-critical code must have benchmarks |
| TR-007 | Property tests required for arithmetic operations |
| TR-008 | Load tests run before releases |

---

## 4. Performance Rules

| Rule | Description |
|------|-------------|
| PR-001 | Benchmark regression > 5% triggers WARNING, > 10% triggers FAILURE |
| PR-002 | Memory usage must not exceed 100 bytes/fact |
| PR-003 | Compression ratio must exceed 5x |
| PR-004 | New operators must report estimated_rows |

---

## 5. Documentation Rules

| Rule | Description |
|------|-------------|
| DR-001 | Architecture changes must update KCM_ARCHITECTURE.md |
| DR-002 | API changes must update KCM_API_SPEC.md |
| DR-003 | New compression must update KCM_COMPRESSION_SPEC.md |
| DR-004 | New tests must update KCM_TESTING_SPEC.md |
| DR-005 | All changes must update this document's change history |

---

## 6. Security Rules

| Rule | Description |
|------|-------------|
| SR-001 | No hardcoded secrets or keys |
| SR-002 | Encryption must use AEAD (AES-256-GCM) |
| SR-003 | Key generation must use CSPRNG |
| SR-004 | All user input must be validated |
| SR-005 | Audit logging for all write operations |

---

## 7. Process Rules

| Rule | Description |
|------|-------------|
| PS-001 | All changes via pull request |
| PS-002 | Minimum 1 approval for merge |
| PS-003 | CI must pass before merge |
| PS-004 | Breaking changes require RFC |
| PS-005 | Version bump on every release |

---

## 8. Change History

| Date | Change | Author |
|------|--------|--------|
| 2026-07-31 | Initial version | KCM Engineering |

---

## 9. References

- **Depends on:** KCM_SPECIFICATION (KCM_SPECIFICATION)
- **Parent specs:** KCM_SPECIFICATION (KCM_SPECIFICATION)
- **Related:** KCM_BENCHMARK_REPORTING_SPEC (KCM_BENCHMARK_REPORTING_SPEC), KCM_PERFORMANCE_SPEC (KCM_PERFORMANCE_SPEC)
