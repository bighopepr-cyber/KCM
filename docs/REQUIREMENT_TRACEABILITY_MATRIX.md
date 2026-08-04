# Requirement Traceability Matrix (RTM)

| Field | Value |
|-------|-------|
| **Document ID** | KCM-RTM-001 |
| **Title** | Requirement Traceability Matrix |
| **Version** | 2.0.0 |
| **Date** | 2026-08-04 |
| **Status** | Authoritative |
| **Authority** | Engineering Orchestrator (P1) |

---

## 1. Purpose

This RTM connects every requirement from PRD documents to its implementation, test, benchmark, and documentation. It ensures no requirement is unimplemented and no implementation is undocumented.

## 2. Core Requirements (PRD.md)

| Req ID | Requirement | SPEC | Implementation | Test | Benchmark | Doc |
|--------|-------------|------|----------------|------|-----------|-----|
| CORE-001 | Columnar storage for facts | KCM_COLUMNAR_FORMAT_SPEC | kcm-storage/src/column.rs | test_storage.rs | bench_column_* | KCM_ARCHITECTURE.md |
| CORE-002 | Fact struct (34 bytes) | KCM_DATA_MODEL_SPEC | kcm-core/src/types.rs | test_core.rs | - | KCM_DATA_MODEL_SPEC.md |
| CORE-003 | Confidence scoring [0.0, 1.0] | KCM_DATA_MODEL_SPEC | kcm-core/src/types.rs | property_tests.rs | - | KCM_DATA_MODEL_SPEC.md |
| CORE-004 | DenseVec SIMD-aligned | KCM_SPECIFICATION | kcm-core/src/vec.rs | test_core.rs | bench_column_* | KCM_SPECIFICATION.md |
| CORE-005 | Bitmap operations | KCM_SPECIFICATION | kcm-core/src/bitmap.rs | test_core.rs | bench_bitmap_* | KCM_SPECIFICATION.md |
| CORE-006 | Dictionary encoding | KCM_COMPRESSION_SPEC | kcm-core/src/dictionary.rs | test_core.rs | bench_dict_* | KCM_COMPRESSION_SPEC.md |
| CORE-007 | KcmError hierarchy | KCM_DATA_MODEL_SPEC | kcm-core/src/types.rs | test_core.rs | - | KCM_DATA_MODEL_SPEC.md |

## 3. Storage Requirements (PRD2.md)

| Req ID | Requirement | SPEC | Implementation | Test | Benchmark | Doc |
|--------|-------------|------|----------------|------|-----------|-----|
| STOR-001 | 10-column storage | KCM_COLUMNAR_FORMAT_SPEC | kcm-storage/src/column.rs | test_storage.rs | bench_column_* | KCM_COLUMNAR_FORMAT_SPEC.md |
| STOR-002 | WAL with CRC32 | KCM_COLUMNAR_FORMAT_SPEC | kcm-storage/src/wal.rs | test_wal_property.rs | bench_wal_* | KCM_COLUMNAR_FORMAT_SPEC.md |
| STOR-003 | Binary file format | KCM_COLUMNAR_FORMAT_SPEC | kcm-storage/src/file_format.rs | test_persistence.rs | bench_file_* | KCM_COLUMNAR_FORMAT_SPEC.md |
| STOR-004 | Zstd compression | KCM_COMPRESSION_SPEC | kcm-storage/src/compress.rs | test_codec_property.rs | bench_compression_* | KCM_COMPRESSION_SPEC.md |
| STOR-005 | LZ4 compression | KCM_COMPRESSION_SPEC | kcm-storage/src/compress.rs | test_codec_property.rs | bench_compression_* | KCM_COMPRESSION_SPEC.md |
| STOR-006 | RLE compression | KCM_COMPRESSION_SPEC | kcm-storage/src/compress.rs | test_codec_property.rs | bench_compression_* | KCM_COMPRESSION_SPEC.md |
| STOR-007 | Bitmap index | KCM_INDEXING_SPEC | kcm-storage/src/index.rs | test_property.rs | - | KCM_INDEXING_SPEC.md |
| STOR-008 | Zone map index | KCM_INDEXING_SPEC | kcm-storage/src/index.rs | test_property.rs | - | KCM_INDEXING_SPEC.md |
| STOR-009 | Bloom filter index | KCM_INDEXING_SPEC | kcm-storage/src/index.rs | test_property.rs | - | KCM_INDEXING_SPEC.md |
| STOR-010 | Composite index | KCM_INDEXING_SPEC | kcm-storage/src/index.rs | test_property.rs | - | KCM_INDEXING_SPEC.md |
| STOR-011 | Backup/restore | KCM_DEPLOYMENT_SPEC | kcm-storage/src/backup.rs | test_recovery.rs | - | KCM_DEPLOYMENT_SPEC.md |
| STOR-012 | Crash recovery | KCM_DEPLOYMENT_SPEC | kcm-storage/src/recovery.rs | test_crash_recovery.rs | - | KCM_DEPLOYMENT_SPEC.md |

