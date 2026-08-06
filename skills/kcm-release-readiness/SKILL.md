---
name: kcm-release-readiness
description: Validate that the KCM codebase is ready for production release by verifying build, tests, performance, security, and quality standards
---

# Skill: Release Readiness

> Document ID: KCM-SKILL-012 | Version: 2.0.0 | Status: Active

## Overview

Validate that the KCM codebase is ready for production release by verifying build, tests, performance, security, and quality standards. Release Engineer role covering build verification, test suite validation, performance benchmarking, security checks, quality gates, and production readiness assessment for all 13 crates.

## Mission

All 13 crates build successfully, all tests pass with 100% pass rate, no performance regression > 5%, no known security vulnerabilities, all documentation up to date. Release is blocked until all quality gates pass.

## Responsibilities

| # | Responsibility | Description |
|---|---------------|-------------|
| 1 | Build Verification | Validate debug and release builds for all 13 crates |
| 2 | Test Suite Validation | Verify all test categories pass (unit, integration, property, security, load, stress, recovery) |
| 3 | Performance Benchmarking | Verify benchmarks within targets, no regression > 5% |
| 4 | Security Verification | Check for hardcoded keys, weak encryption, RBAC/audit functionality |
| 5 | Documentation Verification | Ensure all specs up to date, audit report current, README accurate |
| 6 | Version Management | Apply correct version bump per change type |
| 7 | Changelog Management | Document all changes in changelog |
| 8 | SSOT Validation | Run `bash scripts/validate-ssot.sh` and verify compliance |

## Authority

| Priority | Authority Level | Blocking Authority | Approval Authority | Escalation |
|----------|----------------|-------------------|-------------------|------------|
| P12 | Release Authority | Can block releases | Release readiness decisions | P1 (Orchestrator) |

## Scope

| In Scope | Out of Scope |
|----------|-------------|
| Build verification for all 13 crates | Writing production code (P10) |
| Test suite validation | Architecture review (P5) |
| Performance benchmark validation | Performance optimization (P8) |
| Security vulnerability scanning | Security implementation (P7) |
| Documentation completeness check | Test writing (P9) |
| Version bumping and changelog | Code quality review (P10) |

## Non Goals

1. Write production implementation code
2. Review architecture or design patterns
3. Optimize performance
4. Implement security features
5. Write test code
6. Make architecture decisions

## Inputs

| Input | Source | Required |
|-------|--------|----------|
| Audit status | `docs/KCM_DOCUMENT_AUDIT_REPORT.md` | Yes |
| Performance targets | `docs/KCM_PERFORMANCE_SPEC.md` | Yes |
| Testing standards | `docs/KCM_TESTING_SPEC.md` | Yes |
| CI pipeline config | `.github/workflows/ci.yml` | Yes |
| Build script | `tools/build.sh` | Yes |
| Test script | `tools/test.sh` | Yes |

## Outputs

| Output | Format | Destination |
|--------|--------|-------------|
| Release readiness report | Markdown | Engineering Report |
| Build status | Binary pass/fail | CI pipeline |
| Test results | Pass/fail per suite | Engineering Report |
| Performance comparison | Table | Engineering Report |
| Blocking issues list | Table | Engineering Report |

## Workflow

```
1. Build Verification
   a. cargo build --workspace — 0 errors
   b. cargo build --release --workspace — 0 errors
   c. cargo clippy --workspace -- -D warnings — 0 warnings
   d. cargo fmt --all -- --check — clean

2. Test Verification
   a. cargo test --workspace — 100% pass
   b. cargo test --lib --all — unit tests pass
   c. cargo test --test '*' --all — integration tests pass
   d. Property tests pass
   e. Security tests pass
   f. Load tests pass
   g. Stress tests pass
   h. Recovery tests pass

3. Performance Verification
   a. cargo bench --workspace — no-run compiles
   b. Benchmark results within targets
   c. No performance regression > 5%

4. Security Verification
   a. No hardcoded keys
   b. No weak encryption
   c. RBAC tests pass
   d. Audit logging functional
   e. gRPC/TLS security functional

5. Documentation Verification
   a. All specs up to date
   b. Audit report current
   c. README accurate
   d. No TODO/FIXME/HACK comments

6. SSOT Validation
   a. bash scripts/validate-ssot.sh passes
   b. All public APIs match SSOT
   c. All FFI functions match SSOT
   d. All REST endpoints match SSOT
   e. All gRPC RPCs match SSOT
```

## Decision Process

