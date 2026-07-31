# KCM Columnar Format Specification

**Document ID:** KCM-FORMAT-001  
**Version:** 1.0.0  
**Depends on:** KCM-DATA-001

---

## 1. Purpose

Defines the physical binary storage format for KCM database files.

---

## 2. File Layout

```
┌─────────────────────────────────────────────┐
│                 File Header                  │
│  Magic Bytes (5):     "KCMDB"               │
│  Version (1):         0x02                   │
│  Row Count (8):       u64 little-endian     │
│  Column Count (1):    u8 = 10               │
│  Created Timestamp (8): i64 LE              │
│  Modified Timestamp (8): i64 LE             │
├─────────────────────────────────────────────┤
│              Column Blocks (×10)             │
│  For each column:                            │
│    ┌────────────────────────────────────┐   │
│    │ Length (8):     u64 LE             │   │
│    │ Raw Data:       length × sizeof(T) │   │
│    └────────────────────────────────────┘   │
├─────────────────────────────────────────────┤
│            File Checksum                     │
│  Blake3 Hash (32 bytes)                     │
└─────────────────────────────────────────────┘
```

### 2.1 File Header

| Offset | Size | Type | Description |
|--------|------|------|-------------|
| 0 | 5 | [u8; 5] | Magic bytes: `b"KCMDB"` |
| 5 | 1 | u8 | Format version: `2` |
| 6 | 8 | u64 LE | Total row count |
| 14 | 1 | u8 | Column count: `10` (fixed) |
| 15 | 8 | i64 LE | Created timestamp (nanoseconds since epoch) |
| 23 | 8 | i64 LE | Modified timestamp (nanoseconds since epoch) |

Total header size: 31 bytes.

### 2.2 Column Block

| Field | Size | Description |
|-------|------|-------------|
| Length | 8 bytes (u64 LE) | Number of elements |
| Codec ID | 1 byte (u8) | Compression codec: 0=None, 1=Zstd, 2=LZ4, 3=RLE |
| Compressed Size | 8 bytes (u64 LE) | Size of compressed data in bytes |
| Data | variable | Compressed column data |

Column order (fixed):
1. SubjectColumn (u32) — 4 bytes/element
2. PredicateColumn (u8) — 1 byte/element
3. ObjectColumn (u32) — 4 bytes/element
4. ConfidenceColumn (f64) — 8 bytes/element
5. EvidenceColumn (u8) — 1 byte/element
6. TimestampColumn (i64) — 8 bytes/element
7. ContextColumn (u8) — 1 byte/element
8. VersionColumn (i32) — 4 bytes/element
9. PriorityColumn (i8) — 1 byte/element
10. OwnerColumn (u16) — 2 bytes/element

### 2.3 Checksum

### 2.4 Tombstone Bitmap Block

After all 10 column blocks:

| Field | Size | Description |
|-------|------|-------------|
| Bitmap Length | 8 bytes (u64 LE) | Number of bytes in bitmap |
| Bitmap Data | variable | Packed bits (1 bit per row, LSB first) |

Total tombstone block size: 8 + ceil(row_count / 8) bytes

---

Blake3 hash computed over entire file content excluding the checksum itself (bytes 0 to file_size - 32).

---

## 3. WAL Format

```
┌────────────────────────────────────┐
│         WAL Entry                  │
│  Op Type (1):    u8               │
│    0x01 = Insert                   │
│    0x02 = Delete                   │
│  Payload:        variable          │
└────────────────────────────────────┘
```

### 3.1 Insert Entry (34 bytes)

| Offset | Size | Type | Field |
|--------|------|------|-------|
| 0 | 1 | u8 | Op type: `0x01` |
| 1 | 4 | u32 LE | Subject |
| 5 | 1 | u8 | Predicate |
| 6 | 4 | u32 LE | Object |
| 10 | 8 | f64 LE | Confidence |
| 18 | 8 | i64 LE | Timestamp |
| 26 | 1 | u8 | Context |
| 27 | 4 | i32 LE | Version |
| 31 | 1 | i8 | Priority |
| 32 | 2 | u16 LE | Owner |

### 3.2 Delete Entry (9 bytes)

| Offset | Size | Type | Field |
|--------|------|------|-------|
| 0 | 1 | u8 | Op type: `0x02` |
| 1 | 8 | u64 LE | Row ID |

---

## 4. Compression

### 4.1 Column-Level Compression

Each column can be independently compressed using:

| Codec | Algorithm | Best For |
|-------|-----------|----------|
| None (Noop) | Passthrough | Small columns, test data |
| Zstd | Zstandard | General-purpose, high ratio |
| LZ4 | LZ4 block | Fast decompression |
| RLE | Run-length | Low-cardinality columns (predicate, evidence, context, priority) |

### 4.2 In-Memory vs On-Disk

- **In-memory**: Columns store uncompressed DenseVec data for O(1) random access
- **On-disk**: Columns store compressed bytes; decompressed on load via RecoveryManager

### 4.3 Compression Pipeline

```
Append (uncompressed) → Flush → Compress → Write to disk
Load → Read from disk → Decompress → Populate DenseVec
```

---

## 5. Validation

| Check | When | Method |
|-------|------|--------|
| Magic bytes match | File open | Direct comparison |
| Version supported | File open | Version == 2 |
| Column length matches row count | File load | Per-column length check |
| Checksum valid | File verify | Blake3 recomputation |
| WAL entry format valid | Replay | Op type + boundary check |

---

## 6. Constraints

| Constraint | Rationale |
|------------|-----------|
| Fixed column count (10) | Enables zero-parse column access |
| Little-endian encoding | x86_64 native, avoids byte-swap overhead |
| Blake3 checksum (not CRC32) | Cryptographic integrity verification |
| WAL entries are variable-length | Insert (34 bytes) vs Delete (9 bytes) |

---

## 7. References

- **Depends on:** KCM_DATA_MODEL_SPEC (KCM_DATA_MODEL_SPEC)
- **Parent specs:** KCM_SPECIFICATION (KCM_SPECIFICATION)
- **Related:** KCM_VERSIONING_SPEC (KCM_VERSIONING_SPEC), KCM_COMPRESSION_SPEC (KCM_COMPRESSION_SPEC)
