#!/bin/bash
set -euo pipefail

echo "=== KCM Benchmark Comparison ==="
echo "Date: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
echo ""

RESULTS_DIR="benchmark-results"
mkdir -p "${RESULTS_DIR}/comparisons"

echo "Running benchmarks..."
cargo bench --workspace 2>&1 | tee "${RESULTS_DIR}/raw/bench_current.log"

BASELINE="${RESULTS_DIR}/raw/bench_baseline.log"
if [ ! -f "$BASELINE" ]; then
    echo "No baseline found. Saving current run as baseline."
    cp "${RESULTS_DIR}/raw/bench_current.log" "$BASELINE"
    echo "Baseline saved to $BASELINE"
    exit 0
fi

echo ""
echo "Comparing against baseline..."
echo "Baseline: $BASELINE"
echo "Current:  ${RESULTS_DIR}/raw/bench_current.log"
echo ""
echo "To compare: diff $BASELINE ${RESULTS_DIR}/raw/bench_current.log"
echo "Criterion automatically detects regressions in target/criterion/"
