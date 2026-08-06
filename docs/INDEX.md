# KCM Documentation Index

> Master entry point for all KCM documentation.
> Last updated: 2026-08-06

## Quick Navigation

| Start Here | Description |
|------------|-------------|
| [**Repository Map**](repository-map.md) | Complete repository structure and navigation |
| [**SSOT**](../SSOT.md) | Single Source of Truth — authority hierarchy |
| [**Engineering Constitution**](../AGENTS.md) | Non-negotiable rules, 16 AI skills |
| [**Technical Specification**](../KCM_SPECIFICATION.md) | Fact structure, API surface, error model |
| [**Roadmap**](../ROADMAP.md) | Release plan and milestones |

---

## Documentation Hierarchy

```
Root Documents (Authority)
├── SSOT.md                     ← P1: Absolute authority
├── AGENTS.md                   ← P2: Engineering constitution
├── KCM_SPECIFICATION.md        ← P3: Technical summary
├── ROADMAP.md                  ← P4: Release plan
└── README.md                   ← Project overview

Specifications (docs/specs/)
├── PRD-TESTING-AND-BENCHMARK   ← P1: Testing strategy
├── PRD3.md                     ← P2: Distributed, ML, security
├── PRD2.md                     ← P3: Storage, runtime, interfaces
├── PRD.md                      ← P4: Core types, compute
└── KCM_*_SPEC.md               ← P5: Component specs (15 files)

Governance (docs/governance/)
├── engineering-rules.md        ← Development rules and Rust conventions
├── architecture-matrix.md      ← Component registry and contracts
├── ssot-certification.md       ← SSOT compliance certification
└── documentation-governance.md ← Documentation standards

Handbook (docs/handbook/)
├── repository-structure.md     ← Complete repository reference
└── handbook.md                 ← Developer onboarding guide

Operations (docs/runbook/)
├── OPERATIONAL_RUNBOOK.md      ← Day-to-day operations
└── DISASTER_RECOVERY.md        ← DR procedures

SDK Documentation (docs/sdk/)
├── rust.md, c.md, cpp.md      ← Language-specific guides
├── python.md, javascript.md
├── typescript.md, go.md
├── java.md, dotnet.md
├── compatibility.md            ← Cross-platform matrix
└── spesifikasi.md              ← SDK technical spec

Crate Specifications (docs/<crate>/)
├── kcm-core/spesifikasi.md
├── kcm-storage/spesifikasi.md
├── kcm-compute/spesifikasi.md
├── kcm-reasoning/spesifikasi.md
├── kcm-optimizer/spesifikasi.md
├── kcm-runtime/spesifikasi.md
├── kcm-interface/spesifikasi.md
├── kcm-distributed/spesifikasi.md
├── kcm-ml/spesifikasi.md
├── kcm-security/spesifikasi.md
├── kcm-compliance/spesifikasi.md
├── kcm-testing/spesifikasi.md
└── kcm-server/spesifikasi.md

Architecture Decision Records (docs/adr/)
├── ADR-001 through ADR-010
└── templates/ADR-template.md

Metrics & Reports (docs/metrics/)
└── repository-health.md        ← Repository health report

Templates (docs/templates/)
├── ADR-template.md
├── README-template.md
├── spesifikasi-template.md
└── ... (8 templates)
```

---

## Specifications

