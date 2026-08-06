# Contributing to skills/

> For core engine contribution rules, refer to the repository [CONTRIBUTING.md](../CONTRIBUTING.md).

## Overview

This document describes how to contribute to the `skills/` directory, which contains 16 AI engineering skills for KCM development. Skills are Markdown-based instruction sets that guide AI agents during development.

## Before Contributing

1. Read `AGENTS.md` to understand the engineering constitution
2. Review the existing 16 skills and their authority boundaries
3. Check if a similar skill already exists before creating a new one
4. Understand the priority hierarchy (P1–P16) and authority model

## Coding Standards

| Standard | Requirement |
|----------|-------------|
| Format | All skill files are Markdown (`.md`) |
| Section structure | Every SKILL.md must have consistent section headers |
| Naming | Skill directories use kebab-case: `kcm-<domain>-<role>` |
| Authority | Every skill must define its authority boundary |
| Priority | Every skill must have a priority level (P1–P16) |
| SSOT alignment | Every skill must reference its authoritative SSOT document |

## Module Architecture Rules

| Rule | Description |
|------|-------------|
| Independence | Each skill is independent; no skill depends on another skill at runtime |
| No circular references | Skills do not reference each other directly |
| Single responsibility | Each skill covers exactly one domain |
| Authority boundary | Each skill operates within its defined authority level |
| Conflict resolution | Conflicts between skills are resolved by P1 (engineering-orchestrator) |

## Documentation Rules

Every skill must have a `SKILL.md` file with these sections:

| Section | Description |
|---------|-------------|
| Overview | What the skill does and when to invoke it |
| Authority | The skill's authority level and boundaries |
| Scope | What the skill covers and what it does not |
| Workflow | Step-by-step instructions for the skill |
| SSOT Reference | Which SSOT document the skill aligns with |

## Testing Requirements

| Requirement | Description |
|-------------|-------------|
| Agent invocation | Skills must be testable by invoking them via the `skill` tool |
| Correctness | Skill instructions must produce correct results when followed |
| Conflict resolution | Skills must not conflict with higher-priority skills |
| SSOT compliance | Skill instructions must align with SSOT specifications |

## Performance Rules

| Rule | Description |
|------|-------------|
| Skill loading | Skills load in < 100ms |
| Skill size | SKILL.md files should be < 50KB |
| No runtime overhead | Skills are instructions, not code — zero runtime cost |

## Review Checklist

- [ ] Skill directory follows kebab-case naming
- [ ] SKILL.md has all required sections
- [ ] Authority level is defined and appropriate
- [ ] No conflicts with existing skills
- [ ] SSOT reference is specified
- [ ] Priority level is assigned (P1–P16)
- [ ] No secrets or sensitive data in skill definition
- [ ] Skill is independent (no circular references)

## Pull Request Requirements

| Requirement | Description |
|-------------|-------------|
| Title | `[skills] Add/Update <skill-name>` |
| Description | Explain the skill's purpose and authority |
| Review | Requires approval from kcm-engineering-orchestrator |
| Testing | Skill must be invokable via the `skill` tool |
| SSOT | Skill must reference an authoritative SSOT document |

## References

- `AGENTS.md` — Engineering constitution and skill registry
- `skills/README.md` — Skills overview and execution flow
- `skills/SECURITY.md` — Skills security policy
- `docs/PRD-TESTING& BRACHMARCK.md` — Testing strategy
