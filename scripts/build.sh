#!/bin/bash
set -e

echo "Building KCM..."

cargo build --release --workspace

RUSTFLAGS="-C target-cpu=native -C target-feature=+avx2" \
    cargo build --release --workspace

cargo test --release --all

cargo bench --all

echo "Build complete!"
