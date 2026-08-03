# KCM Java SDK

Java bindings for KCM via JNI.

## Status: Planned

## Architecture

- JNI bindings to `kcm-interface` C API
- Package: `io.kcm:sdk` on Maven Central

## API Design

```java
import io.kcm.KcmDatabase;
import io.kcm.Fact;

KcmDatabase db = new KcmDatabase("my_knowledge.db");
Fact fact = new Fact("planet", "orbits", "sun", 0.99);
db.insert(fact);
List<Fact> results = db.query("SELECT * FROM facts WHERE subject = 'planet'");
db.close();
```

## Installation

```xml
<dependency>
    <groupId>io.kcm</groupId>
    <artifactId>sdk</artifactId>
    <version>0.1.0</version>
</dependency>
```
