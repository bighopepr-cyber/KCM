# kcm-distributed Technical Specification

---

## Overview

`kcm-distributed` is the distributed architecture crate for the KCM (Knowledge Columnar Model) system. It provides sharding strategies for horizontal data partitioning, a two-phase commit (2PC) coordinator for distributed transactions, replication for data durability, and a transport layer for inter-node communication. The crate is designed to enable KCM to scale across multiple nodes while maintaining data consistency and availability.

## Scope

This specification covers the implementation of distributed coordination within the KCM system, including:

- Data partitioning across cluster nodes via configurable sharding strategies
- Atomic distributed transaction execution via two-phase commit protocol
- Data replication across replicas for durability and availability
- Secure, authenticated transport between cluster nodes

## Responsibilities

| Responsibility | Description |
|----------------|-------------|
| Sharding | Partition data across nodes using Hash, Range, or ConsistentHash strategies |
| 2PC Coordination | Execute distributed transactions atomically via prepare/commit/abort phases |
| Replication | Maintain replica copies with configurable consistency (sync/async) |
| Transport | Provide reliable, authenticated message passing between cluster nodes |

## Technical Specification

### Sharding

Three sharding strategies are provided, each suited to different access patterns:

| Strategy | Hash | Range | ConsistentHash |
|----------|------|-------|----------------|
| Partitioning | Consistent hash of key → shard | Key range boundaries → shard | Virtual nodes on hash ring → shard |
| Rebalancing | Requires full remap | Shift boundary between shards | Minimal key movement (1/K of keys) |
| Hot spots | Probabilistic uniform | Range skew possible | Probabilistic uniform |
| Use case | General purpose | Range queries | Frequent node add/remove |

**Hash Sharding**: The subject ID is hashed via a deterministic function and modulo'd against the shard count. Uniform distribution is probabilistic. Adding/removing nodes requires remapping ~1/N of all keys.

**Range Sharding**: Key space is divided into contiguous ranges, each assigned to a node. Range boundaries are stored in the shard map. Optimized for range queries but vulnerable to hot spots when data distribution is skewed.

**Consistent Hashing**: Keys are mapped to positions on a virtual ring. Each physical node owns a configurable number of virtual nodes (default: 150). Adding or removing a node moves only ~1/K of keys, making rebalancing efficient. This is the default strategy.

### Coordinator

The 2PC coordinator manages distributed transaction lifecycle across participating nodes:

| Phase | Action | State |
|-------|--------|-------|
| Prepare | Coordinator sends prepare request to all participants | PREPARING |
| Vote | Each participant votes yes (prepared) or no (aborted) | VOTING |
| Commit | If all voted yes, coordinator sends commit; otherwise sends abort | COMMITTING / ABORTING |
| Ack | Participants acknowledge commit/abort and release locks | ACKNOWLEDGING |
| Complete | Coordinator records transaction as complete | COMPLETE |

**Durability guarantee**: The coordinator writes the decision (commit/abort) to a persistent log before sending the decision to participants. This ensures recovery after coordinator failure can replay the correct decision.

**Timeout handling**: If a participant does not respond within the configured timeout, the coordinator aborts the transaction. Participants that timeout during prepare are treated as abort votes.

**Idempotency**: Participant prepare and commit operations are idempotent, identified by transaction ID. Duplicate requests are safely ignored.

### Replication

Replication maintains data copies across multiple nodes for durability:

| Mode | Behavior | Trade-off |
|------|----------|-----------|
| Synchronous | Write waits for all replicas to confirm | Higher latency, stronger consistency |
| Asynchronous | Write returns after primary confirms; replicas follow | Lower latency, eventual consistency |

**Conflict resolution**: When async replication detects conflicting writes, the last-writer-wins (LWW) strategy is used based on timestamp ordering. Version vectors are maintained to detect concurrent modifications.

**Replication log**: Each node maintains a hash-chained replication log. Log entries contain the operation, version, and a hash of the previous entry for tamper detection.

### Transport

The transport layer provides message passing between cluster nodes:

| Feature | Description |
|---------|-------------|
| Message types | ShardMap, TransactionPrepare, TransactionCommit, TransactionAbort, ReplicationStream, Heartbeat |
| Serialization | Bincode-based binary format for efficiency |
| Authentication | Mutual TLS with node certificate validation |
| Reliability | TCP-based with application-level acknowledgment |
| Heartbeat | Configurable interval (default: 5s); nodes not responding within 3× heartbeat are marked offline |

