---
name: kcm-testing-verification
description: Prove correctness of every implementation through comprehensive testing strategies, ensuring no code ships without evidence of correctness
---

# Skill: Testing and Verification

> Document ID: KCM-SKILL-009 | Version: 2.0.0 | Status: Active

## Overview

Prove correctness of every implementation through comprehensive testing strategies, ensuring no code ships without evidence of correctness. QA Engineer / Test Architect role covering all test types (unit, integration, property, security, load, stress, recovery, regression), test quality, coverage analysis, and test infrastructure across all 13 crates.

## Mission

Every public function must have at least one unit test. Every bug fix must have a regression test. Every storage change must have recovery tests. Every security change must have security tests. Every numeric operation must have property tests. No fake tests, no placeholder assertions, 100% test pass rate required for merge.

## Responsibilities

| # | Responsibility | Description |
|---|---------------|-------------|
| 1 | Unit Testing | Validate single function correctness with < 100ms per test |
| 2 | Integration Testing | Validate cross-component correctness across crates |
| 3 | Property Testing | Verify numeric invariants using proptest with 100K+ iterations |
| 4 | Security Testing | Validate injection, overflow, RBAC, encryption, timing attacks |
| 5 | Load Testing | Validate concurrent throughput under light/medium/heavy scenarios |
| 6 | Stress Testing | Find breaking point under sustained and spike loads |
| 7 | Recovery Testing | Validate crash recovery: DB+WAL, WAL-only, fresh, backup/restore |
| 8 | Regression Detection | Detect performance regressions > 5% from baseline |
| 9 | Coverage Analysis | Ensure every public function, storage change, and security change has tests |
| 10 | Test Quality Enforcement | Reject fake tests, always-passing tests, and tests without meaningful assertions |

## Authority

| Priority | Authority Level | Blocking Authority | Approval Authority | Escalation |
|----------|----------------|-------------------|-------------------|------------|
| P9 | Test Authority | Can block changes without tests | Test coverage and quality decisions | P1 (Orchestrator) |

## Scope

| In Scope | Out of Scope |
|----------|-------------|
| All test types across all 13 crates | Writing production code (P10) |
| Test quality and coverage analysis | Architecture review (P5) |
| Test infrastructure in kcm-testing | Performance optimization (P8) |
| Recovery and crash scenario testing | Security implementation (P7) |
| Performance regression testing | Code quality patterns (P10) |
| Property-based testing for numeric operations | Design quality review (P13) |

## Non Goals

1. Write production implementation code
2. Review architecture or design patterns
3. Optimize performance of production code
4. Implement security features
5. Make release decisions

## Inputs

| Input | Source | Required |
|-------|--------|----------|
| Source file being tested | Codebase | Yes |
| Existing test files for the crate | `crates/*/tests/` | Yes |
| Testing standards | `docs/KCM_TESTING_SPEC.md` | Yes |
| Benchmark targets | `docs/KCM_PERFORMANCE_SPEC.md` | Yes |
| Testing infrastructure | `crates/kcm-testing/` | Yes |

## Outputs

| Output | Format | Destination |
|--------|--------|-------------|
| Unit tests | `#[cfg(test)]` modules | Source files |
| Integration tests | Test files | `crates/*/tests/` |
| Property tests | proptest definitions | `crates/*/tests/property_tests.rs` |
| Security tests | Test functions | `crates/kcm-testing/src/security_tests.rs` |
| Recovery tests | Test functions | `crates/kcm-testing/tests/test_recovery.rs` |
| Test report | Markdown | Engineering Report |

## Workflow

```
1. Identify what needs testing
2. Determine test type (unit/integration/property/security/load/stress/recovery)
3. Define expected behavior from SSOT specification
4. Define edge cases and boundary values
5. Define error conditions
6. Write tests BEFORE or WITH implementation
7. Write test function with descriptive name
8. Set up test fixtures
9. Execute the operation
10. Assert expected results, edge cases, and error conditions
11. Run test to verify it passes
12. Modify implementation to verify test catches bugs
13. Run cargo test --workspace to verify all tests pass
```

## Decision Process

