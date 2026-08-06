#!/usr/bin/env bash
# KCM SDK API Validation Script v1.0
# Validates SDK structure, API surface, tests, examples, and documentation
set -uo pipefail

ERRORS=0
WARNINGS=0

check() {
    local desc="$1"
    local result="$2"
    if [ "$result" = "0" ]; then
        echo "  PASS: $desc"
    else
        echo "  FAIL: $desc"
        ERRORS=$((ERRORS + 1))
    fi
}

warn() {
    local desc="$1"
    local result="$2"
    if [ "$result" = "0" ]; then
        echo "  PASS: $desc"
    else
        echo "  WARN: $desc"
        WARNINGS=$((WARNINGS + 1))
    fi
}

echo "=== KCM SDK API Validation v1.0 ==="
echo ""

# Required SDK directories
SDKS=("rust" "python" "javascript" "typescript" "go" "java" "dotnet" "c" "cpp")

# Required files per SDK
declare -A REQUIRED_FILES
REQUIRED_FILES[rust]="Cargo.toml README.md src/lib.rs"
REQUIRED_FILES[python]="pyproject.toml README.md src/kcm/__init__.py"
REQUIRED_FILES[javascript]="package.json README.md src/index.js"
REQUIRED_FILES[typescript]="package.json README.md src/index.ts"
REQUIRED_FILES[go]="go.mod README.md kcm.go"
REQUIRED_FILES[java]="pom.xml README.md"
REQUIRED_FILES[dotnet]="Kcm.Sdk.csproj README.md"
REQUIRED_FILES[c]="Makefile README.md include/kcm.h"
REQUIRED_FILES[cpp]="CMakeLists.txt README.md include/kcm.hpp"

# Required API operations (with language-specific variations)
# Format: operation_name|rust|python|javascript|typescript|go|java|dotnet|c|cpp
declare -A API_VARIATIONS
API_VARIATIONS[insert]="insert|insert|insert|insert|Insert|insert|Insert|KCM_DatabaseInsert|insert"
API_VARIATIONS[query]="query|query|query|query|Query|query|Query|KCM_DatabaseQuery|query"
API_VARIATIONS[delete]="delete|delete|delete|delete|Delete|delete|Delete|KCM_DatabaseDelete|remove"
API_VARIATIONS[update]="update|update|update|update|Update|update|Update|KCM_DatabaseUpdate|update"
API_VARIATIONS[fact_count]="fact_count|fact_count|factCount|factCount|FactCount|factCount|FactCount|KCM_DatabaseFactCount|fact_count"
API_VARIATIONS[active]="active|active|active|active|Active|active|Active|KCM_DatabaseActive|active"
API_VARIATIONS[begin_transaction]="begin_transaction|begin_transaction|beginTransaction|beginTransaction|BeginTransaction|beginTransaction|BeginTransaction|KCM_DatabaseBeginTransaction|begin_transaction"
API_VARIATIONS[commit]="commit|commit|commit|commit|Commit|commit|Commit|KCM_TransactionCommit|commit"
API_VARIATIONS[rollback]="rollback|rollback|rollback|rollback|Rollback|rollback|Rollback|KCM_TransactionRollback|rollback"
API_VARIATIONS[save]="save|save|save|save|Save|save|Save|KCM_DatabaseSave|save"
API_VARIATIONS[load]="load|load|load|load|Load|load|Load|KCM_DatabaseLoad|load"
API_VARIATIONS[verify]="verify|verify|verify|verify|Verify|verify|Verify|KCM_DatabaseVerify|verify"
API_VARIATIONS[close]="close|close|close|close|Close|close|Dispose|KCM_DatabaseFree|~Database"

echo "=== Structure Validation ==="
for sdk in "${SDKS[@]}"; do
    echo ""
    echo "[$sdk]"
    
    # Check SDK directory exists
    if [ -d "sdk/$sdk" ]; then
        check "SDK directory exists" "0"
    else
        check "SDK directory exists" "1"
        continue
    fi
    
    # Check required files
    for file in ${REQUIRED_FILES[$sdk]}; do
        if [ -f "sdk/$sdk/$file" ]; then
            check "Required file: $file" "0"
        else
            check "Required file: $file" "1"
        fi
    done