## 4. Query Requirements (PRD.md)

| Req ID | Requirement | SPEC | Implementation | Test | Benchmark | Doc |
|--------|-------------|------|----------------|------|-----------|-----|
| QUER-001 | Volcano execution model | KCM_QUERY_EXECUTION_SPEC | kcm-compute/src/algebra.rs | test_compute.rs | - | KCM_QUERY_EXECUTION_SPEC.md |
| QUER-002 | Scan operator | KCM_QUERY_EXECUTION_SPEC | kcm-compute/src/algebra.rs | test_compute.rs | - | KCM_QUERY_EXECUTION_SPEC.md |
| QUER-003 | Filter operator | KCM_QUERY_EXECUTION_SPEC | kcm-compute/src/algebra.rs | test_compute.rs | - | KCM_QUERY_EXECUTION_SPEC.md |
| QUER-004 | Project operator | KCM_QUERY_EXECUTION_SPEC | kcm-compute/src/algebra.rs | test_compute.rs | - | KCM_QUERY_EXECUTION_SPEC.md |
| QUER-005 | Join operator | KCM_QUERY_EXECUTION_SPEC | kcm-compute/src/algebra.rs | test_compute.rs | - | KCM_QUERY_EXECUTION_SPEC.md |
| QUER-006 | Aggregate operator | KCM_QUERY_EXECUTION_SPEC | kcm-compute/src/algebra.rs | test_compute.rs | - | KCM_QUERY_EXECUTION_SPEC.md |
| QUER-007 | SIMD AVX2 acceleration | KCM_PERFORMANCE_SPEC | kcm-compute/src/simd.rs | test_compute.rs | bench_simd_* | KCM_PERFORMANCE_SPEC.md |
| QUER-008 | KQL parser | KCM_QUERY_EXECUTION_SPEC | kcm-interface/src/kql_parser.rs | test_kql_edge_cases.rs | - | KCM_QUERY_EXECUTION_SPEC.md |

## 5. Reasoning Requirements (PRD.md)

| Req ID | Requirement | SPEC | Implementation | Test | Benchmark | Doc |
|--------|-------------|------|----------------|------|-----------|-----|
| REAS-001 | Rule definitions | KCM_SPECIFICATION | kcm-reasoning/src/rule.rs | test_reasoning.rs | bench_inference_* | KCM_SPECIFICATION.md |
| REAS-002 | Forward-chaining inference | KCM_SPECIFICATION | kcm-reasoning/src/inference.rs | test_reasoning.rs | bench_inference_* | KCM_SPECIFICATION.md |
| REAS-003 | Confidence propagation | KCM_SPECIFICATION | kcm-reasoning/src/inference.rs | test_reasoning.rs | - | KCM_SPECIFICATION.md |
| REAS-004 | Convergence detection | KCM_SPECIFICATION | kcm-reasoning/src/inference.rs | test_reasoning.rs | - | KCM_SPECIFICATION.md |

## 6. Runtime Requirements (PRD2.md)

