# KCM Documentation

> Central documentation hub for the KCM project.

## Overview

The `docs/` directory houses all SSOT specifications, Architecture Decision Records, operational handbooks, runbooks, SDK documentation, and governance documents.

## Quick Start

- **New to KCM?** Start with [`../README.md`](../README.md)
- **Looking for specs?** See [`docs/specs/`](specs/)
- **Need SDK help?** See [`docs/sdk/`](sdk/)
- **Contributing?** See [`../CONTRIBUTING.md`](../CONTRIBUTING.md)

## Documentation Structure

```
docs/
├── INDEX.md                    ← Master navigation entry point
├── repository-map.md           ← Complete repository structure
├── README.md                   ← This file
│
├── specs/                      ← SSOT specifications (19 files)
│   ├── PRD.md                  ← Core types, storage, compute
│   ├── PRD2.md                 ← Storage, runtime, interfaces
│   ├── PRD3.md                 ← Distributed, ML, security
│   ├── PRD-TESTING-AND-BENCHMARK.md ← Testing strategy
│   └── KCM_*_SPEC.md           ← Component specs (15 files)
│
├── adr/                        ← Architecture Decision Records (10)
│   ├── ADR-001 through ADR-010
│   └── templates/ADR-template.md
│
├── handbook/                   ← Developer guides
│   ├── repository-structure.md ← Complete repo reference
│   └── handbook.md             ← Onboarding guide
│
├── governance/                 ← Governance documents
│   ├── engineering-rules.md    ← Development rules
│   ├── architecture-matrix.md  ← Component registry
│   ├── ssot-certification.md   ← SSOT compliance
│   └── documentation-governance.md
│
├── runbook/                    ← Operational procedures
│   ├── OPERATIONAL_RUNBOOK.md
│   └── DISASTER_RECOVERY.md
│
├── sdk/                        ← SDK documentation (11 files)
│   ├── rust.md, c.md, cpp.md
│   ├── python.md, javascript.md
│   ├── typescript.md, go.md
│   ├── java.md, dotnet.md
│   ├── compatibility.md
│   └── spesifikasi.md
│
├── metrics/                    ← Reports and metrics
│   └── repository-health.md
│
├── templates/                  ← Documentation templates (8)
│   ├── ADR-template.md
│   ├── README-template.md
│   ├── spesifikasi-template.md
│   └── ...
│
├── <crate>/                    ← Per-crate specifications (13)
│   └── spesifikasi.md
│
├── agents/                     ← AI agent docs (redirect to skills/)
├── assets/                     ← Asset documentation
├── benchmark-results/          ← Benchmark documentation
├── cargo/                      ← Cargo configuration docs
├── deployment/                 ← Deployment documentation
├── docs/                       ← Meta-documentation
├── examples/                   ← Example documentation
├── github/                     ← GitHub configuration docs
├── index/                      ← Index documentation
├── scripts/                    ← Script documentation
├── skills/                     ← Skills specification
├── tests/                      ← Test documentation
└── validation/                 ← Validation documentation
```

## Document Hierarchy

```
P1: SSOT.md (root)              ← Absolute authority
P2: AGENTS.md (root)            ← Engineering constitution
P3: PRD-TESTING-AND-BENCHMARK   ← Testing strategy
P4: PRD3.md                     ← Distributed, ML, security
P5: PRD2.md                     ← Storage, runtime, interfaces
P6: PRD.md                      ← Core types, compute
P7: KCM_*_SPEC.md               ← Component specs
```

## Canonical Locations

| Document Type | Location | Example |
|---------------|----------|---------|
| Specifications | `docs/specs/` | PRD.md, KCM_*_SPEC.md |
| ADRs | `docs/adr/` | ADR-001.md |
| Runbooks | `docs/runbook/` | OPERATIONAL_RUNBOOK.md |
| SDK Docs | `docs/sdk/` | rust.md, python.md |
| Crate Specs | `docs/<crate>/` | kcm-core/spesifikasi.md |
| Templates | `docs/templates/` | ADR-template.md |
| Governance | `docs/governance/` | engineering-rules.md |
| Handbook | `docs/handbook/` | repository-structure.md |
| Metrics | `docs/metrics/` | repository-health.md |

## Validation

```bash
# Validate documentation structure
ls -la docs/

# Check for broken links
bash tools/doc-link-checker/check-links.sh

# Generate documentation index
bash tools/doc-generator/generate-index.sh

# Validate SSOT compliance
bash scripts/validate-ssot.sh
```

## References

- [`../SSOT.md`](../SSOT.md) — Single Source of Truth
- [`../AGENTS.md`](../AGENTS.md) — Engineering constitution
- [`INDEX.md`](INDEX.md) — Master documentation index
- [`repository-map.md`](repository-map.md) — Complete repository map
