# skills/ Community Guidelines

> For project-wide community guidelines, refer to the [CODE_OF_CONDUCT.md](../CODE_OF_CONDUCT.md) located in the repository root.

## Respect

All contributors must treat each other with respect. This includes:

- Respecting authority boundaries defined by the skill hierarchy
- Acknowledging expertise domains owned by each skill
- Accepting decisions made by higher-priority skills when conflicts arise

## Professional Communication

| Standard | Requirement |
|----------|-------------|
| Tone | Professional and technical; no personal attacks |
| Language | English; technical terminology preferred |
| Feedback | Constructive and specific; cite SSOT references |
| Disagreements | Resolve via engineering-orchestrator (P1) if needed |

## Code Review Etiquette

| Practice | Description |
|----------|-------------|
| Focus on code | Review the implementation, not the contributor |
| Cite standards | Reference SSOT documents when identifying issues |
| Be specific | Provide file paths, line numbers, and concrete suggestions |
| Respect authority | Security Engineer (P7) has final say on security matters |
| No gatekeeping | Every contributor's work is evaluated by the same standards |

## Collaboration

| Principle | Description |
|-----------|-------------|
| Single responsibility | Respect each skill's domain ownership |
| Authority hierarchy | Higher-priority skills resolve conflicts |
| SSOT alignment | All work must trace back to SSOT requirements |
| Knowledge sharing | Document decisions in kcm-engineering-decision-record |

## Reporting Issues

| Issue Type | Report To |
|------------|-----------|
| Security vulnerability | kcm-security-engineer, then SECURITY.md process |
| Architecture violation | kcm-architecture-guardian |
| Specification deviation | kcm-specification-lock |
| Code quality issue | kcm-code-quality-guardian |
| Process violation | kcm-engineering-orchestrator |

## Enforcement

| Violation | Consequence |
|-----------|-------------|
| Minor (style, formatting) | Review feedback from kcm-code-quality-guardian |
| Moderate (authority boundary) | Escalation to kcm-engineering-orchestrator |
| Major (security, SSOT) | Blocked by kcm-security-engineer or kcm-specification-lock |
| Critical (data loss, breach) | Immediate rollback, security audit |

## References

- `AGENTS.md` — Engineering constitution
- `CODE_OF_CONDUCT.md` — Project-wide community guidelines
- `SECURITY.md` — Security policy
- `skills/README.md` — Skills registry and authority hierarchy
