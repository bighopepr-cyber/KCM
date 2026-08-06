#!/usr/bin/env bash
# KCM Documentation Search Index Generator
# Generates docs/search-index.json for documentation search
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
INDEX="$REPO_ROOT/docs/search-index.json"
DATE=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

echo '{"generated_at":"'$DATE'","documents":[' > "$INDEX"
FIRST=true

find "$REPO_ROOT" -name "*.md" \
    -not -path "*/target/*" \
    -not -path "*/.git/*" \
    -not -path "*/.kilo/*" \
    -not -path "*/node_modules/*" | sort | while read -r file; do

    rel="${file#$REPO_ROOT/}"
    title=$(head -1 "$file" | sed 's/^#* *//')
    # Escape JSON
    title=$(echo "$title" | sed 's/"/\\"/g')
    size=$(wc -c < "$file")

    # Determine category
    case "$rel" in
        crates/*) category="crates" ;;
        sdk/*) category="sdk" ;;
        docs/specs/*) category="specifications" ;;
        docs/sdk/*) category="sdk-docs" ;;
        docs/adr/*) category="adr" ;;
        deployment/*) category="deployment" ;;
        tests/*) category="testing" ;;
        scripts/*) category="scripts" ;;
        docs/governance/*) category="governance" ;;
        docs/templates/*) category="templates" ;;
        docs/metrics/*) category="metrics" ;;
        *) category="other" ;;
    esac

    if [ "$FIRST" = true ]; then
        FIRST=false
    else
        echo ',' >> "$INDEX"
    fi

    printf '  {"path":"%s","title":"%s","category":"%s","size":%d}' "$rel" "$title" "$category" "$size" >> "$INDEX"
done

echo '' >> "$INDEX"
echo ']}' >> "$INDEX"

echo "Search index generated: $INDEX"
