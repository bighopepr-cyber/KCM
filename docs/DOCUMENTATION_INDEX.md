# KCM Documentation Index

**Document ID:** DOC-INDEX-001
**Version:** 1.0.0
**Status:** Active
**Last Updated:** 2026-08-04

## Purpose

Master index of all KCM documentation. Every document in the repository must be listed here.

## Document Hierarchy

| Priority | Document | ID | Authority |
|----------|----------|-----|-----------|
| P1 | PRD-TESTING& BRACHMARCK.md | KCM-TEST-001 | Testing strategy, benchmarks |
| P2 | PRD3.md | KCM-ADVANCED-001 | Distributed, ML, security, compliance |
| P3 | PRD2.md | KCM-STORAGE-001 | Storage, runtime, interfaces |
| P4 | PRD.md | KCM-ARCH-001 | Core types, storage, compute, reasoning |
| P5 | AGENTS.md | — | Engineering constitution |

## Technical Specifications

| Document | ID | Scope | Status |
|----------|-----|-------|--------|
| KCM_SPECIFICATION.md | KCM-SPEC-001 | Technical constitution | Active |
| KCM_ARCHITECTURE.md | KCM-ARCHDETAIL-001 | System architecture | Derived |
| KCM_DATA_MODEL_SPEC.md | KCM-DATA-001 | Knowledge model, types | Derived |
| KCM_COLUMNAR_FORMAT_SPEC.md | KCM-FORMAT-001 | Binary format, WAL | Derived |
| KCM_QUERY_EXECUTION_SPEC.md | KCM-QUERY-001 | Query pipeline, KQL | Derived |
| KCM_COMPRESSION_SPEC.md | KCM-COMP-001 | Encodings, codecs | Derived |
| KCM_INDEXING_SPEC.md | KCM-INDEX-001 | Bitmap, zone map, bloom filter | Derived |
| KCM_SECURITY_TRUST_SPEC.md | KCM-SEC-001 | RBAC, encryption, GDPR | Derived |
| KCM_API_SPEC.md | KCM-API-001 | C FFI, REST, gRPC contracts | Derived |
| KCM_RUNTIME_SPEC.md | KCM-RUNTIME-001 | Concurrency, metrics, health | Derived |
| KCM_PERFORMANCE_SPEC.md | KCM-PERF-001 | Benchmark targets | Derived |
| KCM_TESTING_SPEC.md | KCM-TESTSPEC-001 | Test standards | Derived |
| KCM_ENGINEERING_RULES.md | KCM-ENG-001 | Development rules | Derived |
| KCM_VERSIONING_SPEC.md | KCM-VER-001 | Versioning, compatibility | Derived |
| KCM_DEPLOYMENT_SPEC.md | KCM-DEPLOY-001 | Docker, Kubernetes | Derived |
| KCM_BENCHMARK_REPORTING_SPEC.md | KCM-BENCH-001 | Benchmark artifacts | Derived |
| KCM_GLOSSARY.md | KCM-GLOSS-001 | Terminology | Active |

## Architecture Decision Records

| ADR | Title | Status |
|-----|-------|--------|
| ADR-001 | Columnar Storage Architecture | Accepted |
| ADR-002 | Volcano-Style Query Execution | Accepted |
| ADR-003 | Forward-Chaining Inference | Accepted |
| ADR-004 | WAL-Based Crash Recovery | Accepted |
| ADR-005 | AES-256-GCM Encryption | Accepted |
| ADR-006 | BLAKE3 for Hashing | Accepted |
| ADR-007 | parking_lot for Synchronization | Accepted |
| ADR-008 | Criterion for Benchmarking | Accepted |
| ADR-009 | Documentation First Development | Accepted |
| ADR-010 | 13-Crate Workspace Architecture | Accepted |

## Guides

| Document | ID | Scope |
|----------|-----|-------|
| backup-recovery.md | GUIDE-BACKUP-001 | Backup and recovery procedures |
| monitoring.md | GUIDE-MONITOR-001 | Monitoring and alerting |
| operations.md | GUIDE-OPS-001 | Operational procedures |
| security-hardening.md | GUIDE-SECURITY-001 | Security hardening |

## Handbooks

