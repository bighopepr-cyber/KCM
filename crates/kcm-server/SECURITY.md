# kcm-server Security Policy

> For project-wide security policy, see the root `SECURITY.md`.

## Overview

This document defines the security policy, threat model, and secure development requirements for the `kcm-server` crate. kcm-server provides the HTTP (actix-web) and gRPC (tonic) server binaries for the KCM knowledge engine. All security-critical behavior is delegated to `kcm-security` via `kcm-interface`; this crate is responsible for transport-level security, request validation, and adherence to the security architecture.

## Security Scope

| Component | Criticality | Description |
|-----------|-------------|-------------|
| HTTP Server | High | actix-web binary serving REST endpoints |
| gRPC Server | High | tonic binary serving protobuf RPCs |
| TLS Termination | High | TLS certificate management and enforcement |
| Authentication | High | Token and credential validation on all endpoints |

## Threat Model

| Threat | Risk Level | Mitigation |
|--------|-----------|------------|
| DDoS | High | Rate limiting, request size limits, connection limits |
| Man-in-the-Middle | High | TLS in production, certificate validation |
| Authentication Bypass | High | Auth middleware on all non-health endpoints |
| Request Smuggling | Medium | Strict HTTP parsing via actix-web |
| Header Injection | Medium | Input sanitization, response header filtering |
| Resource Exhaustion | High | Timeout enforcement, request body size caps, connection pooling limits |

## Security Risks

kcm-server exposes network-facing services. The primary risks are:

- Unauthenticated access to knowledge data via HTTP or gRPC
- TLS misconfiguration allowing plaintext traffic
- Resource exhaustion through unbounded request processing
- Information leakage through error messages or server headers

## Access Control

All access control is delegated to `kcm-security` RBAC through `kcm-interface` middleware. kcm-server does not implement authentication or authorization logic directly. Every request passes through the interface layer which enforces permission checks before any business logic executes.

## RBAC Integration

- kcm-server configures middleware that routes requests through `kcm-interface` authentication handlers
- `kcm-interface` delegates permission checks to `kcm-security` RBAC
- Five permission levels are enforced: Public, Read, Write, Admin, Root
- No endpoint is accessible without passing through the RBAC pipeline (health endpoints are the sole exception)
- Token validation, session management, and permission resolution are handled entirely by the security stack

## Sensitive Assets

| Asset | Protection |
|-------|-----------|
| Server configuration | Environment variables, not hardcoded |
| TLS certificates | File system permissions, not committed to repository |
| API keys | Environment variables, never logged |
| Database path | Configuration only, not exposed via API |

## Secret Management

- All secrets are loaded from environment variables at startup
- Secrets are never written to logs, error messages, or responses
- TLS certificate paths are validated at startup; server refuses to start with invalid paths
- API keys are passed to `kcm-security` through `kcm-interface`; kcm-server does not store them

## Secure Development Rules

1. TLS must be enabled in all production deployments
2. Authentication middleware must be applied to all non-health endpoints
3. Rate limiting must be configured on all public-facing endpoints
4. Request body size limits must be enforced (default 1 MB for REST, configurable)
5. Request timeouts must be enforced (default 30 seconds)
6. Graceful shutdown must drain in-flight requests before terminating
7. No `unwrap()` in production code paths
8. All public handler functions must return `Result<T, KcmError>`
9. Security headers (CSP, X-Frame-Options, HSTS) must be set on all responses
10. CORS policy must be configurable and restrictive by default

## Audit Logging

All HTTP and gRPC requests are logged via middleware. Logged fields include:

- Timestamp
- Client IP address
- HTTP method and URI (or gRPC method)
- Response status code
- Request duration
- Authenticated user (when available)

Audit events are forwarded to `kcm-security` audit log for hash-chained persistence.

## Validation Checklist

- [ ] TLS enabled in production configuration
- [ ] Auth middleware applied to all non-health endpoints
- [ ] Rate limiting configured
- [ ] Request body size limits enforced
- [ ] Timeouts enforced on all handlers
- [ ] Graceful shutdown implemented
- [ ] No `unwrap()` in production code
- [ ] All handlers return `Result<T, KcmError>`
- [ ] Security headers present on responses
- [ ] CORS policy configured
- [ ] Audit logging active
- [ ] No secrets in logs or error messages

## References

- Root `SECURITY.md` — project-wide security policy
- `AGENTS.md` — engineering constitution
- PRD3.md §28 — security architecture
- PRD2.md §19 — interface layer security
- `kcm-security` crate — RBAC, encryption, audit log
- `kcm-interface` crate — middleware, auth handlers
