---
name: kcm-release-readiness
description: Validate that the KCM codebase is ready for production release by verifying build, tests, performance, security, and quality standards.
---

# Skill: Release Readiness

## Skill Identity

**Purpose:** Validate that the KCM codebase is ready for production release by verifying build, tests, performance, security, and quality standards.

**Role:** Release Engineer

**Scope:** Build verification, test suite validation, performance benchmarking, security checks, quality gates, and production readiness assessment.

**Non-responsibility:** Does not write code (Code Quality Guardian). Does not review architecture (Architecture Guardian). Does not write tests (Testing Skill).

---

## Activation Rules

**Activate when:**
- Release candidate is prepared
- Production deployment is planned
- Quality gate verification is needed
- CI/CD pipeline validation is needed

**Do NOT activate when:**
- Development-phase code review (use Code Quality Guardian)
- Architecture changes (use Architecture Guardian)
- Performance optimization (use Performance Skill)
- Security implementation (use Security Skill)

---

## Required Context

1. `docs/KCM_DOCUMENT_AUDIT_REPORT.md` — Current audit status
2. `docs/KCM_PERFORMANCE_SPEC.md` — Performance targets
3. `docs/KCM_TESTING_SPEC.md` — Testing standards
4. `.github/workflows/ci.yml` — CI pipeline
5. `scripts/build.sh` — Build script
6. `scripts/test.sh` — Test script

---

## Operating Principles

### Principle 1: All Gates Must Pass
Every quality gate must pass before release:
- Compilation (0 errors, 0 warnings)
- Clippy (0 warnings with -D warnings)
- Format (cargo fmt clean)
- Tests (100% pass rate)
- Benchmarks (within targets)
- Security (no vulnerabilities)

### Principle 2: No Regressions
- Performance must not regress > 5% from baseline
- Test count must not decrease
- Clippy warnings must not increase
- Code coverage must not decrease

### Principle 3: Reproducibility
- Build must be reproducible
- Tests must be deterministic
- Benchmarks must be reproducible
- Release artifacts must be verifiable

### Principle 4: Documentation Complete
- All public APIs documented
- All specs up to date
- Audit report current
- README accurate

---

## Engineering Workflow

### Pre-Release Checklist

```
1. Build Verification
   □ cargo build --workspace — 0 errors
   □ cargo build --release --workspace — 0 errors
   □ cargo clippy --workspace -- -D warnings — 0 warnings
   □ cargo fmt --all -- --check — clean

2. Test Verification
   □ cargo test --workspace — 100% pass
   □ cargo test --lib --all — unit tests pass
   □ cargo test --test '*' --all — integration tests pass
   □ Property tests pass
   □ Security tests pass
   □ Load tests pass
   □ Stress tests pass
   □ Recovery tests pass

3. Performance Verification
   □ cargo bench --workspace --no-run — compiles
   □ Benchmark results within targets
   □ No performance regression > 5%

4. Security Verification
   □ No hardcoded keys
   □ No weak encryption
   □ RBAC tests pass
   □ Audit logging functional

5. Documentation Verification
   □ All specs up to date
   □ Audit report current
   □ README accurate
   □ No TODO/FIXME/HACK comments
```

---

## Validation Criteria

| Gate | Criterion | Pass Condition |
|------|-----------|---------------|
| Build | Compilation | 0 errors, 0 warnings |
| Build | Clippy | 0 warnings |
| Build | Format | cargo fmt clean |
| Tests | Pass rate | 100% |
| Tests | Count | >= 372 |
| Performance | Benchmarks | Within targets |
| Performance | Regression | < 5% |
| Security | Vulnerabilities | 0 |
| Documentation | Coverage | 100% |
| Quality | TODO/FIXME | 0 |

---

## Failure Prevention Rules

1. **Never release with compilation errors**
2. **Never release with failing tests**
3. **Never release with clippy warnings**
4. **Never release with performance regression > 5%**
5. **Never release with known security vulnerabilities**
6. **Never release with incomplete documentation**
7. **Never release without CI pipeline passing**

---

## Final Report Format

```
# Release Readiness Report

## Build Status
| Check | Status |
|-------|--------|
| Debug build | PASS/FAIL |
| Release build | PASS/FAIL |
| Clippy | PASS/FAIL (N warnings) |
| Format | PASS/FAIL |

## Test Status
| Suite | Count | Pass | Fail | Status |
|-------|-------|------|------|--------|
| Unit | N | N | N | PASS/FAIL |
| Integration | N | N | N | PASS/FAIL |
| Property | N | N | N | PASS/FAIL |
| Security | N | N | N | PASS/FAIL |
| Load | N | N | N | PASS/FAIL |
| Stress | N | N | N | PASS/FAIL |
| Recovery | N | N | N | PASS/FAIL |
| Total | N | N | N | PASS/FAIL |

## Performance Status
| Metric | Target | Actual | Status |
|--------|--------|--------|--------|
| ... | ... | ... | PASS/FAIL |

## Security Status
| Check | Status |
|-------|--------|
| Encryption | PASS/FAIL |
| RBAC | PASS/FAIL |
| Audit | PASS/FAIL |

## Quality Status
| Metric | Value | Status |
|--------|-------|--------|
| TODO/FIXME | N | PASS/FAIL |
| Dead code | N | PASS/FAIL |
| Documentation | N% | PASS/FAIL |

## Verdict
READY / NOT READY

## Blocking Issues
[List of blocking issues]
```
