# KCM Specification — Technical Constitution

**Document ID:** KCM-SPEC-001  
**Version:** 2.0.0  
**Status:** Active  
**Owner:** Specification Lock (P4)  
**Authoritative Sources:** PRD.md (P4), PRD2.md (P3), PRD3.md (P2), AGENTS.md (P5)

> **Authority Notice:** Full specifications live in `docs/specs/`. This is the root summary.

---

## 1. Overview

KCM (Knowledge Columnar Model) is a self-contained columnar knowledge representation, storage, query, and reasoning engine implemented in Rust. It replaces pointer-based knowledge graph traversal with columnar relation spaces supporting SIMD-accelerated scanning, dictionary encoding, compression-native storage, and deterministic inference.

## 2. Technical Goals

| Goal | Metric | Target |
|------|--------|--------|
| Column scan throughput | ops/sec | > 100M |
| Bitmap operations | ops/sec | > 8M |
| Dictionary lookup | latency | < 100ns |
| Insert throughput | facts/sec | > 50K |
| Query latency (1M facts) | P99 ms | < 100ms |
| Memory efficiency | bytes/fact | < 100 |
| Test coverage | line coverage | ≥ 95% |

## 3. Fact Structure (34 bytes uncompressed)

```rust
pub struct Fact {
    pub subject: SubjectID,      // u32 — dictionary-encoded
    pub predicate: PredicateID,  // u8  — dictionary-encoded
    pub object: ObjectID,        // u32 — dictionary-encoded
    pub confidence: f64,         // validated [0.0, 1.0]
    pub evidence: EvidenceID,    // u8
    pub timestamp: i64,          // nanoseconds since epoch
    pub context: ContextID,      // u8
    pub version: i32,            // monotonic on update
    pub priority: i8,            // -128..127
    pub owner: u16,              // dictionary-encoded
}
```

## 4. Column Storage

| Column | Type | Encoding | Compression |
|--------|------|----------|-------------|
| Subject | u32 | Dictionary | Zstd |
| Predicate | u8 | Dictionary | RLE |
| Object | u32 | Dictionary | Zstd |
| Confidence | f64 | Gorilla | Zstd |
| Evidence | u8 | Dictionary | RLE |
| Timestamp | i64 | Delta | Zstd |
| Context | u8 | Dictionary | RLE |
| Version | i32 | Delta | LZ4 |
| Priority | i8 | Identity | RLE |
| Owner | u16 | Dictionary | Zstd |

## 5. Crate Map (13 crates)

| Crate | Responsibility | Stability |
|-------|---------------|-----------|
| kcm-core | Types, DenseVec, Bitmap, Dictionary | Stable |
| kcm-storage | Columns, Codecs, WAL, FileFormat, Index | Stable |
| kcm-compute | Relational algebra, SIMD acceleration | Stable |
| kcm-reasoning | Rules, forward-chaining inference | Stable |
| kcm-optimizer | Cost model, query planner, statistics | Beta |
| kcm-runtime | KnowledgeDatabase, Transactions, Metrics | Stable |
| kcm-interface | C FFI, Python, REST, KQL parser | Stable |
| kcm-distributed | Sharding, 2PC coordinator | Beta |
| kcm-ml | Learned index, confidence learner | Experimental |
| kcm-security | RBAC, AES-256-GCM, audit log | Stable |
| kcm-compliance | GDPR, data classification | Beta |
| kcm-testing | Load, stress, security, recovery tests | Internal |
| kcm-server | HTTP (actix-web) + gRPC (tonic) | Stable |

## 6. API Surface

### C FFI (18 functions)

KCM_DatabaseNew, KCM_DatabaseFree, KCM_DatabaseInsert, KCM_DatabaseUpdate, KCM_DatabaseDelete, KCM_DatabaseFactCount, KCM_DatabaseActiveCount, KCM_DatabaseQuery, KCM_QueryNext, KCM_QueryFree, KCM_DatabaseBeginTransaction, KCM_TransactionFree, KCM_DatabaseSave, KCM_DatabaseLoad, KCM_DatabaseVerify, KCM_TransactionCommit, KCM_TransactionRollback, KCM_ErrorMessage

### REST Endpoints (8)

POST /api/facts, GET /api/facts, GET /api/facts/:id, DELETE /api/facts/:id, POST /api/query, GET /api/stats, GET /health, POST /api/transactions/begin

### gRPC RPCs (4)

InsertFact, QueryFacts, BeginTransaction, GetStats

## 7. Error Model

```
KcmError
├── NotFound(String)
├── OutOfMemory
├── InvalidArgument(String)
├── Io(String)
├── Corrupted(String)
├── Conflict(String)
└── TransactionAborted
```

## 8. Concurrency Model

| Component | Mechanism |
|-----------|-----------|
| Schema | `Arc<RwLock<Schema>>` (parking_lot) |
| Dictionaries | `Arc<RwLock<Dictionary>>` (parking_lot) |
| WAL | `Mutex<File>` (parking_lot) |
| Audit Log | `Mutex<VecDeque<AuditEvent>>` (parking_lot, Arc) |
| Metrics | `AtomicU64` (14 counters) |
| Thread Pool | rayon ThreadPool |
| Async | tokio Runtime |

## 9. Conflict Resolution

| Priority | Document | Authority |
|----------|----------|-----------|
| P1 | PRD-TESTING-AND-BENCHMARK.md | Performance, validation, testing |
| P2 | PRD3.md | Distributed, ML, security, compliance |
| P3 | PRD2.md | Storage, runtime, interfaces |
| P4 | PRD.md | Core types, storage, compute, reasoning |
| P5 | AGENTS.md | Engineering constitution |

## 10. References

- `docs/specs/PRD.md` — Core types, storage, compute, reasoning (P4)
- `docs/specs/PRD2.md` — Storage, runtime, interfaces (P3)
- `docs/specs/PRD3.md` — Distributed, ML, security, compliance (P2)
- `docs/specs/PRD-TESTING-AND-BENCHMARK.md` — Testing, benchmarks (P1)
- `AGENTS.md` — Engineering constitution (P5)
- `docs/handbook/handbook.md` — Development guide
