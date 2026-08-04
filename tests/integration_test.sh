#!/bin/bash
set -e

echo "=== KCM CLI Integration Tests ==="
echo ""

PASS=0
FAIL=0

run_test() {
    local name=$1
    local cmd=$2
    echo -n "  $name ... "
    if eval "$cmd" > /dev/null 2>&1; then
        echo "PASS"
        PASS=$((PASS + 1))
    else
        echo "FAIL"
        FAIL=$((FAIL + 1))
    fi
}

echo "--- kcm-cli ---"
run_test "version" "cargo run --quiet -p kcm -- version"
run_test "create 100 facts" "cargo run --quiet -p kcm -- create --count 100"
run_test "stats" "cargo run --quiet -p kcm -- stats --count 100"
run_test "benchmark" "cargo run --quiet -p kcm -- benchmark --ops 100"

echo ""
echo "--- kcm-backup ---"
run_test "create backup" "cargo run --quiet -p kcm-backup -- create --count 100"

echo ""
echo "--- kcm-doctor ---"
run_test "health check" "cargo run --quiet -p kcm-doctor -- check"

echo ""
echo "--- kcm-inspect ---"
run_test "schema" "cargo run --quiet -p kcm-inspect -- schema"
run_test "columns" "cargo run --quiet -p kcm-inspect -- columns"
run_test "stats" "cargo run --quiet -p kcm-inspect -- stats --count 100"

echo ""
echo "--- kcm-bench ---"
run_test "run benchmark" "cargo run --quiet -p kcm-bench -- run"

echo ""
echo "--- kcm-perf ---"
run_test "analyze" "cargo run --quiet -p kcm-perf -- analyze"

echo ""
echo "--- kcm-migrate ---"
run_test "status" "cargo run --quiet -p kcm-migrate -- status"

echo ""
echo "--- kcm-diagnose ---"
run_test "full diagnostics" "cargo run --quiet -p kcm-diagnose -- full"

echo ""
echo "--- kcm-export ---"
run_test "json export" "cargo run --quiet -p kcm-export -- json --output /tmp/kcm_test.json --count 10"

echo ""
echo "--- kcm-compact ---"
run_test "compact" "cargo run --quiet -p kcm-compact -- run --count 100"

echo ""
echo "--- kcm-profile ---"
run_test "insert profile" "cargo run --quiet -p kcm-profile -- insert --ops 100"

echo ""
echo "--- kcm-schema ---"
run_test "show schema" "cargo run --quiet -p kcm-schema -- show"

echo ""
echo "--- kcm-snapshot ---"
run_test "create snapshot" "cargo run --quiet -p kcm-snapshot -- create"

echo ""
echo "=== Results ==="
echo "  Passed: $PASS"
echo "  Failed: $FAIL"
echo "  Total:  $((PASS + FAIL))"
echo ""

if [ $FAIL -eq 0 ]; then
    echo "ALL TESTS PASSED"
else
    echo "SOME TESTS FAILED"
    exit 1
fi
