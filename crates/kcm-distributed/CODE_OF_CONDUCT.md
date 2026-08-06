# kcm-distributed Community Guidelines

> This document covers community guidelines specific to the `kcm-distributed` crate. For project-wide community guidelines, see the root `CODE_OF_CONDUCT.md`.

---

## Respect

All contributors to the `kcm-distributed` crate are expected to treat each other with respect. Distributed systems engineering is inherently complex — mistakes are expected and应当 be treated as learning opportunities. Focus critique on the code and technical decisions, never on the individual.

## Professional Communication

- Use clear, technical language when discussing distributed systems concepts.
- Reference specific SSOT documents (`PRD3.md` §27, `AGENTS.md`) when proposing changes.
- Provide evidence and rationale for technical positions rather than appeals to authority.
- Acknowledge uncertainty — distributed systems bugs can be subtle and environment-dependent.
- Avoid dismissive language toward proposed solutions or existing implementations.

## Code Review Etiquette

- Review code against the SSOT specification, not personal preferences.
- Distinguish between blocking issues (correctness, security, architecture violations) and suggestions (style, naming, minor improvements).
- For complex distributed systems changes (2PC, replication, sharding), request review from domain experts.
- Provide specific, actionable feedback — "this could cause split-brain under X condition" is more useful than "this looks wrong."
- Acknowledge when a change is well-designed or solves a difficult problem correctly.

## Collaboration

- Coordinate on cross-cutting changes that affect multiple modules (e.g., changes to the transport layer that impact both replication and the coordinator).
- Share context about distributed systems trade-offs (e.g., consistency vs availability, latency vs durability) during design discussions.
- Pair review on high-risk changes (2PC correctness, shard migration, replication conflict resolution).
- Document design decisions using the `kcm-engineering-decision-record` skill for changes with long-term impact.

## Reporting Issues

- Report bugs with reproduction steps, including cluster configuration and node topology where relevant.
- Security vulnerabilities must be reported privately via the root `SECURITY.md` process — not in public issues.
- Performance regressions should include benchmark results and environment details.
- When reporting distributed systems issues, include logs from all affected nodes if possible.

## Enforcement

Violations of these guidelines are handled through the project-wide enforcement process defined in the root `CODE_OF_CONDUCT.md`. Repeated or severe violations may result in temporary or permanent suspension of contribution privileges for the `kcm-distributed` crate.

## References

- Root `CODE_OF_CONDUCT.md` — Project-wide community guidelines
- `AGENTS.md` — Engineering gates and authority hierarchy
- `docs/PRD3.md` §27 — Distributed architecture specification
