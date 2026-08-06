# KCM Benchmark Results

## Overview

The `benchmark-results/` directory serves as the centralized storage and reporting location for all KCM benchmark output. It provides a structured repository for baseline performance data, environment metadata, raw benchmark output, and generated performance reports.

## Purpose

This module exists to:

- Store baseline benchmarks for performance regression tracking
- Track performance regressions across versions and environments
- Generate standardized performance reports in multiple formats (MD, JSON, CSV)
- Provide a single source of truth for benchmark history and trends

## Responsibilities

| Responsibility | Description |
|----------------|-------------|
| Baseline Management | Store and version control baseline benchmark results |
| Regression Detection | Compare current runs against baselines to detect regressions |
| Report Generation | Produce human-readable and machine-parseable performance reports |
| Environment Tracking | Record build environment details for reproducibility |
| Metadata Management | Maintain versioning and git information for each benchmark run |

## Folder Structure

```
benchmark-results/
├── README.md                        # This file
├── SECURITY.md                      # Security policy
├── CONTRIBUTING.md                  # Contribution guidelines
├── CODE_OF_CONDUCT.md               # Community guidelines
├── baseline.json                    # Baseline benchmark results
├── metadata/
│   ├── benchmark-version.json       # Benchmark framework version
│   ├── environment.json             # Build environment details
│   └── git.json                     # Git commit and branch info
├── raw/                             # Raw benchmark output (empty initially)
└── reports/
    ├── KCM_BENCHMARK_REPORT.json    # Machine-readable benchmark report
    ├── KCM_BENCHMARK_REPORT.md      # Human-readable benchmark report
    └── KCM_PERFORMANCE_MATRIX.csv   # Performance comparison matrix
```

## Public API

This module does not expose a public API. It is a data storage directory consumed by tooling and reporting scripts.

## Internal Components

| Component | File | Description |
|-----------|------|-------------|
| Baseline Storage | `baseline.json` | Primary benchmark baseline data |
| Benchmark Version | `metadata/benchmark-version.json` | Framework version metadata |
| Environment Info | `metadata/environment.json` | Build environment snapshot |
| Git Metadata | `metadata/git.json` | Repository state at benchmark time |
| JSON Report | `reports/KCM_BENCHMARK_REPORT.json` | Structured benchmark report |
| Markdown Report | `reports/KCM_BENCHMARK_REPORT.md` | Formatted benchmark report |
| Performance Matrix | `reports/KCM_PERFORMANCE_MATRIX.csv` | Tabular performance data |

## Dependencies

This module depends on:

| Dependency | Source | Purpose |
|------------|--------|---------|
| kcm-testing | Workspace | Benchmark test execution |
| criterion | External | Criterion benchmarking framework |

## Integration

This module integrates with the following components:

| Component | Direction | Integration Type |
|-----------|-----------|-----------------|
| `cargo bench` | Inbound | Feeds benchmark output into `raw/` and `reports/` |
| `bench-compare.py` | Outbound | Reads baseline and reports for regression analysis |
| CI Pipeline | Bidirectional | Generates reports on push, reads for quality gates |

## Build

This is a data directory and does not require compilation.

## Run

Generate benchmarks by running:

```bash
cargo bench --workspace
```

The benchmark runner will:

1. Execute all criterion benchmarks defined in workspace crates
2. Generate raw output in `raw/`
3. Update `baseline.json` if no existing baseline is present
4. Generate reports in `reports/`
5. Record metadata in `metadata/`

## Test

Benchmark validation can be performed using:

```bash
cargo test --workspace
```

Ensure all benchmark-related tests pass before committing new baseline data.

## Examples

### Reading a baseline

The `baseline.json` file contains structured benchmark results:

```json
{
  "version": "0.1.0",
  "timestamp": "2026-08-06T18:41:51Z",
  "benchmarks": {
    "storage_insert": {
      "mean_ns": 1250.0,
      "std_dev_ns": 45.2,
      "sample_size": 100
    }
  }
}
```

### Generating a report

After running benchmarks, the reports directory will contain updated files ready for analysis or CI consumption.

## References

- [PRD-TESTING & BENCHMARK](../docs/PRD-TESTING&%20BRACHMARCK.md) - Benchmark methodology and targets
- [Performance Matrix](reports/KCM_PERFORMANCE_MATRIX.csv) - Tabular performance data
- [Benchmark Report](reports/KCM_BENCHMARK_REPORT.md) - Human-readable report
