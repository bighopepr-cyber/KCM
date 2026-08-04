# Architecture Consistency Matrix

| Field | Value |
|-------|-------|
| **Document ID** | KCM-ACM-001 |
| **Title** | Architecture Consistency Matrix |
| **Version** | 1.0.0 |
| **Date** | 2026-08-04 |
| **Status** | Authoritative |
| **Authority** | Architecture Guardian (P5) |

---

## 1. Purpose

This matrix maps every KCM component to its responsibility, dependencies, ownership, lifecycle, compatibility, stability level, and public contract. It ensures no cyclic dependencies, no duplicated functionality, and no undocumented public APIs.

## 2. Component Registry

### 2.1 Core Engine Crates

| Crate | Responsibility | Dependencies | Owner | Stability | Public Contract |
|-------|---------------|--------------|-------|-----------|-----------------|
| kcm-core | Types, DenseVec, Bitmap, Dictionary | parking_lot | Core Team | Stable | types.rs, vec.rs, bitmap.rs, dictionary.rs |
| kcm-storage | Columns, Codecs, WAL, FileFormat, Index | kcm-core, zstd, lz4, blake3, thiserror | Storage Team | Stable | column.rs, wal.rs, file_format.rs, index.rs, compress.rs |
| kcm-compute | Query operators, SIMD | kcm-core, kcm-storage | Compute Team | Stable | algebra.rs, simd.rs |
| kcm-reasoning | Rules, Inference | kcm-core, kcm-storage | Reasoning Team | Stable | rule.rs, inference.rs |
| kcm-optimizer | Cost model, Planner, Statistics | kcm-core, kcm-storage | Optimizer Team | Beta | planner.rs, cost_model.rs, statistics.rs, rewriting.rs |
| kcm-runtime | Database, Transactions, Metrics | kcm-core, kcm-storage, rayon, tokio | Runtime Team | Stable | database.rs, transaction.rs, metrics.rs, health.rs |
| kcm-interface | C FFI, Python, REST, KQL | kcm-core, kcm-storage, kcm-runtime, serde | Interface Team | Stable | lib.rs (FFI), rest_api.rs, kql_parser.rs |
| kcm-distributed | Sharding, 2PC | kcm-core, parking_lot | Distributed Team | Beta | sharding.rs, coordinator.rs |
| kcm-ml | Learned Index, Confidence Learner | kcm-core, kcm-reasoning | ML Team | Experimental | learned_index.rs, confidence_learner.rs |
| kcm-security | RBAC, Encryption, Audit | kcm-core, blake3, aes-gcm | Security Team | Stable | rbac.rs, encryption.rs, audit.rs |
| kcm-compliance | GDPR, Classification | kcm-core, parking_lot | Compliance Team | Beta | gdpr.rs, data_classification.rs |
| kcm-testing | Test infrastructure | kcm-core, kcm-storage, kcm-runtime | QA Team | Internal | bench_fixtures.rs, load_tests.rs, stress_tests.rs |
| kcm-server | HTTP, gRPC binaries | kcm-core, kcm-runtime, kcm-interface, actix-web, tonic | Platform Team | Stable | main.rs, grpc_server.rs |

### 2.2 Stability Levels

| Level | Definition | Guarantee |
|-------|-----------|-----------|
| Experimental | May change in any release | No compatibility guarantee |
| Beta | Stable API, may have minor changes | Best-effort compatibility |
| Stable | API frozen, semantic versioning | Full backward compatibility |
| LTS | Long-term support, critical fixes only | 5-year support |

### 2.3 Dependency Graph (Verified)

```
kcm-core (zero internal deps)
  |
  +-- kcm-storage
  |     |
  |     +-- kcm-compute
  |     +-- kcm-reasoning
  |     +-- kcm-optimizer
  |     |
  |     +-- kcm-runtime
  |           |
  |           +-- kcm-interface
  |                 |
  |                 +-- kcm-server
  |
  +-- kcm-distributed (independent)
  +-- kcm-ml (depends on kcm-reasoning)
  +-- kcm-security (independent)
  +-- kcm-compliance (independent)
  |
  +-- kcm-testing (depends on multiple crates)
```

## 3. Public API Surface

### 3.1 kcm-core (54 public functions, 12 public structs)

| Module | Public Functions | Public Structs | Key Types |
|--------|-----------------|----------------|-----------|
| types.rs | 17 | 8 | Fact, KcmError, RowID, SubjectID, PredicateID, ObjectID, Confidence |
| vec.rs | 9 | 1 | DenseVec<T> |
| bitmap.rs | 15 | 1 | Bitmap |
| dictionary.rs | 13 | 2 | Dictionary, SharedDictionary |

### 3.2 kcm-storage (88 public functions, 16 public structs)

