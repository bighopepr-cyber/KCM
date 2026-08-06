# kcm-compute Community Guidelines

Community guidelines specific to contributions in the `kcm-compute` crate.

> For project-wide community guidelines, refer to the [CODE_OF_CONDUCT.md](../../CODE_OF_CONDUCT.md) located in the repository root.

## Respect

- Treat all contributors with respect regardless of experience level
- Acknowledge that `kcm-compute` is the execution layer — correctness is paramount
- Be patient with newcomers learning SIMD or relational algebra concepts
- Value correctness and safety over premature performance optimization

## Professional Communication

- Use clear, technical language in issues and PRs
- Provide evidence for claims about performance or correctness
- Reference SSOT specifications when discussing operator design decisions
- Avoid emotional reactions to review feedback
- Cite benchmarks when discussing SIMD performance claims

## Code Review Etiquette

- Review `kcm-compute` changes with care — operator bugs produce silent data corruption
- Focus on correctness, safety, and operator semantics
- Pay special attention to SIMD `unsafe` blocks and their safety comments
- Suggest alternatives rather than just rejecting
- Acknowledge good design decisions

## Collaboration

- Coordinate with other contributors on operator API changes
- Share benchmark results when claiming SIMD performance improvements
- Document design decisions in ADRs when appropriate
- Help maintain backward compatibility of the `Operator` trait

## Reporting Issues

- Include reproduction steps for correctness issues
- Include benchmarks for performance regressions
- Reference the affected operator or SIMD function
- Include data characteristics (size, distribution) for performance issues

## Enforcement

Violations of these guidelines are handled per the project-wide [CODE_OF_CONDUCT.md](../../CODE_OF_CONDUCT.md). Contact: security@kcm.dev.

## References

- [CODE_OF_CONDUCT.md](../../CODE_OF_CONDUCT.md) — Project-wide community guidelines
- [CONTRIBUTING.md](../../CONTRIBUTING.md) — Contribution guidelines
- [AGENTS.md](../../AGENTS.md) — Engineering constitution
