#!/usr/bin/env bash
# KCM Documentation Index Generator
# Auto-generates docs/INDEX.md with navigation to all documentation
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
INDEX="$REPO_ROOT/docs/INDEX.md"
DATE=$(date -u +"%Y-%m-%d")

cat > "$INDEX" << 'HEADER'
# KCM Documentation Index

> Auto-generated documentation index. Do not edit manually.
> Regenerate: `bash tools/doc-generator/generate-index.sh`

## Table of Contents

- [Core Documentation](#core-documentation)
- [Specifications](#specifications)
- [SDK Documentation](#sdk-documentation)
- [Crate Documentation](#crate-documentation)
- [Deployment](#deployment)
- [Testing](#testing)
- [Scripts & CLI](#scripts--cli)
- [Examples](#examples)
- [Engineering Skills](#engineering-skills)
- [Architecture Decision Records](#architecture-decision-records)
- [Operations](#operations)
- [Governance](#governance)
- [Templates](#templates)
- [Metrics & Reports](#metrics--reports)

---

## Core Documentation

| Document | Description |
|----------|-------------|
HEADER

# Add root docs
for doc in SSOT.md AGENTS.md KCM_SPECIFICATION.md CONTRIBUTING.md CODE_OF_CONDUCT.md SECURITY.md README.md REPOSITORY_STRUCTURE.md ROADMAP.md ARCHITECTURE_CONSISTENCY_MATRIX.md; do
    if [ -f "$REPO_ROOT/$doc" ]; then
        name="${doc%.md}"
        echo "| [$name]('../$doc') | Core project document |" >> "$INDEX"
    fi
done

# Add specifications
cat >> "$INDEX" << 'EOF'

## Specifications

| Document | Description |
|----------|-------------|
EOF
for spec in "$REPO_ROOT"/docs/specs/*.md; do
    [ ! -f "$spec" ] && continue
    name=$(basename "$spec" .md)
    echo "| [$name](specs/$name.md) | Specification document |" >> "$INDEX"
done

# Add SDK docs
cat >> "$INDEX" << 'EOF'

## SDK Documentation

| Document | Description |
|----------|-------------|
EOF
for sdk in "$REPO_ROOT"/docs/sdk/*.md; do
    [ ! -f "$sdk" ] && continue
    name=$(basename "$sdk" .md)
    echo "| [$name](sdk/$name.md) | SDK documentation |" >> "$INDEX"
done

# Add crate spesifikasi
cat >> "$INDEX" << 'EOF'

## Crate Documentation

| Crate | Specification | README |
|-------|--------------|--------|
EOF
for spec in "$REPO_ROOT"/docs/kcm-*/spesifikasi.md; do
    [ ! -f "$spec" ] && continue
    crate=$(basename $(dirname "$spec"))
    echo "| $crate | [spesifikasi]($crate/spesifikasi.md) | [README](../crates/$crate/README.md) |" >> "$INDEX"
done

# Add ADRs
cat >> "$INDEX" << 'EOF'

## Architecture Decision Records

| ADR | Title |
|-----|-------|
EOF
for adr in "$REPO_ROOT"/docs/adr/ADR-*.md; do
    [ ! -f "$adr" ] && continue
    name=$(basename "$adr" .md)
    echo "| [$name](adr/$name.md) | Architecture Decision Record |" >> "$INDEX"
done

# Add remaining sections
cat >> "$INDEX" << 'EOF'

## Deployment

| Document | Description |
|----------|-------------|
| [Deployment Specification](specs/KCM_DEPLOYMENT_SPEC.md) | Deployment architecture |
| [Deployment README](../deployment/README.md) | Deployment configurations |

## Testing

| Document | Description |
|----------|-------------|
| [Testing Specification](specs/KCM_TESTING_SPEC.md) | Testing strategy |
| [Tests README](../tests/README.md) | Integration tests |
| [SDK Tests](../tests/sdk/README.md) | SDK cross-language tests |

## Scripts & CLI

| Document | Description |
|----------|-------------|
| [Scripts README](../scripts/README.md) | Build scripts |
| [CLI README](../scripts/kcm-cli/README.md) | CLI tools overview |

## Examples

| Document | Description |
|----------|-------------|
| [Examples README](../examples/README.md) | Code examples |

## Engineering Skills

| Document | Description |
|----------|-------------|
| [Skills README](../skills/README.md) | 16 AI engineering skills |

## Operations

| Document | Description |
|----------|-------------|
| [Operational Runbook](runbook/OPERATIONAL_RUNBOOK.md) | Operations guide |
| [Disaster Recovery](runbook/DISASTER_RECOVERY.md) | DR procedures |
| [Handbook](handbook/handbook.md) | Project handbook |

## Governance

| Document | Description |
|----------|-------------|
| [Documentation Governance](governance/documentation-governance.md) | Documentation governance system |

## Templates

| Template | Description |
|----------|-------------|
| [All Templates](templates/) | Documentation templates |

## Metrics & Reports

| Document | Description |
|----------|-------------|
| [Coverage Report](metrics/coverage.md) | Documentation coverage |
| [Repository Health](../repository-health.md) | Repository health report |

---

> This index was auto-generated on $DATE.
> Regenerate: `bash tools/doc-generator/generate-index.sh`
EOF

echo "Index generated: $INDEX"
