#!/usr/bin/env bash
# KCM Version Synchronization Script
# Reads the canonical VERSION file and updates all version references across the repository.
#
# Usage: bash scripts/release/sync-version.sh [NEW_VERSION]
# If NEW_VERSION is not provided, reads from the VERSION file.
#
# Document ID: KCM-REL-SYNC-001
# Version: 1.0.0
# Status: Active

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
VERSION_FILE="${REPO_ROOT}/VERSION"
OLD_VERSION=""

if [ ! -f "$VERSION_FILE" ]; then
    echo "ERROR: VERSION file not found at ${VERSION_FILE}"
    exit 1
fi

NEW_VERSION="$(tr -d '[:space:]' < "$VERSION_FILE")"

if [ -n "${1:-}" ]; then
    NEW_VERSION="$1"
    echo "$NEW_VERSION" > "$VERSION_FILE"
fi

if [ -z "$NEW_VERSION" ]; then
    echo "ERROR: VERSION is empty"
    exit 1
fi

echo "============================================"
echo "KCM Version Synchronization"
echo "Target Version: ${NEW_VERSION}"
echo "============================================"
echo ""

UPDATED=0
FAILED=0

update_file() {
    local file="$1"
    local search="$2"
    local replace="$3"
    if [ -f "$file" ]; then
        if grep -q "$search" "$file" 2>/dev/null; then
            sed -i "s|${search}|${replace}|g" "$file"
            echo "  UPDATED: ${file}"
            UPDATED=$((UPDATED + 1))
        fi
    fi
}

echo "[1/7] Workspace Cargo.toml"
update_file "${REPO_ROOT}/Cargo.toml" 'version = "0.1.0"' "version = \"${NEW_VERSION}\""

echo "[2/7] SDK Package Files"
update_file "${REPO_ROOT}/sdk/rust/Cargo.toml" 'version = "0.1.0"' "version = \"${NEW_VERSION}\""
update_file "${REPO_ROOT}/sdk/python/pyproject.toml" 'version = "0.1.0"' "version = \"${NEW_VERSION}\""
update_file "${REPO_ROOT}/sdk/javascript/package.json" '"version": "0.1.0"' "\"version\": \"${NEW_VERSION}\""
update_file "${REPO_ROOT}/sdk/typescript/package.json" '"version": "0.1.0"' "\"version\": \"${NEW_VERSION}\""
update_file "${REPO_ROOT}/sdk/java/pom.xml" '<version>0.1.0</version>' "<version>${NEW_VERSION}</version>"
update_file "${REPO_ROOT}/sdk/dotnet/Kcm.Sdk.csproj" '<Version>0.1.0</Version>' "<Version>${NEW_VERSION}</Version>"
update_file "${REPO_ROOT}/sdk/cpp/CMakeLists.txt" 'VERSION 0.1.0' "VERSION ${NEW_VERSION}"

echo "[3/7] Deployment Files"
update_file "${REPO_ROOT}/deployment/helm/kcm/Chart.yaml" 'version: 0.1.0' "version: ${NEW_VERSION}"
update_file "${REPO_ROOT}/deployment/helm/kcm/Chart.yaml" 'appVersion: "0.1.0"' "appVersion: \"${NEW_VERSION}\""

echo "[4/7] Examples"
update_file "${REPO_ROOT}/examples/rust/Cargo.toml" 'version = "0.1.0"' "version = \"${NEW_VERSION}\""

echo "[5/7] SDK README Badges"
for sdk_dir in "${REPO_ROOT}"/sdk/*/; do
    if [ -f "${sdk_dir}README.md" ]; then
        sed -i "s|version-0.1.0-blue|version-${NEW_VERSION}-blue|g" "${sdk_dir}README.md"
        sed -i "s|KCM%20Engine-0.1.0-orange|KCM%20Engine-${NEW_VERSION}-orange|g" "${sdk_dir}README.md"
    fi
done
# Root SDK README
if [ -f "${REPO_ROOT}/sdk/README.md" ]; then
    sed -i "s|version-0.1.0-blue|version-${NEW_VERSION}-blue|g" "${REPO_ROOT}/sdk/README.md"
    sed -i "s|KCM%20Engine-0.1.0-orange|KCM%20Engine-${NEW_VERSION}-orange|g" "${REPO_ROOT}/sdk/README.md"
fi
echo "  UPDATED: SDK README badges"

echo "[6/7] Documentation"
update_file "${REPO_ROOT}/docs/benchmark-results/spesifikasi.md" '"version": "0.1.0"' "\"version\": \"${NEW_VERSION}\""
update_file "${REPO_ROOT}/docs/benchmark-results/spesifikasi.md" '"kcm_version": "0.1.0"' "\"kcm_version\": \"${NEW_VERSION}\""
update_file "${REPO_ROOT}/docs/benchmark-results/spesifikasi.md" '"tag": "v0.1.0'" "\"tag\": \"v${NEW_VERSION}\""
update_file "${REPO_ROOT}/docs/sdk/spesifikasi.md" 'version = "0.2.0"' "version = \"${NEW_VERSION}\""
update_file "${REPO_ROOT}/docs/sdk/spesifikasi.md" '# Requires engine 0.1.x' "# Requires engine ${NEW_VERSION%.*}.x"
update_file "${REPO_ROOT}/benchmark-results/README.md" '"version": "0.1.0"' "\"version\": \"${NEW_VERSION}\""

echo "[7/7] Repository Structure"
update_file "${REPO_ROOT}/REPOSITORY_STRUCTURE.md" 'Helm chart v0.1.0' "Helm chart v${NEW_VERSION}"

echo ""
echo "============================================"
echo "Synchronization Complete"
echo "  Updated: ${UPDATED} files"
echo "  Version: ${NEW_VERSION}"
echo "============================================"
echo ""
echo "Run 'bash scripts/release/verify-version.sh' to validate synchronization."
