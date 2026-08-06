# KCM C SDK

Official SDK for the KCM Knowledge Columnar Model

<!-- badges:start -->
[![Build Status](https://img.shields.io/badge/build-pending-lightgrey)]()
[![Version](https://img.shields.io/badge/version-1.0.0-blue)]()
[![License](https://img.shields.io/badge/license-MIT-green)]()
<!-- badges:end -->

The C SDK provides direct access to all 18 FFI functions implemented in `kcm-interface`. It uses opaque types and explicit resource management.

## Installation

### vcpkg

```bash
vcpkg install kcm
```

### Conan

```ini
# conanfile.txt
[requires]
kcm/1.0.0
```

```bash
conan install .
```

### Manual

Build from source:

```bash
cargo build --release -p kcm-interface
# Headers: sdk/c/kcm.h
# Library: target/release/libkcm.so (or kcm.lib on Windows)
```

## Quickstart

```c
#include "kcm.h"
#include <stdio.h>

int main(void) {
    KCM_Database *db = NULL;
    KCM_Error err = KCM_DatabaseNew(&db);
    if (err != KCM_OK) {
        fprintf(stderr, "Failed to create database: %s\n", KCM_ErrorMessage(err));
        return 1;
    }

    KCM_Fact fact = {
        .subject = 1,
        .predicate = 2,
        .object = 3,
        .confidence = 0.95,
        .evidence = 1,
        .timestamp = 0,
        .context = 1,
        .version = 1,
        .priority = 0,
        .owner = 1,
    };

    err = KCM_DatabaseInsert(db, &fact);
    if (err != KCM_OK) {
        fprintf(stderr, "Insert failed: %s\n", KCM_ErrorMessage(err));
    }

    printf("Total facts: %lu\n", (unsigned long)KCM_DatabaseFactCount(db));
    printf("Active facts: %lu\n", (unsigned long)KCM_DatabaseActiveCount(db));

    KCM_DatabaseFree(db);
    return 0;
}
```

Compile:

```bash
gcc -o kcm_example kcm_example.c -L/path/to/lib -lkcm
```

## API Reference

### Type Definitions

```c
typedef enum {
    KCM_OK = 0,
    KCM_ERR_NOT_FOUND = 1,
    KCM_ERR_OUT_OF_MEMORY = 2,
    KCM_ERR_INVALID_ARGUMENT = 3,
    KCM_ERR_IO = 4,
    KCM_ERR_CORRUPTED = 5,
    KCM_ERR_CONFLICT = 6,
    KCM_ERR_TRANSACTION_ABORTED = 7,
} KCM_Error;

typedef struct KCM_Database KCM_Database;
typedef struct KCM_Query KCM_Query;
typedef struct KCM_Transaction KCM_Transaction;

typedef struct {
    uint32_t subject;
    uint8_t  predicate;
    uint32_t object;
    double   confidence;
    uint8_t  evidence;
    int64_t  timestamp;
    uint8_t  context;
    int32_t  version;
    int8_t   priority;
    uint16_t owner;
} KCM_Fact;
```

### Database Lifecycle

#### `KCM_DatabaseNew`

Create a new in-memory database.

```c
KCM_Error KCM_DatabaseNew(KCM_Database **db_out);
```

| Parameter | Direction | Description |
|-----------|-----------|-------------|
| `db_out` | out | Pointer to receive the new database handle |

| Return | Description |
|--------|-------------|
| `KCM_OK` | Success |

#### `KCM_DatabaseFree`

Free a database and release all resources.

```c
void KCM_DatabaseFree(KCM_Database *db);
```

Safe to call with `NULL`.

### CRUD Operations

#### `KCM_DatabaseInsert`

Insert a fact into the database.

```c
KCM_Error KCM_DatabaseInsert(KCM_Database *db, const KCM_Fact *fact);
```

| Parameter | Description |
|-----------|-------------|
| `db` | Database handle |
| `fact` | Pointer to the fact to insert |

| Return | Description |
|--------|-------------|
| `KCM_OK` | Success |
| `KCM_ERR_OUT_OF_MEMORY` | Memory allocation failed |
| `KCM_ERR_IO` | I/O error |

#### `KCM_DatabaseUpdate`

Update an existing fact by row ID.

```c
KCM_Error KCM_DatabaseUpdate(KCM_Database *db, uint64_t row_id, const KCM_Fact *fact);
```

| Parameter | Description |
|-----------|-------------|
| `db` | Database handle |
| `row_id` | Row ID of the fact to update |
| `fact` | New fact data |

| Return | Description |
|--------|-------------|
| `KCM_OK` | Success |
| `KCM_ERR_NOT_FOUND` | Row ID does not exist |

#### `KCM_DatabaseDelete`

Delete a fact by row ID.

```c
KCM_Error KCM_DatabaseDelete(KCM_Database *db, uint64_t row_id);
```

| Parameter | Description |
|-----------|-------------|
| `db` | Database handle |
| `row_id` | Row ID of the fact to delete |

| Return | Description |
|--------|-------------|
| `KCM_OK` | Success |
| `KCM_ERR_NOT_FOUND` | Row ID does not exist |

### Counts

#### `KCM_DatabaseFactCount`

Get total fact count (including deleted/tombstoned facts).

```c
uint64_t KCM_DatabaseFactCount(KCM_Database *db);
```

#### `KCM_DatabaseActiveCount`

Get active (non-deleted) fact count.

```c
uint64_t KCM_DatabaseActiveCount(KCM_Database *db);
```

### Query

#### `KCM_DatabaseQuery`

Execute a KQL query string and return a query result handle.

```c
KCM_Query *KCM_DatabaseQuery(KCM_Database *db, const char *query);
```

| Parameter | Description |
|-----------|-------------|
| `db` | Database handle |
| `query` | Null-terminated KQL query string |

| Return | Query handle, or `NULL` on error. Must be freed with `KCM_QueryFree`. |

#### `KCM_QueryNext`

Get the next fact from a query result iterator.

```c
KCM_Fact *KCM_QueryNext(KCM_Query *query);
```

| Return | Pointer to the next fact, or `NULL` if no more results. |

#### `KCM_QueryFree`

Free a query result handle.

```c
void KCM_QueryFree(KCM_Query *query);
```

Safe to call with `NULL`.

### Transactions

#### `KCM_DatabaseBeginTransaction`

Begin a new transaction.

```c
KCM_Transaction *KCM_DatabaseBeginTransaction(KCM_Database *db);
```

| Return | Transaction handle. Must be freed with `KCM_TransactionFree`. |

#### `KCM_TransactionCommit`

Commit a transaction.

```c
KCM_Error KCM_TransactionCommit(KCM_Transaction *txn);
```

| Return | Description |
|--------|-------------|
| `KCM_OK` | Success |
| `KCM_ERR_TRANSACTION_ABORTED` | Transaction was aborted |

#### `KCM_TransactionRollback`

Rollback a transaction.

```c
KCM_Error KCM_TransactionRollback(KCM_Transaction *txn);
```

#### `KCM_TransactionFree`

Free a transaction handle.

```c
void KCM_TransactionFree(KCM_Transaction *txn);
```

Safe to call with `NULL`.

### Persistence

#### `KCM_DatabaseSave`

Save the database to a file.

```c
KCM_Error KCM_DatabaseSave(KCM_Database *db, const char *path);
```

| Return | Description |
|--------|-------------|
| `KCM_OK` | Success |
| `KCM_ERR_IO` | I/O error |

#### `KCM_DatabaseLoad`

Load a database from a file into an existing database handle.

```c
KCM_Error KCM_DatabaseLoad(KCM_Database *db, const char *path);
```

| Return | Description |
|--------|-------------|
| `KCM_OK` | Success |
| `KCM_ERR_IO` | I/O error |
| `KCM_ERR_CORRUPTED` | File format invalid |

#### `KCM_DatabaseVerify`

Verify database file integrity.

```c
KCM_Error KCM_DatabaseVerify(const char *path);
```

| Return | Description |
|--------|-------------|
| `KCM_OK` | File is valid |
| `KCM_ERR_CORRUPTED` | File is corrupted |

### Error Handling

#### `KCM_ErrorMessage`

Get a human-readable error message for an error code.

```c
const char *KCM_ErrorMessage(KCM_Error err);
```

| Return | Static string describing the error. Never returns `NULL`. |

## Error Handling

Every function that can fail returns a `KCM_Error` code. Always check return values:

```c
KCM_Error err = KCM_DatabaseInsert(db, &fact);
if (err != KCM_OK) {
    fprintf(stderr, "Error: %s\n", KCM_ErrorMessage(err));
}
```

| Error Code | Description |
|------------|-------------|
| `KCM_OK` | Success |
| `KCM_ERR_NOT_FOUND` | Requested row ID not found |
| `KCM_ERR_OUT_OF_MEMORY` | Memory allocation failed |
| `KCM_ERR_INVALID_ARGUMENT` | Invalid argument |
| `KCM_ERR_IO` | I/O error |
| `KCM_ERR_CORRUPTED` | Data corruption detected |
| `KCM_ERR_CONFLICT` | Conflict (e.g., duplicate key) |
| `KCM_ERR_TRANSACTION_ABORTED` | Transaction was aborted |

## Example Code

### Transactions

```c
KCM_Transaction *txn = KCM_DatabaseBeginTransaction(db);

KCM_Fact fact = { .subject = 1, .predicate = 2, .object = 3, .confidence = 0.9 };
KCM_Error err = KCM_DatabaseInsert(db, &fact);
if (err != KCM_OK) {
    KCM_TransactionRollback(txn);
    KCM_TransactionFree(txn);
    KCM_DatabaseFree(db);
    return 1;
}

err = KCM_TransactionCommit(txn);
if (err != KCM_OK) {
    fprintf(stderr, "Commit failed: %s\n", KCM_ErrorMessage(err));
}
KCM_TransactionFree(txn);
```

### Query Iteration

```c
KCM_Query *q = KCM_DatabaseQuery(db, "SELECT * FROM facts");
if (q) {
    KCM_Fact *f;
    while ((f = KCM_QueryNext(q)) != NULL) {
        printf("Subject=%u, Object=%u, Confidence=%.2f\n",
               f->subject, f->object, f->confidence);
    }
    KCM_QueryFree(q);
}
```

### Save and Load

```c
KCM_DatabaseSave(db, "knowledge.kcm");

KCM_DatabaseFree(db);
KCM_DatabaseNew(&db);
KCM_DatabaseLoad(db, "knowledge.kcm");
```

### Integrity Verification

```c
KCM_Error err = KCM_DatabaseVerify("knowledge.kcm");
if (err != KCM_OK) {
    fprintf(stderr, "Database corrupted: %s\n", KCM_ErrorMessage(err));
}
```

## Benchmark

Build and run the benchmark suite:

```bash
cargo bench --workspace
```

| Metric | Target |
|--------|--------|
| Insert (1M facts) | < 2s |
| Query (100K results) | < 50ms |
| Save/Load (1M facts) | < 5s |
| Memory (1M facts) | < 512MB |

Results are published with each release. See `docs/PRD-TESTING& BRACHMARCK.md` for methodology.
