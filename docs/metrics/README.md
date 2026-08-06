# Documentation Metrics

Documentation metrics and coverage reports for the KCM repository.

## Generated Files

| File | Format | Description |
|------|--------|-------------|
| `coverage.json` | JSON | Machine-readable coverage data |
| `coverage.md` | Markdown | Human-readable coverage report |
| `coverage.html` | HTML | Visual coverage dashboard |

## Regenerating Metrics

```bash
bash tools/doc-coverage/calculate-coverage.sh
```

## Coverage Categories

| Category | Description | Weight |
|----------|-------------|--------|
| Folder Coverage | Required files per folder | 40% |
| Crate Coverage | Spesifikasi files per crate | 20% |
| SSOT Coverage | SSOT Alignment sections | 15% |
| Heading Coverage | Required headings | 15% |
| Reference Coverage | References sections | 10% |

## Quality Gates

- Overall coverage must be 100% to pass CI
- Any category below 100% generates a warning
- Coverage regressions block merge
