# KCM Examples

Working code examples for each supported language.

## Languages

| Language | Directory | Status |
|----------|-----------|--------|
| Rust | examples/rust/ | Available |
| Python | examples/python/ | Planned |
| JavaScript | examples/javascript/ | Planned |
| Go | examples/go/ | Planned |
| Java | examples/java/ | Planned |

## Rust Examples

### Basic Usage

```rust
use kcm_runtime::database::KnowledgeDatabase;
use kcm_core::types::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = KnowledgeDatabase::new()?;
    
    let fact = Fact::new(SubjectID(1), PredicateID(1), ObjectID(1), 0.95)?;
    db.insert(&fact)?;
    
    println!("Fact count: {}", db.fact_count());
    Ok(())
}
```

### KQL Query

```rust
let results = db.query("SELECT * FROM facts WHERE subject = 1")?;
for fact in results {
    println!("{:?}", fact);
}
```

### Transaction

```rust
let mut txn = db.begin_transaction();
txn.insert(fact)?;
txn.commit()?;
```

## Running Examples

```bash
# Rust
cargo run --example basic

# Python (planned)
python examples/python/basic.py
```
