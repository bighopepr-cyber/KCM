# doc-coverage

Documentation coverage calculation tool for the KCM repository.

## Purpose

Calculates comprehensive documentation coverage metrics across all categories and generates reports in JSON, HTML, and Markdown formats.

## Usage

```bash
bash tools/doc-coverage/calculate-coverage.sh
```

## Output Files

| File | Format | Description |
|------|--------|-------------|
| `docs/metrics/coverage.json` | JSON | Machine-readable coverage data |
| `docs/metrics/coverage.md` | Markdown | Human-readable coverage report |
| `docs/metrics/coverage.html` | HTML | Visual coverage dashboard |

## Coverage Categories

| Category | Description |
|----------|-------------|
| Folder Coverage | Required files per folder (README, SECURITY, CONTRIBUTING, CODE_OF_CONDUCT) |
| Crate Coverage | Spesifikasi files per crate |
| SSOT Coverage | SSOT Alignment sections in spesifikasi files |
| Heading Coverage | Required headings in documentation files |
| Reference Coverage | References sections in documentation files |

## Quality Gates

- Overall coverage must be 100% to pass CI
- Any category below 100% generates a warning
