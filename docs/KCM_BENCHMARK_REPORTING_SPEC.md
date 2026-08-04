# KCM Benchmark Reporting Specification

**Document ID:** KCM-BENCH-001
**Version:** 1.0.0
**Status:** Derived
**Depends on:** PRD-TESTING&BRACHMARCK.md

---

## 1. Purpose

Defines the standard for benchmark execution, reporting, and artifact management in KCM. This specification ensures that performance measurements are reproducible, auditable, and comparable across environments.

---

## 2. Benchmark Execution Standard

### 2.1 Tool

All micro-benchmarks use Criterion.rs 0.5+ with default configuration:
- Minimum 10 iterations per sample
- 95% confidence intervals
- Automatic outlier detection

### 2.2 Profile

Benchmarks must run with the `bench` profile:
```toml
[profile.bench]
inherits = "release"
```

This enables LTO and full optimization while maintaining debug information for profiling.

### 2.3 Execution Command

```bash
cargo bench --workspace
```

### 2.4 Environment Requirements

| Parameter | Requirement |
|-----------|------------|
| OS | Linux x86_64 (primary target) |
| CPU | ≥ 8 cores, ≥ 3.0 GHz |
| RAM | ≥ 16 GB |
| Storage | SSD recommended |
| Rust | Stable toolchain |
| Network | Not required |

### 2.5 Reproducibility Rules

- Run on identical hardware or record environment differences
- Never modify benchmark workloads to improve results
- Never reduce dataset sizes to avoid timeouts
- Use fixed Criterion configuration
- Record all environment metadata

---

## 3. Artifact Format

### 3.1 Directory Structure

```
benchmark-results/
├── reports/
│   ├── KCM_BENCHMARK_REPORT.md      # Human-readable report
│   ├── KCM_BENCHMARK_SUMMARY.json   # Machine-readable summary
│   └── KCM_PERFORMANCE_MATRIX.csv   # CSV for analysis
├── raw/
│   ├── build.log                    # Build output
│   ├── bench.log                    # Benchmark output
│   └── criterion-results/           # Criterion stored baselines
└── metadata/
    ├── environment.json             # Hardware/software info
    ├── git.json                     # Repository info
    └── benchmark-version.json       # Benchmark version
```

### 3.2 Environment Metadata

```json
{
    "os": "Linux 6.x",
    "cpu": "Intel Xeon ...",
    "cores": 8,
    "ram_mb": 32768,
    "rust_version": "rustc 1.XX.0",
    "llvm_version": "LLVM version 17.0.0"
}
```

### 3.3 Git Metadata

```json
{
    "commit": "abc123def456...",
    "branch": "main",
    "timestamp": "2026-07-31T00:00:00Z",
    "message": "commit message"
}
```

### 3.4 Benchmark Summary (JSON)

```json
{
    "benchmark_version": "1.0.0",
    "results": [
        {
            "name": "column_sequential_scan/1000000",
            "duration_ns": 1234567.0,
            "throughput_ops_sec": 810000
        }
    ],
    "total_benchmarks": 29
}
```

### 3.5 Performance Matrix (CSV)

```csv
benchmark,duration_ns,throughput_ops_sec
column_sequential_scan/1000000,1234567,810000
```

---

## 4. Report Structure

### 4.1 KCM_BENCHMARK_REPORT.md

```markdown
# KCM Benchmark Report

## Environment
- OS: ...
- CPU: ...
- Cores: N
- RAM: N MB
- Rust: version
- Commit: hash
- Branch: name

## Performance Results
| Benchmark | Duration | Throughput |
|-----------|----------|------------|

## Summary
- Total benchmarks: N
```

### 4.2 Regression Policy

| Change | Threshold | Action |
|--------|-----------|--------|
| ≤ 5% regression | PASS | No action |
| 5-10% regression | WARNING | Investigate |
| > 10% regression | FAIL | Must fix before merge |

Regressions are measured against the stored Criterion baselines in `target/criterion/`.

---

## 5. CI Integration

### 5.1 Workflow Triggers

| Trigger | Action |
|---------|--------|
| Push to main | Run benchmarks, upload artifacts |
| Manual dispatch | Run benchmarks, upload artifacts |
| Weekly schedule | Run benchmarks for trend tracking |

### 5.2 Artifact Retention

Benchmark artifacts are retained for 90 days in GitHub Actions.

### 5.3 Quality Gate

The benchmark CI job:
- Builds benchmarks in release mode
- Executes all benchmarks
- Generates reports
- Uploads artifacts
- **Fails CI** if regression > 10% (per Regression Policy §4.2)
- **Warns** if regression > 5% (per Regression Policy §4.2)

The job fails unconditionally only if:
- Benchmark compilation fails
- Report generation fails

---

## 6. Benchmark Inventory

Per PRD-TESTING&BRACHMARCK.md §5.1 (authoritative), 34 benchmarks:

| Category | Count | Coverage |
|----------|-------|----------|
| Column Operations | 11 | Sequential scan (4 scales), random access (4 scales), SIMD filter (3 scales) |
| Bitmap Operations | 8 | Set (3 scales), count_ones (3 scales), bitwise (2 scales) |
| Dictionary Operations | 6 | Insert (3 scales), lookup (3 scales) |
| Database Operations | 6 | Insert (3 scales), query (3 scales) |
| Inference Operations | 3 | Pattern matching (3 scales) |
| **Total** | **34** | |

---

## 7. References

- **Depends on:** KCM_SPECIFICATION (KCM_SPECIFICATION)
- **Parent specs:** KCM_SPECIFICATION (KCM_SPECIFICATION)
- **Related:** KCM_PERFORMANCE_SPEC (KCM_PERFORMANCE_SPEC), KCM_ENGINEERING_RULES (KCM_ENGINEERING_RULES)
