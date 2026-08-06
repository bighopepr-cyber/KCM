#!/usr/bin/env bash
# KCM Documentation Coverage Calculator
# Generates coverage.json, coverage.html, and coverage.md
set -uo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
METRICS_DIR="$REPO_ROOT/docs/metrics"
mkdir -p "$METRICS_DIR"

DATE=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

REQUIRED_FOLDERS=(
    "crates/kcm-core" "crates/kcm-storage" "crates/kcm-compute"
    "crates/kcm-reasoning" "crates/kcm-optimizer" "crates/kcm-runtime"
    "crates/kcm-interface" "crates/kcm-distributed" "crates/kcm-ml"
    "crates/kcm-security" "crates/kcm-compliance" "crates/kcm-testing"
    "crates/kcm-server" "sdk" "deployment" "tests" "tests/sdk"
    "scripts" "examples" "skills" "docs" "benchmark-results"
    ".github" ".agents" ".cargo" "assets"
)
REQUIRED_FILES=("README.md" "SECURITY.md" "CONTRIBUTING.md" "CODE_OF_CONDUCT.md")

folder_total=0
folder_present=0
for folder in "${REQUIRED_FOLDERS[@]}"; do
    for file in "${REQUIRED_FILES[@]}"; do
        folder_total=$((folder_total + 1))
        if [ -f "$REPO_ROOT/$folder/$file" ]; then
            folder_present=$((folder_present + 1))
        fi
    done
done
if [ "$folder_total" -gt 0 ]; then
    folder_pct=$((folder_present * 100 / folder_total))
else
    folder_pct=0
fi

