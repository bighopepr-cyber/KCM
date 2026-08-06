# tests/ Security Policy

> For project-wide security policies, refer to the [SECURITY.md](../SECURITY.md) located in the repository root.

## Overview

This document defines security requirements, threat models, and secure development practices specific to the `tests/` directory. The `tests/` directory contains integration tests, SDK cross-language consistency tests, and supporting infrastructure (mock server, validation scripts). While test code does not ship to production, it must follow security practices that prevent exposure of secrets, test data leakage, and resource exhaustion.

## Security Scope

| Component | Risk Level | Description |
|-----------|-----------|-------------|
| Integration Tests | Medium | Workspace-level tests that exercise the full KCM engine |
| SDK Tests | Medium | Cross-language consistency tests against mock server |

## Threat Model

| Threat | Vector | Impact | Mitigation |
|--------|--------|--------|------------|
| Test data leakage | Test fixtures containing sensitive patterns committed to repo | Low (test data is synthetic) | Enforce no production data rule |
| Mock server exposure | Mock server bound to non-loopback interface | Medium (network access to test server) | Bind to localhost only |
| Resource exhaustion | Tests consuming excessive memory/CPU | Medium (CI pipeline degradation) | Set test timeout limits |
| Secret exposure | Hardcoded credentials in test scripts | High | Use environment variables, audit logs |

## Security Risks

### Test Data Leakage

Test fixtures and integration tests use synthetic data. No production data, real user information, or realistic PII shall appear in test files. Test data must be clearly synthetic and traceable to test definitions.

### Mock Server Exposure

The mock server (`tests/sdk/mock_server.py`) must bind to `127.0.0.1` (localhost only). It must not be exposed to external networks. The mock server does not implement authentication; it is strictly a testing tool.

### Resource Exhaustion

Integration tests and SDK tests execute against the full engine or mock server. Tests must set appropriate timeouts and resource limits. Long-running tests must be flagged in `consistency_matrix.json`.

## Access Control

| Asset | Access | Justification |
|-------|--------|---------------|
| Test scripts | All engineers | Tests are open for inspection and contribution |
| Mock server | Localhost only | Test infrastructure, not a production endpoint |
| Test reports | CI artifacts | Generated during CI, not committed |
| Test data | Synthetic only | No production data permitted |

## RBAC Integration

The `tests/` directory does not implement RBAC directly. However, integration tests validate that the KCM RBAC system functions correctly by exercising permission checks through the engine. SDK tests validate that permission-related API surfaces are consistent across all SDKs.

## Sensitive Assets

| Asset | Classification | Handling |
|-------|---------------|----------|
| Test data | Non-sensitive | Synthetic data, no PII |
| Mock server | Internal tool | Localhost only, no auth |
| Test reports | Internal artifact | CI-only, gitignored |

## Secret Management

- No secrets shall be hardcoded in test scripts
- Test credentials (if any) must be provided via environment variables
- Mock server does not require or store secrets
- CI secrets (if any) must be injected via CI pipeline configuration

## Secure Development Rules

| Rule | Description | Enforcement |
|------|-------------|-------------|
| Test isolation | Each test must be independent and not depend on execution order | Code review |
| Temp file cleanup | Tests must clean up temporary files and directories | Code review + CI |
| No production data | Test fixtures must use synthetic data exclusively | Code review + CI audit |
| No unwrap in infrastructure | Test infrastructure code (mock server, scripts) must handle errors gracefully | Code review |
| Localhost binding | Mock server must bind to 127.0.0.1 only | Code review |

## Audit Logging

Security-relevant test events should be logged:

| Event | Level | Description |
|-------|-------|-------------|
| Test failure | INFO | Test execution failure (expected or unexpected) |
| Security test result | INFO | Security validation pass/fail |
| Resource limit exceeded | WARN | Test exceeding time or memory limits |

## Validation Checklist

- [ ] No production data in test fixtures
- [ ] Mock server binds to localhost only
- [ ] No hardcoded secrets in test scripts
- [ ] Temporary files cleaned up after test execution
- [ ] Test isolation verified (no shared state between tests)
- [ ] Error handling in test infrastructure (no unwrap)
- [ ] CI pipeline validates security rules

## References

- [Repository Security Policy](../SECURITY.md)
- [KCM Architecture](../AGENTS.md)
- [KCM API Specification](../docs/specs/KCM_API_SPEC.md)
- [Testing Strategy](../docs/PRD-TESTING&%20BRACHMARCK.md)
