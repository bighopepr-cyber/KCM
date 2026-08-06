# Database Engine Specialist

> Document ID: KCM-SKILL-006 | Version: 2.0.0 | Status: Active

## Overview

Ensure KCM's storage engine, query engine, transaction system, and indexing infrastructure are correct, consistent, and production-ready as a database system. This skill validates binary format correctness, WAL durability, compression roundtrips, operator correctness, and recovery completeness.

## Mission

Guarantee deterministic binary format, lossless compression, WAL durability with fsync, complete crash recovery, and correct query execution including tombstone handling and aggregate functions.

## Responsibilities

| # | Responsibility | Description |
|---|---------------|-------------|
| 1 | Binary Format Validation | Verify deterministic file format with 31-byte header, version byte, Blake3 checksum, and tombstone persistence |
| 2 | WAL Durability | Ensure WAL entries preserve all 10 Fact fields, fsync on flush, and idempotent replay |
| 3 | Compression Correctness | Validate lossless compression roundtrips (Zstd, LZ4, RLE) |
| 4 | Codec Correctness | Verify codec roundtrip: encode → decode = identity (Delta, RLE, Gorilla) |
| 5 | Query Operator Correctness | Validate all operators skip tombstoned rows, correct aggregate functions, column extraction |
| 6 | Transaction Integrity | Ensure atomicity, complete rollback, and consistent version store |
| 7 | Recovery Completeness | Validate crash recovery: DB+WAL, WAL-only, and fresh startup scenarios |
| 8 | Backup Roundtrip | Verify backup → restore produces identical data |
| 9 | Index Correctness | Validate BitmapIndex, ZoneMap, BloomFilter, CompositeIndex implementations |

## Authority

| Priority | Authority Level | Blocking Authority | Approval Authority | Escalation |
|----------|----------------|-------------------|-------------------|------------|
| P6 | Database Engine Specialist | Block storage/query/transaction changes | Approve storage engine decisions | Escalate to P5 (Architecture) or P1 (Orchestrator) |

## Scope

| In Scope | Out of Scope |
|----------|-------------|
| kcm-storage: column.rs, codec.rs, compress.rs, file_format.rs, wal.rs, index.rs, dict_codec.rs, errors.rs, backup.rs, recovery.rs | General code quality review |
| kcm-compute: algebra.rs (query operators) | Architecture-level decisions |
| kcm-optimizer: planner.rs, cost_model.rs, statistics.rs | Security or encryption review |
| kcm-runtime: database.rs, transaction.rs (transaction logic) | Performance optimization |
| kcm-server: grpc_server.rs (server-side storage access) | Test writing |
| Data integrity invariants across all storage operations | Documentation authoring |

## Non Goals

1. Reviewing general code quality or style (Code Quality Guardian responsibility)
2. Making architecture-level decisions (Architecture Guardian responsibility)
3. Performance optimization of existing implementations (Performance Engineer responsibility)
4. Security or cryptographic review (Security Engineer responsibility)
5. Writing unit or integration tests (Testing Skill responsibility)
6. Authoring documentation (Documentation Guardian responsibility)

## Inputs

| Input | Source | Required |
|-------|--------|----------|
| KCM_COLUMNAR_FORMAT_SPEC.md | docs/ directory | Yes (for format changes) |
| KCM_COMPRESSION_SPEC.md | docs/ directory | Yes (for codec/compression changes) |
| KCM_QUERY_EXECUTION_SPEC.md | docs/ directory | Yes (for query changes) |
| KCM_RUNTIME_SPEC.md | docs/ directory | Yes (for transaction changes) |
| KCM_DATA_MODEL_SPEC.md | docs/ directory | Yes (for data model changes) |
| KCM_INDEXING_SPEC.md | docs/ directory | Yes (for index changes) |
| Modified source file | crates/ directory | Yes |
| Related test files | tests/ directory | Yes |

## Outputs

| Output | Format | Destination |
|--------|--------|-------------|
| Validation report | Markdown report with tables | Engineering Orchestrator (P1) |
| Data integrity assessment | Checklist-based report | Calling skill or CI |
| Storage engine verdict | PASS/FAIL with details | Release pipeline |

## Workflow

```
1. Receive storage/query/transaction change request
2. Read relevant specification document(s)
3. Compare specification with implementation byte-by-byte
4. Verify header fields (magic, version, row_count, col_count, timestamps)
5. Verify column block format (length, codec_id, compressed_size, data)
6. Verify tombstone persistence (bitmap_length, bitmap_data)
7. Verify checksum computation (Blake3 over file content)
8. Validate codec/compression roundtrip correctness
9. Verify query operators skip tombstoned rows
10. Verify aggregate functions (Count/Sum/Avg/Min/Max) return computed values
11. Verify WAL entry format and field preservation
12. Verify crash recovery scenarios
13. Test save/load roundtrip with verification
14. Produce validation report with PASS/FAIL verdict
```

## Decision Process

```
Storage/Query/Transaction Change
  ↓
Identify Component (Format/WAL/Codec/Operator/Transaction/Recovery)
  ↓
Read Relevant Specification
  ↓
Compare Implementation vs Specification
  ↓
Mismatch Found? ──→ YES → BLOCK with fix requirements
  ↓ (NO)
Run Roundtrip Tests
  ↓
Tests Pass? ──→ NO → BLOCK with test requirements
  ↓ (YES)
Check Data Integrity Invariants
  ↓
Invariants Preserved? ──→ NO → BLOCK
  ↓ (YES)
APPROVE with validation report
```

## Validation

