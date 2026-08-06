#!/usr/bin/env bash
# KCM Version Verification Script
# Validates that all version references in the repository match the canonical VERSION file.
#
# Usage: bash scripts/release/verify-version.sh
#
# Exit codes:
#   0 - All versions are consistent
#   1 - Version mismatches found
#
# Document ID: KCM-REL-VERIFY-001
# Version: 1.0.0
# Status: Active

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
VERSION_FILE="${REPO_ROOT}/VERSION"

if [ ! -f "$VERSION_FILE" ]; then
    echo "FATAL: VERSION file not found at ${VERSION_FILE}"
    exit 1
fi

EXPECTED_VERSION="$(tr -d '[:space:]' < "$VERSION_FILE")"

echo "============================================"
echo "KCM Version Verification"
echo "Expected Version: ${EXPECTED_VERSION}"
echo "============================================"
echo ""

ERRORS=0
WARNINGS=0

check_version() {
    local file="$1"
    local pattern="$2"
    local description="$3"
    if [ -f "$file" ]; then
        if grep -q "$pattern" "$file" 2>/dev/null; then
            echo "  OK: ${description}"
        else
            echo "  FAIL: ${description} - pattern '${pattern}' not found in ${file}"
            ERRORS=$((ERRORS + 1))
        fi
    else
        echo "  WARN: ${description} - file not found: ${file}"
        WARNINGS=$((WARNINGS + 1))
    fi
}

echo "[1/8] Workspace Cargo.toml"
check_version "${REPO_ROOT}/Cargo.toml" "version = \"${EXPECTED_VERSION}\"" "Workspace version"

echo "[2/8] SDK Package Files"
check_version "${REPO_ROOT}/sdk/rust/Cargo.toml" "version = \"${EXPECTED_VERSION}\"" "Rust SDK version"
check_version "${REPO_ROOT}/sdk/python/pyproject.toml" "version = \"${EXPECTED_VERSION}\"" "Python SDK version"
check_version "${REPO_ROOT}/sdk/javascript/package.json" "\"version\": \"${EXPECTED_VERSION}\"" "JavaScript SDK version"
check_version "${REPO_ROOT}/sdk/typescript/package.json" "\"version\": \"${EXPECTED_VERSION}\"" "TypeScript SDK version"
check_version "${REPO_ROOT}/sdk/java/pom.xml" "<version>${EXPECTED_VERSION}</version>" "Java SDK version"
check_version "${REPO_ROOT}/sdk/dotnet/Kcm.Sdk.csproj" "<Version>${EXPECTED_VERSION}</Version>" ".NET SDK version"
check_version "${REPO_ROOT}/sdk/cpp/CMakeLists.txt" "VERSION ${EXPECTED_VERSION}" "C++ SDK version"

echo "[3/8] Deployment Files"
check_version "${REPO_ROOT}/deployment/helm/kcm/Chart.yaml" "version: ${EXPECTED_VERSION}" "Helm chart version"
check_version "${REPO_ROOT}/deployment/helm/kcm/Chart.yaml" "appVersion: \"${EXPECTED_VERSION}\"" "Helm appVersion"

echo "[4/8] Examples"
check_version "${REPO_ROOT}/examples/rust/Cargo.toml" "version = \"${EXPECTED_VERSION}\"" "Examples version"

echo "[5/8] SDK README Badges"
for sdk_dir in "${REPO_ROOT}"/sdk/*/; do
    if [ -f "${sdk_dir}README.md" ]; then
        sdk_name="$(basename "${sdk_dir}")"
        if grep -q "version-${EXPECTED_VERSION}-blue" "${sdk_dir}README.md" 2>/dev/null; then
            echo "  OK: ${sdk_name} SDK badge"
        else
            echo "  FAIL: ${sdk_name} SDK badge - version mismatch"
            ERRORS=$((ERRORS + 1))
        fi
    fi
done
if [ -f "${REPO_ROOT}/sdk/README.md" ]; then
    if grep -q "version-${EXPECTED_VERSION}-blue" "${REPO_ROOT}/sdk/README.md" 2>/dev/null; then
        echo "  OK: Root SDK badge"
    else
        echo "  FAIL: Root SDK badge - version mismatch"
        ERRORS=$((ERRORS + 1))
    fi
fi

echo "[6/8] Documentation"
check_version "${REPO_ROOT}/docs/benchmark-results/spesifikasi.md" "\"version\": \"${EXPECTED_VERSION}\"" "Benchmark spec version"
check_version "${REPO_ROOT}/docs/benchmark-results/spesifikasi.md" "\"kcm_version\": \"${EXPECTED_VERSION}\"" "Benchmark spec KCM version"
check_version "${REPO_ROOT}/docs/benchmark-results/spesifikasi.md" "\"tag\": \"v${EXPECTED_VERSION}\"" "Benchmark spec tag"
check_version "${REPO_ROOT}/docs/sdk/spesifikasi.md" "version = \"${EXPECTED_VERSION}\"" "SDK spec version"
check_version "${REPO_ROOT}/benchmark-results/README.md" "\"version\": \"${EXPECTED_VERSION}\"" "Benchmark README version"

echo "[7/8] CHANGELOG"
CHANGELOG_PATTERN="## [${EXPECTED_VERSION}]"
if [ -f "${REPO_ROOT}/CHANGELOG.md" ] && grep -Fq "${CHANGELOG_PATTERN}" "${REPO_ROOT}/CHANGELOG.md" 2>/dev/null; then
    echo "  OK: CHANGELOG entry"
else
    echo "  FAIL: CHANGELOG entry - '${CHANGELOG_PATTERN}' not found"
    ERRORS=$((ERRORS + 1))
fi

echo "[8/8] No Stale Versions"
STALE_COUNT=0
if [ -f "${REPO_ROOT}/Cargo.toml" ]; then
    if grep -q 'version = "0.1.0"' "${REPO_ROOT}/Cargo.toml" 2>/dev/null; then
        echo "  FAIL: Workspace Cargo.toml still has 0.1.0"
        STALE_COUNT=$((STALE_COUNT + 1))
    fi
fi
if [ -f "${REPO_ROOT}/sdk/rust/Cargo.toml" ]; then
    if grep -q 'version = "0.1.0"' "${REPO_ROOT}/sdk/rust/Cargo.toml" 2>/dev/null; then
        echo "  FAIL: Rust SDK still has 0.1.0"
        STALE_COUNT=$((STALE_COUNT + 1))
    fi
fi
if [ -f "${REPO_ROOT}/deployment/helm/kcm/Chart.yaml" ]; then
    if grep -q 'version: 0.1.0' "${REPO_ROOT}/deployment/helm/kcm/Chart.yaml" 2>/dev/null; then
        echo "  FAIL: Helm chart still has 0.1.0"
        STALE_COUNT=$((STALE_COUNT + 1))
    fi
fi
if [ "$STALE_COUNT" -eq 0 ]; then
    echo "  OK: No stale 0.1.0 versions found"
fi
ERRORS=$((ERRORS + STALE_COUNT))

echo ""
echo "============================================"
echo "Verification Results"
echo "  Version: ${EXPECTED_VERSION}"
echo "  Errors: ${ERRORS}"
echo "  Warnings: ${WARNINGS}"
echo "============================================"

if [ "$ERRORS" -gt 0 ]; then
    echo ""
    echo "RESULT: FAILED - ${ERRORS} version mismatch(es) found"
    exit 1
else
    echo ""
    echo "RESULT: PASSED - All versions are consistent"
    exit 0
fi