| Document | Scope | Priority |
|----------|-------|----------|
| [PRD-TESTING-AND-BENCHMARK](specs/PRD-TESTING-AND-BENCHMARK.md) | Testing strategy, benchmarks, quality gates | P1 |
| [PRD3](specs/PRD3.md) | Distributed, ML, security, compliance | P2 |
| [PRD2](specs/PRD2.md) | Storage, runtime, interfaces | P3 |
| [PRD](specs/PRD.md) | Core types, storage, compute, reasoning | P4 |
| [KCM_API_SPEC](specs/KCM_API_SPEC.md) | API contracts (FFI, REST, gRPC) | P5 |
| [KCM_COLUMNAR_FORMAT_SPEC](specs/KCM_COLUMNAR_FORMAT_SPEC.md) | Binary file format | P5 |
| [KCM_COMPRESSION_SPEC](specs/KCM_COMPRESSION_SPEC.md) | Compression algorithms | P5 |
| [KCM_DATA_MODEL_SPEC](specs/KCM_DATA_MODEL_SPEC.md) | Data model and types | P5 |
| [KCM_DEPLOYMENT_SPEC](specs/KCM_DEPLOYMENT_SPEC.md) | Deployment architecture | P5 |
| [KCM_GLOSSARY](specs/KCM_GLOSSARY.md) | Terminology and definitions | P5 |
| [KCM_INDEXING_SPEC](specs/KCM_INDEXING_SPEC.md) | Index structures | P5 |
| [KCM_PERFORMANCE_SPEC](specs/KCM_PERFORMANCE_SPEC.md) | Performance targets | P5 |
| [KCM_QUERY_EXECUTION_SPEC](specs/KCM_QUERY_EXECUTION_SPEC.md) | Query execution model | P5 |
| [KCM_RUNTIME_SPEC](specs/KCM_RUNTIME_SPEC.md) | Runtime behavior | P5 |
| [KCM_SECURITY_TRUST_SPEC](specs/KCM_SECURITY_TRUST_SPEC.md) | Security model | P5 |
| [KCM_SPECIFICATION](specs/KCM_SPECIFICATION.md) | Technical constitution | P5 |
| [KCM_TESTING_SPEC](specs/KCM_TESTING_SPEC.md) | Testing strategy | P5 |
| [KCM_VERSIONING_SPEC](specs/KCM_VERSIONING_SPEC.md) | Versioning and compatibility | P5 |

---

## Crate Specifications

| Crate | Specification | README |
|-------|--------------|--------|
| kcm-core | [spesifikasi](kcm-core/spesifikasi.md) | [README](../crates/kcm-core/README.md) |
| kcm-storage | [spesifikasi](kcm-storage/spesifikasi.md) | [README](../crates/kcm-storage/README.md) |
| kcm-compute | [spesifikasi](kcm-compute/spesifikasi.md) | [README](../crates/kcm-compute/README.md) |
| kcm-reasoning | [spesifikasi](kcm-reasoning/spesifikasi.md) | [README](../crates/kcm-reasoning/README.md) |
| kcm-optimizer | [spesifikasi](kcm-optimizer/spesifikasi.md) | [README](../crates/kcm-optimizer/README.md) |
| kcm-runtime | [spesifikasi](kcm-runtime/spesifikasi.md) | [README](../crates/kcm-runtime/README.md) |
| kcm-interface | [spesifikasi](kcm-interface/spesifikasi.md) | [README](../crates/kcm-interface/README.md) |
| kcm-distributed | [spesifikasi](kcm-distributed/spesifikasi.md) | [README](../crates/kcm-distributed/README.md) |
| kcm-ml | [spesifikasi](kcm-ml/spesifikasi.md) | [README](../crates/kcm-ml/README.md) |
| kcm-security | [spesifikasi](kcm-security/spesifikasi.md) | [README](../crates/kcm-security/README.md) |
| kcm-compliance | [spesifikasi](kcm-compliance/spesifikasi.md) | [README](../crates/kcm-compliance/README.md) |
| kcm-testing | [spesifikasi](kcm-testing/spesifikasi.md) | [README](../crates/kcm-testing/README.md) |
| kcm-server | [spesifikasi](kcm-server/spesifikasi.md) | [README](../crates/kcm-server/README.md) |

---

## SDK Documentation

