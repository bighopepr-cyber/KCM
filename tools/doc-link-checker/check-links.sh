#!/usr/bin/env bash
# KCM Documentation Link Checker
# Validates all markdown links (internal and external)
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
ERRORS=0
CHECKS=0
EXTERNAL=0
INTERNAL=0

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

pass() { ((CHECKS++)); }
fail() { ((CHECKS++)); ((ERRORS++)); echo -e "${RED}✗${NC} $1"; }
warn() { echo -e "${YELLOW}⚠${NC} $1"; }

echo "========================================="
echo " KCM Documentation Link Checker"
echo "========================================="
echo ""

# Check internal links
echo "=== Checking Internal Links ==="
find "$REPO_ROOT" -name "*.md" \
    -not -path "*/target/*" \
    -not -path "*/.git/*" \
    -not -path "*/.kilo/*" \
    -not -path "*/node_modules/*" | while read -r file; do

    rel="${file#$REPO_ROOT/}"
    dir=$(dirname "$file")

    # Extract markdown links [text](url)
    grep -oE '\[[^]]*\]\([^)]+\)' "$file" 2>/dev/null | while read -r link; do
        url=$(echo "$link" | sed 's/.*(\(.*\))/\1/')

        # Skip external links, anchors, mailto
        [[ "$url" == http* ]] && { ((EXTERNAL++)); continue; }
        [[ "$url" == "#"* ]] && continue
        [[ "$url" == "mailto:"* ]] && continue

        # Resolve relative path
        if [[ "$url" == ".."* ]] || [[ "$url" == "./*" ]]; then
            target="$REPO_ROOT/${url#../}"
        else
            target="$dir/$url"
        fi

        # Remove anchor
        target="${target%%#*}"

        # Check if target exists
        if [ -e "$target" ] || [ -f "$target" ]; then
            pass
        else
            fail "$rel: broken link → $url"
        fi
        ((INTERNAL++))
    done
done

echo ""
echo "=== Checking Anchor Links ==="
find "$REPO_ROOT" -name "*.md" \
    -not -path "*/target/*" \
    -not -path "*/.git/*" \
    -not -path "*/.kilo/*" \
    -not -path "*/node_modules/*" | while read -r file; do

    rel="${file#$REPO_ROOT/}"

    # Extract anchor links [text](#anchor)
    grep -oE '\[[^]]*\]\(#[^)]+\)' "$file" 2>/dev/null | while read -r link; do
        anchor=$(echo "$link" | sed 's/.*(#\([^)]*\))/\1/')
        # Check if anchor exists in same file (simplified check)
        if grep -qiE "^#{1,6} .*$(echo "$anchor" | sed 's/-/ /g')" "$file" 2>/dev/null; then
            pass
        else
            warn "$rel: anchor #$anchor may not exist"
        fi
    done
done

echo ""
echo "========================================="
echo " Link Check Results"
echo "========================================="
echo " Internal links checked: $INTERNAL"
echo " External links skipped: $EXTERNAL"
echo -e " Broken links: ${RED}$ERRORS${NC}"
echo ""

if [ $ERRORS -gt 0 ]; then
    echo -e "${RED}LINK CHECK FAILED${NC}"
    exit 1
else
    echo -e "${GREEN}LINK CHECK PASSED${NC}"
    exit 0
fi