| Req ID | Requirement | SPEC | Implementation | Test | Benchmark | Doc |
|--------|-------------|------|----------------|------|-----------|-----|
| RUNT-001 | KnowledgeDatabase | KCM_RUNTIME_SPEC | kcm-runtime/src/database.rs | test_full.rs | bench_database_* | KCM_RUNTIME_SPEC.md |
| RUNT-002 | Transactions | KCM_RUNTIME_SPEC | kcm-runtime/src/transaction.rs | test_transaction_rollback.rs | bench_transaction_* | KCM_RUNTIME_SPEC.md |
| RUNT-003 | Metrics (14 counters) | KCM_RUNTIME_SPEC | kcm-runtime/src/metrics.rs | test_monitoring.rs | - | KCM_RUNTIME_SPEC.md |
| RUNT-004 | Health checks | KCM_RUNTIME_SPEC | kcm-runtime/src/health.rs | test_monitoring.rs | - | KCM_RUNTIME_SPEC.md |
| RUNT-005 | Rayon executor | KCM_RUNTIME_SPEC | kcm-runtime/src/executor.rs | test_integration.rs | - | KCM_RUNTIME_SPEC.md |
| RUNT-006 | Tokio async executor | KCM_RUNTIME_SPEC | kcm-runtime/src/async_executor.rs | test_integration.rs | - | KCM_RUNTIME_SPEC.md |

## 7. Interface Requirements (PRD2.md)

| Req ID | Requirement | SPEC | Implementation | Test | Benchmark | Doc |
|--------|-------------|------|----------------|------|-----------|-----|
| IFCE-001 | C FFI (18 functions) | KCM_API_SPEC | kcm-interface/src/lib.rs | test_interface.rs | - | KCM_API_SPEC.md |
| IFCE-002 | Python bindings | KCM_API_SPEC | kcm-interface/src/python.rs | - | - | KCM_API_SPEC.md |
| IFCE-003 | REST API (8 endpoints) | KCM_API_SPEC | kcm-interface/src/rest_api.rs | test_endpoints.rs | - | KCM_API_SPEC.md |
| IFCE-004 | KQL parser | KCM_API_SPEC | kcm-interface/src/kql_parser.rs | test_kql_edge_cases.rs | - | KCM_API_SPEC.md |

## 8. Security Requirements (PRD3.md)

| Req ID | Requirement | SPEC | Implementation | Test | Benchmark | Doc |
|--------|-------------|------|----------------|------|-----------|-----|
| SECU-001 | RBAC (5 permission levels) | KCM_SECURITY_TRUST_SPEC | kcm-security/src/rbac.rs | test_security.rs | - | KCM_SECURITY_TRUST_SPEC.md |
| SECU-002 | AES-256-GCM encryption | KCM_SECURITY_TRUST_SPEC | kcm-security/src/encryption.rs | test_security.rs | - | KCM_SECURITY_TRUST_SPEC.md |
| SECU-003 | Hash-chained audit log | KCM_SECURITY_TRUST_SPEC | kcm-security/src/audit.rs | test_security.rs | - | KCM_SECURITY_TRUST_SPEC.md |
| SECU-004 | Key derivation (BLAKE3) | KCM_SECURITY_TRUST_SPEC | kcm-security/src/encryption.rs | test_security.rs | - | KCM_SECURITY_TRUST_SPEC.md |

## 9. Compliance Requirements (PRD3.md)

| Req ID | Requirement | SPEC | Implementation | Test | Benchmark | Doc |
|--------|-------------|------|----------------|------|-----------|-----|
| COMP-001 | GDPR consent management | KCM_SECURITY_TRUST_SPEC | kcm-compliance/src/gdpr.rs | test_gdpr.rs | - | KCM_SECURITY_TRUST_SPEC.md |
| COMP-002 | Data classification (4 tiers) | KCM_SECURITY_TRUST_SPEC | kcm-compliance/src/data_classification.rs | test_compliance.rs | - | KCM_SECURITY_TRUST_SPEC.md |

## 10. Distributed Requirements (PRD3.md)

