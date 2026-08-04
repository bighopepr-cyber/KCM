# Changelog

All notable changes to the KCM project will be documented in this file.

## [Unreleased]

### Added
- Full repository reorganization specification (Phase 1-13)
- Documentation First Development (DFD) methodology
- SDK ecosystem design (9 languages)
- Tool ecosystem design (17 CLI tools)
- Enterprise deployment design (Docker, K8s, Helm, Terraform)
- Integration ecosystem design (15 integrations)
- LICENSE file (MIT)
- CHANGELOG.md
- rust-toolchain.toml
- Per-crate README.md files
- CODEOWNERS file
- PR and issue templates

### Fixed
- WAL entry size contradiction in KCM_COLUMNAR_FORMAT_SPEC.md (34->38 bytes)
- Data classification retention periods in KCM_SECURITY_TRUST_SPEC.md
- C FFI function count updated to 18 (implemented vs 15 documented)
- README.md stale metrics

### Changed
- Centralized dependency management via [workspace.dependencies]
- Docker healthcheck now uses wget instead of curl
- .dockerignore now includes Cargo.lock for reproducible builds
- K8s deployment uses StatefulSet with volumeClaimTemplates

## [0.1.0] - 2026-08-03

### Added
- Initial release of KCM engine
- 13 Rust crates (core, storage, compute, reasoning, optimizer, runtime, interface, distributed, ml, security, compliance, testing, server)
- C FFI interface (18 functions)
- Python bindings (PyO3)
- REST API (8 endpoints)
- gRPC service (4 RPCs: InsertFact, QueryFacts, GetFact, GetStats)
- KQL parser (28 token types)
- 534+ tests across unit, integration, property, and security tiers
- 32 benchmark functions
- CI/CD pipeline (GitHub Actions)
- Docker and Kubernetes deployment configs
