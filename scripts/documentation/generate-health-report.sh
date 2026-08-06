#!/usr/bin/env bash
# KCM Repository Health Report Generator
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
REPORT="$REPO_ROOT/repository-health.md"
DATE=$(date -u +"%Y-%m-%d")

# Count files
TOTAL_MD=$(find "$REPO_ROOT" -name "*.md" -not -path "*/target/*" -not -path "*/.git/*" -not -path "*/.kilo/*" -not -path "*/node_modules/*" | wc -l)
TOTAL_CRATES=$(ls -d "$REPO_ROOT"/crates/kcm-* 2>/dev/null | wc -l)
TOTAL_SDK=$(ls -d "$REPO_ROOT"/sdk/*/ 2>/dev/null | wc -l)
TOTAL_SPECS=$(find "$REPO_ROOT/docs" -name "spesifikasi.md" | wc -l)
TOTAL_ADR=$(find "$REPO_ROOT/docs/adr" -name "ADR-*.md" 2>/dev/null | wc -l)
TOTAL_CLI=$(ls -d "$REPO_ROOT/scripts/kcm-cli/kcm-"* 2>/dev/null | wc -l)

# Check required files
MISSING=0
for folder in crates/kcm-core crates/kcm-storage crates/kcm-compute crates/kcm-reasoning crates/kcm-optimizer crates/kcm-runtime crates/kcm-interface crates/kcm-distributed crates/kcm-ml crates/kcm-security crates/kcm-compliance crates/kcm-testing crates/kcm-server sdk deployment tests scripts examples; do
    for file in README.md SECURITY.md CONTRIBUTING.md CODE_OF_CONDUCT.md; do
        [ ! -f "$REPO_ROOT/$folder/$file" ] && ((MISSING++))
    done
done

# Calculate coverage
TOTAL_REQUIRED=$((16 * 4))  # 16 folders * 4 files
TOTAL_PRESENT=$((TOTAL_REQUIRED - MISSING))
if [ $TOTAL_REQUIRED -gt 0 ]; then
    COVERAGE=$((TOTAL_PRESENT * 100 / TOTAL_REQUIRED))
else
    COVERAGE=100
fi

# Health score
if [ $MISSING -eq 0 ] && [ $COVERAGE -eq 100 ]; then
    HEALTH="✅ HEALTHY"
elif [ $MISSING -le 2 ]; then
    HEALTH="⚠️ DEGRADED"
else
    HEALTH="❌ UNHEALTHY"
fi

cat > "$REPORT" << EOF
# KCM Repository Health Report

**Generated:** $DATE
**Health Status:** $HEALTH

## Summary

| Metric | Value |
|--------|-------|
| Total Markdown Files | $TOTAL_MD |
| Total Crates | $TOTAL_CRATES |
| Total SDKs | $TOTAL_SDK |
| Total Specifications | $TOTAL_SPECS |
| Total ADRs | $TOTAL_ADR |
| Total CLI Tools | $TOTAL_CLI |
| Documentation Coverage | ${COVERAGE}% |
| Missing Required Files | $MISSING |

## Documentation Health

- Coverage: ${COVERAGE}%
- Missing files: $MISSING
- Status: $HEALTH

## Recommendations

EOF

if [ $MISSING -gt 0 ]; then
    echo "- Fix $MISSING missing required documentation files" >> "$REPORT"
fi
if [ $COVERAGE -lt 100 ]; then
    echo "- Increase documentation coverage from ${COVERAGE}% to 100%" >> "$REPORT"
fi
if [ $MISSING -eq 0 ] && [ $COVERAGE -eq 100 ]; then
    echo "- All documentation is complete and up to date" >> "$REPORT"
fi

echo "Health report generated: $REPORT"
