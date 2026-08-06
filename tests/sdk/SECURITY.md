# tests/sdk/ Security Policy

> For project-wide security policies, refer to the [SECURITY.md](../../SECURITY.md) located in the repository root.

## Overview

This document defines security requirements specific to the `tests/sdk/` directory, which contains the mock server, cross-language consistency tests, API validation scripts, and supporting infrastructure for SDK testing.

## Security Scope

| Component | Risk Level | Description |
|-----------|-----------|-------------|
| Mock Server | Medium | Flask-based REST server for SDK testing |
| Test Scripts | Low | Python scripts executing test sequences |

## Threat Model

| Threat | Vector | Impact | Mitigation |
|--------|--------|--------|------------|
| Mock server on network | Server bound to non-loopback | Unauthorized access to test API | Bind to 127.0.0.1 only |
| Test data exposure | Test fixtures containing sensitive patterns | Low (synthetic data only) | Enforce no production data rule |
| Mock server impersonation | Impersonating mock server for testing | Medium (false test results) | Validate server identity in tests |

## Security Risks

### Mock Server Exposure

The mock server (`mock_server.py`) implements the KCM REST API without authentication. It must never be exposed to external networks. All connections must originate from localhost.

### Test Data Exposure

SDK tests use synthetic data for all operations. No production data, real user information, or realistic PII shall appear in test fixtures or mock server state.

## Access Control

| Asset | Access | Justification |
|-------|--------|---------------|
| Mock server | Localhost only | Test infrastructure, not a production endpoint |
| Test scripts | All engineers | Open for inspection and contribution |
| Test reports | CI artifacts | Generated during CI, not committed |
| consistency_matrix.json | All engineers | Test case definitions and results |

## RBAC Integration

SDK tests validate that permission-related API surfaces are consistent across all SDKs. The mock server does not implement RBAC; it validates API surface compliance, not access control.

## Sensitive Assets

| Asset | Classification | Handling |
|-------|---------------|----------|
| Mock server state | Non-sensitive | In-memory, synthetic data only |
| Test fixtures | Non-sensitive | Synthetic data, no PII |
| Test reports | Internal artifact | CI-only, gitignored |

## Secret Management

- No secrets shall be hardcoded in SDK test scripts
- Mock server does not require or store secrets
- Test credentials (if any) must be provided via environment variables
- CI secrets (if any) must be injected via CI pipeline configuration

## Secure Development Rules

| Rule | Description | Enforcement |
|------|-------------|-------------|
| Localhost binding | Mock server must bind to 127.0.0.1 only | Code review |
| No auth bypass | Tests must not bypass or mock authentication mechanisms | Code review |
| Clean up test data | Tests must clear mock server state between runs | Code review + CI |
| No production data | Test fixtures must use synthetic data exclusively | Code review |
| Error handling | Mock server must handle errors gracefully (no unwrap) | Code review |

## Audit Logging

| Event | Level | Description |
|-------|-------|-------------|
| Mock server start | INFO | Server started on localhost |
| Mock server stop | INFO | Server stopped cleanly |
| Test failure | INFO | Test execution failure |
| Security test result | INFO | Security validation pass/fail |

## Validation Checklist

- [ ] Mock server binds to 127.0.0.1 only
- [ ] No hardcoded secrets in test scripts
- [ ] Test data is synthetic (no production data)
- [ ] Mock server state is cleared between test runs
- [ ] Error handling in mock server (no unwrap)
- [ ] Test isolation verified (no shared state)
- [ ] Temporary files cleaned up after test execution

## References

- [Repository Security Policy](../../SECURITY.md)
- [tests/ Security Policy](../SECURITY.md)
- [SDK Test README](README.md)
- [KCM API Specification](../../docs/specs/KCM_API_SPEC.md)
