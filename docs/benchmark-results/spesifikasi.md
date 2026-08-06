# Benchmark Results Technical Specification

## Overview

This document provides the technical specification for the `benchmark-results/` module, defining its data structures, formats, execution flows, and integration points within the KCM system.

## Scope

This specification covers:

- Benchmark data storage and formatting
- Baseline management and versioning
- Regression detection algorithms
- Report generation in multiple formats
- Integration with CI/CD pipelines

## Responsibilities

| Responsibility | Description | Owner |
|----------------|-------------|-------|
| Benchmark Storage | Store and organize benchmark output files | benchmark-results |
| Baseline Management | Maintain and version baseline performance data | benchmark-results |
| Regression Detection | Compare current runs against baselines | kcm-testing |
| Report Generation | Produce standardized performance reports | benchmark-results |

## Technical Specification

### baseline.json Format

```json
{
  "version": "0.1.0",
  "timestamp": "2026-08-06T18:41:51Z",
  "benchmarks": {
    "<benchmark_name>": {
      "mean_ns": 1250.0,
      "std_dev_ns": 45.2,
      "sample_size": 100,
      "unit": "ns"
    }
  }
}
```

### Metadata Structure

#### benchmark-version.json

```json
{
  "framework": "criterion",
  "version": "0.5.1",
  "kcm_version": "0.1.0",
  "generated_at": "2026-08-06T18:41:51Z"
}
```

#### environment.json

```json
{
  "os": "linux",
  "arch": "x86_64",
  "cpu": "Intel(R) Core(TM) i7-12700K",
  "memory_bytes": 34359738368,
  "rust_version": "1.75.0",
  "optimization_level": "release"
}
```

#### git.json

```json
{
  "commit_hash": "abc123def456",
  "branch": "main",
  "dirty": false,
  "tag": "v0.1.0"
}
```

### Report Formats

#### Markdown Report (KCM_BENCHMARK_REPORT.md)

```markdown
# KCM Benchmark Report
Generated: 2026-08-06T18:41:51Z
Version: 0.1.0

## Summary
| Benchmark | Mean | Std Dev | Change |
|-----------|------|---------|--------|
| storage_insert | 1.25μs | 45.2ns | +0.5% |

## Regression Analysis
- No regressions detected
```

#### JSON Report (KCM_BENCHMARK_REPORT.json)

```json
{
  "generated_at": "2026-08-06T18:41:51Z",
  "version": "0.1.0",
  "summary": {
    "total_benchmarks": 15,
    "regressions": 0,
    "improvements": 2
  },
  "results": [
    {
      "name": "storage_insert",
      "mean_ns": 1250.0,
      "std_dev_ns": 45.2,
      "change_percent": 0.5,
      "status": "pass"
    }
  ]
}
```

#### CSV Report (KCM_PERFORMANCE_MATRIX.csv)

```csv
benchmark,mean_ns,std_dev_ns,change_percent,status
storage_insert,1250.0,45.2,0.5,pass
storage_query,2100.0,89.3,-1.2,pass
inference_forward,3400.0,120.5,0.3,pass
```

### Regression Thresholds

| Threshold | Classification | Action |
|-----------|---------------|--------|
| 0-5% | Warning | Noted in report, no block |
| 5-10% | Review Required | Must investigate before merge |
| >10% | Failure | Blocks merge, must resolve |

## Architecture

### Data Flow

```
cargo bench
    │
    ▼
┌─────────────────┐
│ Benchmark Runner │
│   (criterion)    │
└────────┬────────┘
         │
         ▼
┌─────────────────┐     ┌─────────────────┐
│  raw/ (output)  │────▶│  Compare Engine  │
└─────────────────┘     └────────┬────────┘
                                 │
         ┌───────────────────────┼───────────────────────┐
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│  baseline.json  │     │  metadata/      │     │  reports/       │
└─────────────────┘     └─────────────────┘     └─────────────────┘
```

## Internal Components

| Component | Responsibility | Location |
|-----------|---------------|----------|
| Baseline Manager | Store and retrieve baseline data | `baseline.json` |
| Metadata Collector | Gather environment and git info | `metadata/` |
| Report Generator | Produce reports in multiple formats | `reports/` |
| Regression Analyzer | Compare results against baselines | `kcm-testing` |

## Data Model

### JSON Schemas

#### Baseline Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": ["version", "timestamp", "benchmarks"],
  "properties": {
    "version": { "type": "string" },
    "timestamp": { "type": "string", "format": "date-time" },
    "benchmarks": {
      "type": "object",
      "additionalProperties": {
        "type": "object",
        "required": ["mean_ns", "std_dev_ns", "sample_size"],
        "properties": {
          "mean_ns": { "type": "number" },
          "std_dev_ns": { "type": "number" },
          "sample_size": { "type": "integer" },
          "unit": { "type": "string" }
        }
      }
    }
  }
}
```

## Execution Flow

### Benchmark Run → Collect → Compare → Report

```
1. Execute Benchmark
   cargo bench --workspace
       │
       ▼
2. Collect Results
   - Parse criterion output
   - Store in raw/
   - Update metadata/
       │
       ▼
3. Compare Against Baseline
   - Load baseline.json
   - Calculate deltas
   - Apply threshold rules
       │
       ▼
