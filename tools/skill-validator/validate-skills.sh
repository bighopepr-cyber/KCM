#!/usr/bin/env bash
# KCM Skill Validator
# Validates all skills follow AGENTS.md governance
set -uo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
ERRORS=0
CHECKS=0

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

pass() { ((CHECKS++)); echo -e "${GREEN}✓${NC} $1"; }
fail() { ((CHECKS++)); ((ERRORS++)); echo -e "${RED}✗${NC} $1"; }
warn() { echo -e "${YELLOW}⚠${NC} $1"; }

echo "========================================="
echo " KCM Skill Validator"
echo "========================================="
echo ""

SKILLS=(
    "kcm-engineering-orchestrator"
    "kcm-task-planner"
    "kcm-change-impact-analysis"
    "kcm-specification-lock"
    "kcm-architecture-guardian"
    "kcm-database-engine-specialist"
    "kcm-security-engineer"
    "kcm-performance-engineer"
    "kcm-testing-verification"
    "kcm-code-quality-guardian"
    "kcm-documentation-guardian"
    "kcm-release-readiness"
    "kcm-code-review-auditor"
    "kcm-debugging-root-cause"
    "kcm-engineering-decision-record"
    "kcm-repository-intelligence"
)

REQUIRED_SECTIONS=(
    "Overview"
    "Mission"
    "Responsibilities"
    "Authority"
    "Scope"
    "Non Goals"
    "Inputs"
    "Outputs"
    "Workflow"
    "Decision Process"
    "Validation"
    "Quality Gates"
    "Dependencies"
    "Related Skills"
    "SSOT References"
    "Failure Conditions"
    "Escalation"
    "Examples"
    "Checklist"
    "References"
)

# Check directory structure
echo "=== Checking Directory Structure ==="
for skill in "${SKILLS[@]}"; do
    dir="$REPO_ROOT/skills/$skill"
    
    # Check SKILL.md
    if [ -f "$dir/SKILL.md" ]; then
        pass "$skill: SKILL.md exists"
    else
        fail "$skill: SKILL.md MISSING"
    fi
    
    # Check README.md
    if [ -f "$dir/README.md" ]; then
        pass "$skill: README.md exists"
    else
        fail "$skill: README.md MISSING"
    fi
    
    # Check directories
    for subdir in checklists examples templates; do
        if [ -d "$dir/$subdir" ]; then
            pass "$skill: $subdir/ exists"
        else
            fail "$skill: $subdir/ MISSING"
        fi
    done
    
    # Check files in subdirectories
    if [ -d "$dir/checklists" ] && [ "$(ls -A "$dir/checklists" 2>/dev/null)" ]; then
        pass "$skill: checklists/ has files"
    else
        fail "$skill: checklists/ is empty"
    fi
    
    if [ -d "$dir/examples" ] && [ "$(ls -A "$dir/examples" 2>/dev/null)" ]; then
        pass "$skill: examples/ has files"
    else
        fail "$skill: examples/ is empty"
    fi
    
    if [ -d "$dir/templates" ] && [ "$(ls -A "$dir/templates" 2>/dev/null)" ]; then
        pass "$skill: templates/ has files"
    else
        fail "$skill: templates/ is empty"
    fi
done

# Check SKILL.md sections
echo ""
echo "=== Checking SKILL.md Sections ==="
for skill in "${SKILLS[@]}"; do
    skillfile="$REPO_ROOT/skills/$skill/SKILL.md"
    [ ! -f "$skillfile" ] && continue
    
    for section in "${REQUIRED_SECTIONS[@]}"; do
        if grep -q "^## $section" "$skillfile" 2>/dev/null; then
            pass "$skill: section '$section' found"
        else
            fail "$skill: section '$section' MISSING"
        fi
    done
done

# Check Document ID
echo ""
echo "=== Checking Document IDs ==="
for skill in "${SKILLS[@]}"; do
    skillfile="$REPO_ROOT/skills/$skill/SKILL.md"
    [ ! -f "$skillfile" ] && continue
    
    if grep -q "KCM-SKILL-" "$skillfile" 2>/dev/null; then
        pass "$skill: has Document ID"
    else
        fail "$skill: missing Document ID"
    fi
done

# Check authority system exists
echo ""
echo "=== Checking Governance Files ==="
for f in AUTHORITY-SYSTEM.md DECISION-MATRIX.md WORKFLOW.md; do
    if [ -f "$REPO_ROOT/skills/$f" ]; then
        pass "skills/$f exists"
    else
        fail "skills/$f MISSING"
    fi
done

# Check AGENTS.md
if [ -f "$REPO_ROOT/AGENTS.md" ]; then
    pass "AGENTS.md exists"
    if grep -q "Engineering Constitution" "$REPO_ROOT/AGENTS.md" 2>/dev/null; then
        pass "AGENTS.md is Engineering Constitution"
    else
        fail "AGENTS.md missing 'Engineering Constitution'"
    fi
else
    fail "AGENTS.md MISSING"
fi

# Summary
echo ""
echo "========================================="
echo " Results"
echo "========================================="
echo " Total checks: $CHECKS"
echo -e " Passed: ${GREEN}$((CHECKS - ERRORS))${NC}"
echo -e " Failed: ${RED}$ERRORS${NC}"
echo ""

if [ $ERRORS -gt 0 ]; then
    echo -e "${RED}VALIDATION FAILED${NC}"
    exit 1
else
    echo -e "${GREEN}VALIDATION PASSED${NC}"
    exit 0
fi
