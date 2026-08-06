# kcm-storage Technical Specification

## Overview

`kcm-storage` is the columnar storage engine for the KCM (Knowledge Columnar Model) engine. It provides persistent columnar storage, compression (zstd, lz4, RLE), dictionary codec, Write-Ahead Log (WAL), binary database file format, indexing (Bitmap, BloomFilter, ZoneMap, Composite), backup/recovery, Robin Hood hash map, and dictionary cache.

## Scope

This specification covers the `kcm-storage` crate only. It does not cover core types (see `kcm-core`), compute operators (see `kcm-compute`), reasoning (see `kcm-reasoning`), or runtime behavior (see `kcm-runtime`).

## Responsibilities

| Responsibility | Description |
|---------------|-------------|
| Column storage | `Column<T>` — typed columnar storage with 10 physical columns per `Schema` |
| Compression | `Compressor` trait with `ZstdCompressor`, `Lz4Compressor`, `RleCompressor`, `NoopCompressor` |
| Dictionary codec | `DictionaryCodec` — thread-safe string-to-integer encoding backed by `DictionaryCache` |
| Dictionary cache | `DictionaryCache` — bidirectional mapping using `RobinHoodMap` and `Vec<Arc<str>>` |
| Robin Hood map | `RobinHoodMap<K, V>` — open-addressing hash map with Robin Hood probing |
| WAL | `WriteAheadLog` — append-only journal with BLAKE3 checksums for crash recovery |
| WAL state | `WalStateMachine` — state machine managing WAL lifecycle (Fresh → Active → Checkpointing → Replaying → Truncated) |
| File format | `DatabaseFile` — binary format with magic bytes, version, column blocks, and codec metadata |
| Indexing | `BitmapIndex`, `BloomFilter`, `ZoneMap`, `CompositeIndex` — query acceleration structures |
| Backup | `BackupManager` — full and incremental backup with manifest files |
| Recovery | `RecoveryManager` — WAL replay and backup-based crash recovery |
| Error handling | `StorageError` — crate-specific error types with `From<StorageError> for KcmError` |

## Technical Specification

### Column Storage

`Schema` contains 10 physical columns, each stored as a typed vector with per-column encoding and compression.

| Column | Type | Encoding | Compression |
|--------|------|----------|-------------|
| Subject | `SubjectColumn` (u32) | Dictionary | Zstd |
| Predicate | `PredicateColumn` (u8) | Dictionary | RLE |
| Object | `ObjectColumn` (u32) | Dictionary | Zstd |
| Confidence | `ConfidenceColumn` (f64) | Gorilla | Zstd |
| Evidence | `EvidenceColumn` (u8) | Dictionary | RLE |
| Timestamp | `TimestampColumn` (i64) | Delta | Zstd |
| Context | `ContextColumn` (u8) | Dictionary | RLE |
| Version | `VersionColumn` (i32) | Delta | LZ4 |
| Priority | `PriorityColumn` (i8) | Identity | RLE |
| Owner | `OwnerColumn` (u16) | Dictionary | Zstd |

**`ColumnEncoding` variants:**

| Variant | Description |
|---------|-------------|
| `Identity` | Raw byte storage, no encoding |
| `Dictionary` | String-to-integer mapping via `DictionaryCodec` |
| `Delta` | Delta encoding (store differences between consecutive values) |
| `Rle` | Run-length encoding (store repeated values compactly) |
| `Gorilla` | XOR-based floating-point encoding for time-series data |

**`CompressionCodec` variants:**

| Variant | Description |
|---------|-------------|
| `None` | No compression (`NoopCompressor`) |
| `Zstd` | Zstandard compression (default level 3) |
| `Lz4` | LZ4 compression (speed-optimized) |
| `Rle` | Run-length compression |

### Compression

The `Compressor` trait defines the compression interface:

```rust
pub trait Compressor {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, KcmError>;
    fn decompress(&self, data: &[u8], expected_size: usize) -> Result<Vec<u8>, KcmError>;
}
```

**Constants:**

| Constant | Value | Description |
|----------|-------|-------------|
| `MAX_COMPRESSION_LEVEL` | 22 | Maximum zstd compression level |
| `MIN_COMPRESSION_LEVEL` | 1 | Minimum zstd compression level |
| `MAX_DECOMPRESSED_SIZE` | 256 MB | Maximum allowed decompressed output size |
| `MAX_INPUT_SIZE` | 128 MB | Maximum allowed input size for compression |

