# KCM Glossary

**Document ID:** KCM-GLOSS-001  
**Version:** 1.0.0
**Status:** Derived
**Owner:** Documentation Guardian (P11)

---

## 1. Purpose

Defines all technical terms used in KCM documentation.

---

## 2. Core Terms

| Term | Definition |
|------|------------|
| **Fact** | A single knowledge assertion consisting of (subject, predicate, object, confidence, evidence, timestamp, context, version, priority, owner) |
| **Knowledge Object** | Synonym for Fact |
| **Subject** | The entity performing or described by the relation (left side of triple) |
| **Predicate** | The type of relationship between subject and object |
| **Object** | The entity or value being related to (right side of triple) |
| **Confidence** | Probabilistic score in [0.0, 1.0] indicating certainty of the fact |
| **Evidence** | Source identifier proving the fact's origin |
| **Context** | Domain or scope within which the fact is valid |
| **Provenance** | Complete chain of evidence and timestamps for a fact |
| **KnowledgeDatabase** | Central database type wrapping Schema with thread-safe CRUD operations |
| **KcmError** | Primary error type for all KCM public APIs |

---

## 3. Storage Terms

| Term | Definition |
|------|------------|
| **Column** | A contiguous array of values of a single type (e.g., all subject IDs) |
| **Schema** | The complete set of 10 columns plus tombstone bitmap |
| **DenseVec** | SIMD-aligned contiguous memory vector used for column storage |
| **Dictionary** | Bidirectional mapping between string values and integer IDs |
| **Tombstone** | A bitmap marking deleted rows (soft delete) |
| **Segment** | A block of rows within a column |
| **Block** | Physical storage unit containing encoded column data |
| **Column Block** | On-disk representation of a column segment |
| **Bitmap** | 64-bit word bit-vector with O(1) set/clear/get operations |

---

## 4. Encoding Terms

| Term | Definition |
|------|------------|
| **Dictionary Encoding** | Maps values to integer IDs for compact storage |
| **Delta Encoding** | Stores differences between consecutive values |
| **Gorilla Encoding** | XOR-based floating-point compression |
| **RLE** | Run-Length Encoding — stores (value, count) pairs for repeated values |
| **Identity Encoding** | No transformation applied |
| **Frame of Reference** | Block-relative encoding using min value as reference |

---

## 5. Compression Terms

| Term | Definition |
|------|------------|
| **Zstd** | Zstandard compression — general-purpose, high ratio |
| **LZ4** | Fast compression with moderate ratio |
| **Compression Ratio** | Uncompressed size / Compressed size |
| **AEAD** | Authenticated Encryption with Associated Data (e.g., AES-256-GCM) |

---

## 6. Index Terms

| Term | Definition |
|------|------------|
| **Bitmap Index** | One bitmap per unique value for O(1) membership testing |
| **Zone Map** | Min/max statistics per block for range query pruning |
| **Bloom Filter** | Probabilistic set membership test with configurable false positive rate |
| **Cardinality** | Number of unique values in a column |
| **Selectivity** | Fraction of rows matching a predicate |

---

## 7. Query Terms

| Term | Definition |
|------|------------|
| **Query Plan** | Tree of operators representing a query execution strategy |
| **Operator** | A processing unit in the query plan (scan, filter, join, aggregate) |
| **Filter Pushdown** | Optimization that moves filters closer to data source |
| **Join Reordering** | Optimization that selects optimal join order based on cost |
| **Cost Model** | Estimates computational cost of query plan execution |
| **Execution Plan** | Optimized query plan ready for execution |
| **Volcano Model** | Pull-based query execution where each operator requests rows from children |
| **KQL** | Knowledge Query Language — SQL-like query syntax for KCM |

---

## 8. Reasoning Terms

| Term | Definition |
|------|------------|
| **Rule** | A pattern-matching template that derives new facts from existing ones |
| **Rule Pattern** | Structural template (Triple, And, Or, Not) for matching facts |
| **Forward-Chaining** | Iterative inference that repeatedly applies rules until no new facts |
| **Confidence Formula** | Function combining source confidences into derived confidence |
| **Derived Fact** | A new fact produced by rule inference |
| **Conjunction** | AND combination: P(A ∧ B) = P(A) × P(B) |
| **Disjunction** | OR combination: P(A ∨ B) = P(A) + P(B) - P(A) × P(B) |

---

## 9. Security Terms

| Term | Definition |
|------|------------|
| **RBAC** | Role-Based Access Control |
| **ACL** | Access Control List — per-context permission entries |
| **Permission** | An authorized action (Read, Write, Delete, Execute, Admin) |
| **Role** | A named collection of permissions |
| **CSPRNG** | Cryptographically Secure Pseudo-Random Number Generator |
| **KDF** | Key Derivation Function (BLAKE3-based in KCM) |
| **Audit Log** | Immutable record of all system operations |

---

## 10. Runtime Terms

| Term | Definition |
|------|------------|
| **WAL** | Write-Ahead Log — crash recovery journal |
| **MVCC** | Multi-Version Concurrency Control |
| **Transaction** | Atomic unit of work with commit/rollback semantics |
| **Version Store** | Snapshot management for concurrent read access |
| **Health Check** | System status evaluation (Healthy/Degraded/Unhealthy) |
| **Metrics** | Quantitative measurements of system behavior |

---

## 11. Distribution Terms

| Term | Definition |
|------|------------|
| **Shard** | Horizontal partition of data across nodes |
| **Hash Sharding** | Distributes data by hash of key |
| **Range Sharding** | Distributes data by key range boundaries |
| **Consistent Hashing** | Hash ring with virtual nodes for minimal reshuffling |
| **2PC** | Two-Phase Commit — distributed transaction protocol |
| **Coordinator** | Node managing distributed transaction lifecycle |

---

## 12. ML Terms

| Term | Definition |
|------|------------|
| **Learned Index** | ML model that predicts data position from key value |
| **Regression Model** | Linear model mapping value to position |
| **Confidence Learner** | Tracks accuracy of inference rules over time |
| **Rule Discovery** | Automated mining of association patterns from facts |
| **Association Pattern** | Co-occurring predicate pairs suggesting transitive rules |

---

## 13. Testing Terms

| Term | Definition |
|------|------------|
| **Property Test** | Test that verifies invariants hold for random inputs |
| **Mutation Test** | Test that verifies tests kill code mutations |
| **Load Test** | Concurrent user simulation for throughput measurement |
| **Stress Test** | Maximum capacity testing for breaking point identification |
| **Regression Test** | Comparison against performance baselines |
| **Quality Gate** | Pass/fail criteria for CI pipeline stages |

---

## 14. References

- **Depends on:** KCM_SPECIFICATION (KCM_SPECIFICATION)
- **Parent specs:** KCM_SPECIFICATION (KCM_SPECIFICATION)
- **Related:** All KCM documentation files
