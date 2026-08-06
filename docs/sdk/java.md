# KCM Java SDK

Official SDK for the KCM Knowledge Columnar Model

<!-- badges:start -->
[![Build Status](https://img.shields.io/badge/build-pending-lightgrey)]()
[![Version](https://img.shields.io/badge/version-1.0.0-blue)]()
[![License](https://img.shields.io/badge/license-MIT-green)]()
<!-- badges:end -->

The Java SDK wraps the KCM C FFI via JNA, providing a Java-native API.

## Installation

### Maven

```xml
<dependency>
    <groupId>io.kcm</groupId>
    <artifactId>sdk</artifactId>
    <version>1.0.0</version>
</dependency>
```

### Gradle

```groovy
implementation 'io.kcm:sdk:1.0.0'
```

## Quickstart

```java
import io.kcm.*;

public class Main {
    public static void main(String[] args) throws KcmException {
        try (KcmDatabase db = new KcmDatabase()) {
            Fact fact = new Fact();
            fact.setSubject(1);
            fact.setPredicate(2);
            fact.setObject(3);
            fact.setConfidence(0.95);

            db.insert(fact);

            System.out.println("Total facts: " + db.factCount());
            System.out.println("Active facts: " + db.activeCount());
        }
    }
}
```

## API Reference

### `Fact`

Knowledge fact with 10 attributes.

```java
public class Fact {
    public int getSubject();
    public void setSubject(int subject);

    public byte getPredicate();
    public void setPredicate(byte predicate);

    public int getObject();
    public void setObject(int object);

    public double getConfidence();
    public void setConfidence(double confidence);

    public byte getEvidence();
    public void setEvidence(byte evidence);

    public long getTimestamp();
    public void setTimestamp(long timestamp);

    public byte getContext();
    public void setContext(byte context);

    public int getVersion();
    public void setVersion(int version);

    public byte getPriority();
    public void setPriority(byte priority);

    public short getOwner();
    public void setOwner(short owner);
}
```

### `KcmDatabase`

Main entry point for KCM operations. Implements `AutoCloseable`.

```java
public class KcmDatabase implements AutoCloseable {
    public KcmDatabase() throws KcmException;
    public void close();

    public long insert(Fact fact) throws KcmException;
    public void update(long rowId, Fact fact) throws KcmException;
    public void delete(long rowId) throws KcmException;

    public long factCount();
    public long activeCount();

    public KcmQuery query(String kql) throws KcmException;
    public KcmTransaction beginTransaction() throws KcmException;

    public void save(String path) throws KcmException;
    public void load(String path) throws KcmException;

    public static void verify(String path) throws KcmException;
}
```

#### Constructor

```java
KcmDatabase db = new KcmDatabase();
```

Create a new in-memory database. Throws `KcmException` on failure.

#### `insert`

```java
public long insert(Fact fact) throws KcmException;
```

Insert a fact. Returns the assigned row ID.

#### `update`

```java
public void update(long rowId, Fact fact) throws KcmException;
```

Update a fact by row ID. Throws `KcmException(KcmErrorCode.NOT_FOUND)` if not found.

#### `delete`

```java
public void delete(long rowId) throws KcmException;
```

Delete a fact by row ID. Throws `KcmException(KcmErrorCode.NOT_FOUND)` if not found.

#### `factCount`

```java
public long factCount();
```

Returns total fact count (including deleted).

#### `activeCount`

```java
public long activeCount();
```

Returns active (non-deleted) fact count.

#### `query`

```java
public KcmQuery query(String kql) throws KcmException;
```

Execute a KQL query. Returns a `KcmQuery` for iteration.

#### `beginTransaction`

```java
public KcmTransaction beginTransaction() throws KcmException;
```

Begin a new transaction.

#### `save`

```java
public void save(String path) throws KcmException;
```

Save the database to a file.

#### `load`

```java
public void load(String path) throws KcmException;
```

Load a database from a file.

#### `verify`

```java
public static void verify(String path) throws KcmException;
```

Verify database file integrity.

### `KcmQuery`

Query result iterator. Implements `Iterable<Fact>` and `AutoCloseable`.

```java
public class KcmQuery implements Iterable<Fact>, AutoCloseable {
    public boolean hasNext();
    public Fact next();
    public void close();
}
```

### `KcmTransaction`

Transaction handle. Implements `AutoCloseable`.

```java
public class KcmTransaction implements AutoCloseable {
    public void commit() throws KcmException;
    public void rollback() throws KcmException;
    public void close();
}
```

### `KcmException`

Exception type for KCM errors.

```java
public class KcmException extends Exception {
    public KcmErrorCode getCode();
}
```

### `KcmErrorCode`

```java
public enum KcmErrorCode {
    OK,
    NOT_FOUND,
    OUT_OF_MEMORY,
    INVALID_ARGUMENT,
    IO,
    CORRUPTED,
    CONFLICT,
    TRANSACTION_ABORTED,
}
```

## Error Handling

All errors are reported via `KcmException`:

```java
try {
    KcmDatabase db = new KcmDatabase();
    db.insert(fact);
} catch (KcmException e) {
    System.err.println("Error [" + e.getCode() + "]: " + e.getMessage());
}
```

| `KcmErrorCode` | Description |
|-----------------|-------------|
| `NOT_FOUND` | Requested row ID not found |
| `OUT_OF_MEMORY` | Memory allocation failed |
| `INVALID_ARGUMENT` | Invalid argument |
| `IO` | I/O error |
| `CORRUPTED` | Data corruption detected |
| `CONFLICT` | Conflict (e.g., duplicate key) |
| `TRANSACTION_ABORTED` | Transaction was aborted |

## Example Code

### Transactions

```java
try (KcmDatabase db = new KcmDatabase();
     KcmTransaction txn = db.beginTransaction()) {

    Fact fact = new Fact();
    fact.setSubject(1);
    fact.setPredicate(2);
    fact.setObject(3);
    fact.setConfidence(0.9);

    db.insert(fact);
    txn.commit();
} catch (KcmException e) {
    System.err.println("Transaction failed: " + e.getMessage());
}
```

### Query Iteration

```java
try (KcmDatabase db = new KcmDatabase();
     KcmQuery query = db.query("SELECT * FROM facts")) {

    while (query.hasNext()) {
        Fact fact = query.next();
        System.out.printf("Subject=%d, Object=%d, Confidence=%.2f%n",
            fact.getSubject(), fact.getObject(), fact.getConfidence());
    }
}
```

### For-Each Loop

```java
try (KcmDatabase db = new KcmDatabase();
     KcmQuery query = db.query("SELECT * FROM facts")) {

    for (Fact fact : query) {
        System.out.println("Subject=" + fact.getSubject());
    }
}
```

### Save and Load

```java
try (KcmDatabase db = new KcmDatabase()) {
    Fact fact = new Fact();
    fact.setSubject(1);
    fact.setPredicate(2);
    fact.setObject(3);
    fact.setConfidence(0.95);
    db.insert(fact);
    db.save("knowledge.kcm");
}

try (KcmDatabase db = new KcmDatabase()) {
    db.load("knowledge.kcm");
    System.out.println("Loaded " + db.factCount() + " facts");
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