4. Generate Reports
   - Create KCM_BENCHMARK_REPORT.json
   - Create KCM_BENCHMARK_REPORT.md
   - Create KCM_PERFORMANCE_MATRIX.csv
       │
       ▼
5. Update Baseline
   - If no regressions, update baseline.json
   - Commit with benchmark data
```

## Public API

This module does not expose a public API. It is a data directory consumed by:

| Consumer | Access Pattern |
|----------|---------------|
| CI Pipeline | Write benchmark output, read reports |
| bench-compare.py | Read baseline, read reports |
| Developers | Read reports for review |

## Configuration

| Configuration | Default | Description |
|--------------|---------|-------------|
| Warning Threshold | 5% | Performance regression warning |
| Failure Threshold | 10% | Performance regression failure |
| Sample Size | 100 | Minimum benchmark samples |
| Confidence Level | 95% | Statistical confidence interval |

## Dependencies

| Dependency | Version | Purpose |
|------------|---------|---------|
| criterion | 0.5.1 | Benchmark execution |
| kcm-testing | 0.1.0 | Benchmark test infrastructure |

## Error Handling

| Error | Cause | Resolution |
|-------|-------|------------|
| Invalid JSON | Malformed baseline file | Validate schema, restore from git |
| Missing baseline | First run | Create new baseline |
| Benchmark timeout | Performance degradation | Investigate regression |
| Schema mismatch | Version incompatibility | Update to matching version |

## Performance Characteristics

| Metric | Target | Description |
|--------|--------|-------------|
| Benchmark overhead | < 1% | Overhead of benchmarking framework |
| Report generation | < 1s | Time to generate all reports |
| Comparison time | < 500ms | Time to compare against baseline |
| Storage size | < 1MB | Total size of benchmark results |

## Security Considerations

- No secrets or credentials in benchmark data
- Environment metadata must be sanitized
- Baseline files must be integrity-checked
- See [SECURITY.md](../benchmark-results/SECURITY.md) for full policy

## Integration

### CI Pipeline Integration

```yaml
benchmarks:
  stage: test
  script:
    - cargo bench --workspace
    - python3 scripts/bench-compare.py
  artifacts:
    paths:
      - benchmark-results/
    expire_in: 30 days
```

### bench-compare.py Integration

```python
# Read baseline
with open('benchmark-results/baseline.json') as f:
    baseline = json.load(f)

# Compare with current results
for name, result in current_results.items():
    baseline_result = baseline['benchmarks'].get(name)
    if baseline_result:
        change = (result['mean_ns'] - baseline_result['mean_ns']) / baseline_result['mean_ns']
        if change > 0.10:
            print(f"FAILURE: {name} regressed by {change*100:.1f}%")
```

## Sequence Diagram

```
┌─────┐     ┌──────────┐     ┌─────────────┐     ┌─────────────┐
│ CI  │────▶│ cargo    │────▶│ raw/        │────▶│ Compare     │
│     │     │ bench    │     │             │     │ Engine      │
└─────┘     └──────────┘     └─────────────┘     └──────┬──────┘
                                                         │
                    ┌────────────────────────────────────┼────────────────────────────────────┐
                    │                                    │                                    │
                    ▼                                    ▼                                    ▼
             ┌─────────────┐                     ┌─────────────┐                     ┌─────────────┐
             │ baseline    │                     │ reports/    │                     │ metadata/   │
             │ .json       │                     │             │                     │             │
             └─────────────┘                     └─────────────┘                     └─────────────┘
```

## Architecture Diagram

```
                    ┌─────────────────────────────────────────────┐
                    │           benchmark-results/                │
                    ├─────────────────────────────────────────────┤
                    │                                             │
                    │  ┌──────────────┐  ┌──────────────────┐   │
                    │  │ baseline.json│  │    metadata/     │   │
                    │  └──────────────┘  └──────────────────┘   │
                    │                                             │
                    │  ┌──────────────┐  ┌──────────────────┐   │
                    │  │    raw/      │  │    reports/      │   │
                    │  └──────────────┘  └──────────────────┘   │
                    │                                             │
                    └─────────────────────────────────────────────┘
                                       │
                    ┌──────────────────┼──────────────────┐
                    │                  │                  │
                    ▼                  ▼                  ▼
             ┌─────────────┐  ┌─────────────┐  ┌─────────────┐
             │ cargo bench │  │ kcm-testing │  │  CI/CD      │
             └─────────────┘  └─────────────┘  └─────────────┘
```

## References

- [PRD-TESTING & BENCHMARK](../PRD-TESTING&%20BRACHMARCK.md) - Benchmark methodology and targets
- [PRD3 §27](../PRD3.md) - Distributed architecture
- [benchmark-results/README.md](../benchmark-results/README.md) - Module overview
- [benchmark-results/SECURITY.md](../benchmark-results/SECURITY.md) - Security policy

## SSOT Alignment

| SSOT Document | Section | Alignment |
|---------------|---------|-----------|
| PRD-TESTING & BENCHMARK | §4 | Benchmark suite configuration |
| PRD-TESTING & BENCHMARK | §1-8 | Testing strategy and quality gates |
| PRD3 | §27 | Performance targets |
| AGENTS.md | Engineering Gates | Quality gate validation |

This specification is aligned with the KCM Single Source of Truth documentation hierarchy. When this document conflicts with higher-priority documents, the SSOT document takes precedence.
