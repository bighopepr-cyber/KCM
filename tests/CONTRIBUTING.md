# Contributing to tests/

> For core engine contribution rules, refer to the repository [CONTRIBUTING.md](../CONTRIBUTING.md).

## Overview

This document defines contribution standards for the `tests/` directory, which contains integration tests, SDK cross-language consistency tests, and supporting infrastructure. All contributions to this directory must follow the KCM engineering standards outlined in `AGENTS.md` and the parent `CONTRIBUTING.md`.

## Before Contributing

1. Read `AGENTS.md` for engineering constitution and non-negotiable rules
2. Read the parent `CONTRIBUTING.md` for repository-wide contribution standards
3. Review existing tests to understand patterns and conventions
4. Identify the affected test category (integration, SDK, property, security)
5. Verify no duplicate test exists for the same requirement

## Coding Standards

- All shell scripts must use `#!/usr/bin/env bash` with `set -euo pipefail`
- All Python scripts must follow PEP 8 style guidelines
- Test names must be descriptive and indicate what is being validated
- No hardcoded paths — use relative paths from script location
- No hardcoded ports — use configurable port variables
- All test output must be machine-parseable where applicable

## Module Architecture Rules

| Rule | Description |
|------|-------------|
| Single responsibility | Each test script validates one concern |
| Independence | Tests must not depend on execution order |
| Isolation | Tests must not share state or resources |
| Cleanup | Tests must remove all temporary artifacts |
| Idempotency | Tests must produce the same result on re-execution |

## Documentation Rules

- Each test script must have a header comment describing its purpose
- Complex test logic must have inline comments
- Test fixtures must document their expected input/output
- README files must be kept current with directory contents

## Testing Requirements

This directory IS the test directory. All tests written here must:

- Validate against SSOT specifications
- Use synthetic data (no production data)
- Handle errors without panicking or unwrapping
- Produce clear pass/fail output
- Be executable in CI environments
- Clean up after themselves

### Integration Tests

- Must exercise the full workspace build
- Must validate cross-crate interactions
- Must use the workspace root as working directory

### SDK Tests

- Must validate API surface against SSOT
- Must run identical test sequences across all SDKs
- Must use the mock server for isolation
- Must compare results for cross-language consistency

## Performance Rules

- Integration tests must complete within CI timeout limits
- SDK tests must not exceed mock server resource limits
- Tests must not introduce performance regressions in CI pipeline
- Long-running tests must be flagged and justified

## Review Checklist

- [ ] Test validates a documented requirement (SSOT traceability)
- [ ] Test uses synthetic data only
- [ ] Test cleans up temporary artifacts
- [ ] Test handles errors without unwrap/panic
- [ ] Test is independent of execution order
- [ ] Test follows naming conventions
- [ ] Test is documented (header comments, README if new module)
- [ ] No production secrets or credentials

## Pull Request Requirements

1. All existing tests must pass
2. New tests must validate the claimed requirement
3. Test coverage must meet or exceed 95%
4. No clippy warnings in Rust test code
5. No Python style violations in SDK test code
6. PR description must reference the SSOT requirement being validated

## References

- [Repository CONTRIBUTING.md](../CONTRIBUTING.md)
- [Engineering Constitution](../AGENTS.md)
- [Testing Strategy](../docs/PRD-TESTING&%20BRACHMARCK.md)
- [SDK Test README](sdk/README.md)