**Compressor implementations:**

| Compressor | Level | Speed | Ratio | Use Case |
|-----------|-------|-------|-------|----------|
| `ZstdCompressor` | 1–22 (default 3) | Medium | High | Subject, Object, Confidence, Timestamp, Owner columns |
| `Lz4Compressor` | Default | Fast | Medium | Version column |
| `RleCompressor` | N/A | Fast | High (repeated data) | Predicate, Evidence, Context, Priority columns |
| `NoopCompressor` | N/A | N/A | None | Testing, already-compressed data |

### Dictionary Codec

`DictionaryCodec` provides thread-safe string-to-integer encoding backed by `DictionaryCache`.

```rust
pub struct DictionaryCodec {
    inner: Arc<RwLock<DictionaryCache>>,
}
```

| Operation | Complexity | Description |
|-----------|-----------|-------------|
| `encode(value)` | O(1) amortized | Map string → u32 ID |
| `decode(id)` | O(1) | Map u32 ID → String |
| `decode_ref(id)` | O(1) | Map u32 ID → Arc\<str\> (zero-copy) |
| `lookup(value)` | O(1) | Check if string exists, return ID |
| `lookup_batch(values, results)` | O(n) | Batch lookup for multiple strings |
| `lookup_batch_simd(values, results)` | O(n) | SIMD-optimized batch lookup |
| `len()` | O(1) | Number of entries |
| `clear()` | O(1) | Reset all entries |

**`DictionaryCache` internals:**

- `string_to_id`: `RobinHoodMap<String, u32>` — forward mapping
- `id_to_string`: `Vec<Arc<str>>` — reverse mapping with zero-copy Arc strings
- ID 0 is reserved as the empty string sentinel

### Robin Hood Map

`RobinHoodMap<K, V>` is an open-addressing hash map using Robin Hood probing for O(1) amortized operations.

| Property | Value |
|----------|-------|
| Load factor threshold | 90% |
| Initial capacity | 64 |
| Probing strategy | Robin Hood (swap with longer-probed entries) |
| Hash function | ahash (`AHasher`) |

### WAL (Write-Ahead Log)

`WriteAheadLog` provides append-only journaling with BLAKE3 checksums for crash recovery.

**Entry types:**

| Entry | Size (bytes) | Fields |
|-------|-------------|--------|
| `WALEntry::Insert` | 66 | subject (4), predicate (1), object (4), confidence (8), timestamp (8), context (1), version (4), priority (1), owner (2), checksum (32) |
| `WALEntry::Delete` | 41 | row_id (8), checksum (32), padding (1) |

**Constants:**

| Constant | Value | Description |
|----------|-------|-------------|
| `WAL_INSERT_SIZE` | 66 | Fixed size of insert entry in bytes |
| `WAL_DELETE_SIZE` | 41 | Fixed size of delete entry in bytes |
| `WAL_BUFFER_SIZE` | 65536 | Write buffer size for WAL file I/O |

**Operations:**

| Operation | Description |
|-----------|-------------|
| `new(path)` | Create or open a WAL file |
| `append(entry)` | Append entry with BLAKE3 checksum |
| `read_all()` | Read all entries from WAL |
| `truncate()` | Clear WAL after successful flush |
| `flush()` | Flush WAL buffer to disk |

**WAL State Machine (`WalStateMachine`):**

```
Fresh → Active → Checkpointing → Active
                → Replaying → Active
                → Truncated → Fresh
                → Error
```

| State | Description |
|-------|-------------|
| `Fresh` | Initial state, no entries |
| `Active` | Accepting new entries |
| `Checkpointing` | Flushing data to database file |
| `Replaying` | Recovering from WAL during startup |
| `Truncated` | WAL cleared after successful flush |
| `Error(String)` | Unrecoverable error state |

**Checkpoint triggers:**
- Time-based: every 60 seconds (configurable)
- Count-based: every 10,000 entries (configurable)

### File Format

`DatabaseFile` defines the binary database format.

**Header layout:**

