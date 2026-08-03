#!/bin/bash
set -euo pipefail

echo "=== KCM Test Suite ==="
echo "Date: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
echo ""

echo "Step 1: Unit tests..."
cargo test --lib --all

echo "Step 2: Full workspace tests..."
cargo test --workspace

echo "Step 3: Security tests..."
cargo test security_tests --all -- --nocapture

echo "Step 4: Load tests..."
cargo test load_tests --all -- --nocapture

echo "Step 5: Stress tests..."
cargo test stress_tests --all -- --nocapture

echo "Step 6: Recovery tests..."
cargo test recovery --all -- --nocapture

echo "Step 7: Integration tests..."
cargo test --test '*' --all

echo "Step 8: Documentation tests..."
cargo test --doc --all

echo ""
echo "=== All Tests Passed ==="
cargo test --workspace 2>&1 | grep -E "^test result:" | awk '{passed += $4; failed += $8} END {printf "Total: %d passed, %d failed\n", passed, failed}'
