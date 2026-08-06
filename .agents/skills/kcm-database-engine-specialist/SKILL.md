---
name: kcm-database-engine-specialist
description: Ensure KCM storage engine, query engine, transaction system, and indexing infrastructure are correct and production-ready
---

# Skill: Database Engine Specialist

## Skill Identity

**Purpose:** Ensure KCM's storage engine, query engine, transaction system, and indexing infrastructure are correct, consistent, and production-ready as a database system.

**Role:** Database Engine Architect / Storage Engine Engineer

**Scope:** Storage layer (columns, codecs, WAL, file format, indexes, dict codec, backup, recovery, errors), query execution (parser, planner, optimizer, operators), transaction management (ACID, recovery, versioning), and all data integrity invariants.

**Non-responsibility:** Does not review general code quality (Code Quality Guardian). Does not review security (Security Engineer). Does not write tests (Testing Skill). Does not validate architecture (Architecture Guardian). Does not review design quality (Code Review Auditor).

**Measurable Outcomes:**
- Binary format is deterministic and versioned
- WAL entries preserve all Fact fields
- All operators skip tombstoned rows
- Recovery is complete and lossless
- Codec/compression roundtrip tests pass

---

## Activation Rules

**Activate when:**
- Storage format changes (file format, WAL, column layout)
- Codec or compression changes
- Query operator implementation or modification
- Optimizer changes
- Transaction or recovery logic changes
- Index implementation changes
- Data integrity concerns arise
- Backup or recovery logic changes
- Dict codec changes

**Do NOT activate when:**
- General code quality review (use Code Quality Guardian)
- Architecture-level decisions (use Architecture Guardian)
- Performance-only changes (use Performance Skill)
- Security review (use Security Engineer)

---

## Required Context

1. `docs/KCM_COLUMNAR_FORMAT_SPEC.md` — Binary format specification
2. `docs/KCM_COMPRESSION_SPEC.md` — Encoding and compression
3. `docs/KCM_QUERY_EXECUTION_SPEC.md` — Query pipeline specification
4. `docs/KCM_RUNTIME_SPEC.md` — Runtime lifecycle
5. `docs/KCM_DATA_MODEL_SPEC.md` — Data model definition
6. `docs/KCM_INDEXING_SPEC.md` — Index structures
7. The specific source file being modified
8. Related test files for the modified component

---

## Crate Awareness

Primary responsibility: **kcm-storage** — all files:

| File | Responsibility |
|------|---------------|
| `column.rs` | Column<T>, Schema |
| `codec.rs` | Delta, RLE, Gorilla codecs |
| `compress.rs` | Zstd, LZ4, RLE compressors |
| `file_format.rs` | Binary DB format |
| `wal.rs` | Write-Ahead Log |
| `index.rs` | BitmapIndex, ZoneMap, BloomFilter, CompositeIndex |
| `dict_codec.rs` | Dictionary encoding |
| `errors.rs` | Storage-specific error types |
| `backup.rs` | Backup and restore |
| `recovery.rs` | Crash recovery |

Secondary responsibility: Related files in other crates:

| Crate | File | Relevance |
|-------|------|-----------|
| kcm-compute | `algebra.rs` | Query operators (Scan, Filter, Project, Join, Aggregate) |
| kcm-optimizer | `planner.rs`, `cost_model.rs`, `statistics.rs` | Query planning |
| kcm-runtime | `database.rs`, `transaction.rs` | Transaction management |
| kcm-server | `grpc_server.rs` | Server-side storage access |

---

## Operating Principles

### Storage Engine Principles

1. **Binary format must be deterministic** — Same input always produces same bytes
2. **File format must be versioned** — Version byte in header enables migration
3. **Checksums must be cryptographic** — Blake3, not CRC32
4. **Columns must maintain equal length** — Row alignment invariant
5. **Tombstone bitmap must persist** — Soft-delete survives crash/restart
6. **WAL must be fsync'd** — Data durability guarantee
7. **Compression must be lossless** — Roundtrip correctness required

### Query Engine Principles

1. **Operators must skip tombstoned rows** — Soft-delete consistency
2. **Execution must be deterministic** — Same query → same results
3. **Optimizer must be idempotent** — Repeated optimization doesn't change plan
4. **Statistics must be accurate** — Cardinality estimation drives optimization
5. **SIMD must have runtime detection** — Portable across CPU architectures

### Transaction Principles

1. **WAL replay must preserve all fields** — No data loss during recovery
2. **Transaction must be atomic** — All changes applied or none
3. **Rollback must be complete** — Every change reversed
4. **Version store must be consistent** — Snapshot isolation

---

## Engineering Workflow

### Storage Format Review

```
1. Read KCM_COLUMNAR_FORMAT_SPEC.md
2. Compare spec with implementation byte-by-byte
3. Verify header fields (magic, version, row_count, col_count, timestamps)
4. Verify column block format (length, codec_id, compressed_size, data)
5. Verify tombstone persistence (bitmap_length, bitmap_data)
6. Verify checksum computation (Blake3 over file content)
7. Test save/load roundtrip with verification
```

### Query Engine Review

