# KCM Repository Map

> Complete repository structure with relationships, dependencies, and navigation.

## Repository Overview

**KCM (Knowledge Columnar Model)** is a self-contained columnar knowledge representation, storage, query, and reasoning engine implemented in Rust.

- **Language:** Rust (edition 2021, stable toolchain)
- **License:** MIT
- **Repository:** https://github.com/bighopepr-cyber/KCM
- **Architecture:** Monorepo with Cargo workspace (13 crates)

---

## Root-Level Documents

| Document | Purpose | Canonical |
|----------|---------|-----------|
| [`README.md`](README.md) | Project overview, quick start, architecture | Yes |
| [`SSOT.md`](SSOT.md) | Single Source of Truth — authority hierarchy | Yes |
| [`AGENTS.md`](AGENTS.md) | Engineering constitution — 16 AI skills, non-negotiable rules | Yes |
| [`KCM_SPECIFICATION.md`](KCM_SPECIFICATION.md) | Technical summary — fact structure, API surface, error model | Yes |
| [`ROADMAP.md`](ROADMAP.md) | Release plan — timeline, SDK roadmap, LTS policy | Yes |
| [`CHANGELOG.md`](CHANGELOG.md) | Version history — Keep-a-Changelog format | Yes |
| [`LICENSE`](LICENSE) | MIT License | Yes |
| [`VERSION`](VERSION) | Canonical version source (SemVer 2.0.0) | Yes |
| [`Cargo.toml`](Cargo.toml) | Workspace manifest — 13 crates | Yes |
| [`SECURITY.md`](SECURITY.md) | Security policy — vulnerability reporting, best practices | Yes |
| [`CONTRIBUTING.md`](CONTRIBUTING.md) | Contribution guidelines — development setup, PR process | Yes |
| [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md) | Community guidelines — Microsoft Open Source CoC | Yes |

---

## Directory Structure

```
KCM/
├── crates/                    # 13 core Rust crates
├── sdk/                       # 9 language SDKs
├── scripts/                   # Build, test, release, CLI tools
├── docs/                      # Documentation hub
├── deployment/                # Docker, K8s, Helm, monitoring
├── tests/                     # Integration & security tests
├── examples/                  # Code examples
├── skills/                    # 16 AI engineering skills
├── benchmark-results/         # Performance baselines and reports
├── tools/                     # Documentation tooling
├── assets/                    # Logo, icon, branding
├── .github/                   # CI/CD, issue templates, CODEOWNERS
├── .engineering/              # Engineering pipelines and orchestrator
└── .agents/                   # AI skill mirror for agent consumption
```

---

## Core Crates (`crates/`)

| Crate | Responsibility | Stability | Dependencies |
|-------|---------------|-----------|--------------|
| [`kcm-core`](crates/kcm-core/) | Types, DenseVec, Bitmap, Dictionary | Stable | parking_lot |
| [`kcm-storage`](crates/kcm-storage/) | Columns, Codecs, WAL, FileFormat, Index | Stable | kcm-core, zstd, lz4, blake3 |
| [`kcm-compute`](crates/kcm-compute/) | Query operators, SIMD AVX2 | Stable | kcm-core, kcm-storage |
| [`kcm-reasoning`](crates/kcm-reasoning/) | Rule definitions, inference engine | Stable | kcm-core, kcm-storage |
| [`kcm-optimizer`](crates/kcm-optimizer/) | Cost model, planner, statistics | Beta | kcm-core, kcm-storage |
| [`kcm-runtime`](crates/kcm-runtime/) | KnowledgeDatabase, Transactions, Metrics | Stable | kcm-core, kcm-storage, rayon, tokio |
| [`kcm-interface`](crates/kcm-interface/) | C FFI, Python, REST, KQL parser | Stable | kcm-core, kcm-storage, kcm-runtime |
| [`kcm-distributed`](crates/kcm-distributed/) | Sharding, 2PC coordinator | Beta | kcm-core, parking_lot |
| [`kcm-ml`](crates/kcm-ml/) | Learned index, confidence learner | Experimental | kcm-core, kcm-reasoning |
| [`kcm-security`](crates/kcm-security/) | RBAC, AES-256-GCM, audit log | Stable | kcm-core, blake3, aes-gcm |
| [`kcm-compliance`](crates/kcm-compliance/) | GDPR, data classification | Beta | kcm-core, parking_lot |
| [`kcm-testing`](crates/kcm-testing/) | Load, stress, security, recovery tests | Internal | kcm-core, kcm-storage, kcm-runtime |
| [`kcm-server`](crates/kcm-server/) | HTTP (actix-web) + gRPC (tonic) | Stable | kcm-core, kcm-runtime, kcm-interface |

**Dependency Flow:**
```
kcm-core (zero internal deps)
  ↑
kcm-storage
  ↑
kcm-compute, kcm-reasoning, kcm-optimizer
  ↑
kcm-runtime
  ↑
kcm-interface
  ↑
kcm-server
```

---

## SDK Language Bindings (`sdk/`)

