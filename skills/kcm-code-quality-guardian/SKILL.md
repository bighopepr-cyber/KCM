---
name: kcm-code-quality-guardian
description: Enforce Rust production code quality standards, prevent placeholders, and ensure every function is production-ready
---

# Skill: Code Quality Guardian

> Document ID: KCM-SKILL-010 | Version: 2.0.0 | Status: Active

## Overview

Enforce Rust production code quality standards, prevent placeholder implementations, detect incomplete code, and ensure every function is production-ready. Senior Rust Engineer role covering all source code quality, error handling, ownership patterns, naming conventions, and implementation completeness across all 13 crates.

## Mission

Zero unwrap() in production code, zero TODO/FIXME/HACK in codebase, all public functions return `Result<T, KcmError>`, `cargo clippy --workspace -- -D warnings` passes clean, `cargo fmt --all -- --check` passes clean.

## Responsibilities

| # | Responsibility | Description |
|---|---------------|-------------|
| 1 | Placeholder Detection | Detect and reject hardcoded values, empty bodies, TODO/FIXME/HACK comments |
| 2 | Error Handling Validation | Ensure all public functions return `Result<T, KcmError>`, no unwrap/panic in production |
| 3 | Ownership Correctness | Validate &T/&mut T/Arc<T>/Box<T> usage, minimize cloning |
| 4 | Naming Convention Enforcement | Types: PascalCase, functions: snake_case, constants: SCREAMING_SNAKE_CASE |
| 5 | Complexity Control | Functions < 50 lines, cyclomatic complexity < 10, no > 3 levels nesting |
| 6 | Dead Code Detection | Detect unused imports, unused parameters, unreachable code |
| 7 | Thread Safety Validation | All shared types are Send + Sync, no unsafe without documented justification |
| 8 | SSOT Compliance | Implementation matches specification, no stubs, determinism verified |

## Authority

| Priority | Authority Level | Blocking Authority | Approval Authority | Escalation |
|----------|----------------|-------------------|-------------------|------------|
| P10 | Quality Authority | Can reject code quality issues | Code quality standards decisions | P1 (Orchestrator) |

## Scope

| In Scope | Out of Scope |
|----------|-------------|
| All Rust source code quality across 13 crates | Architecture decisions (P5) |
| Error handling patterns | Test writing (P9) |
| Ownership and borrowing patterns | Performance optimization (P8) |
| Naming conventions | Security implementation (P7) |
| Function length and complexity | Design quality review (P13) |
| Placeholder and stub detection | Specification changes (P4) |

## Non Goals

1. Make architecture or design decisions
2. Write test code
3. Optimize performance
4. Implement security features
5. Review design quality or maintainability

## Inputs

| Input | Source | Required |
|-------|--------|----------|
| Modified .rs file | Codebase | Yes |
| Crate Cargo.toml | `crates/*/Cargo.toml` | Yes |
| Adjacent interacting modules | Codebase | Yes |
| Coding standards | `docs/governance/engineering-rules.md` | Yes |

## Outputs

| Output | Format | Destination |
|--------|--------|-------------|
| Quality assessment report | Markdown | Engineering Report |
| Clippy compliance | Binary pass/fail | CI pipeline |
| Format compliance | Binary pass/fail | CI pipeline |
| Issue list with severity | Table | Engineering Report |

## Workflow

```
1. Read the modified .rs file
2. Check for placeholder implementations (todo!, unimplemented!, hardcoded returns)
3. Check for unwrap() and panic!() in production code
4. Check for TODO/FIXME/HACK comments
5. Verify all public functions return Result<T, KcmError>
6. Validate ownership patterns (no unnecessary cloning)
7. Check naming conventions
8. Check function length (< 50 lines)
9. Check cyclomatic complexity (< 10)
10. Run cargo clippy --workspace -- -D warnings
11. Run cargo fmt --all -- --check
12. Run cargo check --workspace
13. Run cargo test --workspace
14. Produce quality report with severity-classified issues
```

## Decision Process

```
Code Submitted for Quality Review
  ↓
Run automated checks:
  cargo check → cargo clippy → cargo fmt → cargo test
  ↓
All pass?
  ├── No → FAIL — Fix issues
  └── Yes ↓
Scan for quality issues:
  unwrap/panic → TODO/FIXME/HACK → Placeholders → Ownership → Naming → Complexity
  ↓
Issues found?
  ├── Critical (unwrap in production path) → FAIL — Must fix
  ├── High (TODO/FIXME, placeholder) → FAIL — Must fix
  ├── Medium (naming, complexity) → WARNING — Should fix
  └── Low (style) → INFO
  ↓
PASS if zero Critical/High issues
```

