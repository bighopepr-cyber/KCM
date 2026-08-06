# KCM Columnar Format Specification

**Document ID:** KCM-FORMAT-001
**Version:** 1.0.0
**Status:** Active
**Owner:** Specification Lock (P4)
**Authority:** P4 (PRD2.md §2, §4)

---

## 1. Purpose

Defines KCM's physical storage format: binary file layout, column block serialization, WAL entry format, and tombstone bitmap structure.

## 2. Binary File Format

### 2.1 File Layout

```
┌─────────────────────────────────────────────────────┐
│ Header (31 bytes)                                    │
│   Magic:           [u8; 5]  = "KCMDB"               │
│   Version:         u8        = 2                     │
│   Row Count:       u64       (little-endian)         │
│   Column Count:    u8        = 10                    │
│   Created:         i64       (nanoseconds, LE)       │
│   Last Modified:   i64       (nanoseconds, LE)       │
├─────────────────────────────────────────────────────┤
│ Column Block 0 — Subject (variable)                  │
│   Element Count:   u64       (LE)                    │
│   Codec ID:        u8        (0=None,1=Zstd,2=LZ4,3=RLE) │
│   Data Length:      u64       (LE)                    │
│   Data:            [u8; Data Length]                  │
├─────────────────────────────────────────────────────┤
│ Column Block 1 — Predicate (variable)                │
│   ...                                                │
├─────────────────────────────────────────────────────┤
│ ... (Column Blocks 2-9) ...                          │
├─────────────────────────────────────────────────────┤
│ Column Block 9 — Owner (variable)                    │
│   ...                                                │
├─────────────────────────────────────────────────────┤
│ Tombstone Bitmap (variable)                          │
│   Row Count:       u64       (LE)                    │
│   Byte Length:      u64       (LE)                    │
│   Bitmap Data:     [u8; Byte Length]                 │
├─────────────────────────────────────────────────────┤
│ Checksum Trailer                                     │
│   BLAKE3 Hash:    [u8; 32]                          │
└─────────────────────────────────────────────────────┘
```

### 2.2 Header Fields

| Offset | Size | Field | Value |
|--------|------|-------|-------|
| 0 | 5 | Magic | `"KCMDB"` |
| 5 | 1 | Version | `2` |
| 6 | 8 | Row Count | Total rows (including deleted) |
| 14 | 1 | Column Count | `10` |
| 15 | 8 | Created | SystemTime nanos since epoch |
| 23 | 8 | Last Modified | SystemTime nanos since epoch |

Total header: 31 bytes.

### 2.3 Column Block Order

| Index | Column | Rust Type | Codec |
|-------|--------|-----------|-------|
| 0 | Subject | `u32` | Zstd |
| 1 | Predicate | `u8` | RLE |
| 2 | Object | `u32` | Zstd |
| 3 | Confidence | `f64` | Zstd |
| 4 | Evidence | `u8` | RLE |
| 5 | Timestamp | `i64` | Zstd |
| 6 | Context | `u8` | RLE |
| 7 | Version | `i32` | LZ4 |
| 8 | Priority | `i8` | RLE |
| 9 | Owner | `u16` | Zstd |

### 2.4 Column Block Layout

| Field | Size | Description |
|-------|------|-------------|
| Element Count | 8 bytes (u64 LE) | Number of elements in column |
| Codec ID | 1 byte | Compression codec identifier |
| Data Length | 8 bytes (u64 LE) | Length of compressed data |
| Data | Data Length bytes | Compressed column data |

### 2.5 Codec IDs

| ID | Codec | Algorithm |
|----|-------|-----------|
| 0 | None | No compression (identity) |
| 1 | Zstd | Zstandard (level 3 default) |
| 2 | LZ4 | LZ4 block (FAST mode) |
| 3 | RLE | Run-Length Encoding |

### 2.6 Tombstone Bitmap Layout

| Field | Size | Description |
|-------|------|-------------|
| Row Count | 8 bytes (u64 LE) | Number of bits in bitmap |
| Byte Length | 8 bytes (u64 LE) | Length of bitmap data |
| Bitmap Data | Byte Length bytes | Raw bitmap bytes |

### 2.7 Checksum Trailer

- 32-byte BLAKE3 hash
- Covers entire file except the checksum itself
- Computed over bytes [0, file_size - 32)

## 3. Write-Ahead Log (WAL)

### 3.1 Entry Format

#### Insert Entry (38 bytes)

| Offset | Size | Field | Description |
|--------|------|-------|-------------|
| 0 | 1 | Op Type | `1` (INSERT) |
| 1 | 4 | Subject | SubjectID (u32 LE) |
| 5 | 1 | Predicate | PredicateID (u8) |
| 6 | 4 | Object | ObjectID (u32 LE) |
| 10 | 8 | Confidence | f64 (LE) |
| 18 | 8 | Timestamp | i64 (LE) |
| 26 | 1 | Context | ContextID (u8) |
| 27 | 4 | Version | i32 (LE) |
| 31 | 1 | Priority | i8 |
| 32 | 2 | Owner | u16 (LE) |
| 34 | 4 | CRC32 | Checksum of bytes [1..34) |

#### Delete Entry (13 bytes)

| Offset | Size | Field | Description |
|--------|------|-------|-------------|
| 0 | 1 | Op Type | `2` (DELETE) |
| 1 | 8 | Row ID | u64 (LE) |
| 9 | 4 | CRC32 | Checksum of bytes [1..9) |

### 3.2 WAL Properties

- Append-only, sequential I/O
- Buffered writes: 64KB threshold before flush to disk
- CRC32 checksums per entry (polynomial 0xEDB88320)
- `sync_all()` after each buffer flush
- Truncate after successful checkpoint
- Evidence field intentionally omitted (reconstructed from source on replay)

### 3.3 WAL Replay

During replay, each entry is validated:
1. Read op type byte
2. Read entry data
3. Compute CRC32 over data bytes
4. Compare with stored checksum
5. If mismatch → `KcmError::Corrupted`
6. If valid → execute callback

Replay is idempotent: re-inserting an existing fact is safe.

## 4. File Invariants

| Invariant | Enforcement |
|-----------|-------------|
| Magic bytes always "KCMDB" | Written at save, verified at load |
| Version byte determines compatibility | Reject unsupported versions |
| Column blocks written in fixed order | Subject→Owner (0-9) |
| Tombstone bitmap tracks deleted rows | Persisted in file |
| BLAKE3 checksum covers entire file | Computed over [0, size-32) |
| All multi-byte values little-endian | Consistent byte order |

## 5. References

- **Implements:** PRD2.md §2 (Storage Engine), §3 (WAL), §4 (File Format)
- **Depends on:** KCM_DATA_MODEL_SPEC (Fact structure)
- **Related:** KCM_COMPRESSION_SPEC, KCM_INDEXING_SPEC
