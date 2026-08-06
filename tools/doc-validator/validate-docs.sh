#!/usr/bin/env bash
# KCM Documentation Validator
# Validates documentation structure, navigation, and consistency.
#
# Usage: bash tools/doc-validator/validate-docs.sh [--json]
#
# Document ID: KCM-DOC-VALIDATOR-001
# Version: 1.0.0
# Status: Active

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
DOCS_DIR="${REPO_ROOT}/docs"
JSON_OUTPUT=false

if [ "${1:-}" = "--json" ]; then
    JSON_OUTPUT=true
fi

PASS=0
FAIL=0
WARN=0

check() {
    local name="$1"
    local file="$2"
    local status="pass"
    if [ ! -f "$file" ]; then
        status="fail"
        FAIL=$((FAIL + 1))
    else
        PASS=$((PASS + 1))
    fi
    if [ "$JSON_OUTPUT" = false ]; then
        if [ "$status" = "pass" ]; then
            echo "  PASS: ${name}"
        else
            echo "  FAIL: ${name} — file not found"
        fi
    fi
}

check_dir() {
    local name="$1"
    local dir="$2"
    local status="pass"
    if [ ! -d "$dir" ]; then
        status="fail"
        FAIL=$((FAIL + 1))
    else
        PASS=$((PASS + 1))
    fi
    if [ "$JSON_OUTPUT" = false ]; then
        if [ "$status" = "pass" ]; then
            echo "  PASS: ${name}"
        else
            echo "  FAIL: ${name} — directory not found"
        fi
    fi
}

echo "============================================"
echo "KCM Documentation Validator"
echo "============================================"
echo ""

echo "[1/6] Root Documents"
check "SSOT.md" "${REPO_ROOT}/SSOT.md"
check "AGENTS.md" "${REPO_ROOT}/AGENTS.md"
check "README.md" "${REPO_ROOT}/README.md"
check "KCM_SPECIFICATION.md" "${REPO_ROOT}/KCM_SPECIFICATION.md"
check "ROADMAP.md" "${REPO_ROOT}/ROADMAP.md"
check "CHANGELOG.md" "${REPO_ROOT}/CHANGELOG.md"
check "LICENSE" "${REPO_ROOT}/LICENSE"
check "VERSION" "${REPO_ROOT}/VERSION"
check "SECURITY.md" "${REPO_ROOT}/SECURITY.md"
check "CONTRIBUTING.md" "${REPO_ROOT}/CONTRIBUTING.md"
check "CODE_OF_CONDUCT.md" "${REPO_ROOT}/CODE_OF_CONDUCT.md"

echo ""
echo "[2/6] Documentation Directories"
check_dir "docs/" "${DOCS_DIR}"
check_dir "docs/specs/" "${DOCS_DIR}/specs"
check_dir "docs/adr/" "${DOCS_DIR}/adr"
check_dir "docs/handbook/" "${DOCS_DIR}/handbook"
check_dir "docs/governance/" "${DOCS_DIR}/governance"
check_dir "docs/runbook/" "${DOCS_DIR}/runbook"
check_dir "docs/sdk/" "${DOCS_DIR}/sdk"
check_dir "docs/metrics/" "${DOCS_DIR}/metrics"
check_dir "docs/templates/" "${DOCS_DIR}/templates"

echo ""
echo "[3/6] Master Index"
check "docs/INDEX.md" "${DOCS_DIR}/INDEX.md"
check "docs/repository-map.md" "${DOCS_DIR}/repository-map.md"
check "docs/README.md" "${DOCS_DIR}/README.md"

echo ""
echo "[4/6] Governance Documents"
check "docs/governance/engineering-rules.md" "${DOCS_DIR}/governance/engineering-rules.md"
check "docs/governance/architecture-matrix.md" "${DOCS_DIR}/governance/architecture-matrix.md"
check "docs/governance/ssot-certification.md" "${DOCS_DIR}/governance/ssot-certification.md"
check "docs/governance/documentation-governance.md" "${DOCS_DIR}/governance/documentation-governance.md"

echo ""
echo "[5/6] Handbook Documents"
check "docs/handbook/repository-structure.md" "${DOCS_DIR}/handbook/repository-structure.md"
check "docs/handbook/handbook.md" "${DOCS_DIR}/handbook/handbook.md"

echo ""
echo "[6/6] No Stale Root References"
STALE=0
for f in KCM_ENGINEERING_RULES.md ARCHITECTURE_CONSISTENCY_MATRIX.md SSOT_CERTIFICATION_REPORT.md repository-health.md REPOSITORY_STRUCTURE.md; do
    if [ -f "${REPO_ROOT}/$f" ]; then
        echo "  FAIL: Stale root file: ${f}"
        STALE=$((STALE + 1))
    fi
done
if [ "$STALE" -eq 0 ]; then
    echo "  PASS: No stale root files"
    PASS=$((PASS + 1))
else
    FAIL=$((FAIL + STALE))
fi

echo ""
echo "============================================"
echo "Validation Results"
echo "  Passed: ${PASS}"
echo "  Failed: ${FAIL}"
echo "============================================"

if [ "$FAIL" -gt 0 ]; then
    echo ""
    echo "RESULT: FAILED — ${FAIL} check(s) failed"
    exit 1
else
    echo ""
    echo "RESULT: PASSED — All documentation checks passed"
    exit 0
fi
