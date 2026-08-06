# kcm-core Security Policy

Security considerations specific to the `kcm-core` crate.

> For project-wide security policies, refer to the [SECURITY.md](../../SECURITY.md) located in the repository root.

## Overview

`kcm-core` defines the foundational data types (`Fact`, `RowID`, `Bitmap`, `Dictionary`, `DenseVec`) used by all other KCM crates. As the lowest-level crate in the dependency graph, any security vulnerability here propagates to every downstream component.

## Security Scope

| Asset | Risk Level | Description |
|-------|-----------|-------------|
| `Fact` struct | High | Core data unit — incorrect validation compromises all storage and queries |
| `KcmError` enum | Medium | Error handling correctness affects error-path security |
| `Dictionary` | Medium | String interning — incorrect bounds can cause out-of-bounds access |
| `Bitmap` | Medium | Bit manipulation — incorrect operations can cause memory corruption |
| `DenseVec<T>` | Medium | Growable vector — incorrect capacity management can cause OOB |

## Threat Model

| Threat | Vector | Mitigation |
|--------|--------|------------|
| Integer overflow in `Confidence` | Malformed input | Validate range [0.0, 1.0] at construction |
| Out-of-bounds access in `DenseVec` | Index from untrusted source | Bounds check on every `get`/`set` |
| Dictionary ID overflow | Exceeding `u32::MAX` entries | Return `KcmError::OutOfMemory` |
| Bitmap corruption | Incorrect bit operations | All operations validated with assertions |
| Memory exhaustion | Unbounded growth | Capacity limits enforced by callers |

## Access Control

`kcm-core` has no access control mechanisms. All types are public within the crate. Access control is enforced by downstream crates (`kcm-security`, `kcm-runtime`).

## RBAC Integration

Not applicable — `kcm-core` is a foundational library with no authentication or authorization logic.

## Sensitive Assets

- `Fact` — Contains knowledge triples with confidence scores. In production, facts may represent sensitive domain knowledge.
- `Dictionary` — Maps human-readable strings to integer IDs. May contain sensitive terminology.

## Secret Management

No secrets are stored or managed in `kcm-core`. The crate has no I/O, networking, or file system access.

## Secure Development Rules

1. All `Confidence` values must be validated as `>= 0.0` and `<= 1.0`
2. `DenseVec` must never expose uninitialized memory
3. `Bitmap` operations must validate bit positions against capacity
4. `Dictionary` must handle `u32::MAX` overflow gracefully
5. No `unsafe` code without documented `// SAFETY:` justification
6. All public APIs must return `Result<T, KcmError>`

## Audit Logging

Not applicable — `kcm-core` performs no operations that require audit logging.

## Validation Checklist

- [ ] All `Confidence` values are within [0.0, 1.0]
- [ ] All index operations are bounds-checked
- [ ] No `unwrap()` in production code paths
- [ ] No `panic!()` in production code paths
- [ ] All `unsafe` blocks have `// SAFETY:` comments
- [ ] `KcmError` variants cover all failure modes
- [ ] `Dictionary` handles capacity overflow
- [ ] `Bitmap` handles edge cases (empty, full, single-bit)

## References

- [SECURITY.md](../../SECURITY.md) — Project-wide security policy
- [AGENTS.md](../../AGENTS.md) — Engineering constitution
- [SSOT.md](../../SSOT.md) — Single Source of Truth
- [docs/kcm-core/spesifikasi.md](../../docs/kcm-core/spesifikasi.md) — Technical specification
