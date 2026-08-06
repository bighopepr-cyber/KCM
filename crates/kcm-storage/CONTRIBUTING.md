# Contributing to kcm-storage

Contribution guidelines specific to the `kcm-storage` crate.

> For core engine contribution rules, refer to the repository [CONTRIBUTING.md](../../CONTRIBUTING.md).

## Overview

`kcm-storage` is the columnar storage engine for KCM. It provides column storage, compression (zstd, lz4, RLE), dictionary codec, WAL (Write-Ahead Log), file format, indexing (Bitmap, BloomFilter, ZoneMap, Composite), backup/recovery, and Robin Hood map. Changes here affect data persistence, crash recovery, and backup integrity.

## Before Contributing

1. Read the [root CONTRIBUTING.md](../../CONTRIBUTING.md)
2. Read the [kcm-storage technical specification](../../docs/kcm-storage/spesifikasi.md)
3. Verify your change does not break the public API without an SSOT-approved reason
4. Check [existing issues](https://github.com/bighopepr-cyber/KCM/issues) for related work
5. Understand the dependency direction: `kcm-core` → `kcm-storage` → downstream crates

## Coding Standards

### Rust Requirements

- Edition 2024
- All public APIs return `Result<T, KcmError>`
- No `unwrap()` in production code
- No `panic!()` in production code
- No `TODO`/`FIXME`/`HACK` markers
- Use `parking_lot` for synchronization (not `std`)
- Use `Send + Sync` bounds on all shared types

### Type Design Rules

- `Column<T>` must validate capacity at construction
- `Schema` must enforce 10 physical columns
- `CompressionCodec` must handle all codec variants
- `StorageError` must cover all failure modes with descriptive messages
- All public types must implement `Debug`

### Naming Conventions

| Element | Convention | Example |
|---------|-----------|---------|
| Types | PascalCase | `DatabaseFile`, `WriteAheadLog` |
| Functions | snake_case | `compress`, `decompress`, `replay_wal` |
| Constants | SCREAMING_SNAKE_CASE | `WAL_INSERT_SIZE`, `DB_MAGIC` |
| Modules | snake_case | `file_format`, `dict_codec` |

## Module Architecture Rules

- `kcm-storage` depends **only** on `kcm-core` (no upward dependencies)
- No dependencies on `kcm-compute`, `kcm-reasoning`, `kcm-optimizer`, `kcm-runtime`, or `kcm-interface`
- All modules must be declared in `lib.rs`
- Internal module boundaries must be respected (e.g., `compress` does not depend on `index`)
- No circular dependencies between internal modules

## Documentation Rules

- Every public function must have a `///` doc comment
- Every public type must have a `///` doc comment
- Doc comments must include at least one code example for public APIs
- Module-level documentation must explain the module's purpose
- WAL entry sizes (`WAL_INSERT_SIZE`, `WAL_DELETE_SIZE`) must be documented with their byte layouts
- File format constants (`DB_MAGIC`, `DB_VERSION`) must be documented

## Testing Requirements

- Property tests for compression codecs (roundtrip, idempotency)
- Property tests for dictionary codec (encode/decode consistency)
- WAL roundtrip tests (append, read back, verify integrity)
- File format tests (save, load, verify magic bytes and version)
- Index tests (BitmapIndex, ZoneMap, BloomFilter, CompositeIndex)
- Backup/recovery tests (full backup, incremental backup, recovery from backup)
- Robin Hood map tests (insert, lookup, delete, load factor)
- Run: `cargo test -p kcm-storage`

## Performance Rules

- Compression ratio targets: zstd > 2.0x, lz4 > 1.5x for typical columnar data
- WAL append latency target: < 1ms per entry
- Dictionary encode/decode must be O(1) amortized
- ZoneMap lookup must be O(number of blocks)
- BitmapIndex lookup must be O(log n) for value search
- Benchmark regressions > 5% require justification
- Decompression must enforce size limits to prevent OOM

## Review Checklist

- [ ] All public APIs return `Result<T, KcmError>`
- [ ] No `unwrap()` in production code
- [ ] No `panic!()` in production code
- [ ] All types have `Debug` implementations
- [ ] All public functions have doc comments
- [ ] WAL integrity is maintained (BLAKE3 checksums)
- [ ] File format validation is correct (`DB_MAGIC`, `DB_VERSION`)
- [ ] Compression size limits are enforced
- [ ] All tests pass
- [ ] No clippy warnings
- [ ] SSOT traceability documented

## Pull Request Requirements

- Reference the SSOT requirement being addressed
- Include test coverage for new/changed APIs
- Include benchmarks if performance-sensitive (especially WAL and compression)
- Do not break backward compatibility without SSOT approval
- Document any changes to WAL entry format or file format

## References

- [CONTRIBUTING.md](../../CONTRIBUTING.md) — Repository-wide contribution guidelines
- [CODE_OF_CONDUCT.md](../../CODE_OF_CONDUCT.md) — Community guidelines
- [docs/kcm-storage/spesifikasi.md](../../docs/kcm-storage/spesifikasi.md) — Technical specification
- [AGENTS.md](../../AGENTS.md) — Engineering constitution