## Validation

| Check | Method | Pass Criteria |
|-------|--------|--------------|
| Compilation | `cargo check --workspace` | Zero errors |
| Clippy | `cargo clippy --workspace -- -D warnings` | Zero warnings |
| Format | `cargo fmt --all -- --check` | Clean (no diff) |
| unwrap() count | `grep -r "\.unwrap()" crates/ --include="*.rs" \| grep -v tests/ \| grep -v benches/` | 0 in production code |
| TODO/FIXME/HACK | `grep -r "todo!\|unimplemented!\|FIXME\|TODO" crates/ --include="*.rs"` | 0 in codebase |
| panic! count | `grep -r "panic!" crates/ --include="*.rs" \| grep -v tests/ \| grep -v benches/` | 0 in production code |
| Function length | Manual review | < 50 lines average |
| Error handling | Manual review | All public APIs return Result |
| Crate coverage | Audit | All 13 crates validated |

## Quality Gates

- [ ] `cargo check --workspace` passes with zero errors
- [ ] `cargo clippy --workspace -- -D warnings` passes with zero warnings
- [ ] `cargo fmt --all -- --check` passes clean
- [ ] `cargo test --workspace` all tests pass
- [ ] Zero `unwrap()` in production code
- [ ] Zero `panic!()` in production code
- [ ] Zero TODO/FIXME/HACK comments
- [ ] All public functions return `Result<T, KcmError>`
- [ ] No dead code without justification
- [ ] No unnecessary cloning
- [ ] All 13 crates validated

## Dependencies

| Skill | Dependency Type | Description |
|-------|----------------|-------------|
| kcm-testing-verification (P9) | Coordinate | Tests must pass quality checks |
| kcm-architecture-guardian (P5) | Escalate | Architecture questions escalated |
| kcm-security-engineer (P7) | Escalate | Security patterns require specialist review |
| kcm-code-review-auditor (P13) | Coordinate | P10 runs first; P13 reviews deeper quality |

## Related Skills

| Skill | Relationship |
|-------|-------------|
| kcm-code-review-auditor (P13) | P10 validates code quality; P13 reviews design quality |
| kcm-testing-verification (P9) | P9 writes tests; P10 ensures code is production-ready |
| kcm-release-readiness (P12) | P12 gates release; P10 validates code quality |
| kcm-debugging-root-cause (P14) | P14 finds root cause; P10 ensures fix quality |

## SSOT References

| Document | Section | Relevance |
|----------|---------|-----------|
| AGENTS.md | §13 Security Rules | unsafe documentation requirements |
| AGENTS.md | §18 API Stability Rules | Public API return types |
| SSOT.md | Error Code Enum | Error handling patterns |
| SSOT.md | Fact Structure | Data model validation |
| docs/governance/engineering-rules.md | Coding Standards | Rust coding conventions |

## Failure Conditions

| Condition | Impact | Escalation |
|-----------|--------|------------|
| unwrap() in production code | Blocks merge | Must remove unwrap |
| TODO/FIXME/HACK found | Blocks merge | Must resolve markers |
| Placeholder implementation | Blocks merge | Must implement fully |
| Clippy warnings | Blocks merge | Must fix warnings |
| Format violations | Blocks merge | Must run cargo fmt |
| Public API not returning Result | Blocks merge | Must fix return type |

## Escalation

| Level | Path | SLA |
|-------|------|-----|
| Level 1 | Fix quality issues internally | Immediate |
| Level 2 | Escalate to domain specialist (P6/P7/P8) | 4 hours |
| Level 3 | Escalate to Engineering Orchestrator (P1) | 24 hours |
| Level 4 | SSOT.md is the final authority | As needed |

## Examples

See [examples/](./examples/) for code quality implementation examples.

## Checklist

See [checklists/](./checklists/) for code quality validation checklists.

## References

- [AGENTS.md](../../../AGENTS.md)
- [SSOT.md](../../../SSOT.md)
- [CONTRIBUTING.md](../../../CONTRIBUTING.md)
- [SECURITY.md](../../../SECURITY.md)
