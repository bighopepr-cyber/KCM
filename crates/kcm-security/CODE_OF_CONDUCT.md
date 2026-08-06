# kcm-security Community Guidelines

> For project-wide community guidelines, see the root `CODE_OF_CONDUCT.md`.

## Respect

All contributors must treat each other with respect. Security work is high-stakes — mistakes can compromise the entire system. Approach every interaction with the assumption that the other person is acting in good faith.

## Professional Communication

- Be direct and technical. Security discussions require precision, not ambiguity.
- Cite evidence. Security claims must be backed by specifications, tests, or formal analysis.
- Disagree with ideas, not people. Challenge implementations without personal attacks.
- Acknowledge uncertainty. If you are unsure about a security property, say so.

## Code Review Etiquette

Security changes require **expert review**. When reviewing or requesting review:

- **Security engineer review is mandatory** for all changes to this crate.
- Focus on correctness, not style. Style is enforced by `rustfmt` and `clippy`.
- Identify root causes, not just symptoms. If a change introduces a risk, explain the threat model.
- Be specific. "This might be insecure" is not actionable. "This nonce reuse allows key recovery" is.
- Respond to review feedback promptly. Security issues should not linger.

## Collaboration

- Share threat models openly. Security is a team effort.
- Document security decisions. Future contributors need to understand why choices were made.
- Escalate quickly. If you discover a potential vulnerability, report it immediately — do not wait for a PR review cycle.
- Cross-pollinate knowledge. Security reviews are learning opportunities for everyone.

## Reporting Issues

**Security vulnerabilities must be reported privately.**

- Email: `security@kcm.dev`
- Include: description, reproduction steps, potential impact, suggested fix (if any).
- Do **not** open public issues for security vulnerabilities.
- Non-security bugs can be reported via normal issue channels.

## Enforcement

Violations of these guidelines result in:

1. **First offense:** Private discussion and clarification.
2. **Second offense:** Formal warning with documented expectations.
3. **Third offense:** Removal from the project.

Security-critical violations (e.g., committing hardcoded keys, bypassing review) may result in immediate removal.

## References

- `AGENTS.md` — Engineering constitution
- `SECURITY.md` — Security policy for this crate
- Root `CODE_OF_CONDUCT.md` — Project-wide community guidelines
