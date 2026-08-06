# KCM Go SDK

Official SDK for the KCM Knowledge Columnar Model.

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![Version](https://img.shields.io/badge/version-0.1.0-blue)]()
[![License](https://img.shields.io/badge/license-MIT-green)]()
[![KCM Engine Compatible](https://img.shields.io/badge/KCM%20Engine-0.1.0-orange)]()

## Installation

```bash
go get github.com/kcm/go-sdk
```

## Quickstart

```go
package main

import (
	"fmt"
	"github.com/kcm/go-sdk"
)

func main() {
	db, err := kcm.NewDatabase("my_knowledge.db")
	if err != nil {
		panic(err)
	}
	defer db.Close()

	fact := kcm.Fact{Subject: 1, Predicate: 0, Object: 2, Confidence: 0.95}
	db.Insert(fact)

	results, _ := db.QueryAll()
	for _, r := range results {
		fmt.Printf("Subject: %d, Object: %d\n", r.Subject, r.Object)
	}

	fmt.Printf("Fact count: %d\n", db.FactCount())
}
```

## API Reference

| Operation | Signature | Description |
|-----------|-----------|-------------|
| `NewDatabase(path)` | `func NewDatabase(path string) (*Database, error)` | Open or create a database |
| `db.Insert(fact)` | `func (db *Database) Insert(fact Fact) error` | Insert a fact |
| `db.Update(rowID, fact)` | `func (db *Database) Update(rowID uint64, fact Fact) error` | Update a fact |
| `db.Delete(rowID)` | `func (db *Database) Delete(rowID uint64) error` | Delete a fact |
| `db.Query(kql)` | `func (db *Database) Query(kql string) ([]Fact, error)` | Execute KQL query |
| `db.QueryAll()` | `func (db *Database) QueryAll() ([]Fact, error)` | Get all active facts |
| `db.GetFact(rowID)` | `func (db *Database) GetFact(rowID uint64) (*Fact, error)` | Retrieve a fact by ID |
| `db.FactCount()` | `func (db *Database) FactCount() uint64` | Total fact count |
| `db.ActiveFactCount()` | `func (db *Database) ActiveFactCount() uint64` | Active fact count |
| `db.BeginTransaction()` | `func (db *Database) BeginTransaction() (*Transaction, error)` | Begin transaction |
| `db.Save(path)` | `func (db *Database) Save(path string) error` | Save to file |
| `db.Load(path)` | `func (db *Database) Load(path string) error` | Load from file |
| `Database.Verify(path)` | `func Verify(path string) error` | Verify file integrity |
| `db.Close()` | `func (db *Database) Close()` | Close database |

## Error Handling

All operations return `error`. Use `errors.Is()` or `errors.As()` to check error types:

| Error | Code | Description |
|-------|------|-------------|
| `ErrNotFound` | 1001 | Resource not found |
| `ErrOutOfMemory` | 1002 | Insufficient memory |
| `ErrInvalidArgument` | 1003 | Invalid argument |
| `ErrIo` | 1004 | I/O error |
| `ErrCorrupted` | 1005 | Data corruption |
| `ErrConflict` | 1006 | Concurrent conflict |
| `ErrTransactionAborted` | 1007 | Transaction aborted |

## Use Cases

### Basic Query

```go
package main

import (
	"fmt"
	"github.com/kcm/go-sdk"
)

func main() {
	db, _ := kcm.NewDatabase("knowledge.db")
	defer db.Close()

	db.Insert(kcm.Fact{Subject: 1, Predicate: 0, Object: 2, Confidence: 0.95})
	db.Insert(kcm.Fact{Subject: 2, Predicate: 1, Object: 3, Confidence: 0.90})

	results, _ := db.QueryAll()
	for _, fact := range results {
		fmt.Printf("Subject: %d, Object: %d\n", fact.Subject, fact.Object)
	}
}
```

### API Integration

```go
package main

import (
	"github.com/kcm/go-sdk"
)

func FetchKnowledge(dbPath string) ([]kcm.Fact, error) {
	db, err := kcm.NewDatabase(dbPath)
	if err != nil {
		return nil, err
	}
	defer db.Close()
	return db.QueryAll()
}
```

### Transaction

```go
package main

import (
	"github.com/kcm/go-sdk"
)

func TransferKnowledge(db *kcm.Database) error {
	txn, err := db.BeginTransaction()
	if err != nil {
		return err
	}

	db.Insert(kcm.Fact{Subject: 10, Predicate: 0, Object: 20, Confidence: 0.85})

	if db.ActiveFactCount() > 0 {
		return txn.Commit()
	}
	return txn.Rollback()
}
```

## Full Documentation

See [docs/sdk/go.md](../../docs/sdk/go.md) for the complete reference.

## Examples

See [examples/](examples/) for more examples including:
- `basic/main.go` — Getting started
- `basic_crud/main.go` — CRUD operations
- `transactions/main.go` — Transaction management
- `persistence/main.go` — Save/load databases
- `query_patterns/main.go` — KQL query patterns
- `error_handling/main.go` — Error handling patterns

## License

MIT

Made by bighopepr-cyber
