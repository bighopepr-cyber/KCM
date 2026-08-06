# Contributing to benchmark-results/

> For core engine contribution rules, refer to the repository [CONTRIBUTING.md](../CONTRIBUTING.md).

## Overview

This document provides guidelines for contributing to the `benchmark-results/` directory, including benchmark data management, report generation, and baseline maintenance.

## Before Contributing

1. Read the repository [CONTRIBUTING.md](../CONTRIBUTING.md) for general contribution rules
2. Review [PRD-TESTING & BENCHMARK](../docs/PRD-TESTING&%20BRACHMARCK.md) for benchmark methodology
3. Understand the regression thresholds (5% warning, 10% failure)
4. Ensure all quality gates pass before submitting changes

## Coding Standards

### JSON Schema

All JSON files must:

- Use 2-space indentation
- Include trailing newlines
- Conform to their expected schema definitions
- Use `snake_case` for property names
- Include version fields where applicable

### CSV Format

CSV files must:

- Use comma as the delimiter
- Include a header row
- Use `snake_case` for column names
- Not include trailing whitespace
- Use UTF-8 encoding

## Module Architecture Rules

| Rule | Description |
|------|-------------|
| SR-01 | Each file has a single responsibility |
| SR-02 | Metadata files must not contain benchmark data |
| SR-03 | Raw output must be immutable once generated |
| SR-04 | Reports are derived data and may be regenerated |
| SR-05 | Baseline files must be version controlled |

## Documentation Rules

- README.md must be updated when folder structure changes
- SECURITY.md must be updated when sensitive assets change
- CONTRIBUTING.md must be updated when contribution rules change
- All reports must include generation timestamps

## Testing Requirements

### Benchmark Validation

- All benchmarks must complete without errors
- Benchmark results must be within expected ranges
- No benchmark may consistently fail across multiple runs
- Regression detection must be validated against known baselines

### Validation Commands

```bash
# Run all benchmarks
cargo bench --workspace

# Run tests
cargo test --workspace

# Validate benchmark output
bash scripts/validate-ssot.sh
```

## Performance Rules

| Threshold | Action | Example |
|-----------|--------|---------|
| 0-5% regression | Warning | Noted in report, no action required |
| 5-10% regression | Review required | Must be investigated before merge |
| >10% regression | Failure | Blocks merge, must be resolved |

### Regression Detection

Regression detection compares current benchmark results against the stored baseline:

1. Read `baseline.json` as reference
2. Execute current benchmark suite
3. Calculate percentage change for each benchmark
4. Apply threshold rules:
   - **5%**: Warning flag in report
   - **10%**: Failure flag, blocks CI

## Review Checklist

- [ ] JSON files validate against schema
- [ ] CSV files have proper headers
- [ ] No secrets or credentials in data files
- [ ] Environment metadata is sanitized
- [ ] Baseline changes are justified
- [ ] Reports include generation timestamps
- [ ] Documentation reflects changes
- [ ] Regression thresholds are respected

## Pull Request Requirements

1. **Title**: Clear description of benchmark change
2. **Description**: Include rationale for baseline changes
3. **Evidence**: CI benchmark results showing no regressions
4. **Review**: At least one approval from performance engineer
5. **Checks**: All CI jobs pass including benchmark validation

### PR Template

```markdown
## Benchmark Changes

### What changed
- [ ] Baseline update
- [ ] Report format change
- [ ] Metadata update
- [ ] Documentation update

### Rationale
[Explain why the change is needed]

### Regression analysis
- [ ] No regressions > 5%
- [ ] No regressions > 10%
- [ ] All benchmarks passing
```

## References

- [Repository CONTRIBUTING.md](../CONTRIBUTING.md) - Core contribution rules
- [PRD-TESTING & BENCHMARK](../docs/PRD-TESTING&%20BRACHMARCK.md) - Benchmark methodology
- [Benchmark Report](reports/KCM_BENCHMARK_REPORT.md) - Current benchmark results
- [Performance Matrix](reports/KCM_PERFORMANCE_MATRIX.csv) - Tabular performance data
