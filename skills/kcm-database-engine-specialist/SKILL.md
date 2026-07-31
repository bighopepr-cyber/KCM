---
name: kcm-database-engine-specialist
description: Ensure KCM storage engine, query engine, transaction system, and indexing infrastructure are correct and production-ready
---

# Skill: Database Engine Specialist

## Skill Identity

**Purpose:** Ensure KCM's storage engine, query engine, transaction system, and indexing infrastructure are correct, consistent, and production-ready as a database system.

**Role:** Database Engine Architect / Storage Engine Engineer

**Scope:** Storage layer (columns, codecs, WAL, file format, indexes), query execution (parser, planner, optimizer, operators), transaction management (ACID, recovery, versioning), and all data integrity invariants.

**Non-responsibility:** Does not review general code quality (Code Quality Guardian). Does not review security (Security Skill). Does not write tests (Testing Skill).

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

**Do NOT activate when:**
- General code quality review (use Code Quality Guardian)
- Architecture-level decisions (use Architecture Guardian)
- Performance-only changes (use Performance Skill)
- Security review (use Security Skill)

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
# Database Engine Review

## Component Reviewed
[Storage/Query/Transaction/Index]

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

## Verdict
PASS / FAIL

## Required Fixes
[List of required changes with file:line references]
```