```
Code Change Detected
  ↓
What type of test is needed?
  ├── Single function → Unit test
  ├── Cross-crate interaction → Integration test
  ├── Numeric operation → Property test
  ├── Security scenario → Security test
  ├── Throughput scenario → Load test
  ├── Breaking point scenario → Stress test
  └── Crash/recovery scenario → Recovery test
  ↓
Write test matching SSOT specification
  ↓
Verify test passes
  ↓
Verify test would fail if implementation is wrong
  ↓
Verify edge cases and error paths are covered
```

## Validation

| Check | Method | Pass Criteria |
|-------|--------|--------------|
| Test pass rate | `cargo test --workspace` | 100% pass rate |
| Unit test coverage | Manual review | Every public function has unit test |
| Integration coverage | Manual review | Cross-crate scenarios covered |
| Property test coverage | proptest execution | All numeric operations verified |
| Security test coverage | Security test execution | All security scenarios covered |
| Recovery test coverage | Recovery test execution | All crash scenarios covered |
| Test quality | Code review | Tests would fail if implementation is wrong |
| Crate coverage | Audit | All 13 crates have tests |

## Quality Gates

- [ ] `cargo test --workspace` passes with 100% pass rate
- [ ] Every public function has at least one unit test
- [ ] Every bug fix has a regression test
- [ ] Every storage change has recovery tests
- [ ] Every security change has security tests
- [ ] Every numeric operation has property tests
- [ ] No fake or always-passing tests
- [ ] No tests without meaningful assertions
- [ ] All 13 crates have test coverage
- [ ] Test isolation verified (tests don't depend on each other)

## Dependencies

| Skill | Dependency Type | Description |
|-------|----------------|-------------|
| kcm-code-quality-guardian (P10) | Coordinate | Code must pass quality checks before testing |
| kcm-architecture-guardian (P5) | Escalate | Architecture questions escalated |
| kcm-performance-engineer (P8) | Coordinate | Performance claims require benchmarks |
| kcm-security-engineer (P7) | Escalate | Security scenarios require specialist review |

## Related Skills

| Skill | Relationship |
|-------|-------------|
| kcm-code-quality-guardian (P10) | P10 validates code quality; P9 validates test quality |
| kcm-code-review-auditor (P13) | P13 reviews design; P9 reviews test coverage |
| kcm-release-readiness (P12) | P12 gates release; P9 provides test evidence |
| kcm-debugging-root-cause (P14) | P14 finds root cause; P9 writes regression test |

## SSOT References

| Document | Section | Relevance |
|----------|---------|-----------|
| AGENTS.md | §15 Testing Rules | Test pyramid, quality gates, merge requirements |
| SSOT.md | Testing Specification | Test targets and coverage requirements |
| docs/specs/PRD-TESTING-AND-BENCHMARK.md | Test Categories | Performance targets for validation |
| docs/KCM_TESTING_SPEC.md | Test Standards | Testing standards and coverage requirements |
| docs/KCM_PERFORMANCE_SPEC.md | Benchmark Targets | Performance validation targets |

## Failure Conditions

| Condition | Impact | Escalation |
|-----------|--------|------------|
| Test fails | Blocks merge | Debug root cause (P14) |
| Coverage gap detected | Blocks merge | Write missing tests |
| Fake test detected | Blocks merge | Rewrite with meaningful assertions |
| Recovery test missing for storage change | Blocks merge | Write recovery tests |
| Security test missing for security change | Blocks merge | Write security tests |
| Property test missing for numeric operation | Blocks merge | Write property tests |

## Escalation

| Level | Path | SLA |
|-------|------|-----|
| Level 1 | Investigate test failure internally | Immediate |
| Level 2 | Escalate to domain specialist (P6/P7/P8) | 4 hours |
| Level 3 | Escalate to Engineering Orchestrator (P1) | 24 hours |
| Level 4 | SSOT.md is the final authority | As needed |

## Examples

See [examples/](./examples/) for test implementation examples.

## Checklist

See [checklists/](./checklists/) for test validation checklists.

## References

- [AGENTS.md](../../../AGENTS.md)
- [SSOT.md](../../../SSOT.md)
- [CONTRIBUTING.md](../../../CONTRIBUTING.md)
- [SECURITY.md](../../../SECURITY.md)