| Offset | Size | Field | Description |
|--------|------|-------|-------------|
| 0 | 5 | `DB_MAGIC` | Magic bytes: `KCMDB` |
| 5 | 1 | `DB_VERSION` | Format version (currently 2) |
| 6 | 8 | Row count | Number of rows (u64 LE) |
| 14 | 1 | Column count | Number of columns (u8, always 10) |
| 15 | 8 | Created timestamp | Creation time (i64 LE nanos) |
| 23 | 8 | Modified timestamp | Last modification time (i64 LE nanos) |

**Column block layout (repeated for each column):**

| Field | Size | Description |
|-------|------|-------------|
| Codec ID | 1 byte | `ColumnCodecId`: None(0), Zstd(1), Lz4(2), Rle(3) |
| Data size | 8 bytes | Compressed data size (u64 LE) |
| Row count | 8 bytes | Number of rows in column (u64 LE) |
| Data | variable | Compressed column data |

**`ColumnCodecId` mapping:**

| ID | Codec |
|----|-------|
| 0 | None |
| 1 | Zstd |
| 2 | Lz4 |
| 3 | Rle |

### Indexing

Four index types accelerate query execution:

**`BitmapIndex`:**

| Property | Description |
|----------|-------------|
| Structure | `Vec<u8>` values + `Vec<Bitmap>` per-value bitmaps |
| Build | O(n) scan of column |
| Lookup | O(log n) binary search on values |
| Range query | O(k) where k = number of distinct values in range |

**`BloomFilter`:**

| Property | Description |
|----------|-------------|
| Structure | Bit vector with k hash functions |
| False positive rate | Configurable (default ~1%) |
| Build | O(n) |
| Lookup | O(k) where k = number of hash functions |

**`ZoneMap`:**

| Property | Description |
|----------|-------------|
| Structure | `Vec<i64>` min/max values per block |
| Block size | Configurable (default 1024 rows) |
| Build | O(n) |
| Range skip | O(number of blocks) |

**`CompositeIndex`:**

| Property | Description |
|----------|-------------|
| Structure | Combines BitmapIndex + ZoneMap + BloomFilter |
| Build | O(n) |
| Lookup | Zone prune → Bloom filter check → Bitmap lookup |

### Backup

`BackupManager` provides full and incremental backup with verification.

| Operation | Description |
|-----------|-------------|
| `create_full_backup(schema)` | Save complete schema to timestamped `.kcm` file, verify, write manifest |
| `create_incremental_backup(schema, last_backup)` | Save only rows added since `last_backup`, write manifest |
| `list_backups()` | List all backup files in backup directory |

**Backup file naming:** `backup_full_{timestamp_ns}.kcm` or `backup_incr_{timestamp_ns}.kcm`

**Backup verification:** Every backup is verified via `DatabaseFile::verify()` immediately after creation.

### Recovery

`RecoveryManager` handles crash recovery via WAL replay and backup fallback.

**Recovery flow:**

```
1. If DB file exists and is valid:
   a. Load database via DatabaseFile::load
   b. If WAL exists → replay WAL entries
   c. Return recovered Schema
2. If DB file is corrupt:
   a. Attempt load from {db_path}.backup
   b. If backup exists → load backup, replay WAL
   c. Copy backup to primary path
3. If only WAL exists (no DB):
   a. Create empty Schema with 1M capacity
   b. Replay all WAL entries
   c. Return recovered Schema
4. If neither exists:
   a. Create fresh Schema with 1M capacity
```

## Architecture

```
┌──────────────────────────────────────────────────────────┐
│                     kcm-storage                          │
├──────────┬──────────┬──────────┬──────────┬─────────────┤
│ column   │ compress │ index    │ wal      │ file_format │
├──────────┼──────────┼──────────┼──────────┼─────────────┤
│ dict_codec│dict_cache│robin_hood│ wal_state│ backup      │
├──────────┴──────────┴──────────┴──────────┴─────────────┤
│                      errors                             │
├─────────────────────────────────────────────────────────┤
│                      recovery                           │
├─────────────────────────────────────────────────────────┤
│  kcm-core (Fact, RowID, Bitmap, Dictionary, DenseVec)   │
└─────────────────────────────────────────────────────────┘

External dependencies:
  log, parking_lot, zstd, lz4, blake3, thiserror, ahash
```

## Internal Components

### column.rs

