# kcm-runtime Security Policy

> This document covers security concerns specific to the `kcm-runtime` crate. For the project-wide security policy, see the root `SECURITY.md`.

## Overview

`kcm-runtime` is the runtime and transaction layer for KCM. It provides the `KnowledgeDatabase` entry point, ACID transaction management, metrics collection, health monitoring, and synchronous/asynchronous executors. Because this crate mediates all access to persistent data and manages concurrency, security is critical to correctness and data integrity.

## Security Scope

| Component | Security Level | Rationale |
|---|---|---|
| KnowledgeDatabase | High | Primary data access point; corruption or unauthorized access compromises all stored knowledge |
| Transaction | High | ACID guarantees must be upheld; isolation violations can cause data corruption |
| Metrics | Medium | Counter manipulation could mask operational issues or trigger incorrect alerts |
| Health | Low | Read-only status; bypass could delay incident response |

## Threat Model

| Threat | Description | Mitigation |
|---|---|---|
| Concurrent access race conditions | Multiple threads or async tasks accessing shared state without proper synchronization | parking_lot RwLock/Mutex with well-defined lock ordering |
| Transaction isolation violations | Transactions seeing uncommitted changes from other transactions | Snapshot isolation via version tracking; Mutex-guarded transaction state |
| Metric counter manipulation | Unchecked concurrent increments leading to data races or incorrect counters | AtomicU64 for all 14 counters; no shared mutable state |
| Health check bypass | Stale or incorrect health status masking real failures | Health status derived from live counters; timeout-bounded checks |
| Unauthorized database access | Unprivileged code paths modifying or reading data | RBAC integration via kcm-security permission checks |

## Security Risks

| Risk | Impact | Likelihood | Severity |
|---|---|---|---|
| Lock contention under high concurrency | Denial of service | Medium | High |
| Transaction deadlock from lock ordering violations | System hang | Low | Critical |
| Metric overflow on AtomicU64 | Incorrect metrics | Very Low | Low |
| Stale health status after crash | Delayed failure detection | Low | Medium |
| Memory exhaustion from unbounded transaction state | OOM | Low | High |

## Access Control

All public APIs on `KnowledgeDatabase` and `Transaction` enforce access control through the `kcm-security` crate. Permission checks are performed before any data mutation or sensitive read operation.

## RBAC Integration

`kcm-runtime` integrates with `kcm-security` for permission checks:

- Every `KnowledgeDatabase::insert` call checks `Write` permission
- Every `KnowledgeDatabase::query` call checks `Read` permission
- Transaction `commit` checks `Write` permission at commit time
- Transaction `rollback` requires at least `Read` permission on the owning database
- Executor task submission checks `Execute` permission

Permission levels follow the 5-tier RBAC model defined in `kcm-security`: `None`, `Read`, `Write`, `Admin`, `SuperAdmin`.

## Sensitive Assets

| Asset | Protection |
|---|---|
| Database handles | Wrapped in `Arc<RwLock>`; not directly exposed outside crate |
| Transaction state | Mutex-guarded; state transitions are atomic |
| WAL file handles | Mutex-guarded; serialized writes only |
| Metric counters | AtomicU64; no mutable shared references |

## Secret Management

`kcm-runtime` does not handle secrets directly. Encryption keys are managed by `kcm-security`. The runtime trusts `kcm-security` for key material and never stores or logs key data.

## Secure Development Rules

1. **Transaction isolation**: All transaction operations must respect snapshot isolation. No transaction may read uncommitted data from another transaction.
2. **Concurrent access safety**: All shared state must be protected by `parking_lot` locks or atomic types. No raw `Mutex`/`RwLock` from `std`.
3. **Metric atomicity**: All metric updates must use atomic operations. No metric may be read-then-written without atomics.
4. **Health check correctness**: Health status must be computed from live counters, not cached values. Stale health data is worse than no health data.
5. **No unwrap in production code**: All fallible operations must use `?` or explicit error handling. Zero `unwrap()` in non-test code.
6. **Result return**: All public APIs must return `Result<T, KcmError>`. Never return raw values that could mask failures.

## Audit Logging

Security-relevant events are logged through the `kcm-security` audit log:

- Database open/close events
- Transaction begin/commit/rollback events
- Permission check failures
- Health status transitions

Audit events are hash-chained via `kcm-security` for tamper detection.

## Validation Checklist

- [ ] All public APIs return `Result<T, KcmError>`
- [ ] No `unwrap()` in production code paths
- [ ] All shared state uses `parking_lot` or atomic types
- [ ] Transaction isolation is maintained under concurrent access
- [ ] Metric counters use `AtomicU64`
- [ ] Health checks compute from live data
- [ ] RBAC permission checks are enforced on all data mutations
- [ ] Audit logging covers security-relevant events
- [ ] No secrets stored or logged by the runtime
- [ ] Lock ordering is consistent to prevent deadlocks

## References

- `AGENTS.md` — Engineering constitution and non-negotiable rules
- `SECURITY.md` (root) — Project-wide security policy
- `docs/PRD2.md §18` — Runtime specification
- `docs/PRD3.md §30` — Security specification (RBAC, encryption, audit)
- `kcm-security` crate — RBAC, encryption, and audit implementation
