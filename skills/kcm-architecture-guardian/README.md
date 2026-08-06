# kcm-architecture-guardian

> Priority: P5 | Authority: Block | Scope: System architecture, dependencies, module boundaries

## Overview

Principal Software Architect maintaining architectural integrity across all KCM changes.

## Purpose

Verify dependency direction, enforce separation of concerns, validate PRD traceability, enforce interface stability, and preserve data integrity invariants.

## Files

| File | Description |
|------|-------------|
| `SKILL.md` | Complete skill instructions |
| `README.md` | This file |
| `checklists/` | Architecture checklists |
| `examples/` | Usage examples |
| `templates/` | Architecture report templates |

## Quick Reference

- Blocks architecture violations
- Enforces 13-crate separation
- Validates dependency direction (no circular deps)

## Related Skills

| Skill | Relationship |
|-------|-------------|
| P4 Spec Lock | Collaborates on contracts |
| P6 DB Specialist | Validates storage architecture |
| P10 Code Quality | Validates code architecture |

## SSOT References

- [SSOT.md](../../SSOT.md)
- [AGENTS.md](../../AGENTS.md)
