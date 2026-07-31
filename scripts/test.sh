#!/bin/bash
set -e

echo "Running tests..."

cargo test --lib --all

cargo test --test '*' --all

cargo test --doc --all

echo "Tests complete!"
