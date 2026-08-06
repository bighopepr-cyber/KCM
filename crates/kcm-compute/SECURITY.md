# kcm-compute Security Policy

Security considerations specific to the `kcm-compute` crate.

> For project-wide security policies, refer to the [SECURITY.md](../../SECURITY.md) located in the repository root.

## Overview

`kcm-compute` is the compute engine of the KCM (Knowledge Columnar Model) system. It implements relational algebra operators (Scan, Filter, Project, Join, Aggregate) using Volcano-style pull-based execution and provides SIMD AVX2 acceleration for column operations. As the execution layer, any correctness or safety failure here directly compromises query results and inference outcomes.

## Security Scope

| Asset | Risk Level | Description |
|-------|-----------|-------------|
| `algebra.rs` operators | High | Query execution — incorrect logic produces wrong results |
| `simd.rs` AVX2 intrinsics | High | Unsafe SIMD — buffer overruns or undefined behavior |
| Aggregation functions | Medium | Integer overflow or division by zero in Sum/Avg/Count |
| Hash join implementation | Medium | Unbounded memory growth from adversarial join patterns |
| Filter predicates | Medium | Incorrect filtering silently drops or includes wrong rows |

## Threat Model

| Threat | Vector | Mitigation |
|--------|--------|------------|
| SIMD buffer overrun | Untrusted data length | Process 32/8-element chunks with scalar remainder fallback |
| Integer overflow in aggregation | Large dataset Sum | Use f64 accumulation (no integer overflow possible) |
| Division by zero in Avg | Empty input set | Early return `Ok(0.0)` when values are empty |
| Unbounded hash join | Adversarial data distribution | Schema-level row count limits enforced by callers |
| Undefined behavior from unsafe | SIMD feature detection failure | `is_x86_feature_detected!` guard before every unsafe call |
| Filter bypass via NaN | NaN comparisons always false | NaN values excluded from valid Confidence range upstream |

## Security Risks

- **SIMD safety**: All AVX2 functions are marked `unsafe` and gated behind runtime feature detection. A fallback scalar path exists for every operation.
- **Integer overflow**: Aggregation operates on `f64` values. The `Count` variant casts `usize` to `f64`, which is lossless up to 2^53 rows — a practical impossibility.
- **Division by zero**: `AggregateOp::execute_aggregate()` returns `Ok(0.0)` for empty input. `Avg` divides by `values.len()` only after the empty check.
- **Memory safety**: SIMD functions use `chunks_exact` with fixed chunk sizes (32 for u8, 8 for u32). Remainder elements are processed scalar. No out-of-bounds access is possible.

## Access Control

`kcm-compute` has no access control mechanisms. All types are public within the crate. Access control is enforced by downstream crates (`kcm-runtime`, `kcm-interface`).

## RBAC Integration

Not applicable — `kcm-compute` is a computation library with no authentication or authorization logic. Query-level permission checks are enforced by `kcm-runtime` before operators are constructed.

## Sensitive Assets

- **Algebra operators** — Query execution results may expose sensitive knowledge triples. The compute engine itself does not enforce data classification; that is handled by `kcm-compliance`.
- **SIMD column data** — Column values processed by SIMD operations may contain sensitive domain knowledge. No data leaves the compute engine.

## Secret Management

No secrets are stored or managed in `kcm-compute`. The crate has no I/O, networking, or file system access.

## Secure Development Rules

1. All SIMD functions must be gated behind `is_x86_feature_detected!("avx2")` runtime checks
2. All SIMD functions must have `// SAFETY:` comments documenting the preconditions
3. All chunk processing must use `chunks_exact` with scalar remainder fallback
4. Aggregation Sum must use `f64` accumulation — no integer types for sum
5. Aggregation Avg must validate non-empty input before division
6. All public APIs must return `Result<T, KcmError>`
7. No `unwrap()` in production code paths
8. No `panic!()` in production code paths
9. No `unsafe` code in public API surface (only in SIMD implementation details)
10. Hash join must not allocate unbounded memory without caller-enforced limits

## Audit Logging

Not applicable — `kcm-compute` performs no operations that require audit logging. Query execution logging is handled by `kcm-runtime`.

## Validation Checklist

- [ ] All SIMD functions are gated behind `is_x86_feature_detected!`
- [ ] All SIMD functions have `// SAFETY:` comments
- [ ] SIMD chunk processing uses `chunks_exact` with scalar fallback
- [ ] Aggregation handles empty input without panic
- [ ] Avg operation validates non-empty input before division
- [ ] Sum uses `f64` accumulation (no integer overflow)
- [ ] No `unwrap()` in production code paths
- [ ] No `panic!()` in production code paths
- [ ] All public APIs return `Result<T, KcmError>`
- [ ] Hash join input is bounded by caller

## References

- [SECURITY.md](../../SECURITY.md) — Project-wide security policy
- [AGENTS.md](../../AGENTS.md) — Engineering constitution
- [SSOT.md](../../SSOT.md) — Single Source of Truth
- [docs/kcm-compute/spesifikasi.md](../../docs/kcm-compute/spesifikasi.md) — Technical specification
