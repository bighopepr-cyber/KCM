# Reporting Engine

> Document ID: KCM-REPORT-001 | Version: 2.0.0 | Status: Active

## Overview

The Reporting Engine generates comprehensive, structured reports for every engineering task. Reports are stored in `.engineering/examples/` for audit trail.

## Report Types

### 1. Executive Summary

```markdown
# Executive Summary

**Task ID:** {{TASK_ID}}
**Task:** {{TASK}}
**Date:** {{DATE}}
**Status:** {{STATUS}}
**Risk Level:** {{RISK}}
**Pipeline:** {{PIPELINE}}

## Summary
{{SUMMARY}}

## Key Decisions
| Decision | Skill | Rationale |
|----------|-------|-----------|
| {{DECISION}} | {{SKILL}} | {{RATIONALE}} |

## Outcome
{{OUTCOME}}

## Metrics
| Metric | Value |
|--------|-------|
| Duration | {{DURATION}} |
| Files Changed | {{FILES}} |
| Tests Added | {{TESTS}} |
| Quality Gates | {{GATES}} |
```

### 2. Task Classification Report

```markdown
# Task Classification

**Task ID:** {{TASK_ID}}
**Date:** {{DATE}}

## Classification
| Field | Value |
|-------|-------|
| Task Type | {{TYPE}} |
| Risk Level | {{RISK}} |
| Pipeline | {{PIPELINE}} |
| Primary Skill | {{PRIMARY}} |
| Supporting Skills | {{SUPPORTING}} |

## Affected Components
| Component | Change Type | Risk |
|-----------|-------------|------|
| {{COMPONENT}} | {{TYPE}} | {{RISK}} |

## Affected APIs
| API | Type | Impact |
|-----|------|--------|
| {{API}} | {{TYPE}} | {{IMPACT}} |
```

### 3. Impact Analysis Report

```markdown
# Impact Analysis Report

**Task ID:** {{TASK_ID}}
**Date:** {{DATE}}

## Direct Impact
| Component | Files | Change | Risk |
|-----------|-------|--------|------|
| {{COMP}} | {{FILES}} | {{CHANGE}} | {{RISK}} |

## Indirect Impact
| Component | Effect | Severity | Mitigation |
|-----------|--------|----------|------------|
| {{COMP}} | {{EFFECT}} | {{SEVERITY}} | {{MITIGATION}} |

## Specification Impact
| Spec | Update Required | Priority |
|------|----------------|----------|
| {{SPEC}} | {{YES/NO}} | {{PRIORITY}} |

## Test Impact
| Test Suite | Impact | Action Required |
|-----------|--------|-----------------|
| {{SUITE}} | {{IMPACT}} | {{ACTION}} |

## Documentation Impact
| Document | Update Required | Reason |
|----------|----------------|--------|
| {{DOC}} | {{YES/NO}} | {{REASON}} |
```

### 4. Approval Report

```markdown
# Approval Report

**Task ID:** {{TASK_ID}}
**Date:** {{DATE}}

## Approval Chain
| # | Skill | Decision | Rationale | Date |
|---|-------|----------|-----------|------|
| 1 | {{SKILL}} | {{DECISION}} | {{RATIONALE}} | {{DATE}} |

## Summary
- **Required:** {{REQUIRED}}
- **Approved:** {{APPROVED}}
- **Rejected:** {{REJECTED}}
- **Status:** {{STATUS}}
```

### 5. Quality Report

```markdown
# Quality Report

**Task ID:** {{TASK_ID}}
**Date:** {{DATE}}

## Gate Results
| Gate | Status | Details | Duration |
|------|--------|---------|----------|
| {{GATE}} | {{STATUS}} | {{DETAILS}} | {{DURATION}} |

## Summary
- **Total Gates:** {{TOTAL}}
- **Passed:** {{PASSED}}
- **Failed:** {{FAILED}}
- **Status:** {{STATUS}}
```

### 6. Completion Report

```markdown
# Completion Report

**Task ID:** {{TASK_ID}}
**Task:** {{TASK}}
**Date:** {{DATE}}
**Status:** COMPLETED

## Deliverables
| Deliverable | Status | Location |
|------------|--------|----------|
| {{DELIVERABLE}} | {{STATUS}} | {{LOCATION}} |

## Metrics
| Metric | Value |
|--------|-------|
| Total Duration | {{DURATION}} |
| Files Changed | {{FILES}} |
| Tests Added | {{TESTS}} |
| Quality Gates Passed | {{GATES}} |
| Approvals Received | {{APPROVALS}} |

## Execution Timeline
| Phase | Skill | Start | End | Duration |
|-------|-------|-------|-----|----------|
| {{PHASE}} | {{SKILL}} | {{START}} | {{END}} | {{DURATION}} |

## Lessons Learned
{{LESSONS}}

## Recommendations
{{RECOMMENDATIONS}}
```

## Report Storage

All reports stored in `.engineering/examples/` with naming:
```
report-{{TASK_ID}}-{{TYPE}}-{{DATE}}.md
```

Example:
```
report-KCM-2026-001-feature-2026-08-06.md
report-KCM-2026-002-bugfix-2026-08-07.md
```

## Report Validation

| Check | Method | Pass Criteria |
|-------|--------|--------------|
| All sections present | Template check | All required sections |
| All fields populated | Field check | No empty required fields |
| All decisions recorded | Decision check | All decisions documented |
| All gates recorded | Gate check | All gate results |
| SSOT traceable | Reference check | All refs valid |