Defines `Column<T>`, `Schema` (10 physical columns), `ColumnEncoding`, and `CompressionCodec`. Implements delta encoding, RLE encoding, and Gorilla encoding for typed columns.

### compress.rs

Implements the `Compressor` trait with `ZstdCompressor`, `Lz4Compressor`, `RleCompressor`, and `NoopCompressor`. Provides `hash_blake3` and `hash_blake3_hex` utility functions.

### dict_codec.rs

`DictionaryCodec` — thread-safe wrapper around `DictionaryCache` using `Arc<RwLock<DictionaryCache>>`.

### dict_cache.rs

`DictionaryCache` — bidirectional string↔u32 mapping using `RobinHoodMap` (forward) and `Vec<Arc<str>>` (reverse). ID 0 is reserved as empty string sentinel.

### robin_hood.rs

`RobinHoodMap<K, V>` — open-addressing hash map with Robin Hood probing, 90% load factor threshold, and ahash hashing.

### wal.rs

`WriteAheadLog` — append-only WAL with BLAKE3 checksums. Defines `WALEntry::Insert` and `WALEntry::Delete` with fixed sizes.

### wal_state.rs

`WalStateMachine` — manages WAL lifecycle transitions. `WalCheckpoint` records offset, timestamp, and entry count.

### file_format.rs

`DatabaseFile` — binary database format with `DB_MAGIC` (`KCMDB`), `DB_VERSION` (2), header, and column blocks. `ColumnCodecId` maps codec identifiers to compressor implementations.

### index.rs

`BitmapIndex`, `BloomFilter`, `ZoneMap`, `CompositeIndex` — query acceleration structures for filtering and range queries.

### backup.rs

`BackupManager` — full and incremental backup with verification and manifest files.

### recovery.rs

`RecoveryManager` — crash recovery via WAL replay, backup fallback, and fresh-schema creation.

### errors.rs

`StorageError` — crate-specific error types converting to `KcmError` via `From` impl.

## Data Model

### Column Layout

```
Schema (10 columns):
  Subject:   SubjectColumn   (u32)  — Dictionary + Zstd
  Predicate: PredicateColumn (u8)   — Dictionary + RLE
  Object:    ObjectColumn    (u32)  — Dictionary + Zstd
  Confidence:ConfidenceColumn(f64)  — Gorilla + Zstd
  Evidence:  EvidenceColumn  (u8)   — Dictionary + RLE
  Timestamp: TimestampColumn (i64)  — Delta + Zstd
  Context:   ContextColumn   (u8)   — Dictionary + RLE
  Version:   VersionColumn   (i32)  — Delta + LZ4
  Priority:  PriorityColumn  (i8)   — Identity + RLE
  Owner:     OwnerColumn     (u16)  — Dictionary + Zstd
```

### WAL Entry Layout

```
Insert entry (66 bytes):
  [0..4)   SubjectID   u32 LE
  [4..5)   PredicateID u8
  [5..9)   ObjectID    u32 LE
  [9..17)  confidence  f64 LE
  [17..25) timestamp   i64 LE
  [25..26) context     u8
  [26..30) version     i32 LE
  [30..31) priority    i8
  [31..33) owner       u16 LE
  [33..65) checksum    [u8; 32] BLAKE3

Delete entry (41 bytes):
  [0..8)   row_id      u64 LE
  [8..40)  checksum    [u8; 32] BLAKE3
  [40..41) padding     u8
```

## Execution Flow

### WAL Append Flow

```
Caller → WriteAheadLog::append(entry)
  → Serialize entry to fixed-size bytes
  → Compute BLAKE3 checksum over entry data
  → Append checksum to entry bytes
  → Write bytes to WAL file (buffered)
  → Flush buffer if WAL_BUFFER_SIZE reached
  → Update WalStateMachine entry count
  → Check if checkpoint should trigger
```

### File Write Flow

```
Caller → DatabaseFile::save(schema, path)
  → Open file for writing
  → Write DB_MAGIC (5 bytes)
  → Write DB_VERSION (1 byte)
  → Write header (row count, column count, timestamps)
  → For each column:
      → Encode column data (Dictionary/Delta/RLE/Gorilla)
      → Compress encoded data (Zstd/Lz4/Rle/None)
      → Write codec ID + data size + row count + compressed data
  → Close file
```

