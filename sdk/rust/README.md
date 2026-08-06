# KCM Rust SDK

Official SDK for the KCM Knowledge Columnar Model.

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![Version](https://img.shields.io/badge/version-1.0.0-blue)]()
[![License](https://img.shields.io/badge/license-MIT-green)]()
[![KCM Engine Compatible](https://img.shields.io/badge/KCM%20Engine-1.0.0-orange)]()

## Installation

```bash
cargo add kcm-sdk
```

Or add to your `Cargo.toml`:

```toml
[dependencies]
kcm-sdk = { path = "sdk/rust" }
```

## Quickstart

```rust
use kcm_sdk::{Database, Fact};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::new()?;

    let fact = Fact::new(1, 2, 3, 0.95)?;
    db.insert(&fact)?;

    println!("Fact count: {}", db.fact_count());

    for fact in db.query_all()? {
        println!("S={} P={} O={}", fact.subject, fact.predicate, fact.object);
    }

    db.close();
    Ok(())
}
```

## API Reference

| Operation | Signature | Description |
|-----------|-----------|-------------|
| `Database::new()` | `-> Result<Database>` | Create a new in-memory database |
| `Database::load(path)` | `-> Result<Database>` | Load a database from a file |
| `db.insert(&fact)` | `-> Result<RowID>` | Insert a fact, returns RowID |
| `db.update(row_id, &fact)` | `-> Result<()>` | Update a fact at the given row ID |
| `db.delete(row_id)` | `-> Result<()>` | Delete a fact at the given row ID |
| `db.get_fact(row_id)` | `-> Result<Fact>` | Retrieve a single fact by row ID |
| `db.query(kql)` | `-> Result<QueryResult>` | Execute a KQL query |
| `db.query_all()` | `-> Result<Vec<Fact>>` | Retrieve all active facts |
| `db.fact_count()` | `-> u64` | Get total fact count (including tombstones) |
| `db.active_fact_count()` | `-> u64` | Get active fact count |
| `db.begin_transaction()` | `-> Result<Transaction>` | Begin a new transaction |
| `db.save(path)` | `-> Result<()>` | Save database to file |
| `Database::verify(path)` | `-> Result<()>` | Verify database file integrity |
| `db.close()` | `-> ()` | Close the database |

## Error Handling

All operations return `Result<T, SdkError>`. Error codes match the SSOT specification:

| Error | Code | Description |
|-------|------|-------------|
| `NotFound` | 1001 | Resource not found |
| `OutOfMemory` | 1002 | Insufficient memory |
| `InvalidArgument` | 1003 | Invalid argument |
| `Io` | 1004 | I/O error |
| `Corrupted` | 1005 | Data corruption |
| `Conflict` | 1006 | Concurrent conflict |
| `TransactionAborted` | 1007 | Transaction aborted |

## Use Cases

### Basic Query

```rust
use kcm_sdk::{Database, Fact};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::new()?;

    db.insert(&Fact::new(1, 0, 2, 0.95)?)?;
    db.insert(&Fact::new(2, 1, 3, 0.90)?)?;

    let results = db.query_all()?;
    for fact in results {
        println!("Subject: {}, Object: {}", fact.subject, fact.object);
    }

    Ok(())
}
```

### API Integration

```rust
use kcm_sdk::Database;

fn fetch_knowledge(db: &Database) -> Result<(), Box<dyn std::error::Error>> {
    let facts = db.query_all()?;
    let json: Vec<_> = facts.iter().map(|f| {
        format!("{{\"s\":{},\"p\":{},\"o\":{}}}", f.subject, f.predicate, f.object)
    }).collect();
    println!("[{}]", json.join(","));
    Ok(())
}
```

### Transaction

```rust
use kcm_sdk::{Database, Fact};

fn transfer_knowledge(db: &Database) -> Result<(), Box<dyn std::error::Error>> {
    let txn = db.begin_transaction()?;

    db.insert(&Fact::new(10, 0, 20, 0.85)?)?;
    db.insert(&Fact::new(20, 1, 30, 0.80)?)?;

    if db.active_fact_count() > 0 {
        txn.commit()?;
    } else {
        txn.rollback()?;
    }

    Ok(())
}
```

## Full Documentation

See [docs/sdk/rust.md](../../docs/sdk/rust.md) for the complete reference.

## Examples

See [examples/](examples/) for more examples including:
- `basic.rs` — Getting started
- `basic_crud.rs` — CRUD operations
- `transactions.rs` — Transaction management
- `persistence.rs` — Save/load databases
- `query_patterns.rs` — KQL query patterns
- `error_handling.rs` — Error handling patterns

## License

MIT

Made by bighopepr-cyber