```
1. Read KCM_QUERY_EXECUTION_SPEC.md
2. Verify operator trait implementation (execute, estimated_rows)
3. Check ScanOp: tombstone skip, context filter, confidence filter
4. Check FilterOp: all 6 predicate variants
5. Check ProjectOp: column extraction correctness
6. Check JoinOp: hash join algorithm, join column support
7. Check AggregateOp: all 5 functions (Count/Sum/Avg/Min/Max)
8. Verify optimizer pipeline (filter pushdown, join reorder)
```

### Transaction Review

```
1. Read KCM_RUNTIME_SPEC.md transaction section
2. Verify WAL entry format (34 bytes for Insert, 9 bytes for Delete)
3. Verify WAL replay preserves all Fact fields
4. Verify crash recovery (DB+WAL, WAL-only, fresh)
5. Verify transaction state machine (Active→Committed/RolledBack)
6. Verify rollback_changes() restores previous state
```

---

## Validation Criteria

| Component | Criterion | Pass Condition |
|-----------|-----------|---------------|
| File Format | Version byte | Present and correct |
| File Format | Checksum | Blake3, verified on load |
| File Format | Tombstone | Persisted and restored |
| WAL | Entry size | 34 bytes (Insert), 9 bytes (Delete) |
| WAL | Field preservation | All 10 Fact fields in WAL |
| WAL | Fsync | Called on every flush |
| Codecs | Roundtrip | encode→decode = identity |
| Compression | Roundtrip | compress→decompress = identity |
| Operators | Tombstone skip | All operators check is_deleted |
| Aggregate | All functions | Count/Sum/Avg/Min/Max implemented |
| Recovery | DB+WAL | Correct after crash |
| Recovery | WAL-only | Correct without DB file |
| Recovery | Fresh | Empty schema created |
| Backup | Roundtrip | backup→restore = identity |

---

## Failure Prevention Rules

1. **Never allow file format changes without version bump**
2. **Never allow WAL entries that don't preserve all Fact fields**
3. **Never allow compression without roundtrip test**
4. **Never allow operators to skip tombstone check**
5. **Never allow AggregateOp to return rowids instead of computed values**
6. **Never allow ProjectOp to pass through without extracting columns**
7. **Never allow recovery that loses data**
8. **Never allow checksum to use non-cryptographic hash**

---

## Final Report Format

```
# KCM Engineering Report

## Skill
kcm-database-engine-specialist

## Component Reviewed
[Storage/Query/Transaction/Index/Backup/Recovery]

## Specification Reference
[Which spec document and section]

## Implementation Verification
| Check | Status | Details |
|-------|--------|---------|
| ... | PASS/FAIL | ... |

## Data Integrity Assessment
- [ ] Binary format deterministic
- [ ] File format versioned
- [ ] Checksum cryptographic
- [ ] Columns equal length
- [ ] Tombstone persisted
- [ ] WAL fsync'd
- [ ] Compression lossless
- [ ] Backup roundtrip correct

## Specification Impact
[files]

## Code Impact
[files]

## Validation Required
[tests/benchmarks]

## Verdict
PASS / FAIL

## Required Fixes
[List of required changes with file:line references]
```

## SSOT-First Storage Engine Protocol

Every storage engine change MUST follow this protocol:

1. **Identify SSOT Requirement**: Find the requirement in PRD.md §3-4 or PRD2.md §2-5
2. **Verify Current Implementation**: Check if current code matches SSOT
3. **Plan Change**: Define how change maintains SSOT compliance
4. **Implement**: Write code matching specification exactly
5. **Test**: Write tests validating against specification
6. **Benchmark**: Verify performance meets SSOT targets
7. **Validate**: Run `bash scripts/validate-ssot.sh`

## Storage Engine Quality Standards

| Standard | Requirement | Verification |
|----------|-------------|-------------|
| WAL Durability | fsync on every flush | Code review |
| WAL Integrity | CRC32 on every entry | Test validation |
| WAL Recovery | Idempotent replay | Recovery tests |
| File Format | 31-byte header, 10 columns | Format tests |
| Column Encoding | Per-column codec assignment | Encoding tests |
| Compression | Roundtrip compress/decompress | Property tests |
| Dictionary | ID 0 = NULL, bidirectional | Unit tests |
| Bitmap | O(1) set/get, O(n/64) bulk | Unit tests |
| DenseVec | 64-byte alignment, no realloc | Memory tests |
| Index | Bitmap, ZoneMap, BloomFilter | Index tests |

## Storage Engine Invariants

These invariants MUST be maintained in all changes:

| Invariant | Enforcement |
|-----------|-------------|
| Column lengths equal | Schema enforces all columns same length |
| Row IDs monotonically increasing | append_fact increments row_count |
| Tombstone bitmap consistent | delete_fact sets tombstone, clear_fact clears |
| WAL entries self-contained | Each entry has complete fact data |
| File checksum covers entire file | Blake3 over all preceding bytes |
| Dictionary ID 0 always NULL | Reserved at construction |
| Confidence ∈ [0.0, 1.0] | Confidence::new validates |
| No unwrap in production | Clippy + code review |