## Public API

### Key Types

| Type | Module | Description |
|------|--------|-------------|
| `Column<T>` | column | Typed columnar storage |
| `Schema` | column | Collection of 10 physical columns |
| `ColumnEncoding` | column | Encoding strategy enum |
| `CompressionCodec` | column | Compression algorithm enum |
| `Compressor` | compress | Compression trait |
| `ZstdCompressor` | compress | Zstd compression implementation |
| `Lz4Compressor` | compress | LZ4 compression implementation |
| `RleCompressor` | compress | RLE compression implementation |
| `NoopCompressor` | compress | No-op compression implementation |
| `DictionaryCodec` | dict_codec | Thread-safe string↔u32 encoding |
| `DictionaryCache` | dict_cache | Bidirectional dictionary mapping |
| `RobinHoodMap<K, V>` | robin_hood | Open-addressing hash map |
| `WriteAheadLog` | wal | Append-only WAL |
| `WALEntry` | wal | WAL entry enum (Insert/Delete) |
| `WalStateMachine` | wal_state | WAL lifecycle state machine |
| `WalCheckpoint` | wal_state | WAL checkpoint metadata |
| `DatabaseFile` | file_format | Binary database format |
| `ColumnCodecId` | file_format | Codec identifier enum |
| `BitmapIndex` | index | Bitmap-based value index |
| `BloomFilter` | index | Probabilistic membership test |
| `ZoneMap` | index | Min/max block index |
| `CompositeIndex` | index | Combined index structure |
| `BackupManager` | backup | Full/incremental backup |
| `RecoveryManager` | recovery | Crash recovery |
| `StorageError` | errors | Storage-specific error types |

## Configuration

| Parameter | Default | Description |
|-----------|---------|-------------|
| `WAL_BUFFER_SIZE` | 65536 bytes | WAL write buffer size |
| `WAL_INSERT_SIZE` | 66 bytes | Fixed insert entry size |
| `WAL_DELETE_SIZE` | 41 bytes | Fixed delete entry size |
| `DB_MAGIC` | `KCMDB` | Database file magic bytes |
| `DB_VERSION` | 2 | Database format version |
| `MAX_COMPRESSION_LEVEL` | 22 | Maximum zstd level |
| `MIN_COMPRESSION_LEVEL` | 1 | Minimum zstd level |
| `MAX_DECOMPRESSED_SIZE` | 256 MB | Maximum decompressed output |
| `MAX_INPUT_SIZE` | 128 MB | Maximum compression input |
| `LOAD_FACTOR_PERCENT` | 90% | Robin Hood map load factor |
| `INITIAL_CAPACITY` | 64 | Robin Hood map initial capacity |
| `PREFETCH_STRIDE` | 8 | Dictionary cache prefetch stride |
| Checkpoint interval | 60 seconds | WAL checkpoint time trigger |
| Max entries before checkpoint | 10,000 | WAL checkpoint count trigger |

## Dependencies

| Dependency | Type | Justification |
|-----------|------|---------------|
| `kcm-core` | Internal | Core types (`Fact`, `RowID`, `Bitmap`, `Dictionary`, `DenseVec`, `KcmError`) |
| `log` | Runtime | Structured logging for WAL and recovery operations |
| `parking_lot` | Runtime | 3-5x faster RwLock/Mutex than std; used by `DictionaryCodec` |
| `zstd` | Runtime | Zstandard compression codec |
| `lz4` | Runtime | LZ4 compression codec (speed-optimized) |
| `blake3` | Runtime | Fast cryptographic hash for WAL checksums and backup verification |
| `thiserror` | Runtime | Derive macro for `StorageError` |
| `ahash` | Runtime | Fast hash function for `RobinHoodMap` and `DictionaryCache` |

## Error Handling

```rust
#[derive(Error, Debug)]
pub enum StorageError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Compression error: {0}")]
    Compression(String),

    #[error("Corruption detected: {0}")]
    Corrupted(String),

    #[error("Column full: capacity {capacity}, current {current}")]
    ColumnFull { capacity: usize, current: usize },

    #[error("Index out of bounds: {index} >= {len}")]
    IndexOutOfBounds { index: usize, len: usize },

    #[error("Invalid encoding: {0}")]
    InvalidEncoding(String),

    #[error("Hash mismatch: expected {expected}, got {actual}")]
    HashMismatch { expected: String, actual: String },
}
```

