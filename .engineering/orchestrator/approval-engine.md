# Approval Engine

> Document ID: KCM-APPROVE-001 | Version: 1.0.0

## Overview

The Approval Engine manages approval chains based on the Authority System.

## Approval Chains

### By Change Type

| Change Type | Approval Chain |
|-------------|---------------|
| Low Risk | P10 → P9 → P12 → P1 |
| Medium Risk | P4 → P5 → P9 → P11 → P12 → P1 |
| High Risk | P4 → P5 → P7 → P9 → P11 → P12 → P1 |
| Critical | All skills → P1 |

### By Component

| Component | Required Approvals |
|-----------|-------------------|
| kcm-core | P5 + P4 |
| kcm-storage | P6 + P5 + P4 |
| kcm-interface | P4 + P7 |
| kcm-security | P7 + P4 |
| kcm-server | P5 + P4 |

## Approval Rules

1. Higher priority skill can override lower priority
2. P4 (Spec Lock) has VETO power on contract changes
3. P7 (Security) has Block power on security changes
4. P1 (Orchestrator) has Override power on everything
5. All approvals must be documented

## Approval Record

```markdown
# Approval Record

**Task:** {{TASK}}
**Date:** {{DATE}}

| Skill | Decision | Rationale | Date |
|-------|----------|-----------|------|
| {{SKILL}} | {{DECISION}} | {{RATIONALE}} | {{DATE}} |
```