done

echo ""
echo "=== API Surface Validation ==="
for sdk in "${SDKS[@]}"; do
    echo ""
    echo "[$sdk]"
    
    if [ ! -d "sdk/$sdk" ]; then
        warn "SDK directory missing, skipping API check" "1"
        continue
    fi
    
    # Get SDK index for variation lookup
    case $sdk in
        rust) SDK_IDX=1 ;;
        python) SDK_IDX=2 ;;
        javascript) SDK_IDX=3 ;;
        typescript) SDK_IDX=4 ;;
        go) SDK_IDX=5 ;;
        java) SDK_IDX=6 ;;
        dotnet) SDK_IDX=7 ;;
        c) SDK_IDX=8 ;;
        cpp) SDK_IDX=9 ;;
    esac
    
    # Check for API operations in source files
    for op in insert query delete update fact_count active begin_transaction commit rollback save load verify close; do
        VARIATION=$(echo "${API_VARIATIONS[$op]}" | cut -d'|' -f$SDK_IDX)
        
        case $sdk in
            rust)
                if grep -r "fn.*$VARIATION" sdk/rust/src/ >/dev/null 2>&1; then
                    check "API: $op ($VARIATION)" "0"
                else
                    check "API: $op ($VARIATION)" "1"
                fi
                ;;
            python)
                if grep -r "def.*$VARIATION" sdk/python/src/ >/dev/null 2>&1; then
                    check "API: $op ($VARIATION)" "0"
                else
                    check "API: $op ($VARIATION)" "1"
                fi
                ;;
            javascript|typescript)
                if grep -r "$VARIATION" sdk/$sdk/src/ >/dev/null 2>&1; then
                    check "API: $op ($VARIATION)" "0"
                else
                    check "API: $op ($VARIATION)" "1"
                fi
                ;;
            go)
                if grep -r "$VARIATION" sdk/go/kcm.go >/dev/null 2>&1; then
                    check "API: $op ($VARIATION)" "0"
                else
                    check "API: $op ($VARIATION)" "1"
                fi
                ;;
            java)
                if grep -r "$VARIATION" sdk/java/src/ >/dev/null 2>&1; then
                    check "API: $op ($VARIATION)" "0"
                else
                    check "API: $op ($VARIATION)" "1"
                fi
                ;;
            dotnet)
                if grep -ri "$VARIATION" sdk/dotnet/src/ >/dev/null 2>&1; then
                    check "API: $op ($VARIATION)" "0"
                else
                    check "API: $op ($VARIATION)" "1"
                fi
                ;;
            c)
                if grep -ri "$VARIATION" sdk/c/include/ >/dev/null 2>&1; then
                    check "API: $op ($VARIATION)" "0"
                else
                    check "API: $op ($VARIATION)" "1"
                fi
                ;;
            cpp)
                if grep -r "$VARIATION" sdk/cpp/include/ >/dev/null 2>&1; then
                    check "API: $op ($VARIATION)" "0"
                else
                    check "API: $op ($VARIATION)" "1"
                fi
                ;;
        esac
    done
done

echo ""
echo "=== Test Validation ==="
for sdk in "${SDKS[@]}"; do
    echo ""
    echo "[$sdk]"
    
    if [ ! -d "sdk/$sdk" ]; then
        warn "SDK directory missing, skipping test check" "1"
        continue
    fi
    
    # Check for test directories or test files
    case $sdk in
        rust)
            if [ -d "sdk/rust/tests" ] || find sdk/rust/src -name "*test*" -type f 2>/dev/null | grep -q .; then
                check "Tests exist" "0"
            else
                check "Tests exist" "1"
            fi
            ;;
        python)
            if [ -d "sdk/python/tests" ]; then
                check "Tests exist" "0"
            else
                check "Tests exist" "1"
            fi
            ;;
        javascript|typescript)
            if [ -d "sdk/$sdk/tests" ]; then
                check "Tests exist" "0"
            else
                check "Tests exist" "1"
            fi
            ;;
        go)
            if find sdk/go -name "*_test.go" -type f 2>/dev/null | grep -q .; then
                check "Tests exist" "0"
            else
                check "Tests exist" "1"
            fi
            ;;
        java)
            if [ -d "sdk/java/src/test" ]; then
                check "Tests exist" "0"
            else
                check "Tests exist" "1"
            fi
            ;;
        dotnet)
            if [ -d "sdk/dotnet/tests" ]; then
                check "Tests exist" "0"
            else
                check "Tests exist" "1"
            fi
            ;;
        c)
            if [ -d "sdk/c/tests" ]; then
                check "Tests exist" "0"
            else
                check "Tests exist" "1"
            fi
            ;;
        cpp)
            if [ -d "sdk/cpp/tests" ]; then
                check "Tests exist" "0"
            else
                check "Tests exist" "1"
            fi
            ;;
    esac
