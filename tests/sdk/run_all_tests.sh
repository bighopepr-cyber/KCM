#!/bin/bash
set -e

# KCM SDK Test Runner
# Runs all SDK tests: mock server validation, cross-language consistency, API compliance

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPORT_DIR="$SCRIPT_DIR/reports"
MOCK_SERVER_PID=""

cleanup() {
    if [ -n "$MOCK_SERVER_PID" ] && kill -0 "$MOCK_SERVER_PID" 2>/dev/null; then
        echo "Stopping mock server (PID $MOCK_SERVER_PID)..."
        kill "$MOCK_SERVER_PID" 2>/dev/null || true
        wait "$MOCK_SERVER_PID" 2>/dev/null || true
    fi
}

trap cleanup EXIT

mkdir -p "$REPORT_DIR"

PASS=0
FAIL=0
TOTAL=0

run_test() {
    local name=$1
    local cmd=$2
    local timeout=${3:-60}
    TOTAL=$((TOTAL + 1))
    echo -n "  $name ... "
    if timeout "$timeout" bash -c "$cmd" > "$REPORT_DIR/${name}.log" 2>&1; then
        echo "PASS"
        PASS=$((PASS + 1))
    else
        echo "FAIL (see $REPORT_DIR/${name}.log)"
        FAIL=$((FAIL + 1))
    fi
}

echo "============================================"
echo "KCM SDK TEST SUITE"
echo "============================================"
echo ""

# Step 1: Start mock server
echo "--- Starting Mock Server ---"
python3 "$SCRIPT_DIR/mock_server.py" --port 8080 &
MOCK_SERVER_PID=$!
sleep 2

# Verify mock server is running
if ! kill -0 "$MOCK_SERVER_PID" 2>/dev/null; then
    echo "FATAL: Mock server failed to start"
    exit 1
fi
echo "  Mock server running on PID $MOCK_SERVER_PID"
echo ""

# Step 2: Validate mock server health
echo "--- Mock Server Health ---"
run_test "mock_server_health" "curl -sf http://127.0.0.1:8080/health"
echo ""

# Step 3: Cross-language consistency tests
echo "--- Cross-Language Consistency Tests ---"
run_test "cross_language_tests" "python3 $SCRIPT_DIR/cross_language_test.py" 120
echo ""

# Step 4: API compliance validation
echo "--- API Compliance Validation ---"
run_test "api_compliance" "python3 $SCRIPT_DIR/validate_sdk_api.py" 60
echo ""

# Step 5: Mock server API endpoint validation
echo "--- Mock Server Endpoint Tests ---"
run_test "rest_health" "curl -sf http://127.0.0.1:8080/health"
run_test "rest_list_facts" "curl -sf http://127.0.0.1:8080/facts"
run_test "rest_get_stats" "curl -sf http://127.0.0.1:8080/stats"
run_test "rest_get_metrics" "curl -sf http://127.0.0.1:8080/metrics"
run_test "rest_insert_fact" "curl -sf -X POST http://127.0.0.1:8080/facts -H 'Content-Type: application/json' -d '{\"subject\":1,\"predicate\":2,\"object\":3,\"confidence\":0.95,\"evidence\":1,\"context\":1,\"priority\":0,\"owner\":1}'"
run_test "rest_get_not_found" "curl -sf -o /dev/null -w '%{http_code}' http://127.0.0.1:8080/facts/99999 | grep -q 404"
echo ""

# Step 6: Validate consistency matrix integrity
echo "--- Consistency Matrix Validation ---"
run_test "matrix_json_valid" "python3 -c \"import json; json.load(open('$SCRIPT_DIR/consistency_matrix.json'))\""
run_test "matrix_has_test_cases" "python3 -c \"import json; d=json.load(open('$SCRIPT_DIR/consistency_matrix.json')); assert len(d.get('test_cases',[])) >= 8, f'Expected >= 8 test cases, got {len(d.get(\\\"test_cases\\\",[]))}'\""
run_test "matrix_has_ffi_functions" "python3 -c \"import json; d=json.load(open('$SCRIPT_DIR/consistency_matrix.json')); assert len(d.get('ffi_functions',[])) == 18, f'Expected 18 FFI functions, got {len(d.get(\\\"ffi_functions\\\",[]))}'\""
run_test "matrix_has_fact_fields" "python3 -c \"import json; d=json.load(open('$SCRIPT_DIR/consistency_matrix.json')); assert len(d.get('fact_fields',[])) == 10, f'Expected 10 fact fields, got {len(d.get(\\\"fact_fields\\\",[]))}'\""
run_test "matrix_has_error_codes" "python3 -c \"import json; d=json.load(open('$SCRIPT_DIR/consistency_matrix.json')); assert len(d.get('error_codes',[])) == 8, f'Expected 8 error codes, got {len(d.get(\\\"error_codes\\\",[]))}'\""
echo ""

# Summary
echo "============================================"
echo "RESULTS SUMMARY"
echo "============================================"
echo "  Passed: $PASS"
echo "  Failed: $FAIL"
echo "  Total:  $TOTAL"
echo ""

if [ $FAIL -eq 0 ]; then
    echo "ALL SDK TESTS PASSED"
    exit 0
else
    echo "SOME SDK TESTS FAILED"
    echo ""
    echo "Review logs in: $REPORT_DIR/"
    exit 1
fi
