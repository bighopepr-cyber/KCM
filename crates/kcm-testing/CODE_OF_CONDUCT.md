# kcm-testing Community Guidelines

> **Note:** This document is crate-specific. The project-wide code of conduct is in the repository root at [`CODE_OF_CONDUCT.md`](../../CODE_OF_CONDUCT.md).

## Respect

All contributors to kcm-testing are expected to treat each other with respect and professionalism. This crate is part of a larger engineering system — contributions here affect the reliability of the entire KCM testing infrastructure.

- Respect differing technical opinions and approaches.
- Acknowledge that testing infrastructure is foundational work.
- Give credit when building on others' test patterns or fixtures.

## Professional Communication

- Use clear, technical language in issues, PRs, and code reviews.
- Provide evidence for claims about test behavior (reproduction steps, logs, metrics).
- When reporting test failures, include: scenario name, configuration, expected vs actual results.
- When suggesting test changes, explain the rationale and expected impact.

## Code Review Etiquette

- Review test code with the same rigor as production code.
- Focus on: determinism, resource management, assertion correctness, and edge cases.
- Suggest improvements constructively — test code benefits from clarity over cleverness.
- Verify that test fixtures are deterministic and reproducible.
- Check that temporary resources are properly cleaned up.

## Collaboration

- kcm-testing is shared infrastructure. Changes affect all crates that run tests.
- Coordinate cross-crate test changes via PRs that touch both crates.
- Share benchmark fixtures and test utilities — avoid duplicating test data generation.
- Document any testing patterns that deviate from existing conventions.

## Reporting Issues

- Report test flakiness immediately — flaky tests undermine the entire testing pipeline.
- Include reproduction steps, environment details, and failure frequency.
- For security-related test issues, follow the root [SECURITY.md](../../SECURITY.md) process.
- For performance regressions detected by `RegressionDetector`, include baseline and current metrics.

## Enforcement

This crate follows the enforcement procedures defined in the root [CODE_OF_CONDUCT.md](../../CODE_OF_CONDUCT.md). Violations are handled by the project maintainers per the Microsoft Open Source Code of Conduct.

## References

- [Root CODE_OF_CONDUCT.md](../../CODE_OF_CONDUCT.md)
- [Root CONTRIBUTING.md](../../CONTRIBUTING.md)
- [AGENTS.md](../../AGENTS.md)
- [Microsoft Open Source Code of Conduct](https://opensource.microsoft.com/codeofconduct/)
