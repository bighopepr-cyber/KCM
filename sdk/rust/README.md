# KCM Rust SDK

The native Rust SDK for KCM. Use the `kcm-core`, `kcm-storage`, `kcm-runtime` crates directly.

## Status: Stable

## Usage

```toml
[dependencies]
kcm-core = "0.1"
kcm-storage = "0.1"
kcm-runtime = "0.1"
```

## Quick Start

```rust
use kcm_runtime::database::KnowledgeDatabase;
use kcm_core::types::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = KnowledgeDatabase::new()?;
    
    let fact = Fact::new(
        SubjectID(1),
        PredicateID(1),
        ObjectID(1),
        0.95,
    )?;
    
    db.insert(&fact)?;
    println!("Fact count: {}", db.fact_count());
    
    Ok(())
}
```

## Examples

See `examples/rust/` for complete examples.
