#!/usr/bin/env bash
# KCM SSOT Alignment Checker
# Validates SSOT compliance across all documentation
set -uo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
ERRORS=0
CHECKS=0

RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'

pass() { ((CHECKS++)); }
fail() { ((CHECKS++)); ((ERRORS++)); echo -e "${RED}✗${NC} $1"; }

echo "========================================="
echo " KCM SSOT Alignment Checker"
echo "========================================="
echo ""

# Check SSOT.md exists
((CHECKS++))
if [ -f "$REPO_ROOT/SSOT.md" ]; then
    pass "SSOT.md exists"
else
    fail "SSOT.md MISSING — root SSOT document"
fi

# Check AGENTS.md exists
((CHECKS++))
if [ -f "$REPO_ROOT/AGENTS.md" ]; then
    pass "AGENTS.md exists"
else
    fail "AGENTS.md MISSING — engineering constitution"
fi

# Check PRD documents exist
for prd in PRD.md PRD2.md PRD3.md PRD-TESTING-AND-BENCHMARK.md; do
    ((CHECKS++))
    if [ -f "$REPO_ROOT/docs/specs/$prd" ]; then
        pass "docs/specs/$prd exists"
    else
        fail "docs/specs/$prd MISSING"
    fi
done

# Check spesifikasi files have SSOT Alignment
echo ""
echo "=== SSOT Alignment in Spesifikasi ==="
for spec in "$REPO_ROOT"/docs/*/spesifikasi.md; do
    [ ! -f "$spec" ] && continue
    rel="${spec#$REPO_ROOT/}"
    ((CHECKS++))
    if grep -q "SSOT Alignment" "$spec" 2>/dev/null; then
        pass "$rel: has SSOT Alignment"
    else
        fail "$rel: missing SSOT Alignment section"
    fi
done

# Check spesifikasi files have References
echo ""
echo "=== References in Spesifikasi ==="
for spec in "$REPO_ROOT"/docs/*/spesifikasi.md; do
    [ ! -f "$spec" ] && continue
    rel="${spec#$REPO_ROOT/}"
    ((CHECKS++))
    if grep -q "References" "$spec" 2>/dev/null; then
        pass "$rel: has References"
    else
        fail "$rel: missing References section"
    fi
done

# Check SSOT references in docs
echo ""
echo "=== SSOT Document References ==="
for spec in "$REPO_ROOT"/docs/*/spesifikasi.md; do
    [ ! -f "$spec" ] && continue
    rel="${spec#$REPO_ROOT/}"
    ((CHECKS++))
    if grep -q "SSOT.md\|AGENTS.md\|PRD" "$spec" 2>/dev/null; then
        pass "$rel: references SSOT documents"
    else
        fail "$rel: does not reference SSOT documents"
    fi
done

echo ""
echo "========================================="
echo " SSOT Check Results"
echo "========================================="
echo " Checks: $CHECKS"
echo -e " Errors: ${RED}$ERRORS${NC}"
echo ""

if [ $ERRORS -gt 0 ]; then
    echo -e "${RED}SSOT CHECK FAILED${NC}"
    exit 1
else
    echo -e "${GREEN}SSOT CHECK PASSED${NC}"
    exit 0
fi
