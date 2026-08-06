# kcm-ml Community Guidelines

Community standards for contributors to the `kcm-ml` crate.

> For project-wide community guidelines, refer to the [CODE_OF_CONDUCT.md](../../CODE_OF_CONDUCT.md) located in the repository root.

## Respect

All contributors must treat each other with respect regardless of experience level, role, or background. Technical disagreements must be resolved through evidence and reasoning, not personal attacks. Review feedback must focus on the code, not the author.

## Professional Communication

- Use clear, concise technical language in issues, pull requests, and code reviews
- Provide context and evidence when proposing changes to ML algorithms or model behavior
- Cite relevant research or benchmarks when introducing new ML techniques
- Avoid dismissive language; explain *why* a change is needed, not just *what* to change
- Document assumptions about training data quality, model accuracy, and prediction guarantees

## Code Review Etiquette

- Review ML changes with attention to numerical correctness, not just syntactic correctness
- Verify that confidence bounds [0.0, 1.0] are maintained in all code paths
- Check that model training handles edge cases (empty data, degenerate inputs, NaN/Inf)
- Ensure rule discovery thresholds are enforced and documented
- Approve changes only when tests demonstrate correctness, not just compilation
- Distinguish between blocking issues (correctness, security) and suggestions (style, clarity)

## Collaboration

- Share knowledge about ML algorithms, regression techniques, and association rule mining
- Review PRs promptly; ML changes require careful verification of mathematical correctness
- When unsure about an algorithm's correctness, write a test that demonstrates the expected behavior before approving
- Cross-reference changes with the technical specification to ensure alignment
- Coordinate with `kcm-reasoning` contributors when rule discovery affects inference behavior

## Reporting Issues

- Report numerical bugs with specific inputs that demonstrate incorrect predictions
- Include expected vs actual output for confidence scores and index predictions
- Tag ML-related issues with appropriate labels for triage
- For security concerns (model poisoning, adversarial inputs), follow the [security policy](../../SECURITY.md)

## Enforcement

Community guidelines are enforced by the repository maintainers. Violations may result in:
1. A request to modify behavior
2. Temporary review restrictions
3. Contribution suspension for repeated violations

Enforcement decisions are made in accordance with the project-wide [CODE_OF_CONDUCT.md](../../CODE_OF_CONDUCT.md).

## References

- [CODE_OF_CONDUCT.md](../../CODE_OF_CONDUCT.md) — Project-wide community guidelines
- [CONTRIBUTING.md](../../CONTRIBUTING.md) — Repository-wide contribution guidelines
- [SECURITY.md](../../SECURITY.md) — Project-wide security policy
- [AGENTS.md](../../AGENTS.md) — Engineering constitution
