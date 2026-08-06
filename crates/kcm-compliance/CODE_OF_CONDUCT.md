# kcm-compliance Community Guidelines

Community guidelines specific to contributions in the `kcm-compliance` crate.

> For project-wide community guidelines, refer to the [CODE_OF_CONDUCT.md](../../CODE_OF_CONDUCT.md) located in the repository root.

## Respect

- Treat all contributors with respect regardless of experience level
- Acknowledge that compliance is critical — errors here have regulatory consequences
- Be patient with contributors learning GDPR requirements
- Value correctness and regulatory alignment over implementation speed

## Professional Communication

- Use clear, technical language in issues and PRs
- Provide evidence for claims about compliance correctness
- Reference GDPR articles and SSOT specifications when discussing design decisions
- Avoid emotional reactions to review feedback on compliance-critical code

## Code Review Etiquette

- Review `kcm-compliance` changes with extra care — they affect regulatory compliance
- Focus on consent lifecycle correctness, classification accuracy, and error handling
- Suggest alternatives rather than just rejecting
- Acknowledge thorough test coverage for compliance scenarios

## Collaboration

- Coordinate with other contributors on consent lifecycle changes
- Share GDPR article references when proposing compliance features
- Document design decisions in ADRs when appropriate
- Help maintain backward compatibility for public compliance APIs

## Reporting Issues

- Include GDPR article references for compliance violations
- Include reproduction steps for consent lifecycle bugs
- Reference the affected public API surface
- Check if the issue affects downstream crates (kcm-runtime, kcm-security)

## Enforcement

Violations of these guidelines are handled per the project-wide [CODE_OF_CONDUCT.md](../../CODE_OF_CONDUCT.md). Contact: security@kcm.dev.

## References

- [CODE_OF_CONDUCT.md](../../CODE_OF_CONDUCT.md) — Project-wide community guidelines
- [CONTRIBUTING.md](../../CONTRIBUTING.md) — Contribution guidelines
- [AGENTS.md](../../AGENTS.md) — Engineering constitution
- [PRD3.md](../../docs/PRD3.md) §32 — GDPR compliance specification
