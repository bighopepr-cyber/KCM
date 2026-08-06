# Contributing to tests/sdk/

> For core engine contribution rules, refer to the repository [CONTRIBUTING.md](../../CONTRIBUTING.md).

## Overview

This document defines contribution standards for the `tests/sdk/` directory, which contains the mock server, cross-language consistency tests, API validation scripts, and supporting infrastructure. All contributions must follow the KCM engineering standards outlined in `AGENTS.md` and the parent `CONTRIBUTING.md`.

## Before Contributing

1. Read `AGENTS.md` for engineering constitution and non-negotiable rules
2. Read the parent `CONTRIBUTING.md` for repository-wide contribution standards
3. Read `tests/sdk/README.md` for SDK test directory documentation
4. Review existing tests to understand patterns and conventions
5. Verify no duplicate test exists for the same requirement

## Coding Standards

### Python Style

- Follow PEP 8 style guidelines
- Use type hints for all function signatures
- Maximum line length: 100 characters
- Use `snake_case` for functions and variables
- Use `PascalCase` for classes
- Use `UPPER_SNAKE_CASE` for constants

### Test Naming

- Test functions: `test_<what_is_tested>()`
- Test classes: `Test<WhatIsTested>`
- Test files: `<module>_test.py` or `test_<module>.py`
- Test descriptions must indicate what is being validated

## Module Architecture Rules

| Rule | Description |
|------|-------------|
| Single responsibility | Each test script validates one concern |
| Independence | Tests must not depend on execution order |
| Isolation | Tests must not share state or resources |
| Cleanup | Tests must remove all temporary artifacts |
| Idempotency | Tests must produce the same result on re-execution |

## Documentation Rules

- Each test script must have a module-level docstring describing its purpose
- Complex test logic must have inline comments
- Test fixtures must document their expected input/output
- README files must be kept current with directory contents

## Testing Requirements

- All new tests must validate against SSOT specifications
- Tests must use synthetic data (no production data)
- Tests must handle errors without raising unhandled exceptions
- Tests must produce clear pass/fail output
- Tests must be executable in CI environments
- Tests must clean up after themselves

### Mock Server Contributions

- New endpoints must follow the KCM API specification
- Endpoints must bind to localhost only
- Endpoints must handle errors gracefully
- Endpoints must validate input parameters

### Cross-Language Test Contributions

- New test cases must be added to `consistency_matrix.json`
- Tests must be runnable across all SDK implementations
- Test results must be comparable across SDKs

## Performance Rules

- Mock server startup must complete within 2 seconds
- Test suites must complete within CI timeout limits
- Tests must not consume excessive memory or CPU
- Long-running tests must be flagged and justified

## Review Checklist

- [ ] Test validates a documented requirement (SSOT traceability)
- [ ] Test uses synthetic data only
- [ ] Test cleans up temporary artifacts
- [ ] Test handles errors without unwrap/panic
- [ ] Test is independent of execution order
- [ ] Test follows Python style guidelines (PEP 8)
- [ ] Test is documented (docstrings, README if new module)
- [ ] No production secrets or credentials
- [ ] Mock server binds to localhost only
- [ ] Consistency matrix updated if new test case added

## Pull Request Requirements

1. All existing tests must pass
2. New tests must validate the claimed requirement
3. Python code must pass linting (pylint/flake8)
4. No style violations in test code
5. PR description must reference the SSOT requirement being validated
6. consistency_matrix.json must be updated for new test cases

## References

- [Repository CONTRIBUTING.md](../../CONTRIBUTING.md)
- [Engineering Constitution](../../AGENTS.md)
- [SDK Test README](README.md)
- [Testing Strategy](../../docs/PRD-TESTING&%20BRACHMARCK.md)
- [KCM API Specification](../../docs/specs/KCM_API_SPEC.md)
