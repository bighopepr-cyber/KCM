#!/usr/bin/env bash
# KCM Documentation Validator
# Validates all documentation across the repository
set -uo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
ERRORS=0
WARNINGS=0
CHECKS=0
PASSED=0

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

pass() { ((CHECKS++)); ((PASSED++)); echo -e "${GREEN}✓${NC} $1"; }
fail() { ((CHECKS++)); ((ERRORS++)); echo -e "${RED}✗${NC} $1"; }
warn() { ((WARNINGS++)); echo -e "${YELLOW}⚠${NC} $1"; }

echo "========================================="
echo " KCM Documentation Validator"
echo "========================================="
echo ""

# Required files per folder
REQUIRED_FOLDERS=(
    "crates/kcm-core"
    "crates/kcm-storage"
    "crates/kcm-compute"
    "crates/kcm-reasoning"
    "crates/kcm-optimizer"
    "crates/kcm-runtime"
    "crates/kcm-interface"
    "crates/kcm-distributed"
    "crates/kcm-ml"
    "crates/kcm-security"
    "crates/kcm-compliance"
    "crates/kcm-testing"
    "crates/kcm-server"
    "sdk"
    "deployment"
    "tests"
    "tests/sdk"
    "scripts"
    "examples"
    "skills"
    "docs"
    "benchmark-results"
    ".github"
    ".agents"
    ".cargo"
    "assets"
)

REQUIRED_FILES=("README.md" "SECURITY.md" "CONTRIBUTING.md" "CODE_OF_CONDUCT.md")

# Check required files
echo "=== Checking Required Files ==="
for folder in "${REQUIRED_FOLDERS[@]}"; do
    for file in "${REQUIRED_FILES[@]}"; do
        if [ -f "$REPO_ROOT/$folder/$file" ]; then
            pass "$folder/$file exists"
        else
            fail "$folder/$file MISSING"
        fi
    done
done

# Check spesifikasi files
echo ""
echo "=== Checking Spesifikasi Files ==="
CRATES=(
    "kcm-core" "kcm-storage" "kcm-compute" "kcm-reasoning"
    "kcm-optimizer" "kcm-runtime" "kcm-interface" "kcm-distributed"
    "kcm-ml" "kcm-security" "kcm-compliance" "kcm-testing" "kcm-server"
)
SPECFOLDERS=("sdk" "deployment" "tests" "scripts" "examples" "skills" "docs" "benchmark-results" "github" "agents" "cargo" "assets")

for crate in "${CRATES[@]}"; do
    if [ -f "$REPO_ROOT/docs/$crate/spesifikasi.md" ]; then
        pass "docs/$crate/spesifikasi.md exists"
    else
        fail "docs/$crate/spesifikasi.md MISSING"
    fi
done

for folder in "${SPECFOLDERS[@]}"; do
    if [ -f "$REPO_ROOT/docs/$folder/spesifikasi.md" ]; then
        pass "docs/$folder/spesifikasi.md exists"
    else
        fail "docs/$folder/spesifikasi.md MISSING"
    fi
done

# Check required headings in SECURITY.md files
echo ""
echo "=== Checking Required Headings ==="
check_heading() {
    local file="$1"
    local heading="$2"
    if [ -f "$file" ] && grep -q "^# $heading" "$file"; then
        pass "$(basename $(dirname $file))/$(basename $file): heading '$heading' found"
    elif [ -f "$file" ]; then
        fail "$(basename $(dirname $file))/$(basename $file): heading '$heading' MISSING"
    fi
}

for folder in "${REQUIRED_FOLDERS[@]}"; do
    check_heading "$REPO_ROOT/$folder/SECURITY.md" "Overview"
    check_heading "$REPO_ROOT/$folder/SECURITY.md" "Security Scope"
    check_heading "$REPO_ROOT/$folder/SECURITY.md" "Threat Model"
    check_heading "$REPO_ROOT/$folder/SECURITY.md" "Secure Development Rules"
    check_heading "$REPO_ROOT/$folder/SECURITY.md" "Validation Checklist"
    check_heading "$REPO_ROOT/$folder/CONTRIBUTING.md" "Overview"
    check_heading "$REPO_ROOT/$folder/CONTRIBUTING.md" "Coding Standards"
    check_heading "$REPO_ROOT/$folder/CONTRIBUTING.md" "Testing Requirements"
    check_heading "$REPO_ROOT/$folder/CONTRIBUTING.md" "Review Checklist"
done

# Check SSOT Alignment in spesifikasi files
echo ""
echo "=== Checking SSOT Alignment ==="
for crate in "${CRATES[@]}"; do
    file="$REPO_ROOT/docs/$crate/spesifikasi.md"
    if [ -f "$file" ] && grep -q "SSOT Alignment" "$file"; then
        pass "docs/$crate/spesifikasi.md: SSOT Alignment found"
    elif [ -f "$file" ]; then
        fail "docs/$crate/spesifikasi.md: SSOT Alignment MISSING"
    fi
done

# Check References in all docs
echo ""
echo "=== Checking References Sections ==="
for folder in "${REQUIRED_FOLDERS[@]}"; do
    for file in SECURITY.md CONTRIBUTING.md CODE_OF_CONDUCT.md; do
        fullpath="$REPO_ROOT/$folder/$file"
        if [ -f "$fullpath" ] && grep -q "References" "$fullpath"; then
            pass "$folder/$file: References section found"
        elif [ -f "$fullpath" ]; then
            warn "$folder/$file: References section missing"
        fi
    done
done

# Summary
echo ""
echo "========================================="
echo " Results"
echo "========================================="
echo " Total checks: $CHECKS"
echo -e " Passed: ${GREEN}$PASSED${NC}"
echo -e " Failed: ${RED}$ERRORS${NC}"
echo -e " Warnings: ${YELLOW}$WARNINGS${NC}"
echo ""

if [ $ERRORS -gt 0 ]; then
    echo -e "${RED}VALIDATION FAILED${NC}"
    exit 1
else
    echo -e "${GREEN}VALIDATION PASSED${NC}"
    exit 0
fi
