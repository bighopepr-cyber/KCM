# Conflict Engine

> Document ID: KCM-CONFLICT-001 | Version: 2.0.0 | Status: Active

## Overview

The Conflict Engine resolves conflicts between skills when they disagree on a decision. It follows deterministic rules based on the Authority System.

## Conflict Resolution Rules

| # | Scenario | Resolution | Rationale |
|---|----------|-----------|-----------|
| 1 | Two skills disagree | Higher priority wins | Authority hierarchy |
| 2 | Same priority, different domain | Domain authority wins | Domain expertise |
| 3 | Same priority, same domain | P1 Orchestrator decides | Ultimate authority |
| 4 | Security vs Performance | Security wins | P7 > P8 |
| 5 | Security vs Functionality | Security wins | P7 > any feature |
| 6 | Performance vs Correctness | Correctness wins | Philosophy: correctness > performance |
| 7 | Speed vs Thoroughness | Thoroughness wins | Quality over speed |
| 8 | Feature vs Stability | Stability wins | No breaking changes |
| 9 | Innovation vs Proven | Proven wins | Enterprise-grade |
| 10 | SSOT vs Any Skill | SSOT wins | SSOT is P1 authority |

## Conflict Detection

```
1. Monitor skill decisions during execution
2. Detect when two skills make contradictory decisions
3. Classify conflict type
4. Apply resolution rules
5. Record conflict and resolution
```

## Conflict Types

| Type | Description | Example |
|------|-------------|---------|
| Decision Conflict | Two skills disagree on approach | P5 says use trait, P6 says use enum |
| Priority Conflict | Two skills claim same authority | P4 and P7 both claim Veto |
| Resource Conflict | Two skills need same resource | P8 and P6 both need to modify same function |
| Scope Conflict | Disagreement on change scope | P2 says small, P3 says large |
| Quality Conflict | Disagreement on quality standard | P10 says pass, P9 says fail |

## Resolution Algorithm

```
1. Identify conflicting skills
2. Compare priority levels
3. Check domain authority
4. Check authority types (Override > Veto > Block > Feedback)
5. Apply resolution rules
6. If unresolved, escalate to P1
7. If P1 cannot resolve, SSOT is final
8. Record resolution
```

## Escalation Path

```
Level 1: Skill internally resolves (SLA: 1 hour)
Level 2: Higher priority skill resolves (SLA: 4 hours)
Level 3: P1 Orchestrator resolves (SLA: 24 hours)
Level 4: SSOT.md is final authority (no SLA)
```

## Conflict Record Format

```markdown
# Conflict Record

**Task ID:** {{TASK_ID}}
**Date:** {{DATE}}
**Conflict ID:** {{CONFLICT_ID}}

## Conflicting Skills
- **Skill 1:** {{SKILL_1}} (Priority: {{P1}})
- **Skill 2:** {{SKILL_2}} (Priority: {{P2}})

## Conflict Description
{{DESCRIPTION}}

## Skill 1 Position
{{POSITION_1}}

## Skill 2 Position
{{POSITION_2}}

## Resolution
{{RESOLUTION}}

## Rationale
{{RATIONALE}}

## Authority Rule Applied
{{RULE}}

## Escalation Level
{{LEVEL}}

## Outcome
{{OUTCOME}}
```

## Prevention

| Strategy | Description |
|----------|-------------|
| Clear scope | Define task scope precisely in planning phase |
| Early alignment | Get P4 and P5 alignment before implementation |
| SSOT reference | Reference SSOT for all decisions |
| Communication | Skills communicate through execution plan |
| Documentation | Record all decisions for audit trail |
