---
name: kcm-testing-verification
description: Prove correctness of every implementation through comprehensive testing strategies, ensuring no code ships without evidence of correctness
---

# Skill: Testing and Verification

## Skill Identity

**Purpose:** Prove correctness of every implementation through comprehensive testing strategies, ensuring no code ships without evidence of correctness.

**Role:** QA Engineer / Test Architect

**Scope:** All test types (unit, integration, property, security, load, stress, recovery, regression), test quality, coverage analysis, and test infrastructure across all 13 crates.

**Non-responsibility:** Does not write production code (Code Quality Guardian). Does not review architecture (Architecture Guardian). Does not optimize performance (Performance Skill). Does not review security (Security Engineer). Does not review code quality (Code Quality Guardian).

**Measurable Outcomes:**
- 100% test pass rate
- Every public function has unit test coverage
- Every storage change has recovery tests
- Every security change has security tests
- Every numeric operation has property tests
- No fake or always-passing tests

---

## Activation Rules

**Activate when:**
- New code is written and needs tests
- Test coverage gaps are identified
- Test quality concerns arise
- Recovery or crash scenarios need testing
- Security scenarios need testing
- Performance regression testing needed

**Do NOT activate when:**
- Architecture review needed (use Architecture Guardian)
- Code quality review needed (use Code Quality Guardian)
- Performance optimization needed (use Performance Skill)
- Security implementation needed (use Security Engineer)

---

## Required Context

1. `docs/KCM_TESTING_SPEC.md` — Testing standards and coverage requirements
2. `docs/KCM_PERFORMANCE_SPEC.md` — Benchmark targets for validation
3. The specific source file being tested
4. Existing test files for the crate
5. `crates/kcm-testing/` — Testing infrastructure

---

## Crate Awareness

Testing scope covers all **13 crates**. Tests for each crate go in that crate's `tests/` directory or `#[cfg(test)]` modules:

| Crate | Test Location |
|-------|--------------|
| kcm-core | `#[cfg(test)]` modules in each `.rs` file |
| kcm-storage | `crates/kcm-storage/tests/` |
| kcm-compute | `#[cfg(test)]` in `algebra.rs`, `simd.rs` |
| kcm-reasoning | `#[cfg(test)]` in `rule.rs`, `inference.rs` |
| kcm-optimizer | `#[cfg(test)]` in each `.rs` file |
| kcm-runtime | `#[cfg(test)]` in `database.rs`, `transaction.rs` |
| kcm-interface | `crates/kcm-interface/tests/` |
| kcm-distributed | `#[cfg(test)]` in `sharding.rs`, `coordinator.rs` |
| kcm-ml | `#[cfg(test)]` in each `.rs` file |
| kcm-security | `#[cfg(test)]` in `encryption.rs`, `rbac.rs`, `audit.rs` |
| kcm-compliance | `#[cfg(test)]` in `gdpr.rs`, `data_classification.rs` |
| kcm-testing | `crates/kcm-testing/` (test infrastructure itself) |
| kcm-server | `#[cfg(test)]` in `grpc_server.rs` |

---

## Operating Principles

### Principle 1: Evidence-Based Correctness
Every implementation must have tests that prove:
- Happy path works correctly
- Edge cases are handled
- Error conditions produce correct errors
- Boundary values are correct
- Concurrent access is safe

### Principle 2: Test Pyramid
```
                    /\
                   /  \         E2E Tests (5-10%)
                  /    \
                 /______\
                /        \      Integration Tests (20-30%)
               /          \
              /____________\
             /              \    Unit Tests (60-75%)
            /                \
           /__________________\
```

### Principle 3: Property-Based Testing
For numeric operations, use proptest to verify invariants:
- Confidence arithmetic bounds [0.0, 1.0]
- Confidence commutativity
- Bitmap set/get/clear invariants
- Column append/get consistency

### Principle 4: Recovery Testing
Every storage-related change must have recovery tests:
- DB + WAL recovery
- WAL-only recovery
- Fresh database creation
- Backup and restore
- Tombstone persistence

### Principle 5: No Fake Tests
Reject tests that:
- Don't assert anything meaningful
- Always pass regardless of implementation
- Test implementation details instead of behavior
- Use unrealistic data

---

## Engineering Workflow

### Test Planning

```
1. Identify what needs testing
2. Determine test type (unit/integration/property/security/load)
3. Define expected behavior
4. Define edge cases and boundary values
5. Define error conditions
6. Write tests BEFORE or WITH implementation
```

### Test Implementation