CRATES=("kcm-core" "kcm-storage" "kcm-compute" "kcm-reasoning" "kcm-optimizer" "kcm-runtime" "kcm-interface" "kcm-distributed" "kcm-ml" "kcm-security" "kcm-compliance" "kcm-testing" "kcm-server")
crate_total=${#CRATES[@]}
crate_present=0
for crate in "${CRATES[@]}"; do
    if [ -f "$REPO_ROOT/docs/$crate/spesifikasi.md" ]; then
        crate_present=$((crate_present + 1))
    fi
done
crate_pct=$((crate_present * 100 / crate_total))

ssot_total=0
ssot_present=0
for crate in "${CRATES[@]}"; do
    ssot_total=$((ssot_total + 1))
    if [ -f "$REPO_ROOT/docs/$crate/spesifikasi.md" ] && grep -q "SSOT Alignment" "$REPO_ROOT/docs/$crate/spesifikasi.md" 2>/dev/null; then
        ssot_present=$((ssot_present + 1))
    fi
done
if [ "$ssot_total" -gt 0 ]; then
    ssot_pct=$((ssot_present * 100 / ssot_total))
else
    ssot_pct=0
fi

heading_total=0
heading_present=0
for folder in "${REQUIRED_FOLDERS[@]}"; do
    for file in SECURITY.md CONTRIBUTING.md CODE_OF_CONDUCT.md; do
        heading_total=$((heading_total + 1))
        if [ -f "$REPO_ROOT/$folder/$file" ] && head -1 "$REPO_ROOT/$folder/$file" | grep -qE "^#[[:space:]]" 2>/dev/null; then
            heading_present=$((heading_present + 1))
        fi
    done
done
if [ "$heading_total" -gt 0 ]; then
    heading_pct=$((heading_present * 100 / heading_total))
else
    heading_pct=0
fi

ref_total=0
ref_present=0
for folder in "${REQUIRED_FOLDERS[@]}"; do
    for file in SECURITY.md CONTRIBUTING.md CODE_OF_CONDUCT.md; do
        ref_total=$((ref_total + 1))
        if [ -f "$REPO_ROOT/$folder/$file" ] && grep -q "References" "$REPO_ROOT/$folder/$file" 2>/dev/null; then
            ref_present=$((ref_present + 1))
        fi
    done
done
if [ "$ref_total" -gt 0 ]; then
    ref_pct=$((ref_present * 100 / ref_total))
else
    ref_pct=0
fi

overall_total=$((folder_total + crate_total + ssot_total + heading_total + ref_total))
overall_present=$((folder_present + crate_present + ssot_present + heading_present + ref_present))
if [ "$overall_total" -gt 0 ]; then
    overall_pct=$((overall_present * 100 / overall_total))
else
    overall_pct=100
fi

cat > "$METRICS_DIR/coverage.json" << EOF
{
  "generated_at": "$DATE",
  "overall_percentage": $overall_pct,
  "overall_present": $overall_present,
  "overall_total": $overall_total,
  "categories": {
    "folder_coverage": {
      "percentage": $folder_pct,
      "present": $folder_present,
      "total": $folder_total,
      "description": "Required files per folder"
    },
    "crate_coverage": {
      "percentage": $crate_pct,
      "present": $crate_present,
      "total": $crate_total,
      "description": "Spesifikasi files per crate"
    },
    "ssot_coverage": {
      "percentage": $ssot_pct,
      "present": $ssot_present,
      "total": $ssot_total,
      "description": "SSOT Alignment in spesifikasi"
    },
    "heading_coverage": {
      "percentage": $heading_pct,
      "present": $heading_present,
      "total": $heading_total,
      "description": "Required headings in docs"
    },
    "reference_coverage": {
      "percentage": $ref_pct,
      "present": $ref_present,
      "total": $ref_total,
      "description": "References sections in docs"
    }
  }
}
EOF

cat > "$METRICS_DIR/coverage.md" << EOF
# Documentation Coverage Report

**Generated:** $DATE

## Overall Coverage: ${overall_pct}%

| Category | Coverage | Present | Total |
|----------|----------|---------|-------|
| Folder Coverage | ${folder_pct}% | $folder_present | $folder_total |
| Crate Coverage | ${crate_pct}% | $crate_present | $crate_total |
| SSOT Coverage | ${ssot_pct}% | $ssot_present | $ssot_total |
| Heading Coverage | ${heading_pct}% | $heading_present | $heading_total |
| Reference Coverage | ${ref_pct}% | $ref_present | $ref_total |
| **Overall** | **${overall_pct}%** | **$overall_present** | **$overall_total** |
EOF

cat > "$METRICS_DIR/coverage.html" << 'HTMLEOF'
<!DOCTYPE html>
<html><head><title>KCM Documentation Coverage</title>
<style>
body{font-family:system-ui,sans-serif;max-width:800px;margin:0 auto;padding:20px}
h1{color:#1a1a2e}table{width:100%;border-collapse:collapse}th,td{padding:8px 12px;border:1px solid #ddd;text-align:left}
th{background:#1a1a2e;color:white}.pass{color:#2ecc71}.fail{color:#e74c3c}
.bar{height:20px;background:#ecf0f1;border-radius:4px;overflow:hidden}
.fill{height:100%;background:#2ecc71;border-radius:4px}
</style></head><body>
<h1>KCM Documentation Coverage</h1>
HTMLEOF

cat >> "$METRICS_DIR/coverage.html" << EOF
<p>Generated: $DATE</p>
<h2>Overall: ${overall_pct}%</h2>
<div class="bar"><div class="fill" style="width:${overall_pct}%"></div></div>
<table><tr><th>Category</th><th>Coverage</th><th>Present</th><th>Total</th></tr>
<tr><td>Folder Coverage</td><td>${folder_pct}%</td><td>$folder_present</td><td>$folder_total</td></tr>
<tr><td>Crate Coverage</td><td>${crate_pct}%</td><td>$crate_present</td><td>$crate_total</td></tr>
<tr><td>SSOT Coverage</td><td>${ssot_pct}%</td><td>$ssot_present</td><td>$ssot_total</td></tr>
<tr><td>Heading Coverage</td><td>${heading_pct}%</td><td>$heading_present</td><td>$heading_total</td></tr>
<tr><td>Reference Coverage</td><td>${ref_pct}%</td><td>$ref_present</td><td>$ref_total</td></tr>
</table></body></html>
EOF

echo "Coverage: ${overall_pct}% (JSON: $METRICS_DIR/coverage.json)"
