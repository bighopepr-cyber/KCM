# kcm-reasoning Community Guidelines

Community guidelines specific to contributions in the `kcm-reasoning` crate.

> For project-wide community guidelines, refer to the [CODE_OF_CONDUCT.md](../../CODE_OF_CONDUCT.md) located in the repository root.

## Respect

- Treat all contributors with respect regardless of experience level
- Recognize that reasoning engine correctness is critical — incorrect inference silently corrupts knowledge
- Be patient with contributors learning Rust ownership and lifetime semantics
- Value correctness and termination guarantees over premature optimization

## Professional Communication

- Use clear, technical language when discussing inference semantics or rule design
- Provide evidence for claims about inference correctness or performance
- Reference SSOT specifications (PRD.md §6) when discussing reasoning behavior
- Avoid emotional reactions to review feedback on inference logic

## Code Review Etiquette

- Review `kcm-reasoning` changes with care — incorrect inference produces silently wrong results
- Focus on correctness, termination guarantees, and confidence propagation
- Verify that new rules cannot create infinite derivation loops
- Acknowledge well-designed rule patterns and confidence formulas

## Collaboration

- Coordinate with other contributors on changes to the inference algorithm
- Share benchmark results when claiming performance improvements
- Document design decisions in ADRs when introducing new `RulePattern` variants
- Help maintain backward compatibility of the `InferenceEngine` public API

## Reporting Issues

- Include reproduction steps for incorrect inference results
- Include the rule definitions and input schema that demonstrate the issue
- Reference the affected public API surface
- Report whether the issue causes non-termination or incorrect confidence values

## Enforcement

Violations of these guidelines are handled per the project-wide [CODE_OF_CONDUCT.md](../../CODE_OF_CONDUCT.md). Contact: security@kcm.dev.

## References

- [CODE_OF_CONDUCT.md](../../CODE_OF_CONDUCT.md) — Project-wide community guidelines
- [CONTRIBUTING.md](../../CONTRIBUTING.md) — Contribution guidelines
- [AGENTS.md](../../AGENTS.md) — Engineering constitution