## Architecture

The crate follows a modular architecture with four independent components:

```
kcm-distributed
├── lib.rs              — Crate root, re-exports public API
├── sharding.rs         — Shard strategies and shard map management
├── coordinator.rs      — 2PC transaction coordinator
├── replication.rs      — Sync/async replication engine
└── transport.rs        — Inter-node message transport
```

### Internal Components

#### sharding.rs

Implements the `ShardStrategy` trait with three concrete strategies:

```rust
pub enum ShardStrategyType {
    Hash,
    Range,
    ConsistentHash { virtual_nodes: usize },
}
```

The `ShardMap` struct holds the current assignment of shards to nodes and provides O(1) lookups via a precomputed mapping table. Shard maps are versioned and validated before activation.

#### coordinator.rs

The `TransactionCoordinator` manages the 2PC protocol:

- Maintains a persistent commit log for crash recovery
- Tracks in-flight transactions and participant states
- Enforces timeouts and handles participant failures
- Provides transaction ID generation and state machine transitions

#### replication.rs

The `ReplicationManager` handles data replication:

- Manages replication streams between primary and replica nodes
- Tracks replication lag per replica
- Handles conflict resolution for concurrent writes
- Maintains the hash-chained replication log

#### transport.rs

The `TransportLayer` provides inter-node communication:

- TCP connection management with TLS
- Message serialization/deserialization
- Connection pooling and health monitoring
- Heartbeat-based node liveness detection

## Data Model

### ShardMap

```rust
pub struct ShardMap {
    version: u64,
    strategy: ShardStrategyType,
    shard_count: usize,
    assignments: Vec<NodeId>,      // shard_id → node_id
    node_shards: HashMap<NodeId, Vec<u32>>,  // node_id → shard_ids
}
```

### TransactionCoordinator

```rust
pub struct TransactionCoordinator {
    coordinator_id: NodeId,
    commit_log: Vec<CommitLogEntry>,
    in_flight: HashMap<TransactionId, TransactionState>,
    participants: HashMap<TransactionId, Vec<NodeId>>,
    config: CoordinatorConfig,
}
```

### ReplicationLog

```rust
pub struct ReplicationLogEntry {
    index: u64,
    operation: ReplicationOp,
    version: Version,
    timestamp: i64,
    previous_hash: [u8; 32],
    entry_hash: [u8; 32],
}
```

### TransportMessage

```rust
pub enum TransportMessage {
    ShardMapUpdate(ShardMap),
    TransactionPrepare(TransactionId, Vec<ColumnBlock>),
    TransactionVote(TransactionId, Vote),
    TransactionCommit(TransactionId),
    TransactionAbort(TransactionId),
    ReplicationStream(Vec<ReplicationLogEntry>),
    Heartbeat(NodeId, ClusterState),
}
```

## Execution Flow

### 2PC Commit Flow

```
Client → Coordinator → Participant A, B, C
  1. Client submits transaction to coordinator
  2. Coordinator writes PREPARING to commit log
  3. Coordinator sends TransactionPrepare to all participants
  4. Each participant acquires locks, writes prepare record, responds with Vote::Yes
  5. Coordinator receives all Yes votes
  6. Coordinator writes COMMIT to commit log (durable)
  7. Coordinator sends TransactionCommit to all participants
  8. Each participant applies changes, releases locks, responds with Ack
  9. Coordinator writes COMPLETE to commit log
  10. Coordinator responds to client with success
```

### Shard Routing

```
Query(key) → ShardRouter
  1. Extract key from query
  2. Apply shard strategy to compute shard_id
  3. Look up node_id from ShardMap[shard_id]
  4. If local node: execute locally
  5. If remote node: send via TransportLayer to target node
  6. Receive result and return to caller
```

### Replication Flow

```
Write(data) → Primary → Replicas
  1. Primary receives write operation
  2. Primary appends to local WAL
  3. Primary creates ReplicationLogEntry with hash chain
  4. If synchronous: send to replicas, wait for Ack before returning
  5. If asynchronous: send to replicas, return immediately
  6. Replicas receive ReplicationLogEntry
  7. Replicas validate hash chain integrity
  8. Replicas apply operation to local storage
  9. Replicas send Ack to primary
  10. Primary updates replication lag metrics
```

## Public API

