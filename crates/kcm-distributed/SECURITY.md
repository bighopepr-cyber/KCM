# kcm-distributed Security Policy

> This document covers the security policies specific to the `kcm-distributed` crate. For project-wide security policies, see the root `SECURITY.md`.

---

## Overview

The `kcm-distributed` crate implements distributed coordination, sharding, replication, and transport for the KCM system. It is responsible for ensuring data consistency across nodes, secure communication between cluster members, and correct execution of the two-phase commit (2PC) protocol. Security failures in this crate can lead to split-brain scenarios, data loss, unauthorized data access, or cluster-wide outages.

## Security Scope

| Component | Sensitivity | Rationale |
|-----------|-------------|-----------|
| Coordinator (2PC) | Critical | Manages distributed transactions; incorrect implementation causes data inconsistency or loss |
| Transport | High | Carries all inter-node communication including shard data and transaction payloads |
| Replication | High | Maintains data durability across replicas; failures can cause data loss |
| Sharding | Medium | Determines data placement; incorrect routing exposes data to wrong nodes |

## Threat Model

| Threat | Description | Mitigation |
|--------|-------------|------------|
| Split-brain | Network partition causes two coordinators to commit conflicting transactions | 2PC with majority quorum; epoch-based leader election |
| Network partition | Nodes lose connectivity, causing stale reads or stale writes | Partition detection via heartbeat; fencing tokens for stale coordinators |
| Man-in-the-middle | Attacker intercepts or modifies inter-node messages | TLS transport encryption; message authentication via HMAC |
| Shard imbalance | Malicious or faulty shard assignment concentrates load on one node | Validate shard maps against configured strategy; reject imbalanced assignments |
| Replication lag | Slow replicas serve stale data | Consistency level enforcement (sync vs async); read-your-writes guarantee via version tracking |

## Security Risks

- **Coordinator failure during commit**: A coordinator crash after prepare but before commit leaves replicas in an uncertain state. Mitigation: persistent commit log and recovery replay.
- **Replication stream interception**: Unencrypted replication streams allow data exfiltration. Mitigation: mandatory TLS for all inter-node transport.
- **Shard map tampering**: A compromised node could modify shard maps to redirect queries. Mitigation: shard map signing and validation via kcm-security.
- **Node impersonation**: An attacker could pose as a valid cluster member. Mitigation: mutual TLS authentication and RBAC checks via kcm-security.
- **Transaction log tampering**: Modifying the 2PC log could cause incorrect recovery. Mitigation: hash-chained commit log with integrity verification.

## Access Control

All inter-node communication requires authenticated identity. The coordinator validates node identity before allowing participation in 2PC rounds. Sharding decisions are validated against the authorized node set. Replication connections are authenticated before any data is transferred.

## RBAC Integration

`kcm-distributed` integrates with `kcm-security` for:

- **Node authentication**: Each node presents a certificate verified against the cluster's trusted certificate authority before joining.
- **Transaction authorization**: Only coordinator-authorized nodes may initiate or vote on 2PC rounds.
- **Shard assignment authorization**: Shard ownership changes require administrative-level authorization through the RBAC system.
- **Replication authorization**: Replica nodes must be authorized before receiving replication streams.

## Sensitive Assets

| Asset | Protection |
|-------|------------|
| Cluster state | Encrypted at rest; integrity-checked on every read |
| Shard maps | Signed by coordinator; validated by all nodes before use |
| Replication logs | Encrypted in transit; hash-chained for tamper detection |
| Transaction coordinator state | Persistent commit log with integrity verification |
| Node certificates | Stored with restricted file permissions; rotated periodically |

## Secret Management

| Secret | Storage | Rotation |
|--------|---------|----------|
| Node credentials | Encrypted credential store via kcm-security | On certificate expiry or security incident |
| TLS certificates | Managed via kcm-security certificate infrastructure | Per configured rotation schedule |
| Replication keys | Stored in secure configuration | On node decommission or key compromise |

## Secure Development Rules

1. **2PC correctness**: The two-phase commit protocol must be implemented exactly as specified. No shortcuts or approximations that could compromise atomicity or durability.
2. **Transport encryption**: All inter-node communication MUST use TLS. Plaintext transport is forbidden regardless of environment.
3. **Shard validation**: Shard map changes MUST be validated against the configured sharding strategy before acceptance. Invalid assignments are rejected.
4. **Replication consistency**: Replication MUST maintain consistency guarantees as specified. Async replication must still guarantee eventual consistency with bounded staleness.
5. **No unwrap**: Zero `unwrap()` calls in production code paths. All error cases must be explicitly handled.
6. **Result return**: All public APIs MUST return `Result<T, KcmError>`. No function may propagate panics to callers.

## Audit Logging

The following security-relevant events must be logged:

| Event | Level | Details |
|-------|-------|---------|
| Node joins cluster | INFO | Node identity, timestamp, source address |
| Node leaves cluster | INFO | Node identity, reason, timestamp |
| 2PC round initiated | INFO | Transaction ID, participating nodes, timestamp |
| 2PC commit/abort | INFO | Transaction ID, outcome, duration |
| Shard map changed | INFO | Old map hash, new map hash, changed shards |
| Authentication failure | WARN | Node identity, source address, reason |
| Transport error | WARN | Error type, participating nodes, message |
| Replication lag exceeds threshold | WARN | Replica ID, lag duration, threshold |

## Validation Checklist

- [ ] All inter-node communication uses TLS
- [ ] Node identity verified before 2PC participation
- [ ] Shard maps validated before activation
- [ ] No `unwrap()` in production code paths
- [ ] All public APIs return `Result<T, KcmError>`
- [ ] 2PC log entries are hash-chained for integrity
- [ ] Replication streams authenticated and encrypted
- [ ] Split-brain detection mechanism active
- [ ] Node credentials stored securely via kcm-security
- [ ] Audit logging captures all security events
- [ ] No plaintext secrets in logs or error messages
- [ ] Certificate validation enforced on all connections

## References

- `docs/PRD3.md` §27 — Distributed architecture specification
- `AGENTS.md` — Project-wide security requirements
- Root `SECURITY.md` — Project-wide security policy
- `docs/PRD-TESTING& BRACHMARCK.md` — Security testing requirements
- `kcm-security` crate — RBAC and encryption implementations