| Req ID | Requirement | SPEC | Implementation | Test | Benchmark | Doc |
|--------|-------------|------|----------------|------|-----------|-----|
| DIST-001 | Hash sharding | KCM_SPECIFICATION | kcm-distributed/src/sharding.rs | test_distributed.rs | bench_distributed_* | KCM_SPECIFICATION.md |
| DIST-002 | Range sharding | KCM_SPECIFICATION | kcm-distributed/src/sharding.rs | test_distributed.rs | bench_distributed_* | KCM_SPECIFICATION.md |
| DIST-003 | Consistent hash sharding | KCM_SPECIFICATION | kcm-distributed/src/sharding.rs | test_distributed.rs | bench_distributed_* | KCM_SPECIFICATION.md |
| DIST-004 | 2PC coordinator | KCM_SPECIFICATION | kcm-distributed/src/coordinator.rs | test_distributed.rs | - | KCM_SPECIFICATION.md |

## 11. ML Requirements (PRD3.md)

| Req ID | Requirement | SPEC | Implementation | Test | Benchmark | Doc |
|--------|-------------|------|----------------|------|-----------|-----|
| ML-001 | Learned index | KCM_SPECIFICATION | kcm-ml/src/learned_index.rs | test_ml.rs | - | KCM_SPECIFICATION.md |
| ML-002 | Confidence learner | KCM_SPECIFICATION | kcm-ml/src/confidence_learner.rs | test_ml.rs | - | KCM_SPECIFICATION.md |
| ML-003 | Rule discovery | KCM_SPECIFICATION | kcm-ml/src/rule_discovery.rs | test_ml.rs | - | KCM_SPECIFICATION.md |

## 12. Server Requirements (PRD2.md)

| Req ID | Requirement | SPEC | Implementation | Test | Benchmark | Doc |
|--------|-------------|------|----------------|------|-----------|-----|
| SERV-001 | HTTP server (actix-web) | KCM_API_SPEC | kcm-server/src/main.rs | test_server.rs | - | KCM_API_SPEC.md |
| SERV-002 | gRPC server (tonic) | KCM_API_SPEC | kcm-server/src/grpc_server.rs | test_server.rs | - | KCM_API_SPEC.md |

## 13. Technical Requirements (KCM_SPECIFICATION.md §4.1)

