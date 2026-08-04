# kcm-distributed

Distributed architecture for KCM: sharding strategies and 2PC transaction coordinator.

## Purpose

Enables horizontal scaling of KCM across multiple nodes via consistent hashing, range partitioning, and two-phase commit for distributed transactions.

## Modules

| Module | Purpose |
|--------|---------|
| `sharding` | Sharding strategies (Hash, Range, ConsistentHash) |
| `coordinator` | Two-phase commit transaction coordinator |
| `transport` | Inter-node communication transport |

## Dependencies

| Dependency | Purpose |
|------------|---------|
| `kcm-core` | Core types |
| `parking_lot` | Thread-safe state |

## Sharding Strategies

| Strategy | Description | Use Case |
|----------|-------------|----------|
| Hash | Hash(shard_key) % num_shards | Uniform distribution |
| Range | Key ranges assigned to shards | Range queries |
| ConsistentHash | Virtual nodes on hash ring | Minimal resharding on scale |

## Transaction Model

Two-phase commit (2PC):
```
Phase 1 (Prepare):
  Coordinator -> All shards: PREPARE(txn_id)
  Each shard: lock resources, write WAL, respond PREPARED or ABORT

Phase 2 (Commit):
  Coordinator -> All shards: COMMIT(txn_id)
  Each shard: apply changes, release locks, respond COMMITTED
```

Failure handling:
- Coordinator timeout: abort and release locks
- Shard failure: coordinator retries, then aborts
- Network partition: timeout-based abort

## Usage

```rust
use kcm_distributed::sharding::{ShardMap, ShardStrategy};
use kcm_distributed::coordinator::TransactionCoordinator;

let shard_map = ShardMap::new(ShardStrategy::ConsistentHash, 16);
let coordinator = TransactionCoordinator::new(shard_map);

let txn = coordinator.begin_transaction()?;
coordinator.prepare(&txn.id, &shard_nodes)?;
coordinator.commit(&txn.id, &shard_nodes)?;
```
