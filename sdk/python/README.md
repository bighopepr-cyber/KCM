# KCM Python SDK

Python bindings for the KCM Knowledge Columnar Model.

## Status

**Planned** — This SDK is not yet published. Installation via `pip install kcm` is not currently available.

## Installation

```bash
pip install kcm
```

## Quick Start

```python
import kcm

# Create a database
db = kcm.Database()

# Insert facts
db.insert(subject=1, predicate=0, object=2, confidence=0.95)
db.insert(subject=2, predicate=1, object=3, confidence=0.90)

# Query all facts
facts = db.query_all()
for subject, predicate, object, confidence in facts:
    print(f"Subject: {subject}, Predicate: {predicate}, Object: {object}, Confidence: {confidence}")

# Check statistics
print(f"Total facts: {db.fact_count()}")
print(f"Active facts: {db.active_fact_count()}")
```

## API Reference

### Database

| Method | Description |
|--------|-------------|
| `Database()` | Create a new in-memory database |
| `insert(subject, predicate, object, confidence)` | Insert a fact |
| `query_all()` | Query all facts |
| `fact_count()` | Get total fact count |
| `active_fact_count()` | Get active fact count |

### Fact

Facts are returned as tuples: `(subject, predicate, object, confidence)`

## Development

```bash
# Install maturin
pip install maturin

# Build and install
maturin develop

# Run tests
pytest tests/
```

## License

MIT
