# KCM SDKs

Official SDKs for the KCM Knowledge Columnar Model.

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![Version](https://img.shields.io/badge/version-0.1.0-blue)]()
[![License](https://img.shields.io/badge/license-MIT-green)]()
[![KCM Engine Compatible](https://img.shields.io/badge/KCM%20Engine-0.1.0-orange)]()

## Overview

KCM provides language-specific SDKs that wrap the core Rust engine, giving you a native API experience in your preferred language. All SDKs expose the same standardized operations defined in the SSOT specification.

## Architecture

```
+-------------+
|  Your App   |
+-------------+
|  Language   |  <- SDK layer (language-native API)
|  SDK        |
+-------------+
|  KCM Core   |  <- Rust engine (compiled)
+-------------+
```

## Available SDKs

| Language | Status | Package | API Style | Documentation |
|----------|--------|---------|-----------|---------------|
| Rust | Stable | `kcm-sdk` (crate) | `Database::new()` | [docs/sdk/rust.md](../docs/sdk/rust.md) |
| C | Stable | `libkcm` (FFI) | `KCM_DatabaseNew()` | [docs/sdk/c.md](../docs/sdk/c.md) |
| C++ | Stable | `libkcm` (header-only) | `kcm::Database()` | [docs/sdk/cpp.md](../docs/sdk/cpp.md) |
| Python | Beta | `kcm` (PyPI) | `kcm.Database()` | [docs/sdk/python.md](../docs/sdk/python.md) |
| JavaScript | Beta | `@kcm/js` (npm) | `new Database()` | [docs/sdk/javascript.md](../docs/sdk/javascript.md) |
| TypeScript | Beta | `@kcm/ts` (npm) | `new Database()` | [docs/sdk/typescript.md](../docs/sdk/typescript.md) |
| Go | Beta | `github.com/kcm/go-sdk` | `kcm.NewDatabase()` | [docs/sdk/go.md](../docs/sdk/go.md) |
| Java | Beta | `io.kcm:sdk` (Maven) | `new KcmDatabase()` | [docs/sdk/java.md](../docs/sdk/java.md) |
| .NET | Beta | `Kcm.Sdk` (NuGet) | `new KcmDatabase()` | [docs/sdk/dotnet.md](../docs/sdk/dotnet.md) |

## Standardized API Surface

All SDKs expose the same core operations:

| Operation | Description |
|-----------|-------------|
| `Database(path?)` | Open or create a database |
| `insert(fact)` | Insert a knowledge fact |
| `query(kql)` | Execute a KQL query |
| `query_all()` | Retrieve all active facts |
| `delete(row_id)` | Delete a fact by ID |
| `update(row_id, fact)` | Update an existing fact |
| `get_fact(row_id)` | Retrieve a single fact by ID |
| `fact_count()` | Get total fact count |
| `active_fact_count()` | Get active fact count |
| `begin_transaction()` | Start a transaction |
| `commit(txn)` | Commit a transaction |
| `rollback(txn)` | Rollback a transaction |
| `save(path)` | Save database to file |
| `load(path)` | Load database from file |
| `verify(path)` | Verify database integrity |
| `close()` | Close database |

## Compatibility Matrix

See [docs/sdk/compatibility.md](../docs/sdk/compatibility.md) for the full SDK compatibility matrix covering OS, architecture, and engine version support.

## Quick Start

### Rust

```rust
use kcm_sdk::{Database, Fact};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::new()?;
    let fact = Fact::new(1, 2, 3, 0.95)?;
    db.insert(&fact)?;
    println!("Fact count: {}", db.fact_count());
    Ok(())
}
```

### Python

```python
import kcm

db = kcm.Database()
db.insert(subject=1, predicate=0, object=2, confidence=0.95)
for fact in db.query_all():
    print(fact)
db.close()
```

### JavaScript

```javascript
const { Database } = require('@kcm/js');

const db = new Database();
db.insert({ subject: 1, predicate: 0, object: 2, confidence: 0.95 });
console.log(`Total facts: ${db.factCount()}`);
db.close();
```

## Installation

Each SDK has its own installation method. See the individual SDK READMEs for details:

| Language | Install Command |
|----------|----------------|
| Rust | `cargo add kcm-sdk` |
| C | Build from source: `make` in `sdk/c/` |
| C++ | Build from source: `cmake` in `sdk/cpp/` |
| Python | `pip install kcm` |
| JavaScript | `npm install @kcm/js` |
| TypeScript | `npm install @kcm/ts` |
| Go | `go get github.com/kcm/go-sdk` |
| Java | Add Maven dependency `io.kcm:sdk` |
| .NET | `dotnet add package Kcm.Sdk` |

## Examples

See individual SDK directories for language-specific examples and use cases:

| SDK | Examples Directory |
|-----|-------------------|
| Rust | [`sdk/rust/examples/`](rust/examples/) |
| C | [`sdk/c/examples/`](c/examples/) |
| C++ | [`sdk/cpp/examples/`](cpp/examples/) |
| Python | [`sdk/python/examples/`](python/examples/) |
| JavaScript | [`sdk/javascript/examples/`](javascript/examples/) |
| TypeScript | [`sdk/typescript/examples/`](typescript/examples/) |
| Go | [`sdk/go/examples/`](go/examples/) |
| .NET | [`sdk/dotnet/examples/`](dotnet/examples/) |

## License

MIT

Made by bighopepr-cyber
