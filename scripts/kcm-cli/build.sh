#!/bin/bash
set -euo pipefail

echo "=== KCM Build ==="
echo "Date: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
echo "Rust: $(rustc --version)"
echo ""

echo "Step 1: Format check..."
cargo fmt --all -- --check

echo "Step 2: Clippy..."
cargo clippy --workspace -- -D warnings

echo "Step 3: Debug build..."
cargo build --workspace

echo "Step 4: Release build..."
cargo build --release --workspace

echo "Step 5: Compile benchmarks..."
cargo bench --workspace --no-run

echo ""
echo "=== Build Complete ==="