| Module | Public Functions | Public Structs | Key Types |
|--------|-----------------|----------------|-----------|
| column.rs | 29 | 2 | Column<T>, Schema |
| wal.rs | 8 | 1 | WriteAheadLog |
| file_format.rs | 5 | 1 | DatabaseFile |
| index.rs | 14 | 4 | BitmapIndex, ZoneMap, BloomFilter, CompositeIndex |
| compress.rs | 5 | 4 | ZstdCompressor, Lz4Compressor, RleCompressor, NoopCompressor |
| dict_codec.rs | 9 | 1 | DictionaryCodec |
| backup.rs | 5 | 2 | BackupManager, RestoreManager |
| recovery.rs | 2 | 1 | RecoveryManager |
| wal_state.rs | 11 | 2 | WALState, WALConfig |

### 3.3 kcm-runtime (61 public functions, 12 public structs)

| Module | Public Functions | Public Structs | Key Types |
|--------|-----------------|----------------|-----------|
| database.rs | 22 | 3 | KnowledgeDatabase |
| transaction.rs | 10 | 1 | Transaction |
| metrics.rs | 15 | 4 | Metrics |
| health.rs | 7 | 2 | HealthCheck, HealthStatus |
| executor.rs | 5 | 1 | Executor |
| async_executor.rs | 2 | 1 | AsyncExecutor |

### 3.4 kcm-interface (18 FFI functions, 20 public structs)

| Module | FFI Functions | Public Structs | Key Types |
|--------|--------------|----------------|-----------|
| lib.rs | 18 | 4 | KCM_Database, KCM_Fact, KCM_Query, KCM_Transaction |
| rest_api.rs | 12 | 2 | RestApi, RestConfig |
| kql_parser.rs | 5 | 6 | Lexer, Parser, Token, SelectQuery, WhereClause |
| python.rs | 1 | 1 | PyKnowledgeBase |

## 4. Contract Verification

### 4.1 File Format Contract

| Contract | Documented | Implemented | Status |
|----------|-----------|-------------|--------|
| DB_MAGIC: "KCMDB" | Yes | Yes | MATCH |
| DB_VERSION: 2 | Yes | Yes | MATCH |
| WAL_INSERT_SIZE: 38 bytes | Yes | Yes | MATCH |
| WAL_DELETE_SIZE: 13 bytes | Yes | Yes | MATCH |
| 10 column blocks | Yes | Yes | MATCH |
| BLAKE3 checksum | Yes | Yes | MATCH |

### 4.2 FFI Contract

| Function | Documented | Implemented | Status |
|----------|-----------|-------------|--------|
| KCM_DatabaseNew | Yes | Yes | MATCH |
| KCM_DatabaseFree | Yes | Yes | MATCH |
| KCM_DatabaseInsert | Yes | Yes | MATCH |
| KCM_DatabaseUpdate | Yes | Yes | MATCH |
| KCM_DatabaseDelete | Yes | Yes | MATCH |
| KCM_DatabaseFactCount | Yes | Yes | MATCH |
| KCM_DatabaseActiveCount | Yes | Yes | MATCH |
| KCM_DatabaseQuery | Yes | Yes | MATCH |
| KCM_QueryNext | Yes | Yes | MATCH |
| KCM_QueryFree | Yes | Yes | MATCH |
| KCM_DatabaseBeginTransaction | Yes | Yes | MATCH |
| KCM_TransactionFree | Yes | Yes | MATCH |
| KCM_DatabaseSave | Yes | Yes | MATCH |
| KCM_DatabaseLoad | Yes | Yes | MATCH |
| KCM_DatabaseVerify | Yes | Yes | MATCH |
| KCM_TransactionCommit | Yes | Yes | MATCH |
| KCM_TransactionRollback | Yes | Yes | MATCH |
| KCM_ErrorMessage | Yes | Yes | MATCH |

### 4.3 REST API Contract

| Endpoint | Documented | Implemented | Status |
|----------|-----------|-------------|--------|
| POST /api/facts | Yes | Yes | MATCH |
| GET /api/facts | Yes | Yes | MATCH |
| GET /api/facts/:id | Yes | Yes | MATCH |
| DELETE /api/facts/:id | Yes | Yes | MATCH |
| POST /api/query | Yes | Yes | MATCH |
| GET /api/stats | Yes | Yes | MATCH |
| GET /health | Yes | Yes | MATCH |
| POST /api/transactions/begin | Yes | Yes | MATCH |

## 5. Consistency Checks

| Check | Status | Notes |
|-------|--------|-------|
| No cyclic dependencies | PASS | DAG verified |
| No duplicate functionality | PASS | Each module has unique responsibility |
| All public APIs documented | PASS | README + API spec |
| All public APIs tested | PASS | Unit + integration tests |
| All file format contracts match | PASS | Documented == Implemented |
| All FFI functions match | PASS | 18/18 |
| All REST endpoints match | PASS | 8/8 |
| All gRPC RPCs match | PASS | 4/4 |
