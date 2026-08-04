---
name: kcm-release-readiness
description: Validate that the KCM codebase is ready for production release by verifying build, tests, performance, security, and quality standards.
---

# Skill: Release Readiness

## Skill Identity

**Purpose:** Validate that the KCM codebase is ready for production release by verifying build, tests, performance, security, and quality standards.

**Role:** Release Engineer

**Scope:** Build verification, test suite validation, performance benchmarking, security checks, quality gates, and production readiness assessment for all 13 crates.

**Non-responsibility:** Does not write code (Code Quality Guardian). Does not review architecture (Architecture Guardian). Does not write tests (Testing Skill). Does not review security (Security Engineer).

**Measurable Outcomes:**
- `cargo build --release --workspace` passes (0 errors)
- `cargo test --workspace` passes (100% pass rate, >= 372 tests)
- `cargo clippy --workspace -- -D warnings` passes (0 warnings)
- `cargo fmt --all -- --check` passes
- No performance regression > 5%
- All 13 crates build successfully

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
- Security implementation (use Security Engineer)

---

## Required Context

1. `docs/KCM_DOCUMENT_AUDIT_REPORT.md` — Current audit status
2. `docs/KCM_PERFORMANCE_SPEC.md` — Performance targets
3. `docs/KCM_TESTING_SPEC.md` — Testing standards
4. `.github/workflows/ci.yml` — CI pipeline
5. `tools/build.sh` — Build script
6. `tools/test.sh` — Test script

---

## Crate Awareness

Build validation covers all **13 crates**:

| Crate | Key Validation |
|-------|---------------|
| kcm-core | Core types compile, all tests pass |
| kcm-storage | Storage engine compiles, format tests pass |
| kcm-compute | Operators and SIMD compile |
| kcm-reasoning | Rules and inference compile |
| kcm-optimizer | Cost model and planner compile |
| kcm-runtime | Database, transactions, executor compile |
| kcm-interface | FFI, REST, KQL, Python compile |
| kcm-distributed | Sharding and coordinator compile |
| kcm-ml | Learned index and confidence learner compile |
| kcm-security | Encryption, RBAC, audit compile |
| kcm-compliance | GDPR and classification compile |
| kcm-testing | Test infrastructure compiles |
| kcm-server | gRPC server, gRPC main, main entry compile |

---

## Operating Principles

### Principle 1: All Gates Must Pass
Every quality gate must pass before release:
- Compilation (0 errors, 0 warnings) — all 13 crates
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
1. Build Verification (all 13 crates)
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
   □ gRPC/TLS security functional

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
| Build | Compilation | 0 errors, 0 warnings (all 13 crates) |
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
8. **Never release without validating all 13 crates build**

---

## Final Report Format

```
# KCM Engineering Report

## Skill
kcm-release-readiness

## Build Status
| Check | Status |
|-------|--------|
| Debug build (13 crates) | PASS/FAIL |
| Release build (13 crates) | PASS/FAIL |
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
| gRPC/TLS | PASS/FAIL |

## Quality Status
| Metric | Value | Status |
|--------|-------|--------|
| TODO/FIXME | N | PASS/FAIL |
| Dead code | N | PASS/FAIL |
| Documentation | N% | PASS/FAIL |

## Specification Impact
[files]

## Code Impact
[files]

## Verdict
READY / NOT READY

## Blocking Issues
[List of blocking issues]
```

## SSOT-First Release Protocol

Every release MUST follow this protocol:

1. **SSOT Validation**: `bash scripts/validate-ssot.sh` passes
2. **All CI Jobs Pass**: format, clippy, build, tests, benchmarks
3. **API Audit**: All public APIs match SSOT
4. **FFI Audit**: All FFI functions match SSOT
5. **REST Audit**: All REST endpoints match SSOT
6. **gRPC Audit**: All gRPC RPCs match SSOT
7. **Benchmark Validation**: No regressions from baseline
8. **Documentation Review**: All SSOT documents current
9. **Changelog Updated**: All changes documented
10. **Version Bump**: Appropriate version increment

## Version Bumping Rules

| Change Type | Version Bump | Example |
|-------------|-------------|---------|
| Bug fix | Patch (0.0.x) | WAL replay fix |
| New feature | Minor (0.x.0) | New codec, new index |
| Breaking API change | Major (x.0.0) | Remove FFI function |
| Format change | Major (x.0.0) | Header layout change |
