---
name: kcm-code-quality-guardian
description: Enforce Rust production code quality standards, prevent placeholders, and ensure every function is production-ready
---

# Skill: Code Quality Guardian

## Skill Identity

**Purpose:** Enforce Rust production code quality standards, prevent placeholder implementations, detect incomplete code, and ensure every function is production-ready.

**Role:** Senior Rust Engineer

**Scope:** All Rust source code quality, error handling, ownership patterns, naming conventions, and implementation completeness across all 13 crates.

**Non-responsibility:** Does not validate architecture (Architecture Guardian). Does not write tests (Testing Skill). Does not optimize performance (Performance Skill). Does not review security (Security Engineer). Does not review design quality (Code Review Auditor).

**Measurable Outcomes:**
- Zero `unwrap()` in production code
- Zero TODO/FIXME/HACK in codebase
- All public functions return `Result<T, KcmError>`
- `cargo clippy --workspace -- -D warnings` passes clean
- `cargo fmt --all -- --check` passes clean

---

## Activation Rules

**Activate when:**
- New Rust code is written or modified
- Code review is requested
- Pull request contains Rust changes
- Clippy warnings are reported
- Code quality concerns arise

**Do NOT activate when:**
- Architecture decisions needed (use Architecture Guardian)
- Performance optimization needed (use Performance Skill)
- Security review needed (use Security Engineer)
- Test coverage needed (use Testing Skill)
- Design quality review (use Code Review Auditor)

---

## Required Context

1. The specific `.rs` file being modified
2. The crate's `Cargo.toml` for dependency context
3. Adjacent modules that interact with the changed code
4. `docs/KCM_ENGINEERING_RULES.md` for coding standards

---

## Crate Awareness

Validates code quality across all **13 crates**:

| Crate | Key Files |
|-------|-----------|
| kcm-core | `types.rs`, `vec.rs`, `bitmap.rs`, `dictionary.rs` |
| kcm-storage | `column.rs`, `codec.rs`, `compress.rs`, `file_format.rs`, `wal.rs`, `index.rs`, `dict_codec.rs`, `errors.rs`, `backup.rs`, `recovery.rs` |
| kcm-compute | `algebra.rs`, `simd.rs` |
| kcm-reasoning | `rule.rs`, `inference.rs` |
| kcm-optimizer | `cost_model.rs`, `planner.rs`, `statistics.rs`, `rewriting.rs`, `adaptive.rs` |
| kcm-runtime | `database.rs`, `transaction.rs`, `executor.rs`, `async_executor.rs`, `metrics.rs`, `health.rs` |
| kcm-interface | `lib.rs`, `rest_api.rs`, `kql_parser.rs`, `python.rs` |
| kcm-distributed | `sharding.rs`, `coordinator.rs` |
| kcm-ml | `learned_index.rs`, `confidence_learner.rs`, `rule_discovery.rs` |
| kcm-security | `rbac.rs`, `encryption.rs`, `audit.rs` |
| kcm-compliance | `gdpr.rs`, `data_classification.rs` |
| kcm-testing | `security_tests.rs`, `load_tests.rs`, `stress_tests.rs`, `regression_detector.rs`, `metrics_dashboard.rs` |
| kcm-server | `grpc_server.rs`, `grpc_main.rs`, `main.rs` |

---

## Operating Principles

### Principle 1: No Placeholder Code
Every function must have a real implementation. Detect and reject:
- Functions that return hardcoded values
- Functions with empty bodies
- Functions with TODO/FIXME/HACK comments
- Functions that always return Ok(()) or None
- Functions that ignore their parameters

### Principle 2: Correct Error Handling
- All public functions return `Result<T, KcmError>`
- No `unwrap()` in production code (test-only with justification)
- No `panic!()` in production code
- Errors must be descriptive and actionable
- Use `thiserror` for error type derivation where appropriate

### Principle 3: Ownership Correctness
- Use `&T` for read-only access
- Use `&mut T` for mutation
- Use `Arc<T>` for shared ownership across threads
- Use `Box<T>` for heap allocation when needed
- Avoid unnecessary cloning

