# KCM .NET SDK

.NET bindings for KCM via P/Invoke.

## Status: Planned

## Architecture

- P/Invoke to `kcm-interface` C API
- Package: `Kcm.Sdk` on NuGet

## API Design

```csharp
using Kcm;

using var db = new KcmDatabase("my_knowledge.db");
var fact = new Fact { Subject = "planet", Predicate = "orbits", Object = "sun", Confidence = 0.99 };
db.Insert(fact);
var results = db.Query("SELECT * FROM facts WHERE subject = 'planet'");
```

## Installation

```bash
dotnet add package Kcm.Sdk
```
