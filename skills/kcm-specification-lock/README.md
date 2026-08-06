# kcm-specification-lock

> Priority: P4 | Authority: Veto | Scope: Frozen contracts, API, FFI, formats, protocols

## Overview

Contract gatekeeper protecting frozen technical contracts from accidental modification.

## Purpose

Protect binary file format, WAL entry format, C FFI signatures, gRPC proto definitions, error code enums, public API return types, and `#[repr(C)]` struct layouts.

## Files

| File | Description |
|------|-------------|
| `SKILL.md` | Complete skill instructions |
| `README.md` | This file |
| `checklists/` | Contract validation checklists |
| `examples/` | Usage examples |
| `templates/` | Contract change templates |

## Quick Reference

- VETO power over format/API/FFI changes
- Owns all frozen contracts
- Requires version bump for any contract change

## Related Skills

| Skill | Relationship |
|-------|-------------|
| P5 Architecture Guardian | Collaborates on architecture |
| P7 Security Engineer | Collaborates on FFI security |
| P11 Documentation Guardian | Validates spec updates |

## SSOT References

- [SSOT.md](../../SSOT.md)
- [AGENTS.md](../../AGENTS.md)
