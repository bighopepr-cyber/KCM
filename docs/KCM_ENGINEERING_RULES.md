# KCM Engineering Rules

**Document ID:** KCM-ENG-001  
**Version:** 1.0.0  
**Status:** Derived  
**Authoritative Source:** AGENTS.md §Non-Negotiable Rules

> **Authority Notice:** The 12 Non-Negotiable Rules in AGENTS.md are the authoritative engineering rules for KCM. This document provides the detailed operational rules that derive from those principles. Where conflicts exist, AGENTS.md §Non-Negotiable Rules wins.

---

## 1. Purpose

Defines mandatory engineering practices for KCM development. Derived from AGENTS.md §Non-Negotiable Rules and §Engineering Gates.

---

## 2. Code Rules

> Core rules: AGENTS.md §Non-Negotiable Rules #1–12.

### 2.1 Rust Conventions

| Rule | Description | AGENTS.md Reference |
|------|-------------|---------------------|
| ER-001 | All public functions return `Result<T, KcmError>` | §Non-Negotiable Rules #1 |
| ER-002 | No `unwrap()` in production code paths | §Non-Negotiable Rules #2 |
| ER-003 | All `unsafe` blocks must have a `// SAFETY:` comment explaining correctness | — |
| ER-004 | All public types must implement `Debug` | — |
| ER-005 | Use `parking_lot` for mutexes/rwlocks (not std) | §Concurrency Model |
| ER-006 | Use `Send + Sync` bounds on all shared types | §Concurrency Model |
| ER-007 | Prefer `is_some_and` over `map_or(false, ...)` for Option checks | — |
| ER-008 | Use `div_ceil` instead of manual ceiling division | — |
| ER-009 | Use `clamp` instead of chained `min/max` | — |
| ER-010 | Use `or_default()` instead of `or_insert_with(Vec::new)` | — |

### 2.2 Architecture Rules

| Rule | Description | AGENTS.md Reference |
|------|-------------|---------------------|
| ER-011 | No circular crate dependencies | §Dependency Policy |
| ER-012 | kcm-core has zero internal dependencies | §Crate Map |
| ER-013 | All inter-crate communication through public API only | §Engineering Philosophy |
| ER-014 | New modules must have corresponding tests | §Non-Negotiable Rules #10 |
| ER-015 | Feature-gated dependencies (e.g., serde, pyo3) must have `#[cfg(feature)]` | — |

### 2.3 API Rules

| Rule | Description | AGENTS.md Reference |
|------|-------------|---------------------|
| ER-016 | No breaking API changes without version bump | — |
| ER-017 | New API functions must have doc comments | §Non-Negotiable Rules #12 |
| ER-018 | C FFI functions must validate null pointers | — |
| ER-019 | Builder pattern methods consume self | — |
| ER-020 | Error messages must be descriptive and actionable | §Error Model |

---

## 3. Testing Rules

> Test pyramid: AGENTS.md §Testing Strategy. Quality gates: AGENTS.md §Engineering Gates (Gate 6).

| Rule | Description | AGENTS.md Reference |
|------|-------------|---------------------|
| TR-001 | Every PR must pass `cargo test --workspace` | §Engineering Gates Gate 6 |
| TR-002 | Every PR must pass `cargo clippy --workspace` | §Engineering Gates Gate 6 |
| TR-003 | Every PR must pass `cargo fmt --check` | §Engineering Gates Gate 6 |
| TR-004 | New code must have ≥ 95% test coverage | §Testing Strategy |
| TR-005 | Security-sensitive code must have security tests | §Testing Strategy (Security tier) |
| TR-006 | Performance-critical code must have benchmarks | §Testing Strategy |
| TR-007 | Property tests required for arithmetic operations | §Testing Strategy (Property tier) |
| TR-008 | Load tests run before releases | §Testing Strategy (Load tier) |

---

## 4. Performance Rules

> Performance targets: PRD.md §8. Benchmark validation: PRD-TESTING&BRACHMARCK.md §4.

| Rule | Description | Authority |
|------|-------------|-----------|
| PR-001 | Benchmark regression > 5% triggers WARNING, > 10% triggers FAILURE | PRD-TESTING&BRACHMARCK.md §4 |
| PR-002 | Memory usage must not exceed 100 bytes/fact | PRD.md §8 |
| PR-003 | Compression ratio must exceed 5x | PRD2.md §2.2 |
| PR-004 | New operators must report estimated_rows | PRD.md §5.1 |

---

## 5. Documentation Rules

> Documentation authority: AGENTS.md §Document Hierarchy. Documentation guardian: AGENTS.md §Skill Governance (P11).

| Rule | Description | AGENTS.md Reference |
|------|-------------|---------------------|
| DR-001 | Architecture changes must update KCM_ARCHITECTURE.md | §Document Hierarchy |
| DR-002 | API changes must update KCM_API_SPEC.md | §Document Hierarchy |
| DR-003 | New compression must update KCM_COMPRESSION_SPEC.md | §Document Hierarchy |
| DR-004 | New tests must update KCM_TESTING_SPEC.md | §Document Hierarchy |
| DR-005 | All changes must update this document's change history | — |

---

## 6. Security Rules

> Security requirements: PRD3.md §4 (Security). Non-negotiable: AGENTS.md §Non-Negotiable Rules.

| Rule | Description | Authority |
|------|-------------|-----------|
| SR-001 | No hardcoded secrets or keys | AGENTS.md §Non-Negotiable Rules #6 |
| SR-002 | Encryption must use AEAD (AES-256-GCM) | PRD3.md §4 |
| SR-003 | Key generation must use CSPRNG | PRD3.md §4 |
| SR-004 | All user input must be validated | AGENTS.md §Non-Negotiable Rules #9 |
| SR-005 | Audit logging for all write operations | PRD3.md §4 |

---

## 7. Process Rules

> Engineering gates: AGENTS.md §Engineering Gates.

| Rule | Description | AGENTS.md Reference |
|------|-------------|---------------------|
| PS-001 | All changes via pull request | §Engineering Gates |
| PS-002 | Minimum 1 approval for merge | §Engineering Gates |
| PS-003 | CI must pass before merge | §Engineering Gates Gate 6 |
| PS-004 | Breaking changes require RFC | §Specification Lock (P4) |
| PS-005 | Version bump on every release | — |

---

## 8. Change History

| Date | Change | Author |
|------|--------|--------|
| 2026-07-31 | Initial version | KCM Engineering |

---

## 9. References

- **Authoritative sources:** AGENTS.md §Non-Negotiable Rules (12 rules), AGENTS.md §Engineering Gates (6 gates), AGENTS.md §Testing Strategy (4-tier pyramid), AGENTS.md §Dependency Policy
- **Depends on:** AGENTS.md (Engineering Constitution), PRD.md (P4), PRD2.md (P3), PRD3.md (P2), PRD-TESTING&BRACHMARCK.md (P1)
- **Parent specs:** KCM_SPECIFICATION
- **Related:** KCM_BENCHMARK_REPORTING_SPEC, KCM_PERFORMANCE_SPEC
