# Architecture Consistency Matrix

**Document ID:** KCM-ACM-001
**Version:** 3.0.0
**Status:** Active
**Owner:** Architecture Guardian (P5)
**Standard:** Microsoft Pragmatic Rust Guidelines 2026

---

## 1. Purpose

Maps every KCM component to its responsibility, dependencies, ownership, stability, and public contract. Ensures no cyclic dependencies, no duplicated functionality, and no undocumented public APIs.

## 2. Component Registry

### 2.1 Core Engine Crates

| Crate | Responsibility | Dependencies | Stability | Public Contract |
|-------|---------------|--------------|-----------|-----------------|
| kcm-core | Types, DenseVec, Bitmap, Dictionary | parking_lot | Stable | types.rs, vec.rs, bitmap.rs, dictionary.rs |
| kcm-storage | Columns, Codecs, WAL, FileFormat, Index | kcm-core, zstd, lz4, blake3, thiserror | Stable | column.rs, wal.rs, file_format.rs, index.rs, compress.rs |
| kcm-compute | Query operators, SIMD | kcm-core, kcm-storage | Stable | algebra.rs, simd.rs |
| kcm-reasoning | Rules, Inference | kcm-core, kcm-storage | Stable | rule.rs, inference.rs |
| kcm-optimizer | Cost model, Planner, Statistics | kcm-core, kcm-storage | Beta | planner.rs, cost_model.rs, statistics.rs, rewriting.rs |
| kcm-runtime | Database, Transactions, Metrics | kcm-core, kcm-storage, rayon, tokio | Stable | database.rs, transaction.rs, metrics.rs, health.rs |
| kcm-interface | C FFI, Python, REST, KQL | kcm-core, kcm-storage, kcm-runtime, serde | Stable | lib.rs (FFI), rest_api.rs, kql_parser.rs |
| kcm-distributed | Sharding, 2PC | kcm-core, parking_lot | Beta | sharding.rs, coordinator.rs |
| kcm-ml | Learned Index, Confidence Learner | kcm-core, kcm-reasoning | Experimental | learned_index.rs, confidence_learner.rs |
| kcm-security | RBAC, Encryption, Audit | kcm-core, blake3, aes-gcm | Stable | rbac.rs, encryption.rs, audit.rs |
| kcm-compliance | GDPR, Classification | kcm-core, parking_lot | Beta | gdpr.rs, data_classification.rs |
| kcm-testing | Test infrastructure | kcm-core, kcm-storage, kcm-runtime | Internal | bench_fixtures.rs, load_tests.rs, stress_tests.rs |
| kcm-server | HTTP, gRPC binaries | kcm-core, kcm-runtime, kcm-interface, actix-web, tonic | Stable | main.rs, grpc_server.rs |

### 2.2 Stability Levels

| Level | Definition | Guarantee |
|-------|-----------|-----------|
| Experimental | May change in any release | No compatibility guarantee |
| Beta | Stable API, may have minor changes | Best-effort compatibility |
| Stable | API frozen, semantic versioning | Full backward compatibility |

### 2.3 Dependency Graph (Verified — No Cycles)

```
kcm-core (zero internal deps)
  ├── kcm-storage
  │   ├── kcm-compute
  │   ├── kcm-reasoning
  │   ├── kcm-optimizer
  │   └── kcm-runtime
  │       └── kcm-interface
  │           └── kcm-server
  ├── kcm-distributed (independent)
  ├── kcm-ml (depends on kcm-reasoning)
  ├── kcm-security (independent)
  ├── kcm-compliance (independent)
  └── kcm-testing (depends on multiple crates)
```

## 3. Contract Verification

### 3.1 File Format

| Contract | Status |
|----------|--------|
| DB_MAGIC: "KCMDB" | MATCH |
| DB_VERSION: 2 | MATCH |
| WAL_INSERT_SIZE: 38 bytes | MATCH |
| WAL_DELETE_SIZE: 13 bytes | MATCH |
| 10 column blocks | MATCH |
| BLAKE3 checksum | MATCH |

### 3.2 FFI Contract (18/18)

All 18 C FFI functions documented and implemented. See `crates/kcm-interface/src/lib.rs`.

### 3.3 REST API Contract (8/8)

All 8 REST endpoints documented and implemented. See `crates/kcm-server/src/main.rs`.

### 3.4 gRPC Contract (4/4)

All 4 gRPC RPCs documented and implemented. See `crates/kcm-interface/proto/kcm.proto`.

## 4. Consistency Checks

| Check | Status |
|-------|--------|
| No cyclic dependencies | PASS |
| No duplicate functionality | PASS |
| All public APIs documented | PASS |
| All public APIs tested | PASS |
| All file format contracts match | PASS |
| All FFI functions match | PASS |
| All REST endpoints match | PASS |
| All gRPC RPCs match | PASS |
| Edition 2021 across all crates | PASS |
| workspace.dependencies used | PASS |