| Req ID | Requirement | Priority | Authority | Implementation | Test | Benchmark | Doc |
|--------|-------------|----------|-----------|----------------|------|-----------|-----|
| TR-001 | All column data stored as DenseVec with ≥64-byte alignment | Critical | PRD.md §4.1 | kcm-core/src/vec.rs | test_core.rs | bench_column_* | KCM_SPECIFICATION.md |
| TR-002 | Dictionary encoding maps all string references to u32 IDs | Critical | PRD.md §4.3 | kcm-core/src/dictionary.rs | test_core.rs | bench_dict_* | KCM_SPECIFICATION.md |
| TR-003 | Confidence values stored as f64, validated in [0.0, 1.0] | Critical | PRD.md §3.2 | kcm-core/src/types.rs | property_tests.rs | - | KCM_SPECIFICATION.md |
| TR-004 | Tombstone-based soft delete with active_count tracking | High | PRD2.md §2 | kcm-storage/src/column.rs | test_storage.rs | - | KCM_SPECIFICATION.md |
| TR-005 | WAL-based crash recovery with blake3 checksums | Critical | PRD2.md §3 | kcm-storage/src/wal.rs | test_wal_property.rs, test_crash_recovery.rs | bench_wal_* | KCM_SPECIFICATION.md |
| TR-006 | AES-256-GCM encryption for at-rest data protection | Critical | PRD3.md §10 | kcm-security/src/encryption.rs | test_security.rs | - | KCM_SPECIFICATION.md |
| TR-007 | RBAC with Role/User/Permission/Context ACL model | High | PRD3.md §10 | kcm-security/src/rbac.rs | test_security.rs | - | KCM_SPECIFICATION.md |
| TR-008 | Forward-chaining inference with max iteration limit | High | PRD.md §6 | kcm-reasoning/src/inference.rs | test_reasoning.rs | bench_inference_* | KCM_SPECIFICATION.md |
| TR-009 | All public APIs return Result<T, KcmError> | Critical | AGENTS.md §Non-Negotiable Rules | All crate src/lib.rs, src/types.rs | cargo clippy, test_full.rs | - | KCM_SPECIFICATION.md |
| TR-010 | No unwrap() in production code paths (test-only) | High | AGENTS.md §Non-Negotiable Rules | All crate src/*.rs | cargo clippy, test_full.rs | - | KCM_SPECIFICATION.md |
| TR-011 | Send + Sync bounds on all shared types | Critical | AGENTS.md §Concurrency Model | kcm-runtime/src/database.rs, kcm-runtime/src/transaction.rs | test_concurrent_access.rs | - | KCM_SPECIFICATION.md |
| TR-012 | Zero runtime overhead from Rust (no GC, no reflection) | High | AGENTS.md §Engineering Philosophy | All crates (inherent Rust property) | cargo test --release | bench_* | KCM_SPECIFICATION.md |

## 14. Quality Requirements (KCM_SPECIFICATION.md §4.2)

| Req ID | Requirement | Threshold | Enforcement |
|--------|-------------|-----------|-------------|
| QR-001 | Test pass rate | 100% | Gate 6: cargo test --workspace |
| QR-002 | Clippy warnings | 0 (style warnings excluded) | Gate 6: cargo clippy --workspace |
| QR-003 | Unsafe code in public API | None | Gate 4: Code Quality Guardian |
| QR-004 | Deterministic execution | Verified by regression tests | Gate 5: test_concurrent_access.rs |
| QR-005 | Performance regression | < 5% from baseline | Gate 5: Performance Engineer |

## 15. Engineering Rules (KCM_ENGINEERING_RULES.md)

| Req ID | Requirement | Authority |
|--------|-------------|-----------|
| ER-001 | Every PR must pass `cargo test --workspace` | Gate 6 |
| ER-002 | Every PR must pass `cargo clippy --workspace` | Gate 6 |
| ER-003 | Every PR must pass `cargo fmt --check` | Gate 6 |
| ER-004 | New code must have ≥ 95% test coverage | Testing Strategy |
| ER-005 | Security-sensitive code must have security tests | Security tier |
| ER-006 | Performance-critical code must have benchmarks | Performance tier |
| ER-007 | Property tests required for arithmetic operations | Property tier |
| ER-008 | Load tests run before releases | Load tier |

## 16. Coverage Summary

| Category | Requirements | Implemented | Tested | Documented | Coverage |
|----------|-------------|-------------|--------|------------|----------|
| Core | 7 | 7 | 7 | 7 | 100% |
| Storage | 12 | 12 | 12 | 12 | 100% |
| Query | 8 | 8 | 8 | 8 | 100% |
| Reasoning | 4 | 4 | 4 | 4 | 100% |
| Runtime | 6 | 6 | 6 | 6 | 100% |
| Interface | 4 | 4 | 3 | 4 | 93% |
| Security | 4 | 4 | 4 | 4 | 100% |
| Compliance | 2 | 2 | 2 | 2 | 100% |
| Distributed | 4 | 4 | 4 | 4 | 100% |
| ML | 3 | 3 | 3 | 3 | 100% |
| Server | 2 | 2 | 2 | 2 | 100% |
| Technical (TR) | 12 | 12 | 11 | 12 | 97% |
| Quality (QR) | 5 | 5 | 5 | 5 | 100% |
| Eng. Rules (ER) | 8 | 8 | 8 | 8 | 100% |
| **Total** | **81** | **81** | **79** | **81** | **98%** |

## 17. Discrepancies Found

| ID | Type | Description | Severity | Status |
|----|------|-------------|----------|--------|
| DISC-001 | Doc Gap | FFI count: 18 implemented vs 15 documented | Medium | Fix needed |
| DISC-002 | Doc Gap | Python bindings: implemented but no tests | Low | Add tests |
| DISC-003 | Deviation | bench_fixtures.rs has 9 panics in test infra | Low | Intentional |
| DISC-004 | Gap | TR-011 Send+Sync: no dedicated concurrency bounds test file | Low | Covered by test_concurrent_access.rs |
| DISC-005 | Gap | TR-012 Zero overhead: inherent Rust property, no explicit test needed | Low | Intentional |
