# Contributing to kcm-compliance

Contribution guidelines specific to the `kcm-compliance` crate.

> For core engine contribution rules, refer to the repository [CONTRIBUTING.md](../../CONTRIBUTING.md).

## Overview

`kcm-compliance` provides GDPR consent management and data classification for the KCM engine. Changes here affect regulatory compliance and data protection — correctness is paramount.

## Before Contributing

1. Read the [root CONTRIBUTING.md](../../CONTRIBUTING.md)
2. Read the [kcm-compliance technical specification](../../docs/kcm-compliance/spesifikasi.md)
3. Verify your change does not break the public API without an SSOT-approved reason
4. Check [existing issues](https://github.com/bighopepr-cyber/KCM/issues) for related work
5. Understand GDPR Article 7 (conditions for consent) and Article 17 (right to erasure)

## Coding Standards

### Rust Requirements

- Edition 2021
- All public APIs return `Result<T, KcmError>`
- No `unwrap()` in production code
- No `panic!()` in production code
- No `TODO`/`FIXME`/`HACK` markers
- Use `parking_lot` for synchronization (not `std`)

### Compliance Rules

- Consent status transitions must be deterministic: `NotProvided → Granted → Withdrawn`
- Withdrawn consent must be immediately effective (no grace period)
- `delete_data()` must fully remove all subject records (right to erasure)
- `export_data()` must return complete subject data (right to portability)
- Data classification must match the 4-tier model exactly

### Naming Conventions

| Element | Convention | Example |
|---------|-----------|---------|
| Types | PascalCase | `GDPRManager`, `DataSubject` |
| Functions | snake_case | `grant_consent`, `withdraw_consent` |
| Constants | SCREAMING_SNAKE_CASE | `DEFAULT_RETENTION_DAYS` |
| Modules | snake_case | `gdpr`, `data_classification` |

## Module Architecture Rules

- `kcm-compliance` depends on `kcm-core` and `parking_lot` only
- No I/O, networking, or file system operations
- No async code — consent and classification are synchronous operations
- All modules must be declared in `lib.rs`
- No circular dependencies with other KCM crates

## Documentation Rules

- Every public function must have a `///` doc comment
- Every public type must have a `///` doc comment
- Doc comments must include at least one code example for public APIs
- Module-level documentation must explain the module's purpose
- GDPR-related changes must reference the relevant GDPR article

## Testing Requirements

### GDPR Consent Tests

- Test consent lifecycle: register → grant → withdraw
- Test right to erasure: `delete_data()` removes all records
- Test right to portability: `export_data()` returns complete data
- Test error cases: nonexistent subjects, duplicate registration
- Test concurrent access via `Arc<RwLock<...>>`

### Classification Tests

- Test all 4 tiers: Public, Internal, Confidential, Restricted
- Test `requires_encryption()` returns correct values per tier
- Test `requires_audit_log()` returns correct values per tier
- Test `max_retention_days()` returns correct values per tier
- Test `validate_encryption()` rejects unencrypted Confidential/Restricted facts
- Test `ClassifiedFact::should_retain()` and `is_expired()` with time math
- Run: `cargo test -p kcm-compliance`

## Performance Rules

- Consent checks must be O(1) via HashMap lookup
- Classification operations must be O(1) via match arms
- No unnecessary allocations in consent check paths
- Benchmark regressions >5% require justification

## Review Checklist

- [ ] All public APIs return `Result<T, KcmError>`
- [ ] No `unwrap()` in production code
- [ ] No `panic!()` in production code
- [ ] Consent state transitions are correct
- [ ] Classification tiers match SSOT specification
- [ ] All tests pass
- [ ] No clippy warnings
- [ ] SSOT traceability documented

## Pull Request Requirements

- Reference the SSOT requirement being addressed
- Include test coverage for new/changed APIs
- Include benchmarks if performance-sensitive
- Do not break backward compatibility without SSOT approval
- Document GDPR article references for compliance changes

## References

- [CONTRIBUTING.md](../../CONTRIBUTING.md) — Repository-wide contribution guidelines
- [CODE_OF_CONDUCT.md](../../CODE_OF_CONDUCT.md) — Community guidelines
- [docs/kcm-compliance/spesifikasi.md](../../docs/kcm-compliance/spesifikasi.md) — Technical specification
- [AGENTS.md](../../AGENTS.md) — Engineering constitution
- [PRD3.md](../../docs/PRD3.md) §32 — GDPR compliance specification
