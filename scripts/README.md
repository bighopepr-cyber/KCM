# KCM Build Scripts

Build automation and utility scripts for development and CI/CD.

## Scripts

| Script | Purpose | Usage |
|--------|---------|-------|
| build.sh | Full build pipeline | ./scripts/build.sh |
| test.sh | Run all test suites | ./scripts/test.sh |
| bench-report.sh | Generate benchmark report | ./scripts/bench-report.sh |
| bench-compare.sh | Compare benchmark results | ./scripts/bench-compare.sh |
| bench-compare.py | Python regression detector | python3 scripts/bench-compare.py |

## build.sh

Runs the complete build pipeline:
1. Format check (cargo fmt)
2. Clippy lint (cargo clippy)
3. Debug build
4. Release build
5. Benchmark compilation

```bash
./scripts/build.sh
```

## test.sh

Runs all test suites:
1. Unit tests
2. Workspace tests
3. Security tests
4. Load tests
5. Stress tests
6. Recovery tests
7. Integration tests
8. Documentation tests

```bash
./scripts/test.sh
```

## bench-report.sh

Generates a complete benchmark report with environment metadata:
1. Collects system info (OS, CPU, RAM, Rust version)
2. Runs criterion benchmarks
3. Generates MD, JSON, CSV reports
4. Saves to benchmark-results/

```bash
./scripts/bench-report.sh
```

## bench-compare.py

Python script for benchmark regression detection:
- Parses Criterion output
- Compares against baseline
- Detects regressions (5% warn, 10% fail)
- Generates comparison report

```bash
python3 scripts/bench-compare.py --baseline baseline.json --current current.json
```
