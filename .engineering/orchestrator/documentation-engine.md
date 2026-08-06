# Documentation Engine

> Document ID: KCM-DOC-ENG-001 | Version: 1.0.0 | Status: Active

## Overview

The Documentation Engine ensures all documentation is updated as part of every engineering task. It identifies which documents must be updated and validates completeness.

## Documentation Update Rules

### Always Updated
| Document | Condition | Validator |
|----------|-----------|-----------|
| CHANGELOG.md | Version bump | `grep` |
| VERSION | Version change | `verify-version.sh` |

### Conditionally Updated
| Document | Condition | Validator |
|----------|-----------|-----------|
| `README.md` | Public API change, new feature | Manual review |
| `docs/<crate>/spesifikasi.md` | Crate change | Manual review |
| `docs/sdk/<lang>.md` | SDK change | Manual review |
| `docs/specs/KCM_*_SPEC.md` | Spec change | Manual review |
| `docs/adr/ADR-*.md` | Architectural decision | Manual review |
| `docs/runbook/OPERATIONAL_RUNBOOK.md` | Operational change | Manual review |
| `skills/<skill>/README.md` | Skill change | Manual review |

### Never Updated (Root Governance)
| Document | Reason |
|----------|--------|
| `SSOT.md` | Only P4 can update |
| `AGENTS.md` | Only P1 can update |
| `SECURITY.md` | Only P7 can update |
| `CONTRIBUTING.md` | Only P1 can update |
| `CODE_OF_CONDUCT.md` | Only P1 can update |

## Documentation Checklist by Task Type

### Feature
- [ ] `CHANGELOG.md` — New entry
- [ ] `docs/<crate>/spesifikasi.md` — Updated if crate changed
- [ ] `docs/sdk/<lang>.md` — Updated if SDK changed
- [ ] `docs/specs/` — Updated if spec changed
- [ ] `README.md` — Updated if public API changed

### Bug Fix
- [ ] `CHANGELOG.md` — New entry
- [ ] Root cause documented in report

### Optimization
- [ ] `CHANGELOG.md` — New entry
- [ ] Performance results documented
- [ ] `docs/<crate>/spesifikasi.md` — Updated if behavior changed

### Security
- [ ] `CHANGELOG.md` — New entry
- [ ] `SECURITY.md` — Updated if policy changed
- [ ] Security advisory (if applicable)

### Documentation
- [ ] All changed documents validated
- [ ] Cross-references validated
- [ ] SSOT alignment validated

### Refactoring
- [ ] `CHANGELOG.md` — New entry
- [ ] No documentation changes needed (behavior unchanged)

### Release
- [ ] `CHANGELOG.md` — Complete entry
- [ ] `VERSION` — Updated
- [ ] Git tag created
- [ ] All docs validated

## Documentation Validation

| Check | Method | Pass Criteria |
|-------|--------|--------------|
| All required docs updated | Checklist | All items checked |
| No broken links | Link checker | Zero broken links |
| SSOT aligned | SSOT check | No conflicts |
| Cross-references valid | Reference check | All refs valid |
| Format consistent | Format check | Consistent style |

## Documentation Report Format

```markdown
# Documentation Report

**Task ID:** {{TASK_ID}}
**Date:** {{DATE}}

## Updated Documents
| Document | Change Type | Status |
|----------|-------------|--------|
| {{DOC}} | {{TYPE}} | {{STATUS}} |

## Validation Results
| Check | Status | Details |
|-------|--------|---------|
| {{CHECK}} | {{STATUS}} | {{DETAILS}} |

## Summary
- **Documents Updated:** {{UPDATED}}
- **Documents Validated:** {{VALIDATED}}
- **Status:** {{STATUS}}
```
