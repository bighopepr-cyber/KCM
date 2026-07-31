# Knowledge Columnar Model (KCM)

A high-performance, columnar knowledge representation and reasoning engine implemented in Rust.

## Architecture

KCM replaces traditional pointer-based knowledge graphs with **columnar relation spaces** that can be processed with SIMD, compressed independently, and optimized for modern reasoning engines.

### Core Thesis

*Knowledge is not an object graph. Knowledge is a columnar relation space.*

## Crates

| Crate | Description |
|-------|-------------|
| `kcm-core` | Core types, DenseVec, Bitmap, Dictionary |
| `kcm-storage` | Columnar storage, codecs, compression, indexes |
| `kcm-compute` | Query algebra operators, SIMD acceleration |
| `kcm-reasoning` | Rule engine, forward-chaining inference, confidence calculus |
| `kcm-optimizer` | Query plan optimization, filter pushdown, join reorder |
| `kcm-runtime` | Transactions, database, async executor |
| `kcm-interface` | C FFI API, Python bindings (PyO3) |

## Features

- **Columnar Storage**: All knowledge stored as independent linear columns
- **SIMD Accelerated**: AVX2 intrinsics with scalar fallback
- **Dictionary Encoding**: All strings/references mapped to integer dictionaries
- **Compression-Native**: Delta, Gorilla, RLE, Zstd, LZ4 per-column
- **Bitmap Indexing**: Fast filtering on low-cardinality columns
- **Forward-Chaining Inference**: Rule-based reasoning with confidence propagation
- **Confidence Calculus**: Conjunction, disjunction, negation, weighted combination
- **ACID Transactions**: Transaction management with version store
- **Thread-Safe**: Lock-free readers, write-locked modifications
- **Deterministic Execution**: Identical input always produces identical output
- **C FFI API**: Full C-compatible interface for cross-language usage
- **Python Bindings**: PyO3-based Python interface (feature-gated)

## Quick Start

```rust
use kcm_runtime::database::KnowledgeDatabase;
use kcm_core::types::*;

fn main() -> Result<(), KcmError> {
    let kb = KnowledgeDatabase::new()?;
    
    // Insert facts
    let fact = Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9)?;
    kb.insert(&fact)?;
    
    // Query
    let results = kb.query()
        .with_subject(SubjectID(1))
        .with_confidence(0.5)
        .execute()?;
    
    // Update
    kb.update(RowID(0), &fact)?;
    
    // Delete
    kb.delete(RowID(0))?;
    
    Ok(())
}
```

## Building

```bash
cargo build --release
```

## Testing

```bash
cargo test --workspace
```

## Benchmarks

```bash
cargo bench --workspace
```

## License

MIT
