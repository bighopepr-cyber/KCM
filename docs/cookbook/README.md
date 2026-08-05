# KCM Cookbook

**Document ID:** COOKBOOK-INDEX-001
**Version:** 2.0.0
**Status:** Active
**Owner:** Runtime/Interface Owner

This cookbook is the operational recipe set for the current implementation. It provides executable examples that correspond to the server and runtime APIs present in the repository today and is limited to the current runtime surface.

## Current Recipe Set

| Recipe | Scope | Format |
|--------|-------|--------|
| docker-compose.md | Local deployment with Docker Compose | Docker |
| kubernetes.md | Stateful deployment with Kubernetes | YAML |

## Runtime Example

### Create Database

```rust
use kcm_runtime::database::KnowledgeDatabase;

let db = KnowledgeDatabase::new()?;
```

### Insert Fact

```rust
use kcm_core::types::*;

let fact = Fact::new(
    SubjectID(1),
    PredicateID(1),
    ObjectID(1),
    0.95,
)?;
db.insert(&fact)?;
```

### Query

```rust
let results = db.query().with_confidence(0.9).execute()?;
```

## REST Example

### Insert via cURL

```bash
curl -X POST http://localhost:8080/facts \
  -H "Content-Type: application/json" \
  -d '{"subject":1,"predicate":1,"object":1,"confidence":0.9}'
```

### Query via cURL

```bash
curl "http://localhost:8080/facts?confidence_min=0.9"
```

## Deployment Examples

The deployment examples in this directory are intentionally limited to the runtime artifacts present in the repository and reflect the current deployment surface only.
