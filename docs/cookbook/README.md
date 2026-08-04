# KCM Cookbook

Practical code recipes for common tasks.

## Quick Recipes

| Recipe | Description | Language |
|--------|-------------|----------|
| create-database.md | Create and configure a database | Rust |
| insert-facts.md | Insert knowledge facts | Rust |
| query-kql.md | Execute KQL queries | Rust |
| use-transactions.md | Transaction management | Rust |
| enable-encryption.md | Enable AES-256-GCM | Rust |
| setup-monitoring.md | Configure Prometheus | YAML |
| docker-compose.md | Local development setup | Docker |
| kubernetes.md | Production K8s deployment | YAML |

## Rust Recipes

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
let results = db.query("SELECT * FROM facts WHERE confidence > 0.9")?;
```

## REST API Recipes

### Insert via cURL

```bash
curl -X POST http://localhost:8080/api/facts \
  -H "Content-Type: application/json" \
  -d '{"subject": "a", "predicate": "b", "object": "c", "confidence": 0.9}'
```

### Query via cURL

```bash
curl "http://localhost:8080/api/query?kql=SELECT * FROM facts"
```

## Docker Recipes

### Development Setup

```yaml
version: '3.8'
services:
  kcm:
    build: .
    ports:
      - "8080:8080"
    volumes:
      - ./data:/data
    environment:
      - RUST_LOG=debug
```

### Production Setup

```yaml
version: '3.8'
services:
  kcm:
    image: kcm:latest
    deploy:
      replicas: 3
    ports:
      - "8080:8080"
    volumes:
      - kcm_data:/data
    environment:
      - RUST_LOG=info
volumes:
  kcm_data:
    driver: local
```
