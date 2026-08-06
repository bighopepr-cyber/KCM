# docs/ Community Guidelines

> For project-wide community guidelines, refer to the [CODE_OF_CONDUCT.md](../CODE_OF_CONDUCT.md) located in the repository root.

## Respect

All documentation contributors must treat each other with respect. This includes:

- Valuing diverse perspectives and experiences
- Accepting constructive feedback gracefully
- Focusing on the technical content, not the person
- Acknowledging contributors' efforts and expertise

## Professional Communication

- Use clear, concise technical language
- Avoid ambiguous phrasing in specifications
- Reference existing terminology from `specs/KCM_GLOSSARY.md`
- Provide context when introducing new terms or concepts
- Write for the intended audience level

## Code Review Etiquette

When reviewing documentation changes:

- Focus on accuracy, clarity, and completeness
- Suggest improvements rather than demanding changes
- Reference specific SSOT requirements when requesting changes
- Acknowledge well-written documentation
- Use inline comments for specific suggestions
- Approve when standards are met, even if minor improvements are possible

## Collaboration

- Share knowledge through documentation, not just code
- Help maintain the SSOT hierarchy
- Cross-reference related documents when making changes
- Update documentation alongside implementation changes
- Participate in specification reviews for significant changes

## Reporting Issues

Documentation issues should be reported via:

- **Typos and broken links**: Direct PR with fix
- **Inaccuracies**: Issue with description of expected vs actual content
- **Missing documentation**: Issue with scope and priority
- **SSOT conflicts**: Issue tagged with Specification Lock for review
- **Security concerns**: Direct message to security team, not public issue

## Enforcement

Documentation quality standards are enforced through:

| Mechanism | Scope |
|-----------|-------|
| CI validation | `scripts/validate-ssot.sh` runs on every PR |
| Link checking | Broken links block merge |
| SSOT hierarchy | Conflicts with higher-priority docs block merge |
| Code review | Documentation Guardian reviews all doc changes |
| Secret scanning | Credential detection blocks merge |

Violations of community guidelines may result in:

1. Request for revision
2. Discussion in PR review
3. Escalation to Engineering Orchestrator
4. Revert of merged changes if standards were not met

## References

- [CODE_OF_CONDUCT.md](../CODE_OF_CONDUCT.md) — Repository root community guidelines
- `AGENTS.md` — Engineering constitution
- `specs/KCM_GLOSSARY.md` — Project terminology
- `docs/README.md` — Documentation structure overview
