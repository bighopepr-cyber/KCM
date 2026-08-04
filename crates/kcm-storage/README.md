# kcm-storage

Columnar storage engine for KCM. Handles persistence, compression, indexing, WAL, backup, and recovery.

## Purpose

Provides the on-disk storage layer for KCM's columnar knowledge model. Supports multiple compression codecs, write-ahead logging for crash recovery, B-tree indexing, and backup/restore.

## Modules

| Module | Purpose |
|--------|---------|
| `column` | Per-column storage with encoding and compression |
| `compress` | Compression dispatcher (Zstd, LZ4) |
| `dict_codec` | Dictionary-encoded column codec |
| `file_format` | Binary file format with headers and block layout |
| `wal` | Write-ahead log for crash recovery |
| `wal_state` | WAL state tracking |
| `index` | B-tree index for fast lookups |
| `backup` | Full and incremental backup |
| `recovery` | WAL replay and crash recovery |
| `errors` | `StorageError` type |

## Dependencies

| Dependency | Purpose |
|------------|---------|
| `kcm-core` | Core types |
| `zstd` | Compression (column data) |
| `lz4` | Compression (version column) |
| `blake3` | Checksums for data integrity |
| `thiserror` | Error derive macros |
| `log` | Logging |

## Storage Model

Each column is stored with independent encoding and compression:

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

## Usage

```rust
use kcm_storage::column::ColumnStore;
use kcm_storage::wal::WriteAheadLog;
use kcm_storage::file_format::FileFormat;

let mut store = ColumnStore::new("/data/kcm.db")?;
store.write_column(&data)?;
let recovered = store.read_column()?;
```

## WAL Protocol

1. Write entry to WAL
2. Write data to column store
3. Mark WAL entry as committed
4. On crash: replay uncommitted WAL entries in order

## Error Model

`StorageError` converts to `KcmError` via `From` impl.
