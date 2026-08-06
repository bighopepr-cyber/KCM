#!/usr/bin/env bash
# KCM Documentation Drift Detector
# Detects drift between code and documentation
set -uo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
DRIFTS=0
CHECKS=0

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

pass() { ((CHECKS++)); }
fail() { ((CHECKS++)); ((DRIFTS++)); echo -e "${RED}✗${NC} $1"; }
warn() { ((DRIFTS++)); echo -e "${YELLOW}⚠${NC} $1"; }

echo "========================================="
echo " KCM Documentation Drift Detector"
echo "========================================="
echo ""

# 1. Check crates in Cargo.toml vs docs
echo "=== Crate Drift ==="
CARGO_CRATES=$(grep -oP 'crates/\K[a-z-]+' "$REPO_ROOT/Cargo.toml" | sort -u)
for crate in $CARGO_CRATES; do
    ((CHECKS++))
    if [ -f "$REPO_ROOT/crates/$crate/README.md" ]; then
        pass
    else
        fail "Crate '$crate' in Cargo.toml but missing crates/$crate/README.md"
    fi
    ((CHECKS++))
    if [ -f "$REPO_ROOT/docs/$crate/spesifikasi.md" ]; then
        pass
    else
        fail "Crate '$crate' in Cargo.toml but missing docs/$crate/spesifikasi.md"
    fi
done

# 2. Check public functions vs documentation
echo ""
echo "=== API Drift ==="
for crate_dir in "$REPO_ROOT"/crates/kcm-*/src; do
    crate_name=$(basename $(dirname "$crate_dir"))
    for src_file in "$crate_dir"/*.rs; do
        [ ! -f "$src_file" ] && continue
        fname=$(basename "$src_file" .rs)
        # Check if module is documented in README
        if grep -q "pub mod $fname" "$src_file" 2>/dev/null; then
            ((CHECKS++))
            if grep -q "$fname" "$crate_dir/../README.md" 2>/dev/null || grep -q "$fname" "$crate_dir/../../docs/$crate_name/spesifikasi.md" 2>/dev/null; then
                pass
            else
                warn "Module '$crate_name::$fname' not documented in README or spesifikasi"
            fi
        fi
    done
done

# 3. Check REST endpoints vs documentation
echo ""
echo "=== REST API Drift ==="
if [ -f "$REPO_ROOT/crates/kcm-server/src/main.rs" ]; then
    ENDPOINTS=$(grep -oP '"/[^"]+' "$REPO_ROOT/crates/kcm-server/src/main.rs" | sort -u)
    for ep in $ENDPOINTS; do
        ((CHECKS++))
        if grep -q "$ep" "$REPO_ROOT/docs/kcm-server/spesifikasi.md" 2>/dev/null || grep -q "$ep" "$REPO_ROOT/docs/kcm-interface/spesifikasi.md" 2>/dev/null; then
            pass
        else
            warn "REST endpoint '$ep' not documented in spesifikasi"
        fi
    done
fi

# 4. Check SDK languages vs documentation
echo ""
echo "=== SDK Drift ==="
for sdk_dir in "$REPO_ROOT"/sdk/*/; do
    sdk_name=$(basename "$sdk_dir")
    ((CHECKS++))
    if [ -f "$sdk_dir/README.md" ]; then
        pass
    else
        fail "SDK '$sdk_name' missing README.md"
    fi
done

# 5. Check workflow files vs documentation
echo ""
echo "=== Workflow Drift ==="
for wf in "$REPO_ROOT"/.github/workflows/*.yml; do
    [ ! -f "$wf" ] && continue
    wf_name=$(basename "$wf" .yml)
    ((CHECKS++))
    if grep -q "$wf_name" "$REPO_ROOT/.github/README.md" 2>/dev/null; then
        pass
    else
        warn "Workflow '$wf_name' not documented in .github/README.md"
    fi
done

# 6. Check CLI tools vs documentation
echo ""
echo "=== CLI Drift ==="
for cli_dir in "$REPO_ROOT"/scripts/kcm-cli/kcm-*/; do
    cli_name=$(basename "$cli_dir")
    ((CHECKS++))
    if [ -f "$cli_dir/README.md" ]; then
        pass
    else
        fail "CLI tool '$cli_name' missing README.md"
    fi
done

echo ""
echo "========================================="
echo " Drift Detection Results"
echo "========================================="
echo " Checks: $CHECKS"
echo -e " Drifts: ${RED}$DRIFTS${NC}"
echo ""

if [ $DRIFTS -gt 0 ]; then
    echo -e "${YELLOW}DRIFT DETECTED${NC}"
    exit 1
else
    echo -e "${GREEN}NO DRIFT DETECTED${NC}"
    exit 0
fi
