# kcm-interface Security Policy

> For organization-wide security policy, see the root `SECURITY.md`.

## Overview

kcm-interface is the external-facing boundary of the KCM system. It exposes C FFI functions, REST API endpoints, Python bindings, and a KQL query parser. Any vulnerability in this crate directly impacts all consumers of the KCM engine.

## Security Scope

| Component | Security Level | Rationale |
|-----------|---------------|-----------|
| C FFI | Critical | Unsafe boundary; memory safety is caller's and crate's shared responsibility |
| REST API | High | Public network-facing; handles authentication and data access |
| Python bindings | High | Foreign runtime; must not leak unsafe primitives |
| KQL parser | Medium | Untrusted input parsing; injection and denial-of-service risk |
| Middleware | High | Auth, CORS, rate limiting — core security enforcement |

## Threat Model

| Threat | Component | Severity | Mitigation |
|--------|-----------|----------|------------|
| Null pointer dereference | FFI | Critical | All FFI functions validate non-null before dereference |
| Buffer overflow | FFI | Critical | All buffer operations bounded; no raw pointer arithmetic without length |
| KQL injection | KQL parser | High | Input sanitization; parameterized queries only |
| REST API abuse | REST API | High | Rate limiting, authentication enforcement on all endpoints |
| Authentication bypass | Middleware (auth) | Critical | Auth middleware applied to all protected routes; no bypass paths |
| CORS misconfiguration | Middleware (cors) | High | Restricted origin whitelist; no wildcard in production |
| Denial of service | Rate limiter | Medium | Configurable rate limits per client; request throttling |

## Security Risks

- FFI functions that accept raw pointers are inherently unsafe; misuse causes undefined behavior.
- KQL parser processes untrusted strings; malformed input must not crash the engine.
- REST endpoints expose database operations; unauthorized access leads to data breach.
- Python bindings bridge managed and unmanaged memory; reference counting errors cause leaks or double-frees.
- Middleware misconfiguration disables entire security layers silently.

## Access Control

All REST API operations require authentication via the auth middleware (`middleware/auth.rs`). Unauthenticated requests receive `401 Unauthorized`. The RBAC system from kcm-security is integrated directly into the middleware layer.

## RBAC Integration

`middleware/auth.rs` integrates with `kcm-security`'s RBAC manager:

- Every request is checked against the caller's permission level before the handler executes.
- Permission levels: `Read`, `Write`, `Admin`, `SuperAdmin`, `Owner`.
- Middleware extracts the authenticated identity and queries the RBAC engine.
- Denied requests return `403 Forbidden` with an audit log entry.

## Sensitive Assets

| Asset | Location | Protection |
|-------|----------|------------|
| FFI database handles | `KCM_Database` pointers | Opaque; no public access to internals |
| API keys | Environment / config | Never logged; redacted in error messages |
| Database file paths | Configuration | Validated against allowlist; no path traversal |
| KQL query strings | Request body | Sanitized before parsing |

## Secret Management

- API keys and encryption keys are never stored in source code.
- Secrets are loaded from environment variables or a secrets manager.
- No secret is logged at any log level.
- FFI handles are opaque pointers; internals are never exposed to external consumers.

## Secure Development Rules

1. **Null-pointer guards** — Every FFI function checks all pointer arguments for null before use.
2. **Bounds checking** — All buffer and slice operations validate length against capacity.
3. **Input validation** — All REST request bodies are validated against schema before processing.
4. **KQL sanitization** — KQL input is stripped of control characters and validated against grammar.
5. **Auth enforcement** — Every protected REST endpoint uses the auth middleware; no endpoint bypasses it.
6. **CORS policy** — CORS middleware uses a strict origin whitelist; `*` is never allowed.
7. **Rate limiting** — All REST endpoints are rate-limited; limits are configurable per endpoint class.
8. **No unwrap** — Production code never calls `unwrap()`; all fallible operations use `Result`.
9. **Result return** — All public FFI functions return `KCM_Error`; all REST handlers return structured errors.
10. **# Safety docs** — Every `unsafe` function and every FFI export has a `# Safety` doc comment.

## Audit Logging

`middleware/logging.rs` logs all API requests:

- Method, path, status code, response time, and client IP are recorded.
- Authentication events (success and failure) are logged.
- Failed FFI calls are logged with the error code.
- Log entries are structured (JSON) for machine consumption.

## Validation Checklist

- [ ] All FFI functions handle null pointers
- [ ] All FFI functions handle out-of-memory conditions
- [ ] KQL parser rejects malformed input without panicking
- [ ] Auth middleware protects all non-public endpoints
- [ ] CORS policy has no wildcard origins
- [ ] Rate limiter is active on all REST endpoints
- [ ] No secrets in source code or logs
- [ ] All `unsafe` blocks have `# Safety` documentation
- [ ] Fuzz tests cover KQL parser and FFI entry points
- [ ] No `unwrap()` in production code paths

## References

- `AGENTS.md` — Non-negotiable rules and error model
- `docs/PRD2.md §19` — Interface specification (SSOT)
- `docs/PRD.md §7` — FFI function definitions (SSOT)
- `docs/SSOT.md` — Single source of truth for all specifications
- Root `SECURITY.md` — Organization-wide security policy