```rust
// Sharding
pub fn create_shard_map(strategy: ShardStrategyType, node_count: usize, shard_count: usize) -> Result<ShardMap, KcmError>
pub fn route_key(shard_map: &ShardMap, key: SubjectID) -> Result<NodeId, KcmError>
pub fn rebalance_shards(shard_map: &ShardMap, nodes: &[NodeId]) -> Result<ShardMap, KcmError>

// Coordinator
pub fn begin_transaction(coordinator: &TransactionCoordinator, participants: Vec<NodeId>) -> Result<TransactionId, KcmError>
pub fn prepare_transaction(coordinator: &TransactionCoordinator, tx_id: TransactionId, data: Vec<ColumnBlock>) -> Result<(), KcmError>
pub fn commit_transaction(coordinator: &TransactionCoordinator, tx_id: TransactionId) -> Result<(), KcmError>
pub fn abort_transaction(coordinator: &TransactionCoordinator, tx_id: TransactionId) -> Result<(), KcmError>

// Replication
pub fn start_replication(manager: &ReplicationManager, primary: NodeId, replicas: Vec<NodeId>) -> Result<(), KcmError>
pub fn replicate_write(manager: &ReplicationManager, entry: ReplicationLogEntry, sync: bool) -> Result<(), KcmError>
pub fn get_replication_lag(manager: &ReplicationManager, replica: NodeId) -> Result<u64, KcmError>

// Transport
pub fn send_message(layer: &TransportLayer, target: NodeId, msg: TransportMessage) -> Result<(), KcmError>
pub fn broadcast_message(layer: &TransportLayer, msg: TransportMessage) -> Result<(), KcmError>
pub fn node_status(layer: &TransportLayer, node: NodeId) -> Result<NodeStatus, KcmError>
```

## Configuration

| Parameter | Default | Range | Description |
|-----------|---------|-------|-------------|
| `shard_count` | 16 | 1–1024 | Number of shards in the cluster |
| `virtual_nodes` | 150 | 10–1000 | Virtual nodes per physical node (consistent hash) |
| `2pc_timeout_ms` | 5000 | 1000–60000 | Timeout for 2PC phases |
| `heartbeat_interval_ms` | 5000 | 1000–30000 | Heartbeat frequency between nodes |
| `replication_mode` | Async | Sync/Async | Default replication consistency mode |
| `commit_log_path` | `./commit_log` | — | Path for persistent 2PC commit log |
| `max_replication_lag_ms` | 10000 | 1000–120000 | Maximum acceptable replication lag before warning |

## Dependencies

| Dependency | Version | Purpose |
|------------|---------|---------|
| kcm-core | — | Core types (Fact, RowID, SubjectID, KcmError, DenseVec) |
| kcm-security | — | Node authentication, RBAC, encryption |
| parking_lot | — | RwLock, Mutex (3–5× faster than std) |
| rayon | — | Work-stealing thread pool for parallel operations |
| thiserror | — | Error derive macro |
| log | — | Logging facade |
| blake3 | — | Hash chain integrity for replication logs |

## Error Handling

All errors map to the `KcmError` hierarchy:

| Error | Usage in kcm-distributed |
|-------|--------------------------|
| `KcmError::NotFound(String)` | Shard not found, node not in cluster |
| `KcmError::InvalidArgument(String)` | Invalid shard strategy, malformed shard map |
| `KcmError::Conflict(String)` | 2PC conflict, shard assignment conflict |
| `KcmError::TransactionAborted` | 2PC abort (participant voted no or timeout) |
| `KcmError::Io(String)` | Transport errors, commit log I/O failures |
| `KcmError::Corrupted(String)` | Replication log integrity failure, shard map corruption |

## Performance Characteristics

| Operation | Target | Notes |
|-----------|--------|-------|
| Shard map lookup | O(1) | Precomputed assignment table |
| Shard routing decision | < 1μs | No allocation on hot path |
| 2PC prepare round-trip | < 10ms (p99) | Network latency dependent |
| 2PC commit round-trip | < 10ms (p99) | After durable write |
| Replication throughput | > 10K ops/s per replica | Async mode |
| Replication lag | < 100ms (p99) | Under normal load |
| Shard rebalance (consistent hash) | O(K log N) | K = keys, N = nodes |

## Security Considerations

### Network Security

All inter-node communication uses mutual TLS. Certificates are managed via kcm-security infrastructure. No plaintext communication is permitted in any environment.

### Node Authentication

Every node presents a certificate signed by the cluster CA. The coordinator validates certificates before allowing participation in 2PC rounds. Unauthenticated connections are rejected immediately.

