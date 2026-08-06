# KCM .NET SDK

Official SDK for the KCM Knowledge Columnar Model.

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![Version](https://img.shields.io/badge/version-0.1.0-blue)]()
[![License](https://img.shields.io/badge/license-MIT-green)]()
[![KCM Engine Compatible](https://img.shields.io/badge/KCM%20Engine-0.1.0-orange)]()

## Installation

```bash
dotnet add package Kcm.Sdk
```

## Quickstart

```csharp
using Kcm;

using var db = new KcmDatabase("my_knowledge.db");

var fact = new Fact { Subject = 1, Predicate = 0, Object = 2, Confidence = 0.95 };
db.Insert(fact);

var results = db.Query("SELECT * FROM facts");
foreach (var f in results) {
    Console.WriteLine($"Subject: {f.Subject}, Object: {f.Object}");
}

Console.WriteLine($"Fact count: {db.FactCount()}");
db.Close();
```

## API Reference

| Operation | Signature | Description |
|-----------|-----------|-------------|
| `KcmDatabase(path)` | Constructor | Open or create a database |
| `Insert(fact)` | `void Insert(Fact fact)` | Insert a fact |
| `Update(rowId, fact)` | `void Update(ulong rowId, Fact fact)` | Update a fact by row ID |
| `Delete(rowId)` | `void Delete(ulong rowId)` | Delete a fact by row ID |
| `Query(kql)` | `IEnumerable<Fact> Query(string kql)` | Execute KQL query |
| `QueryAll()` | `List<Fact> QueryAll()` | Get all active facts |
| `FactCount()` | `ulong FactCount()` | Total fact count |
| `ActiveFactCount()` | `ulong ActiveFactCount()` | Active fact count |
| `BeginTransaction()` | `Transaction BeginTransaction()` | Begin transaction |
| `Save(path)` | `void Save(string path)` | Save to file |
| `Load(path)` | `void Load(string path)` | Load from file |
| `Verify(path)` | `static void Verify(string path)` | Verify file integrity |
| `Close()` | `void Close()` | Close database |

## Error Handling

All operations throw `KcmException` on error with a descriptive message and error code:

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

```csharp
using Kcm;

using var db = new KcmDatabase("knowledge.db");

db.Insert(new Fact { Subject = 1, Predicate = 0, Object = 2, Confidence = 0.95 });
db.Insert(new Fact { Subject = 2, Predicate = 1, Object = 3, Confidence = 0.90 });

foreach (var fact in db.QueryAll()) {
    Console.WriteLine($"Subject: {fact.Subject}, Object: {fact.Object}");
}
```

### API Integration

```csharp
using Kcm;

public static List<Fact> FetchKnowledge(string dbPath) {
    using var db = new KcmDatabase(dbPath);
    return db.QueryAll();
}
```

### Transaction

```csharp
using Kcm;

using var db = new KcmDatabase("knowledge.db");
var txn = db.BeginTransaction();

db.Insert(new Fact { Subject = 10, Predicate = 0, Object = 20, Confidence = 0.85 });

if (db.ActiveFactCount() > 0) {
    txn.Commit();
} else {
    txn.Rollback();
}
```

## Full Documentation

See [docs/sdk/dotnet.md](../../docs/sdk/dotnet.md) for the complete reference.

## Examples

See [examples/](examples/) for more examples including:
- `standalone/BasicCrud.cs` — CRUD operations
- `standalone/Transactions.cs` — Transaction management
- `standalone/Persistence.cs` — Save/load databases
- `standalone/QueryPatterns.cs` — KQL query patterns
- `standalone/ErrorHandling.cs` — Error handling patterns

## License

MIT

Made by bighopepr-cyber
