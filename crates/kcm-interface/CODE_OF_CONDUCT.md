# kcm-interface Community Guidelines

> For the organization-wide code of conduct, see the root `CODE_OF_CONDUCT.md`.

## Respect

All contributors, reviewers, and users of kcm-interface are expected to treat each other with respect. Technical disagreements are resolved through evidence and specification, not authority or volume.

## Professional Communication

- Feedback must be specific, actionable, and constructive.
- Critique the code, not the person.
- Use the SSOT documents as the basis for technical arguments.
- Disagreements about implementation are resolved by referencing the specification.

## Code Review Etiquette

- **FFI changes receive extra scrutiny.** Unsafe code and FFI boundary changes require review from a contributor with systems programming experience.
- REST API changes must be reviewed for backward compatibility.
- Security-relevant changes (auth, CORS, rate limiting) require security-focused review.
- Reviewers should cite specific SSOT requirements when objecting to a change.
- Authors should respond to every review comment, even if only to acknowledge.

## Collaboration

- Large features are broken into incremental PRs that each pass the full quality gate.
- Cross-crate changes require coordination with the affected crate's maintainer.
- The SSOT is the authority — not any individual's opinion or implementation.
- When in doubt, ask in the PR discussion rather than making assumptions.

## Reporting Issues

- Security vulnerabilities are reported privately via the root `SECURITY.md` process.
- Bugs are filed as issues with reproduction steps.
- Specification discrepancies are filed with references to the conflicting documents.
- FFI misuse reports include the calling code and the resulting behavior.

## Enforcement

Violations of these guidelines result in:

1. First occurrence: Private discussion with the contributor.
2. Second occurrence: Written warning with specific examples.
3. Third occurrence: Temporary suspension from kcm-interface contributions.
4. Severe violations (security negligence, malicious code): Immediate removal.

The kcm-interface maintainers have final authority on enforcement decisions.

## References

- Root `CODE_OF_CONDUCT.md`
- `AGENTS.md` — Engineering constitution and non-negotiable rules
- `docs/SSOT.md` — Single source of truth
