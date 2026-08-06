# KCM Glossary

**Document ID:** KCM-GLOSSARY-001
**Version:** 1.0.0
**Status:** Active
**Owner:** Specification Lock (P4)

---

## 1. Core Concepts

| Term | Definition |
|------|-----------|
| **Fact** | A knowledge triple `(subject, predicate, object)` with metadata (confidence, evidence, timestamp, context, version, priority, owner). The atomic unit of knowledge in KCM. |
| **Column** | An independent typed array storing one attribute across all facts. Each column uses dedicated encoding and compression. |
| **Dictionary** | A bidirectional mapping between string values and integer IDs (DictID). ID 0 is reserved as NULL. |
| **Schema** | The collection of 10 typed columns plus a tombstone bitmap that stores all facts in columnar layout. |
| **RowID** | A sequential 64-bit identifier for each fact position in the schema. Monotonically increasing. |
| **Tombstone** | A bitmap entry marking a row as deleted without physically removing it. Soft-delete mechanism. |

## 2. Identity Types

| Term | Rust Type | Range | Purpose |
|------|-----------|-------|---------|
| **RowID** | `u64` | 0..u64::MAX | Sequential row identifier |
| **SubjectID** | `u32` | 0..u32::MAX | Dictionary-encoded subject entity |
| **PredicateID** | `u8` | 0..255 | Dictionary-encoded relationship type |
| **ObjectID** | `u32` | 0..u32::MAX | Dictionary-encoded object entity |
| **ContextID** | `u8` | 0..255 | Context/domain scope (0 = null) |
| **EvidenceID** | `u8` | 0..255 | Evidence type for provenance (0 = unknown) |
| **Confidence** | `f64` | [0.0, 1.0] | Probabilistic confidence score |
| **DictID** | `u32` | 0..u32::MAX | Dictionary entry identifier |

## 3. Storage Concepts

| Term | Definition |
|------|-----------|
| **DenseVec** | SIMD-aligned (64-byte) contiguous memory allocator with fixed capacity and zero-copy slice access. |
| **Bitmap** | 64-bit word bit-vector with O(1) set/clear/get and O(n/64) bulk operations. Used for tombstones and indexing. |
| **ColumnEncoding** | The encoding strategy applied to a column before compression: Identity, Dictionary, Delta, Gorilla, RLE. |
| **CompressionCodec** | The compression algorithm applied after encoding: None, Zstd, LZ4, RLE. |
| **WAL** | Write-Ahead Log. Append-only log of insert/delete operations with CRC32 checksums for crash recovery. |
| **BLAKE3** | Cryptographic hash function used for file integrity checksums and key derivation. |

## 4. Encoding Types

| Term | Definition |
|------|-----------|
| **Identity** | Raw bytes, no transformation. Used for Priority column. |
| **Dictionary** | String/value to u32 mapping. Used for Subject, Predicate, Object, Evidence, Context, Owner. |
| **Delta** | Difference between consecutive values. Used for Timestamp, Version. Reduces storage for monotonically increasing data. |
| **Gorilla** | XOR-based floating-point encoding. Used for Confidence. Exploits temporal correlation in float values. |
| **RLE** | Run-Length Encoding. Used for low-cardinality columns (Predicate, Evidence, Context, Priority). |

## 5. Query Concepts

| Term | Definition |
|------|-----------|
| **Operator** | A query execution unit implementing the `Operator` trait. Produces row ID lists. |
| **ScanOp** | Full table scan with optional context/confidence filtering. |
| **FilterOp** | Predicate evaluation on a set of row IDs. |
| **ProjectOp** | Column selection pass-through. Actual extraction via `execute_projection()`. |
| **JoinOp** | Hash join on a specified column between two row ID sets. |
| **AggregateOp** | Count/Sum/Avg/Min/Max aggregation with optional group-by. |
| **QueryBuilder** | Fluent API for constructing queries with filter chaining. |
| **KQL** | Knowledge Query Language. SQL-like syntax for declarative knowledge queries. |
| **CostModel** | Estimates CPU, I/O, and memory cost of query operators for optimization. |
| **Planner** | Query optimizer that applies filter pushdown, column pruning, join reordering, and index selection. |

## 6. Reasoning Concepts

