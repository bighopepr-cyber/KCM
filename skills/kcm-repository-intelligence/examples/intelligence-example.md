# Repository Intelligence Example

## Task: Add new codec to kcm-storage

### Analysis

#### Crate Mapping
- **Target:** kcm-storage
- **Responsibility:** Columnar storage engine
- **Dependencies:** kcm-core, log, parking_lot, zstd, lz4, blake3, thiserror

#### Module Ownership
- **compress.rs:** Compression codecs (zstd, lz4, RLE, Noop)
- **column.rs:** Column storage
- **wal.rs:** Write-ahead log

#### Existing Implementations
- ZstdCompressor in compress.rs
- Lz4Compressor in compress.rs
- RleCompressor in compress.rs
- NoopCompressor in compress.rs

#### Test Locations
- crates/kcm-storage/tests/
- crates/kcm-storage/src/lib.rs (inline tests)

### Recommendation
- Add NewCompressor to compress.rs following existing pattern
- Add tests in tests/test_codec_property.rs
- Add benchmark in benches/