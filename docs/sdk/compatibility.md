# KCM SDK Compatibility Matrix

Official SDK for the KCM Knowledge Columnar Model

<!-- badges:start -->
[![Build Status](https://img.shields.io/badge/build-pending-lightgrey)]()
[![Version](https://img.shields.io/badge/version-1.0.0-blue)]()
[![License](https://img.shields.io/badge/license-MIT-green)]()
<!-- badges:end -->

## Language Versions Supported

| Language | Minimum Version | Recommended Version | Package |
|----------|----------------|---------------------|---------|
| Rust | 1.70.0 | 1.75.0+ | `kcm-core` (crate) |
| C | C99 | C11 | `kcm.h` (FFI) |
| C++ | C++17 | C++20 | `kcm.hpp` (header-only) |
| Python | 3.8 | 3.11+ | `kcm` (PyPI) |
| JavaScript | Node.js 16 | Node.js 20+ | `@kcm/js` (npm) |
| TypeScript | 4.7 | 5.3+ | `@kcm/ts` (npm) |
| Go | 1.21 | 1.22+ | `github.com/kcm/go-sdk` |
| Java | 11 | 17+ | `io.kcm:sdk` (Maven) |
| .NET | 6.0 | 8.0+ | `Kcm.Sdk` (NuGet) |

## KCM Engine Version Compatibility

| SDK Version | Engine Version | Status |
|-------------|---------------|--------|
| 1.0.x | 1.0.x | Stable |
| 1.1.x | 1.1.x | Stable |
| 2.0.x | 2.0.x | Stable |

SDK versions are tied to the KCM engine version. Each SDK release targets a specific engine version.

## Platform Support

| Platform | Rust | C | C++ | Python | JS | TS | Go | Java | .NET |
|----------|------|---|-----|--------|----|----|----|----|------|
| Linux x86_64 | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes |
| Linux aarch64 | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes |
| macOS arm64 | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes |
| macOS x86_64 | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes |
| Windows x64 | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes |
| Windows arm64 | Yes | Yes | Yes | Planned | Planned | Planned | Yes | Yes | Yes |

## Feature Parity Matrix

| Feature | Rust | C | C++ | Python | JS | TS | Go | Java | .NET |
|---------|------|---|-----|--------|----|----|----|----|------|
| Create database | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes |
| Insert fact | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes |
| Update fact | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes |
| Delete fact | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes |
| Query (KQL) | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes |
| Fact count | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes |
| Active count | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes |
| Transactions | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes |
| Save to file | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes |
| Load from file | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes |
| Integrity verify | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes |
| Context manager | N/A | N/A | N/A | Yes | N/A | N/A | N/A | Yes (try-with) | Yes (IDisposable) |
| Iterator/for-each | Yes | N/A | Yes | Yes | Yes | Yes | Yes | Yes | Yes (IEnumerable) |
| Error codes | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes |
| Exceptions | N/A | N/A | Yes | Yes | Yes | Yes | N/A | Yes | Yes |
| RAII/resource mgmt | Yes | Manual | Yes | Yes | Manual | Manual | Yes | Yes (try-with) | Yes (IDisposable) |

## API Surface Summary

All SDKs expose the same 14 core operations:

| Operation | C FFI | C++ | Python | JS/TS | Go | Java | .NET | Rust |
|-----------|-------|-----|--------|-------|----|----|------|------|
| Database creation | `KCM_DatabaseNew` | `kcm::Database()` | `kcm.Database()` | `new Database()` | `kcm.NewDatabase()` | `new KcmDatabase()` | `new KcmDatabase()` | `KnowledgeDatabase::new()` |
| Insert | `KCM_DatabaseInsert` | `db.insert()` | `db.insert()` | `db.insert()` | `db.Insert()` | `db.insert()` | `db.Insert()` | `db.insert()` |
| Update | `KCM_DatabaseUpdate` | `db.update()` | `db.update()` | `db.update()` | `db.Update()` | `db.update()` | `db.Update()` | `db.update()` |
| Delete | `KCM_DatabaseDelete` | `db.remove()` | `db.delete()` | `db.delete()` | `db.Delete()` | `db.delete()` | `db.Delete()` | `db.delete()` |
| Query | `KCM_DatabaseQuery` | `db.query()` | `db.query()` | `db.query()` | `db.Query()` | `db.query()` | `db.Query()` | `db.query()` |
| Fact count | `KCM_DatabaseFactCount` | `db.fact_count()` | `db.fact_count()` | `db.factCount()` | `db.FactCount()` | `db.factCount()` | `db.FactCount()` | `db.fact_count()` |
| Active count | `KCM_DatabaseActiveCount` | `db.active_count()` | `db.active_count()` | `db.activeCount()` | `db.ActiveCount()` | `db.activeCount()` | `db.ActiveCount()` | `db.active_count()` |
| Begin transaction | `KCM_DatabaseBeginTransaction` | `db.begin_transaction()` | `db.begin_transaction()` | `db.beginTransaction()` | `db.BeginTransaction()` | `db.beginTransaction()` | `db.BeginTransaction()` | `db.begin_transaction()` |
| Commit | `KCM_TransactionCommit` | `txn.commit()` | `txn.commit()` | `txn.commit()` | `txn.Commit()` | `txn.commit()` | `txn.Commit()` | `txn.commit()` |
| Rollback | `KCM_TransactionRollback` | `txn.rollback()` | `txn.rollback()` | `txn.rollback()` | `txn.Rollback()` | `txn.rollback()` | `txn.Rollback()` | `txn.rollback()` |
| Save | `KCM_DatabaseSave` | `db.save()` | `db.save()` | `db.save()` | `db.Save()` | `db.save()` | `db.Save()` | `db.save()` |
| Load | `KCM_DatabaseLoad` | `db.load()` | `db.load()` | `db.load()` | `db.Load()` | `db.load()` | `db.Load()` | `db.load()` |
| Verify | `KCM_DatabaseVerify` | `Database::verify()` | `Database.verify()` | `Database.verify()` | `db.Verify()` | `KcmDatabase.verify()` | `KcmDatabase.Verify()` | `KnowledgeDatabase::verify()` |
| Close/Free | `KCM_DatabaseFree` | Destructor | `db.close()` | `db.close()` | `db.Close()` | `db.close()` | `db.Dispose()` | Drop |

## Error Code Mapping

| KcmError | C FFI | Python | JS/TS | Go | Java | .NET | HTTP | gRPC |
|----------|-------|--------|-------|----|------|------|------|------|
| NotFound | `KCM_ERR_NOT_FOUND` | `ErrorCode.NOT_FOUND` | `ErrorCode.NOT_FOUND` | `ErrNotFound` | `NOT_FOUND` | `NotFound` | 404 | NOT_FOUND |
| OutOfMemory | `KCM_ERR_OUT_OF_MEMORY` | `ErrorCode.OUT_OF_MEMORY` | `ErrorCode.OUT_OF_MEMORY` | `ErrOutOfMemory` | `OUT_OF_MEMORY` | `OutOfMemory` | 507 | RESOURCE_EXHAUSTED |
| InvalidArgument | `KCM_ERR_INVALID_ARGUMENT` | `ErrorCode.INVALID_ARGUMENT` | `ErrorCode.INVALID_ARGUMENT` | `ErrInvalidArgument` | `INVALID_ARGUMENT` | `InvalidArgument` | 400 | INVALID_ARGUMENT |
| Io | `KCM_ERR_IO` | `ErrorCode.IO` | `ErrorCode.IO` | `ErrIo` | `IO` | `Io` | 500 | INTERNAL |
| Corrupted | `KCM_ERR_CORRUPTED` | `ErrorCode.CORRUPTED` | `ErrorCode.CORRUPTED` | `ErrCorrupted` | `CORRUPTED` | `Corrupted` | 500 | DATA_LOSS |
| Conflict | `KCM_ERR_CONFLICT` | `ErrorCode.CONFLICT` | `ErrorCode.CONFLICT` | `ErrConflict` | `CONFLICT` | `Conflict` | 409 | ALREADY_EXISTS |
| TransactionAborted | `KCM_ERR_TRANSACTION_ABORTED` | `ErrorCode.TRANSACTION_ABORTED` | `ErrorCode.TRANSACTION_ABORTED` | `ErrTransactionAborted` | `TRANSACTION_ABORTED` | `TransactionAborted` | 409 | ABORTED |

## Deprecation Policy

| Step | Timeline | Action |
|------|----------|--------|
| 1. Announce | Release N | Mark as deprecated in docs |
| 2. Warn | Release N+1 | Add runtime warnings |
| 3. Remove | Release N+2 | Remove from codebase |

## LTS Versions

| Version | Support Level | End of Life |
|---------|--------------|-------------|
| 1.0.x | Full support | 2030-06 |
| 1.1.x | Full support | 2030-09 |
| 2.0.x | Full support | 2031-06 |

- Security patches for 3 years after EOL
- Bug fixes for 2 years after EOL
- No feature additions after EOL
- Migration guides provided for major versions