All `StorageError` variants convert to `KcmError` via `From` impl:

| StorageError | KcmError |
|-------------|----------|
| `Io(e)` | `KcmError::Io(e.to_string())` |
| `Compression(s)` | `KcmError::Corrupted(s)` |
| `Corrupted(s)` | `KcmError::Corrupted(s)` |
| `ColumnFull` | `KcmError::OutOfMemory` |
| `IndexOutOfBounds` | `KcmError::InvalidArgument(...)` |
| `InvalidEncoding(s)` | `KcmError::InvalidArgument(s)` |
| `HashMismatch` | `KcmError::Corrupted(...)` |

## Performance Characteristics

| Operation | Target | Measurement |
|-----------|--------|-------------|
| WAL append | < 1ms | Criterion micro-benchmark |
| Zstd compress (1 MB) | < 10ms | Criterion benchmark |
| Zstd decompress (1 MB) | < 5ms | Criterion benchmark |
| LZ4 compress (1 MB) | < 2ms | Criterion benchmark |
| LZ4 decompress (1 MB) | < 1ms | Criterion benchmark |
| Dictionary encode | < 100ns | Criterion micro-benchmark |
| Dictionary decode | < 10ns | Criterion micro-benchmark |
| Robin Hood insert | < 200ns | Criterion micro-benchmark |
| Robin Hood lookup | < 100ns | Criterion micro-benchmark |
| BitmapIndex lookup | O(log n) | Theoretical |
| ZoneMap range skip | O(blocks) | Theoretical |
| BloomFilter check | O(k) | Theoretical |
| File save (1M rows) | < 2s | Integration benchmark |
| File load (1M rows) | < 1s | Integration benchmark |
| WAL replay (10K entries) | < 100ms | Integration benchmark |

## Security Considerations

- WAL entries are checksummed with BLAKE3 to detect corruption
- Database header validates `DB_MAGIC` and `DB_VERSION` on every load
- Decompression enforces `MAX_DECOMPRESSED_SIZE` (256 MB) and `MAX_INPUT_SIZE` (128 MB)
- Column capacity overflow returns `StorageError::ColumnFull` (no panic)
- Dictionary overflow returns error (no panic)
- Recovery handles all edge cases: missing files, corrupt data, partial writes
- Backup verification runs after every backup creation
- No `unwrap()` in production code paths
- All public APIs return `Result<T, KcmError>`

## Integration

`kcm-storage` is consumed by the following crates:

```
kcm-storage ← kcm-compute      (columnar scan, filter, join)
kcm-storage ← kcm-reasoning    (inference data storage)
kcm-storage ← kcm-optimizer    (statistics, cost model)
kcm-storage ← kcm-runtime      (KnowledgeDatabase, transactions)
kcm-storage ← kcm-interface    (FFI, REST, KQL parser)
kcm-storage ← kcm-testing      (load, stress, security tests)
```

## Sequence Diagram

### WAL Append

```
Caller → WriteAheadLog::append(entry)
  → Serialize entry bytes
  → blake3::hash(bytes) → checksum
  → Append (bytes || checksum) to buffer
  → If buffer full → flush to disk
  → WalStateMachine::record_entry()
  → If should_checkpoint() → trigger checkpoint
```

### Database Save

```
Caller → DatabaseFile::save(schema, path)
  → File::create(path)
  → write_all(DB_MAGIC)
  → write_all(DB_VERSION)
  → write header (row_count, col_count, timestamps)
  → For each column in schema:
      → encode(column) → encoded_bytes
      → compressor.compress(encoded_bytes) → compressed_bytes
      → write_all(codec_id)
      → write_all(data_size)
      → write_all(row_count)
      → write_all(compressed_bytes)
  → drop(writer) → flush
```

### Recovery

