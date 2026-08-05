# KCM

**Document ID:** REPO-README-001
**Version:** 2.0.0
**Status:** Active
**Owner:** Engineering Orchestrator (P1)

KCM is a Rust-native columnar knowledge engine with persistent storage, query execution, and inference-capable runtime behavior.

## Repository Role

This repository uses a single-source-of-truth documentation model:

- [docs/PRD.md](docs/PRD.md) — authoritative core architecture and domain model
- [docs/PRD2.md](docs/PRD2.md) — authoritative storage, runtime, and interface contract
- [docs/PRD3.md](docs/PRD3.md) — authoritative distributed, security, compliance, and ML contract
- [docs/PRD-TESTING& BRACHMARCK.md](docs/PRD-TESTING& BRACHMARCK.md) — authoritative testing and benchmark contract
- [docs/DOCUMENTATION_INDEX.md](docs/DOCUMENTATION_INDEX.md) — repository navigation and canonical role map
- [docs/DOCUMENT_DEPENDENCY_MAP.md](docs/DOCUMENT_DEPENDENCY_MAP.md) — dependency graph between spec documents
- [docs/DOCUMENT_OWNERSHIP_MATRIX.md](docs/DOCUMENT_OWNERSHIP_MATRIX.md) — topic ownership and review boundaries

## Implementation Surface

The active implementation surface is defined by the crate workspace and the concrete server entrypoints:

- `kcm-core` — core types and memory structures
- `kcm-storage` — storage formats, WAL, codecs, recovery
- `kcm-compute` — query operators and execution
- `kcm-reasoning` — inference and rule execution
- `kcm-runtime` — database lifecycle and transactions
- `kcm-interface` — FFI, REST, gRPC, and parsing surfaces
- `kcm-server` — HTTP and gRPC server process

The executable server currently exposes the implemented routes under the active runtime surface, including `/health`, `/metrics`, `/openapi.json`, `/facts`, `/facts/{id}`, `/stats`, `/api/v1/facts`, `/api/v1/facts/batch`, `/api/v1/facts/{id}`, and `/api/v1/stats`.

## Documentation Responsibility

The repository is organized around one topic = one authoritative document:

- PRD documents define the contract
- KCM_*_SPEC documents provide derived implementation detail
- guides / tutorials / cookbook / handbook provide operational usage only
- audit, report, and retrospective documents remain historical and non-normative

## Verification Entry Points

Use the active repository contract for build and validation:

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
bash scripts/validate-ssot.sh
```

---

## CI/CD Pipeline

| Job | Trigger | What it validates |
|-----|---------|-------------------|
| Format Check | Every push | `cargo fmt --all -- --check` |
| Build | Every push | `cargo build --workspace` |
| Clippy | Every push | `cargo clippy --workspace -- -D warnings` |
| Unit Tests | Every push | `cargo test --lib --all` |
| Integration Tests | Every push | `cargo test --test '*' --all` |
| Security Tests | After unit tests | `cargo test security_tests --all` |
| Property Tests | Every push | `cargo test property_tests --all` |
| Load Tests | After unit tests | `cargo test load_tests --all` |
| Stress Tests | After unit tests | `cargo test stress_tests --all` |
| Recovery Tests | After unit tests | `cargo test recovery --all` |
| Benchmarks | After unit tests | `cargo bench --workspace --no-run` |
| Quality Gate | All above pass | Final merge decision |

---

## Engineering Governance

KCM uses a 16-skill engineering system enforced by AI agents:

| Priority | Skill | Role |
|----------|-------|------|
| P1 | Engineering Orchestrator | Master coordinator |
| P2 | Task Planner | Implementation planning |
| P3 | Change Impact Analysis | Pre-change assessment |
| P4 | Specification Lock | Frozen contract protection |
| P5 | Architecture Guardian | Architecture integrity |
| P6 | Database Engine Specialist | Storage/query correctness |
| P7 | Security Engineer | Security and compliance |
| P8 | Performance Engineer | Performance validation |
| P9 | Testing Verification | Test coverage |
| P10 | Code Quality Guardian | Rust code quality |
| P11 | Documentation Guardian | Spec consistency |
| P12 | Release Readiness | Production validation |
| P13 | Code Review Auditor | Senior review |
| P14 | Debugging Root Cause | Bug investigation |
| P15 | Engineering Decision Record | Decision documentation |
| P16 | Repository Intelligence | Codebase understanding |

**Execution priority:** Correctness → Specification → Data Integrity → Security → Reliability → Performance → Maintainability → Speed

---

## License

MIT
