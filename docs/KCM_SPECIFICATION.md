# KCM Specification — Technical Constitution

**Document ID:** KCM-SPEC-001  
**Version:** 1.0.0  
**Status:** Active  
**Classification:** Technical Constitution / Single Source of Truth

---

## 1. Overview

Knowledge Columnar Model (KCM) is a columnar knowledge representation, storage, and reasoning engine implemented in Rust. KCM replaces pointer-based knowledge graph traversal with columnar relation spaces that support SIMD-accelerated scanning, dictionary encoding, compression-native storage, and deterministic inference.

### 1.1 Problem Statement

Traditional knowledge graphs use adjacency-list representations that:
- Degrade performance on full-scan queries (cache-unfriendly pointer chasing)
- Cannot leverage SIMD instructions for bulk filtering
- Lack native compression for repetitive knowledge patterns
- Provide no column-level encoding optimization

### 1.2 Technical Goals

| Goal | Metric | Target |
|------|--------|--------|
| Column scan throughput | ops/sec | > 100M |
| Bitmap operations | ops/sec | > 8M |
| Dictionary lookup | latency | < 100ns |
| Insert throughput | facts/sec | > 50K |
| Query latency (1M facts) | P99 ms | < 100ms |
| Memory efficiency | bytes/fact | < 100 |
| Test coverage | line coverage | ≥ 95% |

---

## 2. Core Principles

| Principle | Definition |
|-----------|------------|
| **Column-First** | Knowledge is stored as independent linear arrays per attribute, not as object graphs |
| **Deterministic Retrieval** | Identical input queries always produce identical output, regardless of timing |
| **Dictionary-Encoded** | All string/entity references are mapped to compact integer dictionaries |
| **Compression-Native** | Each column uses the optimal encoding (Delta, Gorilla, RLE, Dictionary) and compression (Zstd, LZ4) |
| **Verification-Capable** | Every fact carries confidence, evidence, and timestamp for auditability |
| **Zero-Copy Query** | Query operations iterate over column slices without materialization until projection |

---

## 3. System Scope

### 3.1 In Scope

| Domain | Components |
|--------|------------|
| Storage | Columnar storage, encoding, compression, file format, WAL, backup/recovery |
| Retrieval | Query algebra operators, filter pushdown, index selection |
| Computation | SIMD-accelerated scan, bitmap intersection, aggregate functions |
| Reasoning | Rule engine, forward-chaining inference, confidence calculus |
| Optimization | Cost model, query planner, statistics, adaptive execution |
| Security | RBAC, AES-256-GCM encryption, audit logging |
| Compliance | GDPR data subject management, data classification |
| Distribution | Sharding (hash, range, consistent hash), 2PC coordinator |
| ML Integration | Learned index, confidence learner, rule discovery |
| API | C FFI, Python bindings (PyO3), KQL parser, REST handlers |
| Persistence | WAL, binary file format, backup/restore, crash recovery |
| Operations | Metrics, health checks, logging, Docker/K8s deployment |

### 3.2 Out of Scope

| Exclusion | Reason |
|-----------|--------|
| LLM training | KCM is a storage and reasoning engine, not an ML training platform |
| Chatbot UI | Application-layer concern |
| Application business logic | Consumer responsibility |
| Network protocol implementation | REST/gRPC API is defined; server framework is consumer responsibility |

---

## 4. Technical Requirements

### 4.1 Mandatory Requirements

| ID | Requirement | Priority |
|----|-------------|----------|
| TR-001 | All column data stored as DenseVec with ≥64-byte alignment | Critical |
| TR-002 | Dictionary encoding maps all string references to u32 IDs | Critical |
| TR-003 | Confidence values stored as f64, validated in [0.0, 1.0] | Critical |
| TR-004 | Tombstone-based soft delete with active_count tracking | High |
| TR-005 | WAL-based crash recovery with blake3 checksums | Critical |
| TR-006 | AES-256-GCM encryption for at-rest data protection | Critical |
| TR-007 | RBAC with Role/User/Permission/Context ACL model | High |
| TR-008 | Forward-chaining inference with max iteration limit | High |
| TR-009 | All public APIs return Result<T, KcmError> | Critical |
| TR-010 | No unwrap() in production code paths (test-only) | High |
| TR-011 | Send + Sync bounds on all shared types | Critical |
| TR-012 | Zero runtime overhead from Rust (no GC, no reflection) | High |

### 4.2 Quality Requirements

| ID | Requirement | Threshold |
|----|-------------|-----------|
| QR-001 | Test pass rate | 100% |
| QR-002 | Clippy warnings | 0 (style warnings excluded) |
| QR-003 | Unsafe code in public API | None |
| QR-004 | Deterministic execution | Verified by regression tests |
| QR-005 | Performance regression | < 5% from baseline |

---

## 5. Conflict Resolution

When specifications conflict between source PRD documents:

1. PRD-TESTING&BENCHMARK.md takes precedence for performance targets and validation methodology
2. PRD3.md takes precedence for architectural decisions (distributed, ML, security)
3. PRD2.md takes precedence for persistence and optimizer design
4. PRD.md takes precedence for core data model and type definitions

Undocumented conflicts are recorded in change history rather than resolved by assumption.

---

## 6. Document Map

| Document | Scope |
|----------|-------|
| KCM_ARCHITECTURE.md | System architecture and module responsibilities |
| KCM_DATA_MODEL_SPEC.md | Knowledge representation model |
| KCM_COLUMNAR_FORMAT_SPEC.md | Physical storage format |
| KCM_QUERY_EXECUTION_SPEC.md | Query lifecycle and operators |
| KCM_COMPRESSION_SPEC.md | Encoding and compression strategies |
| KCM_INDEXING_SPEC.md | Index structures and usage |
| KCM_SECURITY_TRUST_SPEC.md | Security, trust, and verification |
| KCM_API_SPEC.md | Public API contracts |
| KCM_RUNTIME_SPEC.md | Runtime lifecycle and concurrency |
| KCM_PERFORMANCE_SPEC.md | Performance targets and benchmarks |
| KCM_TESTING_SPEC.md | Testing standards |
| KCM_ENGINEERING_RULES.md | Development rules |
| KCM_VERSIONING_SPEC.md | Versioning and compatibility |
| KCM_GLOSSARY.md | Terminology definitions |
