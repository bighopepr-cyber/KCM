# Conflict Engine

> Document ID: KCM-CONFLICT-001 | Version: 1.0.0

## Overview

The Conflict Engine resolves conflicts between skills.

## Conflict Resolution Rules

| Scenario | Resolution |
|----------|-----------|
| Two skills disagree | Higher priority wins |
| Same priority, different domain | Domain authority wins |
| Same priority, same domain | P1 Orchestrator decides |
| Security vs Performance | Security wins (P7 > P8) |
| Security vs Functionality | Security wins (P7 > any feature) |
| Performance vs Correctness | Correctness wins |

## Escalation Path

```
Level 1: Skill internally resolves (1 hour)
Level 2: Higher priority skill resolves (4 hours)
Level 3: P1 Orchestrator resolves (24 hours)
Level 4: SSOT.md is final authority
```

## Conflict Record

```markdown
# Conflict Record

**Task:** {{TASK}}
**Date:** {{DATE}}
**Skills:** {{SKILL_1}} vs {{SKILL_2}}

## Conflict
{{DESCRIPTION}}

## Resolution
{{RESOLUTION}}

## Rationale
{{RATIONALE}}
```