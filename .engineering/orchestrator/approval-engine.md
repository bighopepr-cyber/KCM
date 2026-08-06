# Approval Engine

> Document ID: KCM-APPROVE-001 | Version: 2.0.0 | Status: Active

## Overview

The Approval Engine manages approval chains based on the Authority System. It ensures every change receives appropriate approval before proceeding.

## Approval Algorithm

```
1. Receive execution plan from Planning Engine
2. Identify required approvals based on:
   - Task type
   - Affected components
   - Risk level
   - Change type
3. Generate approval chain
4. Track approval status
5. Handle rejections and escalations
6. Record all decisions
```

## Approval Chains by Change Type

### Low Risk Changes
```
P10 Code Quality → P9 Testing → P12 Release → P1 Orchestrator
```

### Medium Risk Changes
```
P4 Spec Lock → P5 Arch Guardian → P9 Testing → P11 Docs → P12 Release → P1 Orchestrator
```

### High Risk Changes
```
P4 Spec Lock → P5 Arch Guardian → P7 Security → P9 Testing → P11 Docs → P12 Release → P1 Orchestrator
```

### Critical Changes
```
All skills → P1 Orchestrator
```

## Approval Chains by Component

| Component | Required Approvals |
|-----------|-------------------|
| kcm-core | P5 + P4 |
| kcm-storage | P6 + P5 + P4 |
| kcm-compute | P6 + P5 |
| kcm-reasoning | P6 + P5 |
| kcm-optimizer | P6 + P5 |
| kcm-runtime | P6 + P5 |
| kcm-interface | P4 + P7 |
| kcm-distributed | P5 + P4 |
| kcm-ml | P5 + P4 |
| kcm-security | P7 + P4 |
| kcm-compliance | P7 + P4 |
| kcm-testing | P9 + P5 |
| kcm-server | P5 + P4 |

## Approval Rules

1. **Higher priority overrides lower** — P1 can override any skill
2. **P4 has VETO power** — Can block contract changes
3. **P7 has Block power** — Can block security violations
4. **All approvals must be documented** — Decision, rationale, date
5. **Rejection requires reason** — Must specify what needs to change
6. **Escalation path exists** — Level 1 → 2 → 3 → 4 (SSOT)

## Approval States

| State | Description | Next |
|-------|-------------|------|
| PENDING | Awaiting decision | APPROVED / REJECTED |
| APPROVED | Decision approved | Next skill or COMPLETE |
| REJECTED | Decision rejected | BLOCKED |
| ESCALATED | Escalated to higher | PENDING at higher level |

## Approval Record Format

```markdown
# Approval Record

**Task ID:** {{TASK_ID}}
**Task:** {{TASK}}
**Date:** {{DATE}}

## Approval Chain
| # | Skill | Decision | Rationale | Date | Duration |
|---|-------|----------|-----------|------|----------|
| 1 | {{SKILL}} | {{DECISION}} | {{RATIONALE}} | {{DATE}} | {{DURATION}} |
| 2 | {{SKILL}} | {{DECISION}} | {{RATIONALE}} | {{DATE}} | {{DURATION}} |

## Summary
- **Total Approvals:** {{COUNT}}
- **Approved:** {{APPROVED}}
- **Rejected:** {{REJECTED}}
- **Escalated:** {{ESCALATED}}
- **Status:** {{STATUS}}
```

## Rejection Handling

When a skill rejects:

1. Record rejection with reason
2. Notify task owner
3. Provide specific remediation steps
4. Block task until resolved
5. Allow re-submission after fix

## Escalation on Rejection

| Scenario | Escalation |
|----------|-----------|
| Skill rejects without reason | Escalate to higher priority |
| Conflicting rejections | Escalate to P1 |
| SSOT violation | SSOT is final authority |
| Timeout (no response) | Auto-escalate after SLA |

## SLA

| Approval Level | SLA |
|---------------|-----|
| Low Risk | 12 hours |
| Medium Risk | 24 hours |
| High Risk | 24 hours |
| Critical | 4 hours |