### Data Protection

Shard maps and replication logs are integrity-verified using blake3 hashes. The 2PC commit log uses hash chaining for tamper detection. Cluster state is encrypted at rest.

## Integration

### With kcm-core

Uses core types (`SubjectID`, `Fact`, `KcmError`, `DenseVec`, `Bitmap`) for data representation. Shard strategies operate on `SubjectID` for routing decisions.

### With kcm-security

Uses `ACLManager` for node authorization, encryption primitives for TLS, and audit logging for security events. All inter-node authentication flows through kcm-security.

### With kcm-runtime

The `KnowledgeDatabase` in kcm-runtime uses kcm-distributed for transparent distributed query execution and transaction coordination. Shard routing is integrated into the query execution path.

## Sequence Diagram: 2PC Commit Flow

```
┌─────────┐         ┌───────────────┐         ┌──────────────┐
│  Client  │         │  Coordinator  │         │ Participants │
└────┬────┘         └───────┬───────┘         └──────┬───────┘
     │  1. begin_tx()       │                        │
     │─────────────────────>│                        │
     │                      │  2. PREPARING (log)    │
     │                      │───────────────┐        │
     │                      │<──────────────┘        │
     │                      │                        │
     │                      │  3. TransactionPrepare │
     │                      │───────────────────────>│
     │                      │                        │  4. prepare (lock + log)
     │                      │                        │──────────┐
     │                      │                        │<─────────┘
     │                      │                        │
     │                      │  5. Vote::Yes          │
     │                      │<───────────────────────│
     │                      │                        │
     │                      │  6. COMMIT (log)       │
     │                      │───────────────┐        │
     │                      │<──────────────┘        │
     │                      │                        │
     │                      │  7. TransactionCommit  │
     │                      │───────────────────────>│
     │                      │                        │  8. apply + release
     │                      │                        │──────────┐
     │                      │                        │<─────────┘
     │                      │                        │
     │                      │  9. Ack                │
     │                      │<───────────────────────│
     │                      │                        │
     │                      │  10. COMPLETE (log)    │
     │                      │───────────────┐        │
     │                      │<──────────────┘        │
     │                      │                        │
     │  11. Success         │                        │
     │<─────────────────────│                        │
```

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                        kcm-distributed                          │
├───────────┬───────────────┬──────────────┬─────────────────────┤
│ sharding  │  coordinator  │ replication  │     transport       │
│           │               │              │                     │
│ ShardMap  │ Transaction   │ Replication  │ TransportMessage    │
│ ShardStrategy│ Coordinator│ Manager      │ TransportLayer      │
│ Hash      │ 2PC Protocol  │ Sync/Async   │ TLS + Auth          │
│ Range     │ Commit Log    │ Conflict Res │ Heartbeat           │
│ Consistent│ Timeout Mgmt  │ Hash Chain   │ Connection Pool     │
├───────────┴───────────────┴──────────────┴─────────────────────┤
│                                                                 │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐     │
│  │   kcm-core   │    │ kcm-security │    │  parking_lot  │     │
│  │  Types, Dict │    │  RBAC, TLS   │    │  RwLock,Mutex │     │
│  └──────────────┘    └──────────────┘    └──────────────┘     │
└─────────────────────────────────────────────────────────────────┘
```

## References

- `docs/PRD3.md` §27 — Distributed architecture specification (authoritative)
- `docs/SSOT.md` — Single Source of Truth index
- `AGENTS.md` — Engineering gates, non-negotiable rules, and architecture
- `crates/kcm-distributed/src/` — Implementation source files
- `crates/kcm-security/` — Security primitives used by this crate
- `crates/kcm-core/` — Core types shared across all crates
- `docs/PRD-TESTING& BRACHMARCK.md` — Testing and benchmark requirements

## SSOT Alignment

| SSOT Document | Section | Alignment |
|---------------|---------|-----------|
| `docs/PRD3.md` | §27 | Distributed architecture, sharding, 2PC, replication |
| `AGENTS.md` | Error Model | `KcmError` hierarchy used throughout |
| `AGENTS.md` | Concurrency Model | parking_lot RwLock/Mutex for shared state |
| `AGENTS.md` | Non-Negotiable Rules | All APIs return `Result<T, KcmError>`, no unwrap |
| `AGENTS.md` | Dependency Policy | Dependencies justified per table |
| `docs/PRD-TESTING& BRACHMARCK.md` | §1–8 | Test pyramid and quality gates |
