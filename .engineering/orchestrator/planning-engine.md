# Planning Engine

> Document ID: KCM-PLAN-001 | Version: 1.0.0

## Overview

The Planning Engine creates structured execution plans for engineering tasks.

## Plan Structure

### Execution Plan Template

```markdown
# Execution Plan

**Task:** {{TASK}}
**Date:** {{DATE}}
**Planner:** P2 Task Planner

## Objectives
{{OBJECTIVES}}

## Affected Files
| File | Change Type | Reason |
|------|------------|--------|
| {{FILE}} | {{TYPE}} | {{REASON}} |

## Affected Specifications
| Spec | Update Required |
|------|----------------|
| {{SPEC}} | {{YES/NO}} |

## Required Skills
| Skill | Phase | Responsibility |
|-------|-------|---------------|
| {{SKILL}} | {{PHASE}} | {{RESPONSIBILITY}} |

## Dependencies
| Dependency | Type | Impact |
|-----------|------|--------|
| {{DEP}} | {{TYPE}} | {{IMPACT}} |

## Risks
| Risk | Probability | Impact | Mitigation |
|------|------------|--------|------------|
| {{RISK}} | {{PROB}} | {{IMPACT}} | {{MITIGATION}} |

## Deliverables
| Deliverable | Type | Validator |
|------------|------|-----------|
| {{DELIVERABLE}} | {{TYPE}} | {{VALIDATOR}} |

## Validation
| Gate | Validator | Pass Criteria |
|------|-----------|--------------|
| {{GATE}} | {{VALIDATOR}} | {{CRITERIA}} |

## Exit Criteria
- [ ] {{CRITERIA}}
```

## Plan Validation

| Check | Method | Pass Criteria |
|-------|--------|--------------|
| All files identified | Grep + glob | No missing files |
| All specs identified | Spec lookup | No missing specs |
| Skills correctly routed | Routing rules | Correct skills |
| Dependencies mapped | Dependency analysis | No missing deps |
| Risks assessed | Risk matrix | All risks have mitigation |