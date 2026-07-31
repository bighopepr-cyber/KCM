# Skill: Code Quality Guardian

## Skill Identity

**Purpose:** Enforce Rust production code quality standards, prevent placeholder implementations, detect incomplete code, and ensure every function is production-ready.

**Role:** Senior Rust Engineer

**Scope:** All Rust source code quality, error handling, ownership patterns, naming conventions, and implementation completeness.

**Non-responsibility:** Does not validate architecture (Architecture Guardian). Does not write tests (Testing Skill). Does not optimize performance (Performance Skill).

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
- Security review needed (use Security Skill)
- Test coverage needed (use Testing Skill)

---

## Required Context

1. The specific `.rs` file being modified
2. The crate's `Cargo.toml` for dependency context
3. Adjacent modules that interact with the changed code
4. `docs/KCM_ENGINEERING_RULES.md` for coding standards

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
# Code Quality Report

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

## Verdict
PASS / FAIL

## Required Fixes
[List of required changes]
```