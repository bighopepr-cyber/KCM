# KCM SDKs

Language-specific SDKs for the KCM Knowledge Columnar Model.

## Available SDKs

| Language | Status | Package | API Style |
|----------|--------|---------|-----------|
| Rust | Stable | kcm-core (native crate) | Direct API |
| Python | Planned | kcm (PyPI) | kcm.Database() |
| JavaScript | Planned | @kcm/js (npm) | new kcm.Database() |
| TypeScript | Planned | @kcm/ts (npm) | new kcm.Database() |
| Go | Planned | github.com/kcm/go-sdk | kcm.NewDatabase() |
| Java | Planned | io.kcm:sdk (Maven) | new KcmDatabase() |
| .NET | Planned | Kcm.Sdk (NuGet) | new KcmDatabase() |
| C | Stable | FFI via kcm-interface | KCM_DatabaseNew() |
| C++ | Planned | libkcm (system lib) | kcm::Database() |

## Architecture

Each SDK wraps the core KCM engine via language-specific bindings:

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

## Quick Start

### Rust

```rust
use kcm_runtime::database::KnowledgeDatabase;

let db = KnowledgeDatabase::new()?;
// Use db...
```

### Python (Planned)

```python
import kcm

db = kcm.Database("my_knowledge.db")
db.insert(subject="planet", predicate="orbits", object="sun", confidence=0.99)
results = db.query("SELECT * FROM facts WHERE subject = 'planet'")
db.close()
```

### JavaScript (Planned)

```javascript
const { Database } = require('@kcm/js');

const db = new Database('my_knowledge.db');
db.insert({ subject: 'planet', predicate: 'orbits', object: 'sun', confidence: 0.99 });
const results = db.query("SELECT * FROM facts WHERE subject = 'planet'");
db.close();
```

## API Reference

All SDKs expose the same core operations:

| Operation | Description |
|-----------|-------------|
| Database(path) | Open or create a database |
| insert(fact) | Insert a knowledge fact |
| query(kql) | Execute a KQL query |
| delete(row_id) | Delete a fact by ID |
| update(fact) | Update an existing fact |
| fact_count() | Get total fact count |
| active_count() | Get active fact count |
| begin_transaction() | Start a transaction |
| commit(txn) | Commit a transaction |
| rollback(txn) | Rollback a transaction |
| save(path) | Save database to file |
| load(path) | Load database from file |
| verify() | Verify database integrity |
| close() | Close database |

## Examples

See individual SDK directories for language-specific examples.
