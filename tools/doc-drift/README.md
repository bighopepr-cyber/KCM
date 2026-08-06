# doc-drift

Documentation drift detection tools for the KCM repository.

## Tools

### detect-drift.sh

Detects drift between code structure and documentation.

Checks:
- Crates in Cargo.toml vs documentation
- Public modules vs README/spesifikasi
- REST endpoints vs API documentation
- SDK directories vs SDK documentation
- Workflow files vs documentation
- CLI tools vs documentation

### ssot-check.sh

Validates SSOT alignment across all documentation.

Checks:
- SSOT.md exists
- AGENTS.md exists
- PRD documents exist
- Spesifikasi files have SSOT Alignment sections
- Spesifikasi files have References sections
- Documents reference SSOT documents

## Usage

```bash
bash tools/doc-drift/detect-drift.sh
bash tools/doc-drift/ssot-check.sh
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | No drift / SSOT compliant |
| 1 | Drift detected / SSOT violations |
