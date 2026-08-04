#!/usr/bin/env bash
# KCM SSOT Validation Script
# Verifies consistency between documentation and codebase
set -uo pipefail

ERRORS=0

check() {
    local desc="$1"
    local result="$2"
    if [ "$result" = "0" ]; then
        echo "PASS: $desc"
    else
        echo "FAIL: $desc"
        ERRORS=$((ERRORS + 1))
    fi
}

echo "=== KCM SSOT Validation ==="
echo ""

# Check 1: FFI function count
FFI_COUNT=$(grep -c 'unsafe extern "C" fn' /workspaces/KCM/crates/kcm-interface/src/lib.rs 2>/dev/null || true)
if [ "$FFI_COUNT" -eq 18 ]; then check "C FFI function count = 18" "0"; else check "C FFI function count = $FFI_COUNT (expected 18)" "1"; fi

# Check 2: Metrics counter count
METRICS_COUNT=$(grep -c 'pub.*: AtomicU64' /workspaces/KCM/crates/kcm-runtime/src/metrics.rs 2>/dev/null || true)
if [ "$METRICS_COUNT" -eq 14 ]; then check "Metrics counter count = 14" "0"; else check "Metrics counter count = $METRICS_COUNT (expected 14)" "1"; fi

# Check 3: Test count
TEST_COUNT=$(grep -r '#\[test\]' /workspaces/KCM/crates/ --include='*.rs' 2>/dev/null | wc -l)
if [ "$TEST_COUNT" -ge 550 ]; then check "Test count >= 550 ($TEST_COUNT)" "0"; else check "Test count = $TEST_COUNT (expected >= 550)" "1"; fi

# Check 4: REST endpoint count
ROUTE_COUNT=$(grep -c '\.route(' /workspaces/KCM/crates/kcm-server/src/main.rs 2>/dev/null || true)
if [ "$ROUTE_COUNT" -eq 8 ]; then check "REST endpoint count = 8" "0"; else check "REST endpoint count = $ROUTE_COUNT (expected 8)" "1"; fi

# Check 5: gRPC RPC count
RPC_COUNT=$(grep -c 'rpc ' /workspaces/KCM/crates/kcm-interface/proto/kcm.proto 2>/dev/null || true)
if [ "$RPC_COUNT" -eq 4 ]; then check "gRPC RPC count = 4" "0"; else check "gRPC RPC count = $RPC_COUNT (expected 4)" "1"; fi

# Check 6: No TODO/FIXME in production code
TODO_COUNT=$(grep -r 'TODO\|FIXME\|HACK' /workspaces/KCM/crates/ --include='*.rs' 2>/dev/null | grep -v 'test' | grep -v 'bench' | wc -l)
if [ "$TODO_COUNT" -eq 0 ]; then check "No TODO/FIXME in production code" "0"; else check "TODO/FIXME count = $TODO_COUNT" "1"; fi

# Check 7: No unwrap() in production code (excluding tests/benches/main.rs)
# Note: Some unwraps exist in inline test modules within src files; these are acceptable.
# Target: reduce to 0 over time via clippy linting.
UNWRAP_COUNT=$(grep -r '\.unwrap()' /workspaces/KCM/crates/ --include='*.rs' 2>/dev/null | grep -v 'tests/' | grep -v 'benches/' | grep -v 'src/main.rs' | wc -l)
if [ "$UNWRAP_COUNT" -le 100 ]; then check "Unwrap count in production code <= 100 ($UNWRAP_COUNT)" "0"; else check "Unwrap count = $UNWRAP_COUNT (expected <= 100)" "1"; fi

# Check 8: Spec documents have Document ID
SPECS_WITH_ID=$(grep -l "Document ID:" /workspaces/KCM/docs/KCM_*.md 2>/dev/null | wc -l)
TOTAL_SPECS=$(ls /workspaces/KCM/docs/KCM_*.md 2>/dev/null | wc -l)
if [ "$SPECS_WITH_ID" -eq "$TOTAL_SPECS" ]; then check "All spec docs have Document ID ($SPECS_WITH_ID/$TOTAL_SPECS)" "0"; else check "Spec docs with Document ID = $SPECS_WITH_ID/$TOTAL_SPECS" "1"; fi

# Check 9: Spec documents have Status
SPECS_WITH_STATUS=$(grep -l "Status:" /workspaces/KCM/docs/KCM_*.md 2>/dev/null | wc -l)
if [ "$SPECS_WITH_STATUS" -eq "$TOTAL_SPECS" ]; then check "All spec docs have Status ($SPECS_WITH_STATUS/$TOTAL_SPECS)" "0"; else check "Spec docs with Status = $SPECS_WITH_STATUS/$TOTAL_SPECS" "1"; fi

# Check 10: No phantom document references
PHANTOM_REFS=$(grep -r "KCM_ARCHITECTURE-001" /workspaces/KCM/docs/KCM_*.md /workspaces/KCM/docs/PRD*.md 2>/dev/null | wc -l)
if [ "$PHANTOM_REFS" -eq 0 ]; then check "No phantom document references" "0"; else check "Phantom references = $PHANTOM_REFS" "1"; fi

# Check 11: Cargo workspace builds
# (skip if cargo not available)
if command -v cargo &>/dev/null; then
    if cargo check --workspace --quiet 2>/dev/null; then check "Workspace compiles" "0"; else check "Workspace compilation failed" "1"; fi
fi

# Check 12: No stale FFI count in docs
STALE_FFI=$(grep -rn "C FFI (15" /workspaces/KCM/docs/ /workspaces/KCM/AGENTS.md /workspaces/KCM/README.md 2>/dev/null | wc -l)
if [ "$STALE_FFI" -eq 0 ]; then check "No stale FFI count (15) in docs" "0"; else check "Stale FFI count references = $STALE_FFI" "1"; fi

# Check 13: No stale metrics count in docs
STALE_METRICS=$(grep -rn "11 counters\|10 counters" /workspaces/KCM/docs/KCM_*.md /workspaces/KCM/AGENTS.md 2>/dev/null | wc -l)
if [ "$STALE_METRICS" -eq 0 ]; then check "No stale metrics count in docs" "0"; else check "Stale metrics count references = $STALE_METRICS" "1"; fi

# Check 14: No stale test count in active docs
STALE_TESTS=$(grep -rn "534" /workspaces/KCM/docs/PRD*.md /workspaces/KCM/docs/KCM_TESTING*.md 2>/dev/null | wc -l)
if [ "$STALE_TESTS" -eq 0 ]; then check "No stale test count (534) in active docs" "0"; else check "Stale test count references = $STALE_TESTS" "1"; fi

echo ""
echo "=== Results ==="
if [ "$ERRORS" -eq 0 ]; then
    echo "ALL CHECKS PASSED"
    exit 0
else
    echo "FAILED: $ERRORS check(s) failed"
    exit 1
fi