| Term | Definition |
|------|-----------|
| **Rule** | A pattern-matching production rule with a confidence formula that derives new facts from existing ones. |
| **RulePattern** | A pattern tree (Triple, And, Or, Not) that matches facts in the schema. |
| **ConfidenceFormula** | A closure `Fn(&[f64]) -> f64` that computes derived confidence from matched fact confidences. |
| **InferenceEngine** | Forward-chaining inference system that iteratively applies rules until convergence or limits. |
| **Derivation** | A derived fact with its source rule ID and computed confidence. |
| **Convergence** | State where no new facts are derived in an iteration. Terminates inference. |
| **Forward-Chaining** | Inference strategy that applies rules to known facts to derive new facts eagerly. |

## 7. Runtime Concepts

| Term | Definition |
|------|-----------|
| **KnowledgeDatabase** | Central database wrapping Schema with thread-safe CRUD operations. |
| **Transaction** | Buffering transaction system. Changes buffered in memory, applied atomically on commit. |
| **Metrics** | 14 lock-free AtomicU64 counters tracking queries, inserts, cache, memory, inference. |
| **HealthStatus** | Threshold-based health determination: Healthy, Degraded, Unhealthy. |
| **Executor** | Rayon thread pool with work-stealing parallelism for parallel map/filter operations. |
| **AsyncExecutor** | Tokio runtime bridge for I/O-bound async operations. |

## 8. Security Concepts

| Term | Definition |
|------|-----------|
| **RBAC** | Role-Based Access Control with 5 permission levels: Read, Write, Delete, Execute, Admin. |
| **ACLManager** | Manages users, roles, permissions, and context-level access grants. |
| **AES-256-GCM** | Authenticated encryption with associated data. Used for at-rest encryption. |
| **AuditLog** | Hash-chained tamper-evident audit trail. Ring buffer with max 100,000 events. |
| **CSPRNG** | Cryptographically Secure Pseudo-Random Number Generator. Used for nonce generation. |

## 9. Compliance Concepts

| Term | Definition |
|------|-----------|
| **GDPR** | General Data Protection Regulation. KCM supports data subject management and right to be forgotten. |
| **DataClassification** | 4-tier classification: Public, Internal, Confidential, Restricted. |
| **ConsentState** | GDPR consent status: Granted, Withdrawn, NotProvided. |
| **ClassifiedFact** | A Fact wrapper with classification tier enforcing encryption and audit requirements. |

## 10. Distributed Concepts

| Term | Definition |
|------|-----------|
| **ShardMap** | Maps keys to shard locations across distributed nodes. |
| **HashSharding** | Uniform distribution via `hash(key) % num_shards`. |
| **RangeSharding** | Range-based distribution using sorted boundaries. |
| **ConsistentHashSharding** | Virtual nodes on hash ring for minimal reshuffling on scale. |
| **2PC** | Two-Phase Commit protocol for distributed transaction coordination. |
| **ParticipantTransport** | Trait abstracting network communication for distributed shards. |

## 11. ML Concepts

| Term | Definition |
|------|-----------|
| **LearnedIndex** | Piecewise linear regression models for index position prediction. |
| **ConfidenceLearner** | Online learning of confidence calibration using exponential moving average. |
| **RuleDiscovery** | Association rule mining from fact predicate chain patterns. |

## 12. Quality Concepts

| Term | Definition |
|------|-----------|
| **SSOT** | Single Source of Truth. The authoritative reference for all KCM specifications. |
| **QualityGate** | Mandatory validation checks that must pass before merge (fmt, clippy, build, test, SSOT). |
| **Benchmark** | Criterion-based performance measurement with statistical regression detection. |
| **PropertyTest** | Proptest-based invariant verification for arithmetic and data structure operations. |
| **SecurityTest** | Attack surface validation tests for injection, overflow, RBAC, timing, memory safety. |

## 13. File Format Concepts

| Term | Definition |
|------|-----------|
| **DB_MAGIC** | 5-byte magic number `"KCMDB"` identifying KCM database files. |
| **DB_VERSION** | Format version byte. Currently version 2. |
| **ColumnBlock** | A serialized column section in the database file: element count, codec ID, data length, compressed data. |
| **ChecksumTrailer** | 32-byte BLAKE3 hash appended to the end of database files for integrity verification. |
| **CRC32** | 32-bit cyclic redundancy check used for WAL entry integrity. |
