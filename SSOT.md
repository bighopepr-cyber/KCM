# KCM Single Source of Truth (SSOT)

**Document ID:** KCM-SSOT-001  
**Version:** 2.0.0  
**Status:** Active  
**Owner:** Specification Lock (P4)  
**Last Updated:** 2026-08-06

---

## 1. Authority

This document is the absolute reference for the KCM project. All implementation, documentation, and tooling must conform to this document. When conflicts arise between any source and this document, **this document wins**.

## 2. Repository Structure (SSOT v2.0)

```
KCM/
├── crates/                          # 13 core crates
│   ├── kcm-core/                    # Types, DenseVec, Bitmap, Dictionary
│   ├── kcm-storage/                 # Columns, Codecs, WAL, FileFormat, Index
│   ├── kcm-compute/                 # Relational algebra operators, SIMD AVX2
│   ├── kcm-reasoning/               # Rule definitions, inference engine
│   ├── kcm-optimizer/               # Cost model, query planner, statistics
│   ├── kcm-runtime/                 # KnowledgeDatabase, Transactions, Metrics
│   ├── kcm-interface/               # C FFI, Python (PyO3), REST, KQL parser
│   ├── kcm-distributed/             # Sharding (Hash/Range/ConsistentHash), 2PC
│   ├── kcm-ml/                      # Learned index, confidence learner
│   ├── kcm-security/                # RBAC, AES-256-GCM, audit log
│   ├── kcm-compliance/              # GDPR, data classification
│   ├── kcm-testing/                 # Load, stress, security, recovery tests
│   └── kcm-server/                  # HTTP (actix-web) + gRPC (tonic)
├── scripts/                         # Build, test, release, CLI tools
│   ├── kcm-cli/                     # All CLI tool crates
│   ├── validate-ssot.sh             # SSOT validation script
│   └── bench-regression.py          # Benchmark regression detection
├── docs/                            # Documentation (SSOT v2.0 — 3 subfolders only)
│   ├── adr/                         # Architecture Decision Records (max 10)
│   ├── specs/                       # PRD.md, PRD2.md, PRD3.md, PRD-TESTING, KCM_SPECIFICATION
│   └── handbook/                    # handbook.md (single consolidated guide)
├── deployment/                      # Docker, K8s, Helm, Terraform, Grafana, Prometheus
├── tests/                           # Integration & security tests
├── sdk/                             # SDK: C, C++, Python, Rust, JS, TS, Go, Java, .NET
├── assets/                          # Logo, icon, banner
├── benchmark-results/               # Benchmark baselines and reports
├── skills/                          # 16 AI engineering skills (AGENTS.md)
├── .github/workflows/               # CI/CD pipelines
├── .gitignore
├── .dockerignore
├── README.md
├── KCM_SPECIFICATION.md             # Root specification summary
├── ROADMAP.md                       # Release plan and targets
├── ARCHITECTURE_CONSISTENCY_MATRIX.md # Component registry and contracts
├── SSOT_CERTIFICATION_REPORT.md     # SSOT compliance certification
├── KCM_ENGINEERING_RULES.md         # Engineering rules summary
├── AGENTS.md                        # AI engineering governance
├── LICENSE                          # MIT
├── CHANGELOG.md
├── Cargo.toml                       # Workspace manifest
├── Cargo.lock
└── rust-toolchain.toml
```

## 3. Document Hierarchy

| Priority | Document | Authority |
|----------|----------|-----------|
| P1 | `docs/specs/PRD-TESTING-AND-BENCHMARK.md` | Performance targets, validation, testing strategy |
| P2 | `docs/specs/PRD3.md` | Distributed, ML, security, compliance |
| P3 | `docs/specs/PRD2.md` | Storage, runtime, interfaces |
| P4 | `docs/specs/PRD.md` | Core types, storage, compute, reasoning |
| P5 | `AGENTS.md` | Engineering constitution, non-negotiable rules |

## 4. Crate Dependency Flow

```
kcm-core (zero deps)
  ↑
kcm-storage (core + log + zstd + lz4 + blake3 + thiserror)
  ↑
kcm-compute (core + storage)
kcm-reasoning (core + storage)
kcm-optimizer (core + storage)
  ↑
kcm-runtime (core + storage + parking_lot + rayon + tokio)
  ↑
kcm-interface (core + storage + runtime + parking_lot + serde + serde_json)
  ↑
kcm-server (core + runtime + interface + actix-web + tonic + prost + tokio)

kcm-distributed (core + parking_lot)
kcm-ml (core + reasoning)
kcm-security (core + parking_lot + blake3 + aes-gcm + getrandom)
kcm-compliance (core + parking_lot)
kcm-testing (core + storage + runtime + reasoning + security + distributed + compliance)
```

## 5. Non-Negotiable Rules

1. All public APIs return `Result<T, KcmError>`
2. No `unwrap()` in production code paths
3. No `panic!()` in production code
4. No TODO/FIXME/HACK in production code
5. No placeholder implementations
6. No fake success responses
7. All tests must pass before commit
8. All clippy warnings resolved
9. Every requirement maps to an implementation
10. Every implementation maps to a test
11. Every benchmark validates a documented requirement
12. No documentation describes behavior that does not exist

## 6. Quality Gates

| Gate | Command | Blocks Merge |
|------|---------|-------------|
| Format | `cargo fmt --all -- --check` | Yes |
| Clippy | `cargo clippy --workspace -- -D warnings` | Yes |
| Build | `cargo build --workspace` | Yes |
| Unit Tests | `cargo test --lib --all` | Yes |
| Integration Tests | `cargo test --test '*' --all` | Yes |
| Property Tests | `cargo test property_tests --all` | Yes |
| Security Tests | `cargo test security_tests --all` | Yes |
| Benchmarks | `cargo bench --workspace --no-run` | Yes |
| SSOT Validation | `bash scripts/validate-ssot.sh` | Yes |

## 7. Version Bumping Rules

| Change Type | Version Bump | Example |
|-------------|-------------|---------|
| Bug fix | Patch (0.0.x) | WAL replay fix |
| New feature | Minor (0.x.0) | New codec, new index |
| Breaking API change | Major (x.0.0) | Remove FFI function |
| Format change | Major (x.0.0) | Header layout change |

## 8. SSOT Compliance

SSOT compliance is validated by `scripts/validate-ssot.sh`. The script verifies:
- FFI function count = 18
- Metrics counter count = 14
- Test count >= 550
- REST endpoint count >= 8
- gRPC RPC count = 4
- No TODO/FIXME in production code
- No unwrap in production code (target: 0)
- Spec documents have Document ID and Status
- No phantom document references
- Workspace compiles
- No stale counts in documentation