done

echo ""
echo "=== Example Validation ==="
for sdk in "${SDKS[@]}"; do
    echo ""
    echo "[$sdk]"
    
    if [ ! -d "sdk/$sdk" ]; then
        warn "SDK directory missing, skipping example check" "1"
        continue
    fi
    
    if [ -d "sdk/$sdk/examples" ]; then
        EXAMPLE_COUNT=$(find sdk/$sdk/examples -type f | wc -l)
        if [ "$EXAMPLE_COUNT" -ge 1 ]; then
            check "Examples exist ($EXAMPLE_COUNT files)" "0"
        elif [ "$sdk" = "java" ] && [ -d "sdk/java/src/main/java/io/kcm/examples" ]; then
            # Java examples are in src/main/java/io/kcm/examples/
            EXAMPLE_COUNT=$(find sdk/java/src/main/java/io/kcm/examples -type f | wc -l)
            if [ "$EXAMPLE_COUNT" -ge 1 ]; then
                check "Examples exist ($EXAMPLE_COUNT files)" "0"
            else
                check "Examples exist" "1"
            fi
        else
            check "Examples exist" "1"
        fi
    elif [ "$sdk" = "java" ]; then
        # Java examples are in src/main/java/io/kcm/examples/
        if [ -d "sdk/java/src/main/java/io/kcm/examples" ]; then
            EXAMPLE_COUNT=$(find sdk/java/src/main/java/io/kcm/examples -type f | wc -l)
            if [ "$EXAMPLE_COUNT" -ge 1 ]; then
                check "Examples exist ($EXAMPLE_COUNT files)" "0"
            else
                check "Examples exist" "1"
            fi
        elif [ -f "sdk/java/src/main/java/io/kcm/Example.java" ]; then
            check "Examples exist (1 file)" "0"
        else
            check "Examples exist" "1"
        fi
    else
        check "Examples directory exists" "1"
    fi
done

echo ""
echo "=== Documentation Validation ==="
for sdk in "${SDKS[@]}"; do
    echo ""
    echo "[$sdk]"
    
    if [ ! -d "sdk/$sdk" ]; then
        warn "SDK directory missing, skipping docs check" "1"
        continue
    fi
    
    # Check README exists and has content
    if [ -f "sdk/$sdk/README.md" ]; then
        README_LINES=$(wc -l < "sdk/$sdk/README.md")
        if [ "$README_LINES" -ge 10 ]; then
            check "README.md has content ($README_LINES lines)" "0"
        else
            check "README.md has content ($README_LINES lines, expected >= 10)" "1"
        fi
    else
        check "README.md exists" "1"
    fi
done

echo ""
echo "=== Cross-SDK Consistency ==="

# Check that sdk/README.md exists and documents API surface
if [ -f "sdk/README.md" ]; then
    for op in "${REQUIRED_API[@]}"; do
        if grep -q "$op" sdk/README.md; then
            check "API surface documented: $op" "0"
        else
            check "API surface documented: $op" "1"
        fi
    done
else
    check "sdk/README.md exists" "1"
fi

echo ""
echo "=== Results ==="
TOTAL_CHECKS=$((ERRORS + WARNINGS))
PASSED=$((TOTAL_CHECKS - ERRORS - WARNINGS))
echo "  Checks passed: $PASSED"
echo "  Errors: $ERRORS"
echo "  Warnings: $WARNINGS"
echo ""
if [ "$ERRORS" -eq 0 ]; then
    echo "ALL CHECKS PASSED"
    exit 0
else
    echo "FAILED: $ERRORS check(s) failed"
    exit 1
fi
