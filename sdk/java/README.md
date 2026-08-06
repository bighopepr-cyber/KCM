# KCM Java SDK

Official SDK for the KCM Knowledge Columnar Model.

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![Version](https://img.shields.io/badge/version-0.1.0-blue)]()
[![License](https://img.shields.io/badge/license-MIT-green)]()
[![KCM Engine Compatible](https://img.shields.io/badge/KCM%20Engine-0.1.0-orange)]()

## Installation

Maven:

```xml
<dependency>
    <groupId>io.kcm</groupId>
    <artifactId>sdk</artifactId>
    <version>0.1.0</version>
</dependency>
```

Gradle:

```groovy
implementation 'io.kcm:sdk:0.1.0'
```

## Quickstart

```java
import io.kcm.KcmDatabase;
import io.kcm.Fact;

KcmDatabase db = new KcmDatabase("my_knowledge.db");

Fact fact = new Fact(1, 0, 2, 0.95);
db.insert(fact);

List<Fact> results = db.queryAll();
for (Fact f : results) {
    System.out.println("Subject: " + f.getSubject() + ", Object: " + f.getObject());
}

System.out.println("Fact count: " + db.factCount());
db.close();
```

## API Reference

| Operation | Signature | Description |
|-----------|-----------|-------------|
| `KcmDatabase(path)` | Constructor | Open or create a database |
| `insert(fact)` | `void insert(Fact fact)` | Insert a fact |
| `update(rowId, fact)` | `void update(long rowId, Fact fact)` | Update a fact by row ID |
| `delete(rowId)` | `void delete(long rowId)` | Delete a fact by row ID |
| `query(kql)` | `List<Fact> query(String kql)` | Execute KQL query |
| `queryAll()` | `List<Fact> queryAll()` | Get all active facts |
| `getFact(rowId)` | `Fact getFact(long rowId)` | Retrieve a fact by ID |
| `factCount()` | `long factCount()` | Total fact count |
| `activeFactCount()` | `long activeFactCount()` | Active fact count |
| `beginTransaction()` | `Transaction beginTransaction()` | Begin transaction |
| `save(path)` | `void save(String path)` | Save to file |
| `load(path)` | `void load(String path)` | Load from file |
| `verify(path)` | `static void verify(String path)` | Verify file integrity |
| `close()` | `void close()` | Close database |

## Error Handling

All operations throw `KcmException` on error:

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

```java
import io.kcm.KcmDatabase;
import io.kcm.Fact;
import java.util.List;

KcmDatabase db = new KcmDatabase("knowledge.db");

db.insert(new Fact(1, 0, 2, 0.95));
db.insert(new Fact(2, 1, 3, 0.90));

List<Fact> results = db.queryAll();
for (Fact fact : results) {
    System.out.println("Subject: " + fact.getSubject() + ", Object: " + fact.getObject());
}

db.close();
```

### API Integration

```java
import io.kcm.KcmDatabase;
import io.kcm.Fact;
import java.util.List;

public static List<Fact> fetchKnowledge(String dbPath) {
    try (KcmDatabase db = new KcmDatabase(dbPath)) {
        return db.queryAll();
    }
}
```

### Transaction

```java
import io.kcm.KcmDatabase;
import io.kcm.Fact;

KcmDatabase db = new KcmDatabase("knowledge.db");
var txn = db.beginTransaction();

db.insert(new Fact(10, 0, 20, 0.85));

if (db.activeFactCount() > 0) {
    txn.commit();
} else {
    txn.rollback();
}

db.close();
```

## Full Documentation

See [docs/sdk/java.md](../../docs/sdk/java.md) for the complete reference.

## License

MIT

Made by bighopepr-cyber