| Document | ID | Scope |
|----------|-----|-------|
| contributor.md | HANDBOOK-CONTRIB-001 | Contributor guide |
| enterprise.md | HANDBOOK-ENTERPRISE-001 | Enterprise deployment |
| maintainer.md | HANDBOOK-MAINTAINER-001 | Maintainer guide |

## Tutorials

| Document | ID | Scope |
|----------|-----|-------|
| 01-installation.md | TUTORIAL-INSTALL-001 | Installation guide |
| 02-first-database.md | TUTORIAL-FIRSTDB-001 | Creating your first database |
| 03-basic-queries.md | TUTORIAL-QUERIES-001 | Basic query operations |
| 04-transactions.md | TUTORIAL-TXN-001 | Transaction management |
| 05-reasoning.md | TUTORIAL-REASON-001 | Reasoning engine usage |

## Cookbook

| Document | ID | Scope |
|----------|-----|-------|
| docker-compose.md | COOKBOOK-DOCKER-001 | Docker Compose deployment |
| kubernetes.md | COOKBOOK-K8S-001 | Kubernetes deployment |

## Crate Documentation

| Crate | README | Spec Reference |
|-------|--------|---------------|
| kcm-core | crates/kcm-core/README.md | PRD.md §3-4 |
| kcm-storage | crates/kcm-storage/README.md | PRD2.md §2-5 |
| kcm-compute | crates/kcm-compute/README.md | PRD.md §5 |
| kcm-reasoning | crates/kcm-reasoning/README.md | PRD.md §6 |
| kcm-optimizer | crates/kcm-optimizer/README.md | PRD2.md §16 |
| kcm-runtime | crates/kcm-runtime/README.md | PRD2.md §18 |
| kcm-interface | crates/kcm-interface/README.md | PRD2.md §19 |
| kcm-distributed | crates/kcm-distributed/README.md | PRD3.md §27 |
| kcm-ml | crates/kcm-ml/README.md | PRD3.md §29 |
| kcm-security | crates/kcm-security/README.md | PRD3.md §30 |
| kcm-compliance | crates/kcm-compliance/README.md | PRD3.md §32 |
| kcm-testing | crates/kcm-testing/README.md | PRD-TESTING§1-8 |
| kcm-server | crates/kcm-server/README.md | PRD2.md §19 |

## Tools

| Tool | Status | README |
|------|--------|--------|
| kcm-cli | Active | tools/kcm-cli/README.md |
| kcm-bench | Active | tools/kcm-bench/README.md |
| kcm-perf | Active | tools/kcm-perf/README.md |
| kcm-profile | Active | tools/kcm-profile/README.md |
| kcm-doctor | Active | tools/kcm-doctor/README.md |
| kcm-diagnose | Active | tools/kcm-diagnose/README.md |
| kcm-inspect | Active | tools/kcm-inspect/README.md |
| kcm-backup | Active | tools/kcm-backup/README.md |
| kcm-restore | Active | tools/kcm-restore/README.md |
| kcm-snapshot | Active | tools/kcm-snapshot/README.md |
| kcm-compact | Active | tools/kcm-compact/README.md |
| kcm-migrate | Active | tools/kcm-migrate/README.md |
| kcm-import | Active | tools/kcm-import/README.md |
| kcm-export | Active | tools/kcm-export/README.md |
| kcm-schema | Active | tools/kcm-schema/README.md |
| kcm-cluster | Active | tools/kcm-cluster/README.md |
| kcm-docs | Active | tools/kcm-docs/README.md |

## SDKs

| Language | Status | Path |
|----------|--------|------|
| Rust | Active | sdk/rust/ |
| C | Active | sdk/c/ |
| C++ | Active | sdk/cpp/ |
| Python | Active | sdk/python/ |
| Java | Active | sdk/java/ |
| JavaScript | Active | sdk/javascript/ |
| TypeScript | Active | sdk/typescript/ |
| Go | Active | sdk/go/ |
| .NET | Active | sdk/dotnet/ |

## Integrations

