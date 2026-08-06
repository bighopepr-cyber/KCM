# tests/sdk/ Community Guidelines

> For project-wide community guidelines, refer to the [CODE_OF_CONDUCT.md](../../CODE_OF_CONDUCT.md) located in the repository root.

## Respect

All contributors to the `tests/sdk/` directory must treat fellow engineers with respect. This includes:

- Respecting different technical perspectives on SDK testing approaches
- Acknowledging the effort required to maintain cross-language consistency
- Providing constructive feedback on test quality and coverage
- Not dismissing SDK test concerns as "just tests"

## Professional Communication

- Use clear, technical language in test documentation and PR descriptions
- Provide evidence when claiming a test is necessary (reference SSOT requirements)
- Report test failures with full context (SDK version, environment, steps to reproduce)
- Acknowledge when a test issue is a real bug vs a test infrastructure problem

## Code Review Etiquette

- Review test code with the same rigor as production code
- Focus on correctness, not personal preference
- Suggest improvements with rationale (not just "change this")
- Acknowledge good test design and cross-language consistency coverage
- Block PRs that violate engineering standards (no unwrap, no production data, etc.)

## Collaboration

- Share testing patterns and best practices across SDK teams
- Help maintain cross-language consistency tests when adding new SDKs
- Contribute to mock server improvements
- Review and update test documentation when specifications change

## Reporting Issues

- Report test failures with actionable information
- Include SDK version, Python version, and environment details
- Provide minimal reproduction steps
- Tag relevant maintainers for security-related test issues

## Enforcement

Violations of these guidelines will be addressed through the same mechanisms as the project-wide `CODE_OF_CONDUCT.md`. Repeated violations may result in restricted access to test infrastructure.

## References

- [Repository Code of Conduct](../../CODE_OF_CONDUCT.md)
- [tests/ Community Guidelines](../CODE_OF_CONDUCT.md)
- [Engineering Constitution](../../AGENTS.md)
- [Repository CONTRIBUTING.md](../../CONTRIBUTING.md)
