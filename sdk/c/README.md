# KCM C SDK

Official SDK for the KCM Knowledge Columnar Model.

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![Version](https://img.shields.io/badge/version-1.0.0-blue)]()
[![License](https://img.shields.io/badge/license-MIT-green)]()
[![KCM Engine Compatible](https://img.shields.io/badge/KCM%20Engine-1.0.0-orange)]()

## Installation

```bash
cd sdk/c && make
```

Or using the build script:

```bash
bash sdk/c/build.sh
```

Requires `libkcm` built from the `kcm-interface` crate with `--release`.

## Quickstart

```c
#include <kcm.h>
#include <stdio.h>

int main() {
    KCM_Database *db = NULL;
    KCM_DatabaseNew(&db);

    KCM_Fact fact = { .subject = 1, .predicate = 0, .object = 2, .confidence = 0.95 };
    KCM_DatabaseInsert(db, &fact);

    printf("Fact count: %lu\n", KCM_DatabaseFactCount(db));

    KCM_DatabaseFree(db);
    return 0;
}
```

## API Reference

All 18 FFI functions defined in `kcm-interface/src/lib.rs`:

| Operation | Signature | Description |
|-----------|-----------|-------------|
| `KCM_DatabaseNew` | `(KCM_Database **db_out) -> KCM_Error` | Create a new database |
| `KCM_DatabaseFree` | `(KCM_Database *db) -> void` | Free database resources |
| `KCM_DatabaseInsert` | `(KCM_Database *db, const KCM_Fact *fact) -> KCM_Error` | Insert a fact |
| `KCM_DatabaseUpdate` | `(KCM_Database *db, uint64_t row_id, const KCM_Fact *fact) -> KCM_Error` | Update a fact |
| `KCM_DatabaseDelete` | `(KCM_Database *db, uint64_t row_id) -> KCM_Error` | Delete a fact |
| `KCM_DatabaseFactCount` | `(KCM_Database *db) -> uint64_t` | Get total fact count |
| `KCM_DatabaseActiveCount` | `(KCM_Database *db) -> uint64_t` | Get active fact count |
| `KCM_DatabaseQuery` | `(KCM_Database *db, const char *query) -> KCM_Query*` | Execute KQL query |
| `KCM_QueryNext` | `(KCM_Query *query) -> KCM_Fact*` | Get next query result |
| `KCM_QueryFree` | `(KCM_Query *query) -> void` | Free query resources |
| `KCM_DatabaseBeginTransaction` | `(KCM_Database *db) -> KCM_Transaction*` | Begin transaction |
| `KCM_TransactionCommit` | `(KCM_Transaction *txn) -> KCM_Error` | Commit transaction |
| `KCM_TransactionRollback` | `(KCM_Transaction *txn) -> KCM_Error` | Rollback transaction |
| `KCM_TransactionFree` | `(KCM_Transaction *txn) -> void` | Free transaction resources |
| `KCM_DatabaseSave` | `(KCM_Database *db, const char *path) -> KCM_Error` | Save to file |
| `KCM_DatabaseLoad` | `(KCM_Database *db, const char *path) -> KCM_Error` | Load from file |
| `KCM_DatabaseVerify` | `(const char *path) -> KCM_Error` | Verify file integrity |
| `KCM_ErrorMessage` | `(KCM_Error err) -> const char*` | Get error description |

## Error Handling

All functions return `KCM_Error`. Use `KCM_ErrorMessage()` to get human-readable descriptions:

| Error | Code | Description |
|-------|------|-------------|
| `KCM_OK` | 0 | Success |
| `KCM_NOT_FOUND` | 1 | Resource not found |
| `KCM_OUT_OF_MEMORY` | 2 | Insufficient memory |
| `KCM_INVALID_ARGUMENT` | 3 | Invalid argument |
| `KCM_IO` | 4 | I/O error |
| `KCM_CORRUPTED` | 5 | Data corruption |
| `KCM_CONFLICT` | 6 | Concurrent conflict |
| `KCM_TRANSACTION_ABORTED` | 7 | Transaction aborted |

## Use Cases

### Basic Query

```c
#include <kcm.h>

int main() {
    KCM_Database *db = NULL;
    KCM_DatabaseNew(&db);

    KCM_Fact f1 = { .subject = 1, .predicate = 0, .object = 2, .confidence = 0.95 };
    KCM_Fact f2 = { .subject = 2, .predicate = 1, .object = 3, .confidence = 0.90 };
    KCM_DatabaseInsert(db, &f1);
    KCM_DatabaseInsert(db, &f2);

    KCM_Query *q = KCM_DatabaseQuery(db, "SELECT * FROM facts");
    KCM_Fact *fact;
    while ((fact = KCM_QueryNext(q)) != NULL) {
        printf("Subject: %u, Object: %u\n", fact->subject, fact->object);
    }
    KCM_QueryFree(q);
    KCM_DatabaseFree(db);
    return 0;
}
```

### API Integration

```c
#include <kcm.h>

void export_json(KCM_Database *db) {
    uint64_t count = KCM_DatabaseFactCount(db);
    printf("{\"fact_count\": %lu}\n", count);
}
```

### Transaction

```c
#include <kcm.h>

int main() {
    KCM_Database *db = NULL;
    KCM_DatabaseNew(&db);

    KCM_Transaction *txn = KCM_DatabaseBeginTransaction(db);
    KCM_Fact f1 = { .subject = 10, .predicate = 0, .object = 20, .confidence = 0.85 };
    KCM_DatabaseInsert(db, &f1);

    if (KCM_DatabaseActiveCount(db) > 0) {
        KCM_TransactionCommit(txn);
    } else {
        KCM_TransactionRollback(txn);
    }
    KCM_TransactionFree(txn);
    KCM_DatabaseFree(db);
    return 0;
}
```

## Build

```bash
make          # Build example and tests
make test     # Build and run tests
make clean    # Remove build artifacts
```

## Full Documentation

See [docs/sdk/c.md](../../docs/sdk/c.md) for the complete reference.

## Examples

See [examples/](examples/) for more examples including:
- `basic/basic.c` — Getting started
- `basic_crud/01_basic_crud.c` — CRUD operations
- `basic_crud/02_transactions.c` — Transaction management
- `basic_crud/03_persistence.c` — Save/load databases
- `basic_crud/04_query_patterns.c` — KQL query patterns
- `basic_crud/05_error_handling.c` — Error handling patterns

## License

MIT

Made by bighopepr-cyber
