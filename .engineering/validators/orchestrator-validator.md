# Engineering Orchestrator Validator

> Document ID: KCM-VAL-001 | Version: 1.0.0

## Overview

Validates that the Engineering Orchestrator system is complete, consistent, and compliant with AGENTS.md.

## Validation Checks

### 1. Structure Validation

| Check | File/Directory | Pass Criteria |
|-------|---------------|--------------|
| README exists | `.engineering/README.md` | File exists |
| ENGINE exists | `.engineering/ENGINE.md` | File exists |
| Orchestrator dir | `.engineering/orchestrator/` | Directory exists |
| Pipelines dir | `.engineering/pipelines/` | Directory exists |
| Templates dir | `.engineering/templates/` | Directory exists |
| Examples dir | `.engineering/examples/` | Directory exists |
| Checklists dir | `.engineering/checklists/` | Directory exists |
| Validators dir | `.engineering/validators/` | Directory exists |

### 2. Orchestrator Engine Validation

| Check | File | Pass Criteria |
|-------|------|--------------|
| Routing | `orchestrator/routing.md` | File exists, has routing rules |
| Execution | `orchestrator/execution-engine.md` | File exists, has execution phases |
| Planning | `orchestrator/planning-engine.md` | File exists, has plan structure |
| Approval | `orchestrator/approval-engine.md` | File exists, has approval chains |
| Conflict | `orchestrator/conflict-engine.md` | File exists, has resolution rules |
| Escalation | `orchestrator/escalation-engine.md` | File exists, has escalation levels |
| Quality | `orchestrator/quality-engine.md` | File exists, has quality gates |
| Reporting | `orchestrator/reporting-engine.md` | File exists, has report formats |
| State Machine | `orchestrator/state-machine.md` | File exists, has states and transitions |
| Documentation | `orchestrator/documentation-engine.md` | File exists, has doc rules |

### 3. Pipeline Validation

| Check | File | Pass Criteria |
|-------|------|--------------|
| Standard | `pipelines/standard.md` | File exists, has pipeline steps |
| Feature | `pipelines/feature.md` | File exists, has pipeline steps |
| Bugfix | `pipelines/bugfix.md` | File exists, has pipeline steps |
| Optimization | `pipelines/optimization.md` | File exists, has pipeline steps |
| Refactor | `pipelines/refactor.md` | File exists, has pipeline steps |
| Documentation | `pipelines/documentation.md` | File exists, has pipeline steps |
| Release | `pipelines/release.md` | File exists, has pipeline steps |
| Emergency | `pipelines/emergency.md` | File exists, has pipeline steps |

### 4. Template Validation

| Check | File | Pass Criteria |
|-------|------|--------------|
| Execution Plan | `templates/execution-plan-template.md` | File exists |
| Impact Analysis | `templates/impact-analysis-template.md` | File exists |
| Quality Report | `templates/quality-report-template.md` | File exists |
| Completion Report | `templates/completion-report-template.md` | File exists |
| Approval Record | `templates/approval-record-template.md` | File exists |
| Conflict Record | `templates/conflict-record-template.md` | File exists |

### 5. Example Validation

| Check | File | Pass Criteria |
|-------|------|--------------|
| Feature Example | `examples/feature-example.md` | File exists |
| Bugfix Example | `examples/bugfix-example.md` | File exists |
| Release Example | `examples/release-example.md` | File exists |

### 6. Checklist Validation

| Check | File | Pass Criteria |
|-------|------|--------------|
| Feature Checklist | `checklists/feature-checklist.md` | File exists |
| Bugfix Checklist | `checklists/bugfix-checklist.md` | File exists |
| Release Checklist | `checklists/release-checklist.md` | File exists |
| Security Checklist | `checklists/security-checklist.md` | File exists |

### 7. Cross-Reference Validation

| Check | Method | Pass Criteria |
|-------|--------|--------------|
| README references ENGINE | Grep | Link valid |
| ENGINE references engines | Grep | All links valid |
| Engines reference AGENTS.md | Grep | Link valid |
| Engines reference skills/ | Grep | All links valid |
| Engines reference SSOT.md | Grep | Link valid |
| No broken internal links | Link checker | Zero broken links |

### 8. AGENTS.md Compliance

| Check | Method | Pass Criteria |
|-------|--------|--------------|
| Authority System followed | Authority matrix | No violations |
| Decision Matrix followed | Decision matrix | Correct routing |
| Workflow followed | Workflow definition | Correct phases |
| Quality gates enforced | Quality engine | All gates present |
| SSOT compliant | SSOT check | No conflicts |

## Validation Script

```bash
#!/usr/bin/env bash
# Validate Engineering Orchestrator
set -euo pipefail

ENGINEERING_DIR=".engineering"
PASS=0
FAIL=0

check() {
    local name="$1"
    local file="$2"
    if [ -f "$file" ]; then
        echo "  PASS: $name"
        PASS=$((PASS + 1))
    else
        echo "  FAIL: $name — not found"
        FAIL=$((FAIL + 1))
    fi
}

echo "=== Engineering Orchestrator Validation ==="

echo "[1/4] Entry Points"
check "README.md" "$ENGINEERING_DIR/README.md"
check "ENGINE.md" "$ENGINEERING_DIR/ENGINE.md"

echo "[2/4] Orchestrator Engines"
for f in routing execution-engine planning-engine approval-engine conflict-engine escalation-engine quality-engine reporting-engine state-machine documentation-engine; do
    check "$f.md" "$ENGINEERING_DIR/orchestrator/$f.md"
done

echo "[3/4] Pipelines"
for f in standard feature bugfix optimization refactor documentation release emergency; do
    check "$f.md" "$ENGINEERING_DIR/pipelines/$f.md"
done

echo "[4/4] Templates & Examples"
for f in execution-plan-template impact-analysis-template quality-report-template completion-report-template approval-record-template conflict-record-template; do
    check "$f.md" "$ENGINEERING_DIR/templates/$f.md"
done
for f in feature-example bugfix-example release-example; do
    check "$f.md" "$ENGINEERING_DIR/examples/$f.md"
done

echo ""
echo "Results: $PASS passed, $FAIL failed"
[ "$FAIL" -eq 0 ] && echo "STATUS: PASS" || echo "STATUS: FAIL"
exit "$FAIL"
```
