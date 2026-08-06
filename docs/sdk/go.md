# KCM Go SDK

Official SDK for the KCM Knowledge Columnar Model

<!-- badges:start -->
[![Build Status](https://img.shields.io/badge/build-pending-lightgrey)]()
[![Version](https://img.shields.io/badge/version-1.0.0-blue)]()
[![License](https://img.shields.io/badge/license-MIT-green)]()
<!-- badges:end -->

The Go SDK wraps the KCM C FFI via cgo, providing idiomatic Go APIs with error handling.

## Installation

```bash
go get github.com/kcm/go-sdk
```

## Quickstart

```go
package main

import (
    "fmt"
    "log"

    kcm "github.com/kcm/go-sdk"
)

func main() {
    db, err := kcm.NewDatabase()
    if err != nil {
        log.Fatal(err)
    }
    defer db.Close()

    fact := kcm.Fact{
        Subject:    1,
        Predicate:  2,
        Object:     3,
        Confidence: 0.95,
        Evidence:   1,
        Context:    1,
        Priority:   0,
        Owner:      1,
    }

    _, err = db.Insert(fact)
    if err != nil {
        log.Fatal(err)
    }

    fmt.Printf("Total facts: %d\n", db.FactCount())
    fmt.Printf("Active facts: %d\n", db.ActiveCount())
}
```

## API Reference

### `Fact`

Knowledge fact with 10 attributes.

```go
type Fact struct {
    Subject    uint32
    Predicate  uint8
    Object     uint32
    Confidence float64
    Evidence   uint8
    Timestamp  int64
    Context    uint8
    Version    int32
    Priority   int8
    Owner      uint16
}
```

### `Database`

Main entry point for KCM operations.

```go
type Database struct { /* ... */ }

func NewDatabase() (*Database, error)
func (db *Database) Close()

func (db *Database) Insert(fact Fact) (uint64, error)
func (db *Database) Update(rowID uint64, fact Fact) error
func (db *Database) Delete(rowID uint64) error

func (db *Database) FactCount() uint64
func (db *Database) ActiveCount() uint64

func (db *Database) Query(kql string) (*QueryResult, error)
func (db *Database) BeginTransaction() (*Transaction, error)

func (db *Database) Save(path string) error
func (db *Database) Load(path string) error
func (db *Database) Verify(path string) error
```

#### `NewDatabase`

```go
func NewDatabase() (*Database, error)
```

Create a new in-memory database. Returns `nil` and an error on failure.

#### `Close`

```go
func (db *Database) Close()
```

Free the database and release all resources. Safe to call on a `nil` database.

#### `Insert`

```go
func (db *Database) Insert(fact Fact) (uint64, error)
```

Insert a fact. Returns the assigned row ID and any error.

#### `Update`

```go
func (db *Database) Update(rowID uint64, fact Fact) error
```

Update a fact by row ID. Returns `ErrNotFound` if the row ID does not exist.

#### `Delete`

```go
func (db *Database) Delete(rowID uint64) error
```

Delete a fact by row ID. Returns `ErrNotFound` if the row ID does not exist.

#### `FactCount`

```go
func (db *Database) FactCount() uint64
```

Returns total fact count (including deleted).

#### `ActiveCount`

```go
func (db *Database) ActiveCount() uint64
```

Returns active (non-deleted) fact count.

#### `Query`

```go
func (db *Database) Query(kql string) (*QueryResult, error)
```

Execute a KQL query. Returns a `QueryResult` for iteration.

#### `BeginTransaction`

```go
func (db *Database) BeginTransaction() (*Transaction, error)
```

Begin a new transaction.

#### `Save`

```go
func (db *Database) Save(path string) error
```

Save the database to a file.

#### `Load`

```go
func (db *Database) Load(path string) error
```

Load a database from a file.

#### `Verify`

```go
func (db *Database) Verify(path string) error
```

Verify database file integrity. Returns `ErrCorrupted` if invalid.

### `QueryResult`

Query result iterator.

```go
type QueryResult struct { /* ... */ }

func (q *QueryResult) Next() bool
func (q *QueryResult) Fact() Fact
func (q *QueryResult) Close()
```

#### `Next`

```go
func (q *QueryResult) Next() bool
```

Advance to the next result. Returns `false` when exhausted.

#### `Fact`

```go
func (q *QueryResult) Fact() Fact
```

Return the current fact. Call only after `Next()` returns `true`.

#### `Close`

```go
func (q *QueryResult) Close()
```

Free the query result.

### `Transaction`

```go
type Transaction struct { /* ... */ }

func (t *Transaction) Commit() error
func (t *Transaction) Rollback() error
func (t *Transaction) Close()
```

#### `Commit`

```go
func (t *Transaction) Commit() error
```

Commit the transaction.

#### `Rollback`

```go
func (t *Transaction) Rollback() error
```

Rollback the transaction.

#### `Close`

```go
func (t *Transaction) Close()
```

Free the transaction handle.

### `Error`

```go
type Error struct {
    Code    ErrorCode
    Message string
}

func (e *Error) Error() string
```

#### `ErrorCode`

```go
type ErrorCode int

const (
    ErrOk                 ErrorCode = 0
    ErrNotFound           ErrorCode = 1
    ErrOutOfMemory        ErrorCode = 2
    ErrInvalidArgument    ErrorCode = 3
    ErrIo                 ErrorCode = 4
    ErrCorrupted          ErrorCode = 5
    ErrConflict           ErrorCode = 6
    ErrTransactionAborted ErrorCode = 7
)
```

#### Sentinel Errors

```go
var (
    ErrNotFound           = errors.New("not_found")
    ErrOutOfMemory        = errors.New("out_of_memory")
    ErrInvalidArgument    = errors.New("invalid_argument")
    ErrIo                 = errors.New("io")
    ErrCorrupted          = errors.New("corrupted")
    ErrConflict           = errors.New("conflict")
    ErrTransactionAborted = errors.New("transaction_aborted")
)
```

## Error Handling

Go SDK uses standard Go error handling:

```go
db, err := kcm.NewDatabase()
if err != nil {
    log.Fatal(err)
}
defer db.Close()

rowID, err := db.Insert(fact)
if err != nil {
    log.Printf("Insert failed: %v", err)
}
```

Check specific error types:

```go
if errors.Is(err, kcm.ErrNotFound) {
    fmt.Println("Row not found")
}
```

| Error Code | Description |
|------------|-------------|
| `ErrNotFound` | Requested row ID not found |
| `ErrOutOfMemory` | Memory allocation failed |
| `ErrInvalidArgument` | Invalid argument |
| `ErrIo` | I/O error |
| `ErrCorrupted` | Data corruption detected |
| `ErrConflict` | Conflict (e.g., duplicate key) |
| `ErrTransactionAborted` | Transaction was aborted |

## Example Code

### Transactions

```go
db, _ := kcm.NewDatabase()
defer db.Close()

txn, err := db.BeginTransaction()
if err != nil {
    log.Fatal(err)
}

fact := kcm.Fact{Subject: 1, Predicate: 2, Object: 3, Confidence: 0.9}
_, err = db.Insert(fact)
if err != nil {
    txn.Rollback()
    log.Fatal(err)
}

if err := txn.Commit(); err != nil {
    log.Fatal(err)
}
txn.Close()
```

### Query Iteration

```go
qr, err := db.Query("SELECT * FROM facts")
if err != nil {
    log.Fatal(err)
}
defer qr.Close()

for qr.Next() {
    f := qr.Fact()
    fmt.Printf("Subject=%d, Object=%d, Confidence=%.2f\n",
        f.Subject, f.Object, f.Confidence)
}
```

### Save and Load

```go
db, _ := kcm.NewDatabase()
db.Insert(kcm.Fact{Subject: 1, Predicate: 2, Object: 3, Confidence: 0.95})
db.Save("knowledge.kcm")
db.Close()

db2, _ := kcm.NewDatabase()
defer db2.Close()
db2.Load("knowledge.kcm")
fmt.Printf("Loaded %d facts\n", db2.FactCount())
```

### Integrity Verification

```go
db, _ := kcm.NewDatabase()
defer db.Close()

if err := db.Verify("knowledge.kcm"); err != nil {
    fmt.Printf("Corruption detected: %v\n", err)
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