| Language | Package | Status | Documentation |
|----------|---------|--------|---------------|
| [Rust](sdk/rust/) | kcm-sdk | Stable | [docs](docs/sdk/rust.md) |
| [C](sdk/c/) | libkcm | Stable | [docs](docs/sdk/c.md) |
| [C++](sdk/cpp/) | libkcm-cpp | Stable | [docs](docs/sdk/cpp.md) |
| [Python](sdk/python/) | kcm | Planned | [docs](docs/sdk/python.md) |
| [JavaScript](sdk/javascript/) | @kcm/js | Planned | [docs](docs/sdk/javascript.md) |
| [TypeScript](sdk/typescript/) | @kcm/ts | Planned | [docs](docs/sdk/typescript.md) |
| [Go](sdk/go/) | go-sdk | Planned | [docs](docs/sdk/go.md) |
| [Java](sdk/java/) | io.kcm:sdk | Planned | [docs](docs/sdk/java.md) |
| [.NET](sdk/dotnet/) | Kcm.Sdk | Planned | [docs](docs/sdk/dotnet.md) |

---

## Documentation (`docs/`)

| Folder | Contents | Purpose |
|--------|----------|---------|
| [`specs/`](docs/specs/) | PRD.md, PRD2.md, PRD3.md, KCM_*_SPEC.md | SSOT specifications |
| [`adr/`](docs/adr/) | ADR-001 through ADR-010 | Architecture Decision Records |
| [`handbook/`](docs/handbook/) | repository-structure.md, handbook.md | Developer guides |
| [`governance/`](docs/governance/) | engineering-rules, architecture-matrix, certification | Governance documents |
| [`runbook/`](docs/runbook/) | OPERATIONAL_RUNBOOK, DISASTER_RECOVERY | Operational procedures |
| [`sdk/`](docs/sdk/) | Per-language SDK documentation | SDK usage guides |
| [`metrics/`](docs/metrics/) | repository-health, coverage reports | Metrics and reports |
| [`templates/`](docs/templates/) | ADR, README, spesifikasi templates | Documentation templates |
| [`<crate>/`](docs/) | Per-crate spesifikasi.md files | Component specifications |

---

## Deployment (`deployment/`)

| Component | Files | Purpose |
|-----------|-------|---------|
| Docker | `Dockerfile` | Multi-stage build |
| Kubernetes | `k8s/deployment.yaml` | StatefulSet + Service + NetworkPolicy |
| Helm | `helm/kcm/` | Chart with templates |
| Monitoring | `prometheus/`, `grafana/` | Observability stack |

---

## CI/CD (`.github/workflows/`)

| Workflow | Trigger | Purpose |
|----------|---------|---------|
| `ci.yml` | push/PR to main | Format, lint, build, test |
| `ci-full.yml` | push to main | Full test suite |
| `benchmark.yml` | weekly | Performance regression |
| `version.yml` | push/PR | Version consistency validation |
| `sdk-ci.yml` | push/PR | SDK validation |
| `docs.yml` | push/PR | Documentation validation |

---

## Engineering Skills (`skills/`)

| Priority | Skill | Authority |
|----------|-------|-----------|
| P1 | kcm-engineering-orchestrator | Master coordinator |
| P2 | kcm-task-planner | Can block implementation |
| P3 | kcm-change-impact-analysis | Can block changes |
| P4 | kcm-specification-lock | Can VETO contract changes |
| P5 | kcm-architecture-guardian | Can block architecture violations |
| P6 | kcm-database-engine-specialist | Can block storage changes |
| P7 | kcm-security-engineer | Can block security violations |
| P8 | kcm-performance-engineer | Can block perf regressions |
| P9 | kcm-testing-verification | Can block untested changes |
| P10 | kcm-code-quality-guardian | Can reject code quality |
| P11 | kcm-documentation-guardian | Can block undocumented changes |
| P12 | kcm-release-readiness | Can block releases |
| P13 | kcm-code-review-auditor | Advisory review |
| P14 | kcm-debugging-root-cause | Diagnostic analysis |
| P15 | kcm-engineering-decision-record | Decision documentation |
| P16 | kcm-repository-intelligence | Codebase understanding |

---

## Version Governance

- **Canonical Source:** `VERSION` file at repository root
- **Current Version:** 1.0.0
- **Scheme:** Semantic Versioning 2.0.0
- **Sync:** `bash scripts/release/sync-version.sh`
- **Verify:** `bash scripts/release/verify-version.sh`

---

## Quick Navigation

| I want to... | Go to |
|---|---|
| Understand the project | [`README.md`](README.md) |
| Read the specification | [`SSOT.md`](SSOT.md) |
| Learn engineering rules | [`AGENTS.md`](AGENTS.md) |
| See the roadmap | [`ROADMAP.md`](ROADMAP.md) |
| Find a specification | [`docs/specs/`](docs/specs/) |
| Read an ADR | [`docs/adr/`](docs/adr/) |
| Use an SDK | [`sdk/<language>/`](sdk/) |
| Deploy KCM | [`deployment/`](deployment/) |
| Run tests | [`tests/`](tests/) |
| See examples | [`examples/`](examples/) |
| Contribute | [`CONTRIBUTING.md`](CONTRIBUTING.md) |
| Report security | [`SECURITY.md`](SECURITY.md) |
| Check version | [`VERSION`](VERSION) |
