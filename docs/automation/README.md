# Documentation Automation

Automated documentation maintenance and governance tools.

## Automation Tools

| Tool | Purpose | Usage |
|------|---------|-------|
| `validate-docs.sh` | Validate required files and headings | `bash scripts/documentation/validate-docs.sh` |
| `validate-structure.sh` | Validate markdown structure | `bash scripts/documentation/validate-structure.sh` |
| `calculate-coverage.sh` | Calculate documentation coverage | `bash tools/doc-coverage/calculate-coverage.sh` |
| `check-links.sh` | Check for broken links | `bash tools/doc-link-checker/check-links.sh` |
| `detect-drift.sh` | Detect code-doc drift | `bash tools/doc-drift/detect-drift.sh` |
| `ssot-check.sh` | Validate SSOT alignment | `bash tools/doc-drift/ssot-check.sh` |
| `generate-index.sh` | Generate documentation index | `bash tools/doc-generator/generate-index.sh` |
| `generate-search-index.sh` | Generate search index | `bash tools/doc-generator/generate-search-index.sh` |
| `generate-health-report.sh` | Generate health report | `bash scripts/documentation/generate-health-report.sh` |

## Automation Schedule

| Automation | Trigger | Frequency |
|-----------|---------|-----------|
| Documentation validation | Every push/PR | Continuous |
| Coverage calculation | Every push/PR | Continuous |
| Link checking | Every push/PR | Continuous |
| Drift detection | Every push/PR | Continuous |
| SSOT validation | Every push/PR | Continuous |
| Index regeneration | After merge to main | On merge |
| Search index update | After merge to main | On merge |
| Health report | Weekly | Scheduled |

## CI Pipeline

```
Push/PR → Lint → Validate → Link Check → SSOT Check → Coverage → Report → Artifact
```

## Governance

All automation is governed by the [Documentation Governance](../governance/documentation-governance.md) system.
