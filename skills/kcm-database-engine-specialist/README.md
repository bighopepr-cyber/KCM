# kcm-database-engine-specialist

> Priority: P6 | Authority: Block | Scope: Storage, query, transactions, indexing

## Overview

Database engine architect ensuring storage engine, query engine, transaction system, and indexing infrastructure are correct and production-ready.

## Purpose

Validate binary format determinism, WAL entry preservation, operator tombstone handling, recovery completeness, and codec/compression roundtrip correctness.

## Files

| File | Description |
|------|-------------|
| `SKILL.md` | Complete skill instructions |
| `README.md` | This file |
| `checklists/` | Storage checklists |
| `examples/` | Usage examples |
| `templates/` | Storage report templates |

## Quick Reference

- Blocks storage/query changes
- Owns kcm-storage implementation
- Cannot change public contracts without P4 approval

## Related Skills

| Skill | Relationship |
|-------|-------------|
| P4 Spec Lock | Validates contract changes |
| P5 Arch Guardian | Validates architecture |
| P8 Performance | Validates performance |

## SSOT References

- [SSOT.md](../../SSOT.md)
- [AGENTS.md](../../AGENTS.md)
