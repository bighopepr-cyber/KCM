# KCM Rust SDK

Official SDK for the KCM Knowledge Columnar Model

<!-- badges:start -->
[![Build Status](https://img.shields.io/badge/build-pending-lightgrey)]()
[![Version](https://img.shields.io/badge/version-1.0.0-blue)]()
[![License](https://img.shields.io/badge/license-MIT-green)]()
<!-- badges:end -->

The Rust SDK provides direct access to the KCM engine as a native Rust crate. No FFI or bindings required.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
kcm-core = "1.0.0"
kcm-runtime = "1.0.0"
```

Or for the full stack:

```toml
[dependencies]
kcm = "1.0.0"
```

## Quickstart

```rust
use kcm_runtime::database::KnowledgeDatabase;
use kcm_core::types::Fact;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = KnowledgeDatabase::new()?;

    let fact = Fact {
        subject: 1,
        predicate: 2,
        object: 3,
        confidence: 0.95,
        evidence: 1,
        timestamp: 0,
        context: 1,
        version: 1,
        priority: 0,
        owner: 1,
    };

    db.insert(fact)?;

    println!("Total facts: {}", db.fact_count());
    println!("Active facts: {}", db.active_count());

    Ok(())
}
```

## API Reference

### `Fact`

Knowledge fact with 10 attributes.

```rust
pub struct Fact {
    pub subject: u32,
    pub predicate: u8,
    pub object: u32,
    pub confidence: f64,
    pub evidence: u8,
    pub timestamp: i64,
    pub context: u8,
    pub version: i32,
    pub priority: i8,
    pub owner: u16,
}
```

### `KnowledgeDatabase`

Main entry point for KCM operations.

```rust
impl KnowledgeDatabase {
    pub fn new() -> Result<Self, KcmError>;
    pub fn insert(&self, fact: Fact) -> Result<u64, KcmError>;
    pub fn update(&self, row_id: u64, fact: Fact) -> Result<(), KcmError>;
    pub fn delete(&self, row_id: u64) -> Result<(), KcmError>;

    pub fn fact_count(&self) -> u64;
    pub fn active_count(&self) -> u64;

    pub fn query(&self, kql: &str) -> Result<QueryResult, KcmError>;
    pub fn begin_transaction(&self) -> Result<Transaction, KcmError>;

    pub fn save(&self, path: &str) -> Result<(), KcmError>;
    pub fn load(&self, path: &str) -> Result<(), KcmError>;

    pub fn verify(path: &str) -> Result<(), KcmError>;
}
```

#### Constructor

```rust
let db = KnowledgeDatabase::new()?;
```

Create a new in-memory database. Returns `Err(KcmError)` on failure.

#### `insert`

```rust
let row_id = db.insert(fact)?;
```

Insert a fact. Returns the assigned row ID.

#### `update`

```rust
db.update(row_id, fact)?;
```

Update a fact by row ID. Returns `Err(KcmError::NotFound)` if not found.

#### `delete`

```rust
db.delete(row_id)?;
```

Delete a fact by row ID. Returns `Err(KcmError::NotFound)` if not found.

#### `fact_count`

```rust
let count = db.fact_count(); // -> u64
```

Returns total fact count (including deleted).

#### `active_count`

```rust
let count = db.active_count(); // -> u64
```

Returns active (non-deleted) fact count.

#### `query`

```rust
let results = db.query("SELECT * FROM facts")?; // -> QueryResult
```

Execute a KQL query.

#### `begin_transaction`

```rust
let txn = db.begin_transaction()?; // -> Transaction
```

Begin a new transaction.

#### `save`

```rust
db.save("knowledge.kcm")?;
```

Save the database to a file.

#### `load`

```rust
db.load("knowledge.kcm")?;
```

Load a database from a file.

#### `verify`

```rust
KnowledgeDatabase::verify("knowledge.kcm")?;
```

Verify database file integrity.

### `QueryResult`

Query result iterator.

```rust
impl Iterator for QueryResult {
    type Item = Fact;
}

impl QueryResult {
    pub fn collect_vec(self) -> Vec<Fact>;
}
```

### `Transaction`

```rust
impl Transaction {
    pub fn commit(self) -> Result<(), KcmError>;
    pub fn rollback(self) -> Result<(), KcmError>;
}
```

### `KcmError`

```rust
pub enum KcmError {
    NotFound(String),
    OutOfMemory,
    InvalidArgument(String),
    Io(String),
    Corrupted(String),
    Conflict(String),
    TransactionAborted,
}

impl std::fmt::Display for KcmError { /* ... */ }
impl std::error::Error for KcmError { /* ... */ }
```

## Error Handling

All fallible operations return `Result<T, KcmError>`:

```rust
match db.insert(fact) {
    Ok(row_id) => println!("Inserted at row {}", row_id),
    Err(KcmError::NotFound(msg)) => eprintln!("Not found: {}", msg),
    Err(e) => eprintln!("Error: {}", e),
}
```

| `KcmError` Variant | HTTP Status | Description |
|---------------------|-------------|-------------|
| `NotFound` | 404 | Requested row ID not found |
| `OutOfMemory` | 507 | Memory allocation failed |
| `InvalidArgument` | 400 | Invalid argument |
| `Io` | 500 | I/O error |
| `Corrupted` | 500 | Data corruption detected |
| `Conflict` | 409 | Conflict (e.g., duplicate key) |
| `TransactionAborted` | 409 | Transaction was aborted |

## Example Code

### Transactions

```rust
let txn = db.begin_transaction()?;

let fact = Fact {
    subject: 1,
    predicate: 2,
    object: 3,
    confidence: 0.9,
    ..Default::default()
};

db.insert(fact)?;
txn.commit()?;
```

### Query with Iterator

```rust
let results = db.query("SELECT * FROM facts")?;

for fact in results {
    println!("Subject={}, Object={}, Confidence={:.2}",
        fact.subject, fact.object, fact.confidence);
}
```

### Collect Results

```rust
let facts: Vec<Fact> = db.query("SELECT * FROM facts")?.collect();
println!("Found {} facts", facts.len());
```

### Save and Load

```rust
db.save("knowledge.kcm")?;

let db2 = KnowledgeDatabase::new()?;
db2.load("knowledge.kcm")?;
println!("Loaded {} facts", db2.fact_count());
```

### Integrity Verification

```rust
match KnowledgeDatabase::verify("knowledge.kcm") {
    Ok(()) => println!("Database is valid"),
    Err(e) => eprintln!("Corruption detected: {}", e),
}
```

## Benchmark

Run the benchmark suite:

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
