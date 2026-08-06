# KCM .NET SDK

Official SDK for the KCM Knowledge Columnar Model

<!-- badges:start -->
[![Build Status](https://img.shields.io/badge/build-pending-lightgrey)]()
[![Version](https://img.shields.io/badge/version-1.0.0-blue)]()
[![License](https://img.shields.io/badge/license-MIT-green)]()
<!-- badges:end -->

The .NET SDK wraps the KCM C FFI via P/Invoke, providing a managed API for C# and .NET applications.

## Installation

### NuGet

```bash
dotnet add package Kcm.Sdk
```

### Package Manager

```powershell
Install-Package Kcm.Sdk
```

### Manual

Build from source and reference the compiled `Kcm.Sdk.dll`.

## Quickstart

```csharp
using Kcm;

using var db = new KcmDatabase();

var fact = new Fact
{
    Subject = 1,
    Predicate = 2,
    Object = 3,
    Confidence = 0.95,
    Evidence = 1,
    Context = 1,
    Priority = 0,
    Owner = 1
};

db.Insert(fact);

Console.WriteLine($"Total facts: {db.FactCount()}");
Console.WriteLine($"Active facts: {db.ActiveCount()}");
```

## API Reference

### `Fact`

Knowledge fact with 10 attributes.

```csharp
public class Fact
{
    public uint Subject { get; set; }
    public byte Predicate { get; set; }
    public uint Object { get; set; }
    public double Confidence { get; set; }
    public byte Evidence { get; set; }
    public long Timestamp { get; set; }
    public byte Context { get; set; }
    public int Version { get; set; }
    public sbyte Priority { get; set; }
    public ushort Owner { get; set; }
}
```

### `KcmDatabase`

Main entry point for KCM operations. Implements `IDisposable`.

```csharp
public class KcmDatabase : IDisposable
{
    public KcmDatabase();
    public void Dispose();

    public void Insert(Fact fact);
    public void Update(ulong rowId, Fact fact);
    public void Delete(ulong rowId);

    public ulong FactCount();
    public ulong ActiveCount();

    public KcmQuery Query(string kql);
    public KcmTransaction BeginTransaction();

    public void Save(string path);
    public void Load(string path);

    public static void Verify(string path);
}
```

#### Constructor

```csharp
using var db = new KcmDatabase();
```

Creates a new in-memory database. Throws `KcmError` on failure.

#### `Insert`

```csharp
public void Insert(Fact fact);
```

Insert a fact. Throws `KcmError` on failure.

#### `Update`

```csharp
public void Update(ulong rowId, Fact fact);
```

Update a fact by row ID. Throws `KcmError` if not found.

#### `Delete`

```csharp
public void Delete(ulong rowId);
```

Delete a fact by row ID. Throws `KcmError` if not found.

#### `FactCount`

```csharp
public ulong FactCount();
```

Returns total fact count (including deleted).

#### `ActiveCount`

```csharp
public ulong ActiveCount();
```

Returns active (non-deleted) fact count.

#### `Query`

```csharp
public KcmQuery Query(string kql);
```

Execute a KQL query string. Returns a `KcmQuery` for enumeration.

#### `BeginTransaction`

```csharp
public KcmTransaction BeginTransaction();
```

Begin a new transaction.

#### `Save`

```csharp
public void Save(string path);
```

Save the database to a file. Throws `KcmError` on I/O failure.

#### `Load`

```csharp
public void Load(string path);
```

Load a database from a file. Throws `KcmError` on I/O or corruption.

#### `Verify`

```csharp
public static void Verify(string path);
```

Verify database file integrity. Throws `KcmError` if corrupted.

### `KcmQuery`

Query result enumerator.

```csharp
public class KcmQuery : IEnumerable<Fact>, IDisposable
{
    public IEnumerator<Fact> GetEnumerator();
    public void Dispose();
}
```

### `KcmTransaction`

Transaction handle. Implements `IDisposable`.

```csharp
public class KcmTransaction : IDisposable
{
    public void Commit();
    public void Rollback();
    public void Dispose();
}
```

#### `Commit`

```csharp
public void Commit();
```

Commit the transaction. Throws `KcmError` on failure.

#### `Rollback`

```csharp
public void Rollback();
```

Rollback the transaction. Safe to call multiple times.

### `KcmError`

Exception type for KCM errors.

```csharp
public class KcmError : Exception
{
    public KcmErrorCode Code { get; }
}
```

### `KcmErrorCode`

```csharp
public enum KcmErrorCode
{
    Ok = 0,
    NotFound = 1,
    OutOfMemory = 2,
    InvalidArgument = 3,
    Io = 4,
    Corrupted = 5,
    Conflict = 6,
    TransactionAborted = 7,
}
```

## Error Handling

All errors are reported via `KcmError` exceptions:

```csharp
try
{
    using var db = new KcmDatabase();
    db.Insert(fact);
}
catch (KcmError e)
{
    Console.WriteLine($"Error [{e.Code}]: {e.Message}");
}
```

| `KcmErrorCode` | HTTP Status | Description |
|-----------------|-------------|-------------|
| `NotFound` | 404 | Requested row ID not found |
| `OutOfMemory` | 507 | Memory allocation failed |
| `InvalidArgument` | 400 | Invalid argument |
| `Io` | 500 | I/O error |
| `Corrupted` | 500 | Data corruption detected |
| `Conflict` | 409 | Conflict (e.g., duplicate key) |
| `TransactionAborted` | 409 | Transaction was aborted |

## Example Code

### Transactions

```csharp
using var db = new KcmDatabase();
using var txn = db.BeginTransaction();

try
{
    db.Insert(new Fact { Subject = 1, Predicate = 2, Object = 3, Confidence = 0.9 });
    txn.Commit();
}
catch (KcmError)
{
    txn.Rollback();
    throw;
}
```

### Query with LINQ

```csharp
using var db = new KcmDatabase();
var facts = db.Query("SELECT * FROM facts").ToList();

foreach (var fact in facts)
{
    Console.WriteLine($"Subject={fact.Subject}, Object={fact.Object}, Confidence={fact.Confidence}");
}
```

### Save and Load

```csharp
using (var db = new KcmDatabase())
{
    db.Insert(new Fact { Subject = 1, Predicate = 2, Object = 3, Confidence = 0.95 });
    db.Save("knowledge.kcm");
}

using (var db = new KcmDatabase())
{
    db.Load("knowledge.kcm");
    Console.WriteLine($"Loaded {db.FactCount()} facts");
}
```

### Integrity Verification

```csharp
try
{
    KcmDatabase.Verify("knowledge.kcm");
    Console.WriteLine("Database is valid");
}
catch (KcmError e)
{
    Console.WriteLine($"Corruption detected: {e.Message}");
}
```

## Benchmark

Build and run the benchmark suite:

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
