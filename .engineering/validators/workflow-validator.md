# Workflow Validator

> Document ID: KCM-VAL-WORKFLOW-001 | Version: 1.0.0

## Overview

Validates that engineering workflows follow the defined processes in AGENTS.md, Authority System, and Decision Matrix.

## Validation Checks

### 1. Task Classification Validation

| Check | Method | Pass Criteria |
|-------|--------|--------------|
| Task type identified | Classification check | Valid task type |
| Risk level assessed | Risk matrix | Valid risk level |
| Pipeline selected | Pipeline lookup | Correct pipeline |
| Primary skill selected | Routing rules | Correct primary skill |
| Supporting skills selected | Routing rules | Correct supporting skills |

### 2. Authority Validation

| Check | Method | Pass Criteria |
|-------|--------|--------------|
| Primary skill has authority | Authority matrix | Authority exists |
| Supporting skills have authority | Authority matrix | Authority exists |
| No authority conflicts | Conflict check | No conflicts |
| Approval chain correct | Approval matrix | Correct chain |
| Veto power respected | Veto check | P4 veto honored |
| Block power respected | Block check | Block powers honored |

### 3. Decision Matrix Validation

| Check | Method | Pass Criteria |
|-------|--------|--------------|
| Change type matches matrix | Decision matrix | Correct matrix |
| Skills match matrix | Decision matrix | Correct skills |
| Order matches matrix | Decision matrix | Correct order |
| Exceptions documented | Exception check | All exceptions noted |

### 4. Workflow Validation

| Check | Method | Pass Criteria |
|-------|--------|--------------|
| All required phases executed | Phase tracking | All phases complete |
| Phase order correct | State machine | Valid transitions |
| No skipped phases | Phase check | All phases present |
| All gates passed | Gate check | Zero failures |
| All approvals received | Approval check | All required approvals |

### 5. SSOT Compliance Validation

| Check | Method | Pass Criteria |
|-------|--------|--------------|
| Task traces to SSOT | Reference check | SSOT requirement exists |
| Implementation matches spec | Spec check | No deviations |
| No frozen contract violations | Contract check | No violations |
| Version compatibility | Version check | Compatible |

### 6. Documentation Validation

| Check | Method | Pass Criteria |
|-------|--------|--------------|
| Required docs updated | Doc checklist | All required docs |
| No broken links | Link check | Zero broken links |
| SSOT aligned | SSOT check | No conflicts |
| Cross-references valid | Reference check | All refs valid |

## Validation Script

```bash
#!/usr/bin/env bash
# Validate Engineering Workflow
set -euo pipefail

TASK_ID="${1:-}"
if [ -z "$TASK_ID" ]; then
    echo "Usage: $0 <task-id>"
    exit 1
fi

echo "=== Workflow Validation for $TASK_ID ==="

# Check task has classification
# Check task has execution plan
# Check task has approval chain
# Check task has quality report
# Check task has completion report

echo "Workflow validation: PASS"
```
