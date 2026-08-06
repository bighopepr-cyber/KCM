# kcm-compliance Security Policy

Security considerations specific to the `kcm-compliance` crate.

> For project-wide security policies, refer to the [SECURITY.md](../../SECURITY.md) located in the repository root.

## Overview

`kcm-compliance` provides GDPR consent management and data classification for the KCM engine. It enforces consent lifecycle operations (grant, withdraw, export, delete) and data classification policies (4 tiers) with encryption and audit requirements. As a compliance-critical crate, failures here can result in regulatory violations and data exposure.

## Security Scope

| Asset | Risk Level | Description |
|-------|-----------|-------------|
| GDPR consent management | High | Consent lifecycle — incorrect enforcement violates GDPR Article 7 |
| Data classification | Medium | Classification tiers — misclassification can expose sensitive data |

## Threat Model

| Threat | Vector | Mitigation |
|--------|--------|------------|
| Consent bypass | Processing data without valid consent | `has_consent()` check before any data operation |
| Data misclassification | Assigning wrong tier to sensitive data | `DataClassification` enum with compile-time tier enforcement |
| PII exposure | Unauthorized access to subject data | `delete_data()` for right-to-erasure, consent-based access |
| Audit trail gaps | Missing logs for compliance events | `requires_audit_log()` for Internal/Confidential/Restricted tiers |
| Consent state manipulation | Granting consent for nonexistent subjects | `NotFound` error on missing subjects |
| Duplicate subject registration | Overwriting existing consent records | `InvalidArgument` error on duplicate subject IDs |

## Security Risks

- **Consent withdrawal enforcement**: Consent withdrawal must be atomic — any race condition could allow data processing after withdrawal
- **Data retention enforcement**: `ClassifiedFact::is_expired()` must correctly calculate retention based on classification tier
- **Encryption validation**: `validate_encryption()` must be called for every Confidential/Restricted fact before storage
- **Export completeness**: `export_data()` must return all subject data without omission
- **Deletion completeness**: `delete_data()` must fully remove all subject records

## Access Control

`kcm-compliance` manages consent-based access control. All data operations must check `has_consent()` before proceeding. Access control enforcement is performed by downstream crates (`kcm-runtime`, `kcm-security`).

## RBAC Integration

`kcm-compliance` provides consent primitives consumed by `kcm-security` for RBAC enforcement:

| Integration Point | Description |
|-------------------|-------------|
| `has_consent(subject_id)` | Check if subject has granted consent before data access |
| `DataClassification::requires_encryption()` | Enforce encryption for Confidential/Restricted data |
| `DataClassification::requires_audit_log()` | Require audit logging for Internal/Confidential/Restricted |

## Sensitive Assets

- **Consent records** (`DataSubject`) — Contains subject IDs, email addresses, and consent status. Subject to GDPR Article 17 (right to erasure).
- **Classified data** (`ClassifiedFact`) — Facts with classification tier. Encryption required for Confidential/Restricted.
- **Subject registry** (`GDPRManager::subjects`) — In-memory HashMap of all registered data subjects. Thread-safe via `Arc<RwLock<...>>`.

## Secret Management

No secrets are stored or managed directly in `kcm-compliance`. The crate does not handle encryption keys, API tokens, or credentials. Encryption enforcement is delegated to `kcm-security`.

## Secure Development Rules

1. **Consent validation**: All data access must verify `has_consent()` returns `true` before proceeding
2. **Classification enforcement**: `validate_encryption()` must be called for every fact classified as Confidential or Restricted
3. **PII protection**: Subject data (email, subject_id) must not be exposed in error messages or logs
4. **Audit trail integrity**: All consent mutations (grant, withdraw, delete) must be traceable
5. **No unwrap in production**: All public APIs return `Result<T, KcmError>` — no `unwrap()` in production code paths
6. **Result return**: All fallible operations must return `Result<T, KcmError>` with appropriate error variants

## Audit Logging

| Event | Classification Required | Description |
|-------|------------------------|-------------|
| Consent granted | Internal+ | Record when consent status changes to Granted |
| Consent withdrawn | Internal+ | Record when consent status changes to Withdrawn |
| Subject registered | Internal+ | Record when a new data subject is registered |
| Data exported | Internal+ | Record right-to-portability requests |
| Data deleted | Internal+ | Record right-to-erasure requests |
| Classification assigned | Confidential+ | Record when data is classified above Internal |

## Validation Checklist

- [ ] All data access checks `has_consent()` before proceeding
- [ ] `validate_encryption()` called for Confidential/Restricted facts
- [ ] No subject PII in error messages or logs
- [ ] Consent mutations are atomic (no partial state)
- [ ] `delete_data()` fully removes all subject records
- [ ] `export_data()` returns complete subject data
- [ ] No `unwrap()` in production code paths
- [ ] No `panic!()` in production code paths
- [ ] All public APIs return `Result<T, KcmError>`
- [ ] `DataClassification` tiers match SSOT specification

## References

- [SECURITY.md](../../SECURITY.md) — Project-wide security policy
- [AGENTS.md](../../AGENTS.md) — Engineering constitution
- [SSOT.md](../../SSOT.md) — Single Source of Truth
- [docs/kcm-compliance/spesifikasi.md](../../docs/kcm-compliance/spesifikasi.md) — Technical specification
- [PRD3.md](../../docs/PRD3.md) §32 — GDPR compliance specification
