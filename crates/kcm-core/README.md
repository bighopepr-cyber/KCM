# kcm-core

Core types, dense vectors, bitmap engine, and dictionary encoding for the KCM knowledge columnar model engine.

## Purpose

Provides the foundational data structures and algorithms used by all other KCM crates. Zero external dependencies beyond `parking_lot`.

## Modules

| Module | Purpose |
|--------|---------|
| `types` | `Fact`, `RowID`, `SubjectID`, `PredicateID`, `ObjectID`, `Confidence`, `KcmError` |
| `vec` | `DenseVec<T>` — growable, cache-friendly vector with O(1) index access |
| `bitmap` | `Bitmap` — bit-vector with rank, select, and next-set-bit operations |
| `dictionary` | `Dictionary` — string interning with bidirectional mapping (u32 <-> String) |

## Dependencies

| Dependency | Justification |
|------------|---------------|
| `parking_lot` | 3-5x faster RwLock/Mutex than std |

## Usage

```rust
use kcm_core::types::*;
use kcm_core::vec::DenseVec;
use kcm_core::bitmap::Bitmap;
use kcm_core::dictionary::Dictionary;

// Create a fact
let fact = Fact::new(SubjectID(1), PredicateID(1), ObjectID(1), 0.95)?;

// Dense vector
let mut vec = DenseVec::<u32>::new();
vec.push(42);
assert_eq!(vec[0], 42);

// Bitmap
let mut bm = Bitmap::new(1024);
bm.set(42);
assert!(bm.get(42));

// Dictionary
let mut dict = Dictionary::new();
let id = dict.intern("hello");
assert_eq!(dict.resolve(id).unwrap(), "hello");
```

## Features

- `serialization` — Enables serde derive for core types

## Error Model

All public APIs return `Result<T, KcmError>`. The `KcmError` enum contains:

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
```
