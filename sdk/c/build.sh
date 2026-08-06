#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
INCLUDE_DIR="$SCRIPT_DIR/include"
BUILD_DIR="$SCRIPT_DIR/build"

echo "Building KCM C SDK..."
mkdir -p "$BUILD_DIR/examples/basic" "$BUILD_DIR/tests"

echo "  Compiling basic example..."
gcc -Wall -Wextra -O2 -I"$INCLUDE_DIR" \
    "$SCRIPT_DIR/examples/basic/basic.c" \
    -o "$BUILD_DIR/examples/basic/basic" -lkcm

echo "  Compiling tests..."
gcc -Wall -Wextra -O2 -I"$INCLUDE_DIR" \
    "$SCRIPT_DIR/tests/test_kcm.c" \
    -o "$BUILD_DIR/tests/test_kcm" -lkcm

echo "Built:"
echo "  $BUILD_DIR/examples/basic/basic"
echo "  $BUILD_DIR/tests/test_kcm"