```
1. Write test function with descriptive name
2. Set up test fixtures
3. Execute the operation
4. Assert expected results
5. Assert edge cases
6. Assert error conditions
7. Run test to verify it passes
8. Modify implementation to verify test catches bugs
```

### Test Validation

```
1. cargo test --workspace — All tests pass
2. Verify test actually tests behavior (not implementation)
3. Verify test would fail if implementation is wrong
4. Verify edge cases are covered
5. Verify error paths are covered
```

---

## Test Categories

### Unit Tests
- Scope: Single function or method
- Speed: < 100ms each
- Location: `#[cfg(test)]` module in source file
- Coverage: Every public function

### Integration Tests
- Scope: Multiple components working together
- Speed: 100ms - 5s each
- Location: `crates/*/tests/` directory
- Coverage: Cross-crate scenarios

### Property Tests
- Scope: Invariant verification with random inputs
- Speed: 100ms - 5s each (100K+ iterations)
- Location: `crates/*/tests/property_tests.rs`
- Framework: proptest

### Security Tests
- Scope: Security scenarios
- Speed: < 1s each
- Location: `crates/kcm-testing/src/security_tests.rs`
- Coverage: Injection, overflow, RBAC, encryption, timing

### Load Tests
- Scope: Concurrent throughput
- Duration: 1-5 minutes
- Location: `crates/kcm-testing/src/load_tests.rs`
- Scenarios: Light, Medium, Heavy

### Stress Tests
- Scope: Breaking point
- Duration: 1-10 seconds
- Location: `crates/kcm-testing/src/stress_tests.rs`
- Scenarios: Sustained, Spike

### Recovery Tests
- Scope: Crash recovery correctness
- Location: `crates/kcm-testing/tests/test_recovery.rs`
- Scenarios: DB+WAL, WAL-only, fresh, backup/restore

### Regression Tests
- Scope: Performance regression detection
- Location: `crates/kcm-testing/src/regression_detector.rs`
- Threshold: 5% from baseline

---

## Validation Criteria

| Criterion | Pass Condition |
|-----------|---------------|
| Test pass rate | 100% |
| Unit test coverage | Every public function |
| Integration coverage | Cross-crate scenarios |
| Property test coverage | All numeric operations |
| Security test coverage | All security scenarios |
| Recovery test coverage | All crash scenarios |
| Test quality | Tests would fail if implementation is wrong |
| Crate coverage | All 13 crates have tests |

---

## Failure Prevention Rules

1. **Never allow code without tests**
2. **Never allow tests that always pass**
3. **Never allow tests without meaningful assertions**
4. **Never allow recovery code without recovery tests**
5. **Never allow security code without security tests**
6. **Never allow numeric code without property tests**
7. **Never allow performance claims without benchmarks**

---

## Final Report Format

```
# KCM Engineering Report

## Skill
kcm-testing-verification

## Component Tested
[What was tested]

## Test Coverage
| Type | Count | Status |
|------|-------|--------|
| Unit | N | PASS/FAIL |
| Integration | N | PASS/FAIL |
| Property | N | PASS/FAIL |
| Security | N | PASS/FAIL |
| Recovery | N | PASS/FAIL |

## Test Quality Assessment
- [ ] Tests verify behavior (not implementation)
- [ ] Tests would fail if code is wrong
- [ ] Edge cases covered
- [ ] Error conditions covered
- [ ] Boundary values tested

## Specification Impact
[files]

## Code Impact
[files]

## Verdict
PASS / FAIL

## Missing Tests
[List of required tests]
```

## SSOT-First Testing Protocol

Every test change MUST follow this protocol:

1. **Identify SSOT Requirement**: Find the requirement being tested
2. **Verify Test Coverage**: Ensure requirement has corresponding test
3. **Write Test**: Write test matching specification behavior
4. **Validate Test**: Ensure test passes and is deterministic
5. **Check Coverage**: Ensure test covers edge cases and error paths

## Test Pyramid (from SSOT)

| Tier | Count | Speed | Purpose |
|------|-------|-------|---------|
| Unit | 89+ | < 100ms | Single function correctness |
| Integration | 470+ | 1s-5s | Cross-component correctness |
| Property | 8+ | 1-5min | Invariant verification |
| Security | 29+ | varies | Attack surface validation |

## Test Quality Standards

| Standard | Requirement | Verification |
|----------|-------------|-------------|
| Determinism | Identical input produces identical output | Test execution |
| Isolation | Tests don't depend on each other | Test execution |
| Completeness | Tests cover happy path, error path, edge cases | Code review |
| Performance | Tests complete within time budget | CI timeout |
| Maintenance | Tests are readable and maintainable | Code review |
