#!/bin/bash
set -e
echo "Building KCM C SDK example..."
mkdir -p build
gcc -Wall -Wextra -O2 -I. -o build/kcm_example example.c -lkcm
echo "Built: build/kcm_example"
echo "Run: ./build/kcm_example"