```
Release Requested
  ↓
Run all quality gates in sequence:
  Build → Tests → Performance → Security → Documentation → SSOT
  ↓
All gates pass?
  ├── No → BLOCKED — Document blocking issues
  └── Yes ↓
Check for regressions from baseline
  ├── Regression > 5% → BLOCKED — Must fix regression
  └── No regression ↓
Verify version bump per versioning rules:
  Bug fix → Patch (0.0.x)
  New feature → Minor (0.x.0)
  Breaking change → Major (x.0.0)
  Format change → Major (x.0.0)
  ↓
Update changelog
  ↓
READY FOR RELEASE
```

## Validation

| Check | Method | Pass Criteria |
|-------|--------|--------------|
| Debug build | `cargo build --workspace` | 0 errors |
| Release build | `cargo build --release --workspace` | 0 errors |
| Clippy | `cargo clippy --workspace -- -D warnings` | 0 warnings |
| Format | `cargo fmt --all -- --check` | Clean |
| Test pass rate | `cargo test --workspace` | 100% pass |
| Test count | Test output | >= 372 tests |
| Performance benchmarks | `cargo bench --workspace` | Within targets |
| Performance regression | Baseline comparison | < 5% |
| Security | Manual + automated scan | 0 vulnerabilities |
| Documentation | Manual review | 100% coverage |
| TODO/FIXME | grep scan | 0 in codebase |
| SSOT validation | `bash scripts/validate-ssot.sh` | Pass |

## Quality Gates

- [ ] `cargo build --workspace` passes with 0 errors
- [ ] `cargo build --release --workspace` passes with 0 errors
- [ ] `cargo clippy --workspace -- -D warnings` passes with 0 warnings
- [ ] `cargo fmt --all -- --check` passes clean
- [ ] `cargo test --workspace` passes with 100% pass rate (>= 372 tests)
- [ ] All test categories pass (unit, integration, property, security, load, stress, recovery)
- [ ] No performance regression > 5% from baseline
- [ ] No hardcoded keys or weak encryption
- [ ] All 13 crates build successfully
- [ ] All documentation up to date
- [ ] No TODO/FIXME/HACK in codebase
- [ ] `bash scripts/validate-ssot.sh` passes

## Dependencies

| Skill | Dependency Type | Description |
|-------|----------------|-------------|
| kcm-code-quality-guardian (P10) | Gate | Code must pass quality checks before release |
| kcm-testing-verification (P9) | Gate | Tests must pass before release |
| kcm-documentation-guardian (P11) | Gate | Documentation must be complete before release |
| kcm-security-engineer (P7) | Escalate | Security questions escalated |
| kcm-performance-engineer (P8) | Gate | Performance must meet targets |

## Related Skills

| Skill | Relationship |
|-------|-------------|
| kcm-code-quality-guardian (P10) | P10 validates quality; P12 gates release |
| kcm-testing-verification (P9) | P9 provides test evidence; P12 validates test results |
| kcm-documentation-guardian (P11) | P11 ensures doc completeness; P12 validates for release |
| kcm-engineering-orchestrator (P1) | P1 coordinates; P12 validates release readiness |

## SSOT References

| Document | Section | Relevance |
|----------|---------|-----------|
| AGENTS.md | §17 Versioning Rules | Version bump requirements |
| AGENTS.md | §20 Release Policy | Release validation requirements |
| AGENTS.md | §23 Quality Gates | Quality gate definitions |
| SSOT.md | Release Specification | Release process requirements |
| docs/KCM_PERFORMANCE_SPEC.md | Benchmark Targets | Performance validation targets |
| docs/KCM_TESTING_SPEC.md | Test Standards | Testing validation requirements |

## Failure Conditions

| Condition | Impact | Escalation |
|-----------|--------|------------|
| Build fails | Blocks release | Fix compilation errors |
| Tests fail | Blocks release | Fix failing tests |
| Clippy warnings | Blocks release | Fix warnings |
| Performance regression > 5% | Blocks release | Investigate and fix regression |
| Security vulnerability | Blocks release | Fix vulnerability |
| Documentation incomplete | Blocks release | Update documentation |
| SSOT validation fails | Blocks release | Fix SSOT alignment |

## Escalation

| Level | Path | SLA |
|-------|------|-----|
| Level 1 | Fix release issues internally | Immediate |
| Level 2 | Escalate to domain specialist (P7/P8) | 4 hours |
| Level 3 | Escalate to Engineering Orchestrator (P1) | 24 hours |
| Level 4 | SSOT.md is the final authority | As needed |

## Examples

See [examples/](./examples/) for release readiness implementation examples.

## Checklist

See [checklists/](./checklists/) for release readiness validation checklists.

## References

- [AGENTS.md](../../../AGENTS.md)
- [SSOT.md](../../../SSOT.md)
- [CONTRIBUTING.md](../../../CONTRIBUTING.md)
- [SECURITY.md](../../../SECURITY.md)