| Integration | Status | README |
|-------------|--------|--------|
| REST API | Active | integrations/rest/README.md |
| gRPC | Active | integrations/grpc/README.md |
| Apache Arrow | Active | integrations/arrow/README.md |
| Arrow Flight | Active | integrations/arrow-flight/README.md |
| Apache Parquet | Active | integrations/parquet/README.md |
| Apache Kafka | Active | integrations/kafka/README.md |
| Apache Iceberg | Active | integrations/iceberg/README.md |
| Apache Delta Lake | Active | integrations/delta/README.md |
| Apache DataFusion | Active | integrations/datafusion/README.md |
| DuckDB | Active | integrations/duckdb/README.md |
| Polars | Active | integrations/polars/README.md |
| Pandas | Active | integrations/pandas/README.md |
| NATS | Active | integrations/nats/README.md |
| MQTT | Active | integrations/mqtt/README.md |
| MCP | Active | integrations/mcp/README.md |

## Reports

| Document | Scope |
|----------|-------|
| KCM_DOCUMENT_AUDIT_REPORT.md | Documentation audit (v1) |
| DOCUMENTATION_CONSISTENCY_REPORT.md | Consistency audit (v1) |
| DOCUMENTATION_CONSISTENCY_REPORT_V2.md | Post-remediation audit (v2) |
| KCM_STABILITY_READINESS_REPORT.md | Stability readiness assessment |
| KCM_PERFORMANCE_ENGINEERING_REPORT.md | Performance engineering report |
| PROJECT_COMPLIANCE_REPORT.md | Project compliance report |
| DESIGN_REVIEW_REPORT.md | Design review report |
| CODEBASE_AUDIT_REPORT.md | Codebase audit report |

## Repository Specifications

| Document | Scope |
|----------|-------|
| specs/repository/CRATE_OWNERSHIP.md | Crate ownership assignments |
| specs/repository/RELEASE_POLICY.md | Release policy |
| specs/repository/VERSIONING_POLICY.md | Versioning policy |
| specs/repository/DEPENDENCY_POLICY.md | Dependency policy |
| specs/repository/DOCUMENTATION_STRUCTURE.md | Documentation structure |
| specs/repository/REPOSITORY_EVOLUTION.md | Repository evolution |
| specs/repository/REPOSITORY_GOVERNANCE.md | Repository governance |
| specs/repository/FOLDER_CONVENTION.md | Folder conventions |
| specs/repository/WORKSPACE_LAYOUT.md | Workspace layout |
| specs/repository/REPOSITORY_ARCHITECTURE.md | Repository architecture |
| specs/repository/NAMING_CONVENTION.md | Naming conventions |

## Ecosystem Specifications

| Document | Scope |
|----------|-------|
| specs/ecosystem/DEVELOPER_ECOSYSTEM.md | Developer ecosystem |
| specs/ecosystem/DEPLOYMENT_STRATEGY.md | Deployment strategy |
| specs/ecosystem/CLOUD_STRATEGY.md | Cloud strategy |
| specs/ecosystem/OBSERVABILITY.md | Observability |
| specs/ecosystem/COMMUNITY_ROADMAP.md | Community roadmap |
| specs/ecosystem/PLUGIN_SYSTEM.md | Plugin system |
| specs/ecosystem/SDK_ROADMAP.md | SDK roadmap |
| specs/ecosystem/LONG_TERM_VISION.md | Long-term vision |
| specs/ecosystem/EXTENSION_SYSTEM.md | Extension system |
| specs/ecosystem/CLI_ROADMAP.md | CLI roadmap |
| specs/ecosystem/ENTERPRISE_ECOSYSTEM.md | Enterprise ecosystem |
| specs/ecosystem/INTEGRATION_ROADMAP.md | Integration roadmap |

## Other Documentation

| Document | Scope |
|----------|-------|
| CHANGELOG.md | Version changelog |
| CONTRIBUTING.md | Contribution guidelines |
| CODE_OF_CONDUCT.md | Code of conduct |
| SECURITY.md | Security policy |
| LICENSE | License |
| PLATFORM_ROADMAP.md | Platform roadmap |
| CICD_QUALITY_GATES.md | CI/CD quality gates |
| ARCHITECTURE_CONSISTENCY_MATRIX.md | Architecture consistency |
| DESIGN_SYSTEM.md | Design system |
| DESIGN_SYSTEM_SPEC.md | Design system specification (deprecated) |
| analisis-benchmark.md | Benchmark analysis |
| adr/README.md | ADR index |

## Deprecated/Redirect Documents

| Document | Redirect |
|----------|----------|
| DESIGN_SYSTEM_SPEC.md | → DESIGN_SYSTEM.md |
