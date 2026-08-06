# kcm-optimizer Security Policy

> For project-wide security policies, refer to the [SECURITY.md](../../SECURITY.md) located in the repository root.

---

## Overview

This document defines the security policy specific to the `kcm-optimizer` crate. The optimizer is responsible for cost-based query optimization, query planning, statistics collection, plan rewriting, and adaptive execution within the KCM knowledge engine. Because the optimizer controls which execution paths are chosen and how resources are allocated, vulnerabilities here can lead to denial of service, information leakage, or data corruption.

## Security Scope

The `kcm-optimizer` crate is in scope for security review when changes affect:

- Cost model calculations or operator cost estimation
- Query plan generation or plan node construction
- Statistics collection, storage, or retrieval
- Plan rewriting rules (filter pushdown, column pruning, join reordering, index selection)
- Adaptive execution logic (runtime plan adjustment)
- Any public API exposed to `kcm-runtime`, `kcm-interface`, or external callers

Out-of-scope items are governed by the project-wide [SECURITY.md](../../SECURITY.md).

## Threat Model

### Cost Model Manipulation

An adversary may craft queries or manipulate statistics to force the optimizer into selecting suboptimal plans, causing excessive resource consumption or degraded performance. The cost model must be resilient to adversarial input.

| Threat | Impact | Mitigation |
|--------|--------|------------|
| Fabricated statistics inflating costs | Suboptimal plan selection | Bounds validation on all statistics inputs |
| Negative or zero cost estimates | Undefined planner behavior | Enforced minimum cost bounds in `CostModel` |
| Overflow in cost accumulation | Incorrect comparisons | Saturated arithmetic for cost accumulation |

### Plan Cache Poisoning

If query plans are cached, an attacker may inject malformed plan nodes that corrupt the cache or cause incorrect execution.

| Threat | Impact | Mitigation |
|--------|--------|------------|
| Malformed plan node in cache | Execution failure or incorrect results | Plan verification before cache insertion |
| Cache eviction triggering re-optimization storm | Denial of service | Rate limiting on re-optimization triggers |
| Serialized plan deserialization exploit | Remote code execution | Strict deserialization validation |

### Statistics Corruption

Statistics drive optimizer decisions. Corrupted statistics can lead to catastrophic plan selection.

| Threat | Impact | Mitigation |
|--------|--------|------------|
| Integer overflow in cardinality estimates | Incorrect join ordering | Bounded cardinality values |
| Timestamp manipulation | Stale statistics used | Freshness validation on statistics |
| Null or missing statistics | Planner panic or fallback | Default statistics fallback with warning |

## Security Risks

| Risk | Severity | Likelihood | Mitigation |
|------|----------|------------|------------|
| Optimizer accepts unbounded input | High | Medium | Input validation at API boundary |
| Cost model integer overflow | High | Low | Saturated arithmetic |
| Plan node with circular references | Medium | Low | Plan verification with depth limits |
| Statistics poisoning via concurrent writes | Medium | Medium | `Arc<RwLock<Statistics>>` synchronization |
| Memory exhaustion from large plan trees | High | Medium | Plan depth and node count limits |

## Access Control

The `kcm-optimizer` crate does not directly enforce access control. It relies on `kcm-security` for permission checks at the query execution layer. However, the optimizer must not bypass or weaken access control decisions made by upstream components.

| Operation | Required Permission | Enforcement Layer |
|-----------|--------------------|--------------------|
| Query optimization | `READ` on target columns | `kcm-runtime` |
| Statistics collection | `READ` on schema metadata | `kcm-runtime` |
| Plan rewriting | None (internal) | N/A |
| Adaptive execution adjustment | `READ` on query context | `kcm-runtime` |

## RBAC Integration

The optimizer operates within the context of an already-authenticated and authorized query session. It does not perform its own permission checks but must propagate the security context through all plan nodes.

- Plan nodes must carry the originating session's security context
- Adaptive execution must not escalate privileges beyond the original query context
- Statistics collection must respect column-level access controls

## Sensitive Assets

| Asset | Sensitivity | Protection |
|-------|-------------|------------|
| Query plan trees | Internal | In-memory only, not serialized to disk |
| Cost model parameters | Internal | Read-only after initialization |
| Column statistics | Internal | `Arc<RwLock<>>` synchronization |
| Plan cache entries | Internal | Ephemeral, not persisted |

## Secret Management

The `kcm-optimizer` crate does not handle secrets, keys, or credentials. It operates on structural query metadata only. All secret management is handled by `kcm-security` and `kcm-storage`.

## Secure Development Rules

### Cost Model Bounds

- All cost values must be non-negative (`f64::MIN` is not a valid cost)
- Cost accumulation must use saturating arithmetic to prevent overflow
- Operator cost estimates must be bounded by configurable maximums
- Cost comparisons must handle `NaN` and `Infinity` explicitly

### Statistics Validation

- Cardinality estimates must be non-negative integers
- Selectivity values must be in the range `[0.0, 1.0]`
- Statistics freshness must be validated before use in planning
- Null statistics must trigger a default fallback, never panic

### Plan Verification

- Plan trees must have a maximum depth (configurable, default 64)
- Plan nodes must not contain circular references
- All plan nodes must be fully materialized before execution
- Plan cost estimates must be finite and non-negative

### No Unwrap

- Zero `unwrap()` calls in production code paths
- All fallible operations must return `Result<T, KcmError>`
- Error propagation must preserve context at each call site

## Audit Logging

The optimizer itself does not produce audit events. Audit logging for optimizer operations is handled by `kcm-runtime` and `kcm-security`:

| Event | Logged By | Trigger |
|-------|-----------|---------|
| Query optimization request | `kcm-runtime` | Query execution start |
| Plan selection | `kcm-runtime` | After optimization completes |
| Statistics refresh | `kcm-runtime` | Statistics staleness detected |
| Adaptive plan adjustment | `kcm-runtime` | Runtime re-optimization |

## Validation Checklist

Before any change to `kcm-optimizer`, verify:

- [ ] Cost model inputs are validated at API boundary
- [ ] Statistics values are within defined bounds
- [ ] Plan node depth is bounded
- [ ] No `unwrap()` in production code paths
- [ ] All public APIs return `Result<T, KcmError>`
- [ ] Concurrency uses `parking_lot` synchronization primitives
- [ ] No sensitive data is logged or serialized
- [ ] Cost accumulation uses saturating arithmetic
- [ ] Plan verification catches circular references
- [ ] Security context is propagated through plan nodes

## References

| Document | Scope |
|----------|-------|
| [Project SECURITY.md](../../SECURITY.md) | Project-wide security policy |
| [AGENTS.md](../../AGENTS.md) | Engineering constitution |
| [PRD2.md §16](../../docs/PRD2.md) | Optimizer specification |
| [kcm-security](../kcm-security/) | RBAC and encryption implementation |
| [kcm-runtime](../kcm-runtime/) | Query execution and audit logging |