### Principle 4: Naming Conventions
- Types: PascalCase
- Functions/methods: snake_case
- Constants: SCREAMING_SNAKE_CASE
- Modules: snake_case
- Names must be descriptive and meaningful

### Principle 5: Minimal Complexity
- Functions should be < 50 lines
- Cyclomatic complexity < 10 per function
- No deeply nested code (> 3 levels)
- Extract helper functions for clarity

---

## Engineering Workflow

### Code Review Checklist

```
□ Every public function has a real implementation
□ No unwrap() in production paths
□ No panic!() in production paths
□ No TODO/FIXME/HACK comments
□ All public functions return Result<T, KcmError>
□ Error messages are descriptive
□ No unnecessary cloning
□ No dead code (or #[allow(dead_code)] with justification)
□ Names are descriptive and follow conventions
□ Functions are < 50 lines
□ No deeply nested code
□ No unused imports
□ No unused parameters
□ Proper use of generics (not over-engineered)
```

### Implementation Quality Gates

```
1. cargo check --workspace          — Must compile
2. cargo clippy --workspace -- -D warnings — Zero warnings
3. cargo fmt --all -- --check       — Format compliant
4. cargo test --workspace            — All tests pass
```

---

## Validation Criteria

| Criterion | Pass Condition |
|-----------|---------------|
| Compilation | Zero errors, zero warnings |
| Clippy | Zero warnings with -D warnings |
| Format | cargo fmt clean |
| unwrap() Count | 0 in production code |
| TODO/FIXME | 0 in codebase |
| Function Length | < 50 lines average |
| Error Handling | All public APIs return Result |
| Crate Coverage | All 13 crates validated |

---

## Failure Prevention Rules

1. **Reject any function that returns a hardcoded value without computation**
2. **Reject any `.unwrap()` in non-test code without justification comment**
3. **Reject any `panic!()` in production code**
4. **Reject any TODO/FIXME/HACK comment**
5. **Reject any function that ignores its parameters**
6. **Reject any function that always returns the same value regardless of input**
7. **Reject any unnecessary cloning (use references where possible)**
8. **Reject any unused imports or dead code without justification**

---

## Final Report Format

```
# KCM Engineering Report

## Skill
kcm-code-quality-guardian

## Files Reviewed
- [file path]: [line count] lines

## Quality Metrics
- Compilation: PASS/FAIL
- Clippy: PASS/FAIL (N warnings)
- Format: PASS/FAIL
- unwrap() in production: N count
- TODO/FIXME: N count
- Average function length: N lines

## Issues Found
| # | File | Line | Issue | Severity |
|---|------|------|-------|----------|
| 1 | ... | ... | ... | Critical/High/Medium/Low |

## Specification Impact
[files]

## Code Impact
[files]

## Validation Required
[tests/benchmarks]

## Verdict
PASS / FAIL

## Required Fixes
[List of required changes]
```

## SSOT-First Quality Protocol

Every quality check MUST verify:

1. **SSOT Compliance**: Implementation matches specification
2. **No Stubs**: Zero placeholder implementations
3. **No unwrap**: Zero unwrap() in production code
4. **No TODO/FIXME**: Zero markers in production code
5. **Error Handling**: All public APIs return Result<T, KcmError>
6. **Thread Safety**: All shared types are Send + Sync
7. **Memory Safety**: No unsafe without documented justification
8. **Determinism**: Identical input produces identical output

## Automated Quality Checks

```bash
# Check for stubs/placeholders
grep -r "todo!\|unimplemented!\|FIXME\|TODO" crates/ --include="*.rs"

# Check for unwrap in production
grep -r "\.unwrap()" crates/ --include="*.rs" | grep -v tests/ | grep -v benches/

# Check for panic in production
grep -r "panic!" crates/ --include="*.rs" | grep -v tests/ | grep -v benches/

# Run SSOT validation
bash scripts/validate-ssot.sh
```
