# Documentation Validation

Validation rules and configuration for documentation quality gates.

## Validation Rules

### Required Files

Every folder must have:

| File | Description |
|------|-------------|
| `README.md` | Folder overview and documentation |
| `SECURITY.md` | Security policy |
| `CONTRIBUTING.md` | Contribution guidelines |
| `CODE_OF_CONDUCT.md` | Community guidelines |

### Required Headings

| Document | Required Headings |
|----------|------------------|
| SECURITY.md | Overview, Security Scope, Threat Model, Secure Development Rules, Validation Checklist |
| CONTRIBUTING.md | Overview, Coding Standards, Testing Requirements, Review Checklist |
| CODE_OF_CONDUCT.md | Respect, Professional Communication, Code Review Etiquette |
| spesifikasi.md | Overview, Scope, Responsibilities, Technical Specification, SSOT Alignment, References |

### SSOT Requirements

- All spesifikasi files must have an SSOT Alignment section
- All spesifikasi files must reference SSOT.md and AGENTS.md
- All documentation must be traceable to SSOT requirements

## Running Validation

```bash
bash scripts/documentation/validate-docs.sh
bash scripts/documentation/validate-structure.sh
```

## CI Integration

Validation runs automatically on every push and pull request via `.github/workflows/docs.yml`.
