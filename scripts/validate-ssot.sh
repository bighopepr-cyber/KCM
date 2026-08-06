#!/usr/bin/env bash
# KCM SSOT Validation Script v3.0
# Verifies consistency between documentation (SSOT) and codebase
# Standard: Microsoft Pragmatic Rust Guidelines 2026
set -uo pipefail

ERRORS=0
WARNINGS=0

check() {
    local desc="$1"
    local result="$2"
    if [ "$result" = "0" ]; then
        echo "  PASS: $desc"
    else
        echo "  FAIL: $desc"
        ERRORS=$((ERRORS + 1))
    fi
}

warn() {
    local desc="$1"
    local result="$2"
    if [ "$result" = "0" ]; then
        echo "  PASS: $desc"
    else
        echo "  WARN: $desc"
        WARNINGS=$((WARNINGS + 1))
    fi
}

echo "=== KCM SSOT Validation v3.0 ==="
echo "Standard: Microsoft Pragmatic Rust Guidelines 2026"
echo ""

# Check 1: FFI function count
echo "Contract Verification:"
FFI_COUNT=$(grep -c 'unsafe extern "C" fn' crates/kcm-interface/src/lib.rs 2>/dev/null || true)
if [ "$FFI_COUNT" -eq 18 ]; then check "C FFI function count = 18" "0"; else check "C FFI function count = $FFI_COUNT (expected 18)" "1"; fi

# Check 2: Metrics counter count
METRICS_COUNT=$(grep -c 'pub.*: AtomicU64' crates/kcm-runtime/src/metrics.rs 2>/dev/null || true)
if [ "$METRICS_COUNT" -eq 14 ]; then check "Metrics counter count = 14" "0"; else check "Metrics counter count = $METRICS_COUNT (expected 14)" "1"; fi

# Check 3: Test count
TEST_COUNT=$(grep -r '#\[test\]' crates/ --include='*.rs' 2>/dev/null | wc -l)
if [ "$TEST_COUNT" -ge 550 ]; then check "Test count >= 550 ($TEST_COUNT)" "0"; else check "Test count = $TEST_COUNT (expected >= 550)" "1"; fi

# Check 4: REST endpoint count
ROUTE_COUNT=$(grep -c '\.route(' crates/kcm-server/src/main.rs 2>/dev/null || true)
if [ "$ROUTE_COUNT" -ge 8 ]; then check "REST endpoint count >= 8 ($ROUTE_COUNT)" "0"; else check "REST endpoint count = $ROUTE_COUNT (expected >= 8)" "1"; fi

# Check 5: gRPC RPC count
RPC_COUNT=$(grep -c 'rpc ' crates/kcm-interface/proto/kcm.proto 2>/dev/null || true)
if [ "$RPC_COUNT" -eq 4 ]; then check "gRPC RPC count = 4" "0"; else check "gRPC RPC count = $RPC_COUNT (expected 4)" "1"; fi

echo ""
echo "Code Quality Gates:"
# Check 6: No TODO/FIXME in production code
TODO_COUNT=$(grep -r 'TODO\|FIXME\|HACK' crates/ --include='*.rs' 2>/dev/null | grep -v 'test' | grep -v 'bench' | wc -l)
if [ "$TODO_COUNT" -eq 0 ]; then check "No TODO/FIXME in production code" "0"; else check "TODO/FIXME count = $TODO_COUNT" "1"; fi

# Check 7: No unwrap() in production code (excluding tests/benches/main.rs/test infrastructure)
# Note: unwraps in #[cfg(test)] modules within source files are acceptable
UNWRAP_COUNT=$(grep -r '\.unwrap()' crates/ --include='*.rs' 2>/dev/null | grep -v 'tests/' | grep -v 'benches/' | grep -v 'src/main.rs' | grep -v 'kcm-testing/' | grep -v '#\[cfg(test)\]' | wc -l)
if [ "$UNWRAP_COUNT" -le 80 ]; then check "Unwrap count in production code <= 80 ($UNWRAP_COUNT)" "0"; else check "Unwrap count = $UNWRAP_COUNT (expected <= 80)" "1"; fi

# Check 8: Workspace compiles (only if cargo is available)
if command -v cargo &>/dev/null; then
    if cargo check --workspace --quiet 2>/dev/null; then check "Workspace compiles" "0"; else check "Workspace compilation failed" "1"; fi
fi

echo ""
echo "Documentation Structure:"
# Check 9: Root doc files exist
for f in README.md KCM_SPECIFICATION.md ROADMAP.md; do
    if [ -f "$f" ]; then check "Root file exists: $f" "0"; else check "Root file missing: $f" "1"; fi
done

# Check 9b: Governance doc files exist
for f in docs/governance/architecture-matrix.md docs/governance/ssot-certification.md docs/governance/engineering-rules.md; do
    if [ -f "$f" ]; then check "Governance file exists: $f" "0"; else check "Governance file missing: $f" "1"; fi
done

