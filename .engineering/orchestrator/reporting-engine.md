# Reporting Engine

> Document ID: KCM-REPORT-001 | Version: 1.0.0

## Overview

The Reporting Engine generates comprehensive reports for every engineering task.

## Report Types

### 1. Executive Summary

```markdown
# Executive Summary

**Task:** {{TASK}}
**Date:** {{DATE}}
**Status:** {{STATUS}}
**Risk Level:** {{RISK}}

## Summary
{{SUMMARY}}

## Key Decisions
{{DECISIONS}}

## Outcome
{{OUTCOME}}
```

### 2. Impact Analysis Report

```markdown
# Impact Analysis Report

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
```

### 3. Approval Report

```markdown
# Approval Report

| Skill | Decision | Rationale | Date |
|-------|----------|-----------|------|
| {{SKILL}} | {{DECISION}} | {{RATIONALE}} | {{DATE}} |
```

### 4. Quality Report

```markdown
# Quality Report

| Gate | Status | Details |
|------|--------|---------|
| {{GATE}} | {{STATUS}} | {{DETAILS}} |
```

### 5. Completion Report

```markdown
# Completion Report

**Task:** {{TASK}}
**Date:** {{DATE}}

## Deliverables
| Deliverable | Status | Location |
|------------|--------|----------|
| {{DELIVERABLE}} | {{STATUS}} | {{LOCATION}} |

## Metrics
| Metric | Value |
|--------|-------|
| Files Changed | {{COUNT}} |
| Tests Added | {{COUNT}} |
| Bugs Fixed | {{COUNT}} |
| Performance | {{CHANGE}} |

## Lessons Learned
{{LESSONS}}
```

## Report Storage

All reports stored in `.engineering/examples/` with naming:
- `report-{{TASK_ID}}-{{DATE}}.md`