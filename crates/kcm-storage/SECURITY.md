# kcm-storage Security Policy

Security considerations specific to the `kcm-storage` crate.

> For project-wide security policies, refer to the [SECURITY.md](../../SECURITY.md) located in the repository root.

## Overview

`kcm-storage` is the columnar storage engine for KCM. It manages persistent data including WAL files, database files, and backup files — all of which contain the complete knowledge dataset. A security vulnerability in this crate directly exposes the entire knowledge base to data corruption, unauthorized access, or data loss.

## Security Scope

| Asset | Risk Level | Description |
|-------|-----------|-------------|
| WAL files | Critical | Append-only journal containing all insert/delete operations before flush |
| Database files | Critical | Persistent columnar storage — corruption causes permanent data loss |
| Backup files | High | Full/incremental snapshots containing all knowledge data |
| Column data | High | Per-column encoded and compressed physical storage |
| Dictionary codec | Medium | String-to-integer mapping — incorrect encoding causes data corruption |
| Index structures | Medium | Bitmap, BloomFilter, ZoneMap, Composite — incorrect indexes cause query errors |
| Compression layer | Medium | zstd/lz4/RLE — decompression bombs or malformed data can cause OOM |
| Robin Hood map | Medium | Hash map used by dictionary cache — collision handling correctness matters |
| Recovery manager | High | WAL replay and backup recovery — incorrect replay loses data |

## Threat Model

| Threat | Vector | Mitigation |
|--------|--------|------------|
| WAL corruption | Crash during write, disk failure | WAL entries use BLAKE3 checksums; replay validates each entry |
| Database file tampering | Unauthorized file modification | DB header contains magic bytes (`KCMDB`) and version; backup verification via `DatabaseFile::verify` |
| Decompression bomb | Malformed compressed data | `MAX_DECOMPRESSED_SIZE` (256 MB) and `MAX_INPUT_SIZE` (128 MB) limits enforced |
| Integer overflow in column indexing | Exceeding column capacity | `ColumnFull` error returned when capacity reached |
| Dictionary ID overflow | Exceeding `u32::MAX` entries | `DictionaryCache` returns error on overflow |
| Backup data exposure | Unencrypted backup files | Backup files contain all knowledge data — encryption must be applied at the `kcm-security` layer |
| Recovery replay failure | Partially written WAL entries | WAL entry sizes are fixed (`WAL_INSERT_SIZE=66`, `WAL_DELETE_SIZE=41`); partial entries are discarded |
| Hash collision in Robin Hood map | Degraded performance | Load factor capped at 90%; automatic rehashing on threshold |
| Index inconsistency | Stale or missing index entries | Indexes rebuilt on database load; `CompositeIndex` merges bitmap + zone + bloom |

## Security Risks

1. **WAL integrity** — The WAL is the primary recovery mechanism. Any corruption here can cause data loss or duplication during recovery.
2. **File format validation** — The binary database format must be validated on every load to prevent exploitation of malformed headers.
3. **Compression safety** — Both zstd and lz4 decompression must enforce size limits to prevent denial-of-service via decompression bombs.
4. **Backup confidentiality** — Backup files contain the complete knowledge dataset. Without encryption at the `kcm-security` layer, backups are plaintext.
5. **Dictionary consistency** — The dictionary codec is shared across columns. Incorrect synchronization can cause encoding/decoding mismatches.

## Access Control

`kcm-storage` has no built-in access control. All types are public within the crate. Access control is enforced by downstream crates (`kcm-security`, `kcm-runtime`).

## RBAC Integration

Not applicable — `kcm-storage` is a storage library with no authentication or authorization logic. RBAC enforcement is handled by `kcm-security`.

## Sensitive Assets

- **WAL files** — Contain all insert/delete operations in chronological order. Exposes the complete write history of the knowledge base.
- **Database files** — Contain all columnar data. The complete knowledge dataset is stored in these files.
- **Backup files** — Full and incremental snapshots containing all knowledge data. Backup manifests reference base backups and include row counts.

## Secret Management

No secrets are stored or managed in `kcm-storage`. The crate has no networking or authentication logic. Encryption keys are managed by `kcm-security`.

## Secure Development Rules

1. **WAL integrity** — Every WAL entry must be validated with BLAKE3 checksums during replay. Partial entries must be discarded silently.
2. **File format validation** — `DatabaseFile::load` must verify the `DB_MAGIC` bytes and `DB_VERSION` before parsing any header fields.
3. **Compression safety** — All decompression paths must enforce `MAX_DECOMPRESSED_SIZE` (256 MB) and reject inputs exceeding `MAX_INPUT_SIZE` (128 MB).
4. **Recovery correctness** — `RecoveryManager::recover` must handle all edge cases: missing DB, missing WAL, corrupt DB, corrupt WAL, and backup fallback.
5. **No `unwrap()` in production code** — All error paths must return `Result<T, KcmError>` or `Result<T, StorageError>`.
6. **Result return** — All public APIs must return `Result<T, KcmError>`. No panics, no fake success responses.

## Audit Logging

`kcm-storage` does not perform audit logging. Audit logging is handled by `kcm-security`. Storage operations that require auditing (inserts, deletes, backups) must be audit-logged by the calling layer.

## Validation Checklist

- [ ] WAL entries are checksummed with BLAKE3
- [ ] Database header validates `DB_MAGIC` and `DB_VERSION`
- [ ] Decompression enforces size limits (`MAX_DECOMPRESSED_SIZE`, `MAX_INPUT_SIZE`)
- [ ] Column capacity overflow returns `StorageError::ColumnFull`
- [ ] Dictionary overflow returns error (not panic)
- [ ] Recovery handles all edge cases (missing files, corrupt data)
- [ ] Backup verification runs after every backup creation
- [ ] No `unwrap()` in production code paths
- [ ] No `panic!()` in production code paths
- [ ] All public APIs return `Result<T, KcmError>`
- [ ] Robin Hood map load factor stays below 90%
- [ ] WAL state machine transitions are validated

## References

- [SECURITY.md](../../SECURITY.md) — Project-wide security policy
- [AGENTS.md](../../AGENTS.md) — Engineering constitution
- [SSOT.md](../../SSOT.md) — Single Source of Truth
- [docs/kcm-storage/spesifikasi.md](../../docs/kcm-storage/spesifikasi.md) — Technical specification
- [docs/PRD2.md](../../docs/PRD2.md) §15 — Storage format specification
