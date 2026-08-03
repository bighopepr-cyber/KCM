# KCM Python SDK

Python bindings for KCM via PyO3.

## Status: Planned

## Architecture

- PyO3-based bindings to `kcm-interface`
- Feature-gated: `python` feature in `kcm-interface`
- Package: `kcm` on PyPI

## API Design

```python
import kcm

# Open database
db = kcm.Database("my_knowledge.db")

# Insert fact
db.insert(subject="planet", predicate="orbits", object="sun", confidence=0.99)

# Query
results = db.query("SELECT * FROM facts WHERE subject = 'planet'")
for fact in results:
    print(fact.subject, fact.predicate, fact.object, fact.confidence)

# Transaction
txn = db.begin_transaction()
txn.insert(subject="moon", predicate="orbits", object="earth", confidence=0.999)
txn.commit()

# Close
db.close()
```

## Installation

```bash
pip install kcm
```

## Examples

See `examples/python/` for complete examples.
