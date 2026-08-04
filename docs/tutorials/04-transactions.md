# Tutorial 04: Transactions

## Objective

Learn how to use transactions for atomic operations.

## What are Transactions?

Transactions ensure that a group of operations either all succeed or all fail. This maintains data consistency.

## ACID Properties

- **Atomicity**: All operations in a transaction succeed or all fail
- **Consistency**: Database remains in a valid state
- **Isolation**: Concurrent transactions don't interfere
- **Durability**: Committed changes persist

## Using Transactions

### Via REST API

```bash
# Begin transaction
curl -X POST http://localhost:8080/api/transactions/begin

# Insert facts (with transaction ID)
curl -X POST http://localhost:8080/api/facts \
  -H "X-Transaction-ID: <txn_id>" \
  -d '{"subject": "a", "predicate": "relates", "object": "b", "confidence": 0.9}'

# Commit
curl -X POST http://localhost:8080/api/transactions/<txn_id>/commit
```

### Via Rust

```rust
let mut txn = db.begin_transaction();
txn.insert(fact1)?;
txn.insert(fact2)?;
txn.commit()?;
```

### Rollback

```rust
let mut txn = db.begin_transaction();
txn.insert(fact)?;
// Something went wrong
txn.rollback()?;
// fact is NOT inserted
```

## When to Use Transactions

| Scenario | Use Transaction? |
|----------|-----------------|
| Single insert | No |
| Multiple related inserts | Yes |
| Insert + delete | Yes |
| Read-only query | No |

## Next Steps

- Tutorial 05: Learn about reasoning