```
RecoveryManager::recover(db_path, wal_path)
  → If db_path exists && size > 32:
      → DatabaseFile::load(db_path)
      → If success && wal_path exists:
          → replay_wal(&mut schema, wal_path)
          → Return Ok(schema)
      → If load fails:
          → recover_from_backup(db_path, wal_path)
  → Else if wal_path exists:
      → Schema::new(1_000_000)
      → replay_wal(&mut schema, wal_path)
      → Return Ok(schema)
  → Else:
      → Schema::new(1_000_000)
```

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                       kcm-storage                           │
├─────────────┬──────────────┬─────────────┬─────────────────┤
│  column.rs  │  compress.rs │  index.rs   │   wal.rs        │
│  Schema     │  Compressor  │  BitmapIdx  │   WALEntry      │
│  Column<T>  │  Zstd/Lz4    │  BloomFltr  │   WriteAheadLog │
│  Encoding   │  Rle/Noop    │  ZoneMap    │   WAL_CONSTS    │
├─────────────┼──────────────┼─────────────┼─────────────────┤
│ dict_codec  │ dict_cache   │robin_hood   │  wal_state.rs   │
│ DictCodec   │ DictCache    │ RobinHoodMap│  WalStateMachine│
├─────────────┴──────────────┴─────────────┴─────────────────┤
│                    file_format.rs                           │
│              DatabaseFile, DB_MAGIC, DB_VERSION             │
├────────────────────────────────────────────────────────────┤
│  backup.rs              │  recovery.rs                     │
│  BackupManager          │  RecoveryManager                 │
├─────────────────────────┴──────────────────────────────────┤
│                      errors.rs                             │
│                   StorageError                             │
├────────────────────────────────────────────────────────────┤
│                    kcm-core                                │
│  Fact, RowID, Bitmap, Dictionary, DenseVec, KcmError       │
└────────────────────────────────────────────────────────────┘
```

## References

- [PRD2.md](../PRD2.md) §15 — Storage format specification
- [PRD.md](../PRD.md) §4 — Storage engine specification
- [SSOT.md](../../SSOT.md) — Single Source of Truth
- [AGENTS.md](../../AGENTS.md) — Engineering constitution
- [KCM_SPECIFICATION.md](../../KCM_SPECIFICATION.md) — Technical constitution

## SSOT Alignment

| SSOT Requirement | Specification | Implementation | Test |
|-----------------|---------------|----------------|------|
| R-STORE-001 | Column storage with 10 physical columns | `column.rs:Schema` | `tests/test_column.rs` |
| R-STORE-002 | Per-column encoding (Dictionary, Delta, RLE, Gorilla) | `column.rs:ColumnEncoding` | `tests/test_column.rs` |
| R-COMPRESS-001 | Zstd/Lz4/RLE/Noop compression | `compress.rs:Compressor` | `tests/test_compress.rs` |
| R-COMPRESS-002 | Size limits (256MB decompressed, 128MB input) | `compress.rs:MAX_*` constants | `tests/test_compress.rs` |
| R-DICT-001 | Dictionary codec with Robin Hood map | `dict_codec.rs`, `robin_hood.rs` | `tests/test_dict.rs` |
| R-DICT-002 | Thread-safe dictionary via Arc\<RwLock\> | `dict_codec.rs:DictionaryCodec` | `tests/test_dict.rs` |
| R-WAL-001 | Append-only WAL with BLAKE3 checksums | `wal.rs:WriteAheadLog` | `tests/test_wal.rs` |
| R-WAL-002 | Fixed entry sizes (INSERT=66, DELETE=41) | `wal.rs:WAL_INSERT_SIZE/DELETE_SIZE` | `tests/test_wal.rs` |
| R-WAL-003 | WAL state machine lifecycle | `wal_state.rs:WalStateMachine` | `tests/test_wal.rs` |
| R-FILE-001 | Binary format with DB_MAGIC and DB_VERSION | `file_format.rs:DatabaseFile` | `tests/test_file_format.rs` |
| R-FILE-002 | Column blocks with codec metadata | `file_format.rs:ColumnCodecId` | `tests/test_file_format.rs` |
| R-INDEX-001 | Bitmap, BloomFilter, ZoneMap, Composite indexes | `index.rs` | `tests/test_index.rs` |
| R-BACKUP-001 | Full and incremental backup | `backup.rs:BackupManager` | `tests/test_backup.rs` |
| R-RECOVERY-001 | WAL replay and backup fallback | `recovery.rs:RecoveryManager` | `tests/test_recovery.rs` |
| R-ERROR-001 | StorageError with From\<StorageError\> for KcmError | `errors.rs:StorageError` | `tests/test_errors.rs` |