| Check | Method | Pass Criteria |
|-------|--------|---------------|
| Binary format version | Header inspection | Version byte present and correct |
| File checksum | Blake3 verification | Verified on load |
| Tombstone persistence | Save/load roundtrip | Bitmap restored correctly |
| WAL entry size | Byte inspection | 34 bytes (Insert), 9 bytes (Delete) |
| WAL field preservation | Field count check | All 10 Fact fields in WAL |
| WAL fsync | Code inspection | Called on every flush |
| Codec roundtrip | encode → decode test | Identity |
| Compression roundtrip | compress → decompress test | Identity |
| Tombstone skip | Operator execution | All operators check is_deleted |
| Aggregate functions | Unit tests | Count/Sum/Avg/Min/Max implemented |
| Recovery DB+WAL | Crash simulation | Correct after crash |
| Recovery WAL-only | Missing DB test | Correct without DB file |
| Recovery fresh | Empty state test | Empty schema created |
| Backup roundtrip | backup → restore test | Identity |

## Quality Gates

- [ ] `cargo check --workspace` passes clean
- [ ] File format matches specification exactly
- [ ] WAL entries preserve all Fact fields
- [ ] Compression roundtrip tests pass
- [ ] Codec roundtrip tests pass
- [ ] All operators skip tombstoned rows
- [ ] All aggregate functions return computed values (not rowids)
- [ ] Crash recovery is complete and lossless
- [ ] Backup → restore roundtrip produces identical data
- [ ] Blake3 checksum covers entire file
- [ ] No `unwrap()` in production code paths

## Dependencies

| Skill | Dependency Type | Description |
|-------|----------------|-------------|
| kcm-architecture-guardian (P5) | Upstream gate | Validates storage architecture before implementation |
| kcm-specification-lock (P4) | Upstream gate | Validates frozen format contracts |
| kcm-code-quality-guardian (P10) | Downstream | Validates code quality after storage review |
| kcm-testing-verification (P9) | Downstream | Validates test coverage for storage changes |
| kcm-performance-engineer (P8) | Parallel | Validates storage performance targets |

## Related Skills

| Skill | Relationship |
|-------|-------------|
| kcm-architecture-guardian (P5) | P5 validates storage architecture; P6 validates storage correctness |
| kcm-code-quality-guardian (P10) | P10 reviews code quality; P6 reviews data integrity |
| kcm-performance-engineer (P8) | P8 measures storage performance; P6 validates functional correctness |
| kcm-testing-verification (P9) | P9 writes storage tests; P6 validates storage semantics |

## SSOT References

| Document | Section | Relevance |
|----------|---------|-----------|
| SSOT.md | Binary File Format | DB_MAGIC, DB_VERSION, header layout |
| SSOT.md | WAL Entry Format | WAL_INSERT_SIZE, WAL_DELETE_SIZE |
| docs/KCM_COLUMNAR_FORMAT_SPEC.md | All sections | Binary format specification |
| docs/KCM_COMPRESSION_SPEC.md | All sections | Encoding and compression |
| docs/KCM_QUERY_EXECUTION_SPEC.md | All sections | Query pipeline specification |
| docs/KCM_RUNTIME_SPEC.md | Transaction section | Transaction lifecycle |
| docs/KCM_DATA_MODEL_SPEC.md | All sections | Fact structure, 34 bytes, 10 fields |
| docs/KCM_INDEXING_SPEC.md | All sections | Index structures |
| AGENTS.md | §5.3 Immutable Contracts | Frozen format contracts |

## Failure Conditions

| Condition | Impact | Escalation |
|-----------|--------|------------|
| File format mismatch with specification | Data corruption risk | BLOCK immediately |
| WAL entry doesn't preserve all fields | Data loss risk | BLOCK immediately |
| Compression not lossless | Data corruption risk | BLOCK immediately |
| Operator doesn't skip tombstones | Incorrect query results | BLOCK immediately |
| Aggregate returns rowids instead of values | Incorrect results | BLOCK immediately |
| Recovery loses data | Data loss risk | BLOCK immediately |
| Checksum uses non-cryptographic hash | Integrity risk | BLOCK immediately |
| Format change without version bump | Compatibility risk | BLOCK immediately |

## Escalation

| Level | Path | SLA |
|-------|------|-----|
| Level 1 | Database Engine Specialist resolves internally | 4 hours |
| Level 2 | Escalate to Architecture Guardian (P5) for architecture disputes | 8 hours |
| Level 3 | Escalate to Engineering Orchestrator (P1) | 24 hours |
| Level 4 | SSOT.md is final authority for format specifications | 48 hours |

## Examples

See [examples/](examples/) for storage engine review examples.

## Checklist

See [checklists/](checklists/) for database engine validation checklists.

## References

- [SSOT.md](../../SSOT.md)
- [AGENTS.md](../../AGENTS.md)
- [KCM_SPECIFICATION.md](../../KCM_SPECIFICATION.md)
- [docs/KCM_COLUMNAR_FORMAT_SPEC.md](../../docs/KCM_COLUMNAR_FORMAT_SPEC.md)
- [docs/KCM_COMPRESSION_SPEC.md](../../docs/KCM_COMPRESSION_SPEC.md)
- [docs/KCM_QUERY_EXECUTION_SPEC.md](../../docs/KCM_QUERY_EXECUTION_SPEC.md)
- [docs/KCM_RUNTIME_SPEC.md](../../docs/KCM_RUNTIME_SPEC.md)
- [docs/KCM_INDEXING_SPEC.md](../../docs/KCM_INDEXING_SPEC.md)
