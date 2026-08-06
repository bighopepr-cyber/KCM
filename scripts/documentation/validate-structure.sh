#!/usr/bin/env bash
# KCM Markdown Structure Validator
# Validates markdown file structure consistency
set -uo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
ERRORS=0
CHECKS=0

RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'

pass() { ((CHECKS++)); echo -e "${GREEN}✓${NC} $1"; }
fail() { ((CHECKS++)); ((ERRORS++)); echo -e "${RED}✗${NC} $1"; }

echo "========================================="
echo " KCM Markdown Structure Validator"
echo "========================================="

# Find all markdown files
find "$REPO_ROOT" -name "*.md" \
    -not -path "*/target/*" \
    -not -path "*/.git/*" \
    -not -path "*/.kilo/*" \
    -not -path "*/node_modules/*" | while read -r file; do

    rel="${file#$REPO_ROOT/}"

    # Check for duplicate headings
    headings=$(grep -E "^#{1,6} " "$file" | sort)
    dupes=$(echo "$headings" | uniq -d)
    if [ -n "$dupes" ]; then
        fail "$rel: duplicate headings found"
    else
        pass "$rel: no duplicate headings"
    fi

    # Check first line is a heading
    first_line=$(head -1 "$file")
    if echo "$first_line" | grep -qE "^#{1,6} "; then
        pass "$rel: starts with heading"
    else
        fail "$rel: does not start with heading"
    fi
done

echo ""
if [ $ERRORS -gt 0 ]; then
    echo -e "${RED}STRUCTURE VALIDATION FAILED${NC}"
    exit 1
else
    echo -e "${GREEN}STRUCTURE VALIDATION PASSED${NC}"
    exit 0
fi