| Language | Guide | Specification |
|----------|-------|---------------|
| [Rust](sdk/rust.md) | Native Rust SDK | [spesifikasi](sdk/spesifikasi.md) |
| [C](sdk/c.md) | FFI bindings | [spesifikasi](sdk/spesifikasi.md) |
| [C++](sdk/cpp.md) | FFI wrapper | [spesifikasi](sdk/spesifikasi.md) |
| [Python](sdk/python.md) | PyO3 bindings | [spesifikasi](sdk/spesifikasi.md) |
| [JavaScript](sdk/javascript.md) | N-API bindings | [spesifikasi](sdk/spesifikasi.md) |
| [TypeScript](sdk/typescript.md) | N-API bindings | [spesifikasi](sdk/spesifikasi.md) |
| [Go](sdk/go.md) | cgo bindings | [spesifikasi](sdk/spesifikasi.md) |
| [Java](sdk/java.md) | JNI bindings | [spesifikasi](sdk/spesifikasi.md) |
| [.NET](sdk/dotnet.md) | P/Invoke bindings | [spesifikasi](sdk/spesifikasi.md) |
| [Compatibility](sdk/compatibility.md) | Cross-platform matrix | — |

---

## Architecture Decision Records

| ADR | Title | Status |
|-----|-------|--------|
| [ADR-001](adr/ADR-001.md) | Architecture Decision Record | Accepted |
| [ADR-002](adr/ADR-002.md) | Architecture Decision Record | Accepted |
| [ADR-003](adr/ADR-003.md) | Architecture Decision Record | Accepted |
| [ADR-004](adr/ADR-004.md) | Architecture Decision Record | Accepted |
| [ADR-005](adr/ADR-005.md) | Architecture Decision Record | Accepted |
| [ADR-006](adr/ADR-006.md) | Architecture Decision Record | Accepted |
| [ADR-007](adr/ADR-007.md) | Architecture Decision Record | Accepted |
| [ADR-008](adr/ADR-008.md) | Architecture Decision Record | Accepted |
| [ADR-009](adr/ADR-009.md) | Architecture Decision Record | Accepted |
| [ADR-010](adr/ADR-010.md) | Architecture Decision Record | Accepted |

---

## Operations

| Document | Purpose |
|----------|---------|
| [Operational Runbook](runbook/OPERATIONAL_RUNBOOK.md) | Day-to-day operations guide |
| [Disaster Recovery](runbook/DISASTER_RECOVERY.md) | DR procedures |

---

## Engineering Skills

| Document | Purpose |
|----------|---------|
| [Skills Overview](../skills/README.md) | 16 AI engineering skills |
| [Authority System](../skills/AUTHORITY-SYSTEM.md) | Skill authority hierarchy |
| [Decision Matrix](../skills/DECISION-MATRIX.md) | Conflict resolution |
| [Workflow](../skills/WORKFLOW.md) | Execution workflow |

---

## Governance

| Document | Purpose |
|----------|---------|
| [Engineering Rules](governance/engineering-rules.md) | Development rules and Rust conventions |
| [Architecture Matrix](governance/architecture-matrix.md) | Component registry and contracts |
| [SSOT Certification](governance/ssot-certification.md) | SSOT compliance certification |
| [Documentation Governance](governance/documentation-governance.md) | Documentation standards |

---

## Templates

| Template | Purpose |
|----------|---------|
| [ADR Template](templates/ADR-template.md) | Architecture Decision Record |
| [README Template](templates/README-template.md) | Directory README |
| [spesifikasi Template](templates/spesifikasi-template.md) | Component specification |
| [Runbook Template](templates/runbook-template.md) | Operational runbook |
| [Benchmark Report Template](templates/benchmark-report-template.md) | Performance report |

---

## Related Resources

| Resource | Location |
|----------|----------|
| CI/CD Pipelines | [`.github/workflows/`](../.github/workflows/) |
| Engineering Pipelines | [`.engineering/pipelines/`](../.engineering/pipelines/) |
| Documentation Tools | [`tools/`](../tools/) |
| Examples | [`examples/`](../examples/) |
| Benchmarks | [`benchmark-results/`](../benchmark-results/) |
| Deployment | [`deployment/`](../deployment/) |

---

> This index is the single entry point for all KCM documentation.
> For the complete repository structure, see [`repository-map.md`](repository-map.md).
