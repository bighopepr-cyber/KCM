# kcm-runtime Community Guidelines

> This document covers community guidelines specific to the `kcm-runtime` crate. For the project-wide code of conduct, see the root `CODE_OF_CONDUCT.md`.

## Respect

- Treat all contributors with respect regardless of experience level, background, or role
- Assume good intent in all interactions
- Acknowledge that reasonable people can disagree on technical approaches
- Respect the SSOT authority hierarchy when technical disagreements arise

## Professional Communication

- Use clear, concise, and technical language
- Provide evidence and rationale for technical positions
- Reference SSOT documents when making specification-based arguments
- Avoid personal attacks, dismissive language, or sarcasm
- Focus feedback on the code and its alignment with specifications, not on the author

## Code Review Etiquette

- Review code against `AGENTS.md` non-negotiable rules and SSOT specifications
- Provide actionable feedback with specific line references
- Distinguish between blocking issues (security, correctness, SSOT violation) and suggestions (style, optimization)
- Acknowledge good work and well-structured implementations
- Use the `kcm-code-review-auditor` skill for structured review guidance

## Collaboration

- Follow the engineering gate process for all changes
- Use the skill governance hierarchy for domain-specific decisions
- Share knowledge through documentation and code comments
- Mentor less experienced contributors through constructive feedback
- Participate in design discussions before implementation begins

## Reporting Issues

- Report security vulnerabilities through the root `SECURITY.md` process, not in public issues
- Report bugs with reproduction steps and expected vs. actual behavior
- Report SSOT divergences with references to the relevant specification
- Use issue templates where available

## Enforcement

- First offense: Private discussion with the contributor
- Repeated offenses: Temporary restriction from the crate
- Severe violations (security, harassment): Immediate escalation per root `CODE_OF_CONDUCT.md`
- Technical enforcement: The `kcm-engineering-orchestrator` skill mediates technical disputes

## References

- `CODE_OF_CONDUCT.md` (root) — Project-wide code of conduct
- `AGENTS.md` — Engineering constitution and skill governance
- `docs/PRD-TESTING& BRACHMARCK.md` — Quality gate requirements