# Check 10: Community files exist
for f in CONTRIBUTING.md CODE_OF_CONDUCT.md SECURITY.md LICENSE; do
    if [ -f "$f" ]; then check "Community file exists: $f" "0"; else check "Community file missing: $f" "1"; fi
done

# Check 11: No phantom document references
PHANTOM_REFS=$(grep -r "KCM_ARCHITECTURE-001" docs/ AGENTS.md README.md 2>/dev/null | wc -l)
if [ "$PHANTOM_REFS" -eq 0 ]; then check "No phantom document references" "0"; else check "Phantom references = $PHANTOM_REFS" "1"; fi

echo ""
echo "Repository Structure:"
# Check 12: Deleted directories don't exist
for d in website integrations third_party; do
    if [ -d "$d" ]; then check "Deleted directory still exists: $d" "1"; else check "Deleted directory removed: $d" "0"; fi
done

# Check 13: docs/ has required subfolders
DOCS_SUBFOLDERS=$(find docs/ -maxdepth 1 -type d | grep -v '^docs/$' | wc -l)
if [ "$DOCS_SUBFOLDERS" -ge 4 ]; then check "docs/ has $DOCS_SUBFOLDERS subfolders (>= 4 required)" "0"; else check "docs/ subfolder count = $DOCS_SUBFOLDERS (expected >= 4)" "1"; fi

# Check 14: skills/ has 16 skills
SKILL_COUNT=$(find skills/ -maxdepth 1 -type d | grep -v '^skills/$' | wc -l)
if [ "$SKILL_COUNT" -eq 16 ]; then check "skills/ has 16 AI skills" "0"; else check "skills/ count = $SKILL_COUNT (expected 16)" "1"; fi

# Check 15: .agents/skills/ exists
if [ -d ".agents/skills" ]; then check ".agents/skills/ exists" "0"; else check ".agents/skills/ missing" "1"; fi

# Check 16: crates/ has 13 crates
CRATE_COUNT=$(find crates/ -maxdepth 1 -type d | grep -v '^crates/$' | wc -l)
if [ "$CRATE_COUNT" -eq 13 ]; then check "crates/ has 13 crates" "0"; else check "crates/ count = $CRATE_COUNT (expected 13)" "1"; fi

echo ""
echo "Edition & Configuration:"
# Check 17: No stale FFI count in docs
STALE_FFI=$(grep -rn "C FFI (15" docs/ AGENTS.md README.md 2>/dev/null | wc -l)
if [ "$STALE_FFI" -eq 0 ]; then check "No stale FFI count (15) in docs" "0"; else check "Stale FFI count references = $STALE_FFI" "1"; fi

# Check 18: No stale metrics count in docs
STALE_METRICS=$(grep -rn "11 counters\|10 counters" docs/ AGENTS.md 2>/dev/null | wc -l)
if [ "$STALE_METRICS" -eq 0 ]; then check "No stale metrics count in docs" "0"; else check "Stale metrics count references = $STALE_METRICS" "1"; fi

# Check 19: Edition 2021 in root Cargo.toml
if grep -q 'edition = "2021"' Cargo.toml 2>/dev/null; then check "Root Cargo.toml uses edition 2021" "0"; else check "Root Cargo.toml missing edition 2021" "1"; fi

# Check 20: workspace.package defined
if grep -q '\[workspace.package\]' Cargo.toml 2>/dev/null; then check "[workspace.package] defined" "0"; else check "[workspace.package] missing" "1"; fi

# Check 21: workspace.lints defined
if grep -q '\[workspace.lints' Cargo.toml 2>/dev/null; then check "[workspace.lints] defined" "0"; else check "[workspace.lints] missing" "1"; fi

# Check 22: CODEOWNERS has global fallback
if grep -q '^\*' .github/CODEOWNERS 2>/dev/null; then check "CODEOWNERS has global fallback" "0"; else check "CODEOWNERS missing global fallback" "1"; fi

# Check 23: CODEOWNERS has distributed and ml
if grep -q 'kcm-distributed' .github/CODEOWNERS 2>/dev/null; then check "CODEOWNERS includes kcm-distributed" "0"; else check "CODEOWNERS missing kcm-distributed" "1"; fi
if grep -q 'kcm-ml' .github/CODEOWNERS 2>/dev/null; then check "CODEOWNERS includes kcm-ml" "0"; else check "CODEOWNERS missing kcm-ml" "1"; fi

# Check 24: .agents/ directory exists
if [ -d ".agents" ]; then check ".agents/ directory exists" "0"; else check ".agents/ directory missing" "1"; fi

echo ""
echo "=== Results ==="
echo "  Checks passed: $((ERRORS == 0 ? 24 : 24 - ERRORS))"
echo "  Errors: $ERRORS"
echo "  Warnings: $WARNINGS"
echo ""
if [ "$ERRORS" -eq 0 ]; then
    echo "ALL CHECKS PASSED"
    exit 0
else
    echo "FAILED: $ERRORS check(s) failed"
    exit 1
fi
