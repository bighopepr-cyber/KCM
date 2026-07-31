# KCM Architecture Specification

**Document ID:** KCM-ARCH-001  
**Version:** 1.0.0  
**Depends on:** KCM-SPEC-001  
**Authoritative Sources:** PRD.md §2 (Design Principles), AGENTS.md §System Architecture, AGENTS.md §Crate Map

> **Authority Notice:** The canonical crate map, dependency flow, and dependency policy are defined in AGENTS.md §Crate Map and §Dependency Flow. Design principles are defined in PRD.md §2. This document provides operational detail and component specifications beyond the authoritative definitions.

---

## 1. Purpose

Defines the system architecture, component boundaries, dependency graph, and data flow for KCM. Derived from PRD.md §2 and AGENTS.md §System Architecture.

---

## 2. Architecture Overview

> Full crate map: AGENTS.md §Crate Map. Dependency flow: AGENTS.md §Dependency Flow.

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                         │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │
│  │ C FFI    │  │ Python   │  │ REST API │  │ KQL      │   │
│  │ (FFI)    │  │ (PyO3)   │  │ (HTTP)   │  │ (Parser) │   │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘   │
│       └──────────────┼──────────────┼──────────────┘        │
│                      ▼                                      │
│  ┌──────────────────────────────────────────────────────┐   │
│  │              kcm-runtime (Orchestration)              │   │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐           │   │
│  │  │ Database │  │Transact. │  │ Executor │           │   │
│  │  └────┬─────┘  └────┬─────┘  └────┬─────┘           │   │
│  │       └──────────────┼──────────────┘                │   │
│  └──────────────────────┼───────────────────────────────┘   │
│                         ▼                                   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │              kcm-compute (Query Execution)            │   │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐           │   │
│  │  │ Algebra  │  │ SIMD     │  │ Optimizer│           │   │
│  │  └────┬─────┘  └────┬─────┘  └────┬─────┘           │   │
│  └───────┼──────────────┼─────────────┼─────────────────┘   │
│          ▼              ▼             ▼                     │
│  ┌──────────────────────────────────────────────────────┐   │
│  │              kcm-storage (Data Layer)                 │   │
│  │  ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐      │   │
│  │  │Column│ │Codec │ │Compr.│ │Index │ │ WAL  │      │   │
│  │  └──┬───┘ └──┬───┘ └──┬───┘ └──┬───┘ └──┬───┘      │   │
│  └─────┼────────┼────────┼────────┼────────┼────────────┘   │
│        ▼        ▼        ▼        ▼        ▼                │
│  ┌──────────────────────────────────────────────────────┐   │
│  │              kcm-core (Foundation)                    │   │
│  │  ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐               │   │
│  │  │Types │ │Vec   │ │Bitmap│ │Dict  │               │   │
│  │  └──────┘ └──────┘ └──────┘ └──────┘               │   │
│  └──────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

---

## 3. Crate Dependency Graph

> Canonical dependency flow: AGENTS.md §Dependency Flow.

```
kcm-core (foundation, zero internal dependencies)
    ↑
kcm-storage (depends on: kcm-core)
    ↑
kcm-compute (depends on: kcm-core, kcm-storage)
kcm-reasoning (depends on: kcm-core, kcm-storage)
kcm-optimizer (depends on: kcm-core, kcm-storage)
kcm-distributed (depends on: kcm-core, kcm-storage)
kcm-ml (depends on: kcm-core, kcm-reasoning)
    ↑
kcm-runtime (depends on: kcm-core, kcm-storage, kcm-compute, kcm-reasoning, kcm-optimizer)
    ↑
kcm-interface (depends on: kcm-core, kcm-runtime)
kcm-security (depends on: kcm-core, kcm-storage)
kcm-compliance (depends on: kcm-core, kcm-storage)
kcm-testing (depends on: kcm-core, kcm-storage, kcm-runtime, kcm-security)
    ↑
kcm-server (depends on: kcm-core, kcm-runtime, kcm-interface, kcm-distributed, kcm-security)
```

---

## 4. Component Specifications

> Full crate responsibilities: AGENTS.md §Crate Map. Dependency policy: AGENTS.md §Dependency Policy.

### 4.1 kcm-core

| Field | Value |
|-------|-------|
| **Purpose** | Foundation types and data structures |
| **Responsibility** | Primitive types (RowID, SubjectID, PredicateID, ObjectID, Confidence, Fact), DenseVec (SIMD-aligned vector), Bitmap (64-bit word operations), Dictionary (string-to-integer mapping) |
| **Input** | Raw values, capacity specifications |
| **Output** | Typed IDs, aligned memory buffers, bitmap bitsets, dictionary IDs |
| **Dependency** | parking_lot (only) |
| **Constraint** | Must be no_std-compatible in future; no heap allocation in hot paths |

### 4.2 kcm-storage

| Field | Value |
|-------|-------|
| **Purpose** | Columnar storage engine with persistence |
| **Responsibility** | Column<T>, Schema (10-column fact store), encoding/compression, WAL, file format, backup/restore, recovery, bitmap/zone/bloom indexes |
| **Input** | Facts, capacity specs, file paths |
| **Output** | Persisted column data, compressed blocks, WAL entries |
| **Dependency** | kcm-core, zstd, lz4, blake3, thiserror, parking_lot |
| **Constraint** | Each column must support compress/decompress roundtrip; WAL must be fsync'd before acknowledgment |

### 4.3 kcm-compute

| Field | Value |
|-------|-------|
| **Purpose** | Query execution engine |
| **Responsibility** | ScanOp, FilterOp, ProjectOp, JoinOp, AggregateOp; AVX2 SIMD intrinsics with scalar fallback |
| **Input** | Schema references, filter predicates, row ID sets |
| **Output** | Filtered row ID sets, aggregate values, projected column data |
| **Dependency** | kcm-core, kcm-storage |
| **Constraint** | All operators must skip tombstone-deleted rows; SIMD path must have runtime feature detection |

### 4.4 kcm-reasoning

| Field | Value |
|-------|-------|
| **Purpose** | Rule-based inference engine |
| **Responsibility** | Rule/RulePattern/RuleRegistry, forward-chaining inference, confidence calculus (conjunction, disjunction, negation, chain, weighted) |
| **Input** | Rules with patterns and confidence formulas, schema with facts |
| **Output** | Derived facts with computed confidence |
| **Dependency** | kcm-core, kcm-storage |
| **Constraint** | Must have max iteration limit; derived facts must be appended to schema for cross-iteration chaining; negation must be implemented |

### 4.5 kcm-optimizer

| Field | Value |
|-------|-------|
| **Purpose** | Query plan optimization |
| **Responsibility** | CostModel, Planner, Statistics, OptimizerPipeline, FilterPushdown, JoinOrdering, IndexSelection, AdaptiveExecutor |
| **Input** | Query plans, schema statistics |
| **Output** | Optimized execution plans |
| **Dependency** | kcm-core, kcm-storage |
| **Constraint** | Must handle missing statistics gracefully; optimization must be idempotent |

### 4.6 kcm-runtime

| Field | Value |
|-------|-------|
| **Purpose** | Execution lifecycle and operations |
| **Responsibility** | KnowledgeDatabase (Insert/Query/Update/Delete), Transaction with apply_to_schema/rollback, VersionStore, Metrics, HealthCheck, Logging, AsyncExecutor (tokio), Executor (rayon) |
| **Input** | Facts, queries, operations |
| **Output** | Query results, metrics snapshots, health reports |
| **Dependency** | kcm-core, kcm-storage, kcm-compute, kcm-reasoning, kcm-optimizer, parking_lot, rayon, tokio |
| **Constraint** | Schema cloning per query is acceptable for current scale; future optimization needed for >10M facts |

### 4.7 kcm-interface

| Field | Value |
|-------|-------|
| **Purpose** | External API surface |
| **Responsibility** | C FFI (13 functions), Python bindings (PyO3), REST API handlers, KQL parser, example implementations |
| **Input** | C types, Python objects, HTTP requests, KQL strings |
| **Output** | C-compatible results, Python objects, HTTP responses, parsed ASTs |
| **Dependency** | kcm-core, kcm-runtime, parking_lot, serde_json |
| **Constraint** | C API must use static null-terminated strings (no dangling pointers); KQL parser must handle all token types |

### 4.8 kcm-distributed

| Field | Value |
|-------|-------|
| **Purpose** | Distributed system primitives |
| **Responsibility** | Hash/Range/ConsistentHash sharding, ShardMap, TransactionCoordinator (2PC) |
| **Input** | Shard configurations, transaction requests |
| **Output** | Shard routing decisions, transaction status |
| **Dependency** | kcm-core, parking_lot |
| **Constraint** | ShardingStrategy must be Send + Sync; 2PC must handle all-votes-committed-or-abort |

### 4.9 kcm-ml

| Field | Value |
|-------|-------|
| **Purpose** | Machine learning integration |
| **Responsibility** | RegressionModel/LearnedIndex (position prediction), ConfidenceLearner (accuracy tracking), RuleDiscoveryEngine (pattern mining) |
| **Input** | Value-position pairs, fact observations, pattern frequencies |
| **Output** | Predicted positions, confidence adjustments, discovered rules |
| **Dependency** | kcm-core, kcm-reasoning |
| **Constraint** | LearnedIndex search must return ranges for binary search refinement |

### 4.10 kcm-security

| Field | Value |
|-------|-------|
| **Purpose** | Security and access control |
| **Responsibility** | RBAC (Permission/Role/User/ACLManager), AES-256-GCM encryption, AuditLog |
| **Input** | User IDs, permissions, plaintext data |
| **Output** | Permission decisions, ciphertext, audit events |
| **Dependency** | kcm-core, blake3, aes-gcm, getrandom, parking_lot |
| **Constraint** | Key generation must use CSPRNG; encryption must use AEAD; audit log must cap at 100K events |

### 4.11 kcm-compliance

| Field | Value |
|-------|-------|
| **Purpose** | Regulatory compliance |
| **Responsibility** | GDPR manager (consent, export, delete), data classification (Public/Internal/Confidential/Restricted with retention policies) |
| **Input** | Subject data, consent actions |
| **Output** | Consent status, exported data, retention decisions |
| **Dependency** | kcm-core, parking_lot |
| **Constraint** | GDPR delete must be irreversible within session |

### 4.12 kcm-testing

| Field | Value |
|-------|-------|
| **Purpose** | Testing infrastructure |
| **Responsibility** | Load test runner, stress test runner, security test suite, regression detector, metrics dashboard |
| **Input** | Test scenarios, baselines |
| **Output** | Test results, performance reports, regression alerts |
| **Dependency** | kcm-core, kcm-storage, kcm-runtime, kcm-security, parking_lot |
| **Constraint** | All test runners must be invokable via `cargo test`; load/stress tests must be configurable for CI duration |

### 4.13 kcm-server

| Field | Value |
|-------|-------|
| **Purpose** | Production server binary |
| **Responsibility** | HTTP/gRPC server, configuration loading, graceful shutdown, signal handling |
| **Input** | Configuration files, command-line arguments |
| **Output** | Running server process |
| **Dependency** | kcm-core, kcm-runtime, kcm-interface, kcm-distributed, kcm-security, parking_lot |
| **Constraint** | Must support graceful shutdown on SIGTERM/SIGINT; must expose health endpoint |

---

## 5. Data Flow

### 5.1 Insert Flow

```
Application
    │
    ▼
KCM_DatabaseInsert (FFI)
    │
    ▼
KnowledgeDatabase::insert()
    │
    ├──► Schema::append_fact()
    │       ├──► subject_col.append()
    │       ├──► predicate_col.append()
    │       ├──► object_col.append()
    │       ├──► confidence_col.append()
    │       ├──► evidence_col.append()
    │       ├──► timestamp_col.append()
    │       ├──► context_col.append()
    │       ├──► version_col.append()
    │       ├──► priority_col.append()
    │       └──► owner_col.append()
    │
    └──► WAL::append_fact()
            └──► fsync to disk
```

### 5.2 Query Flow

```
Application
    │
    ▼
QueryBuilder::new(schema_snapshot)
    │
    ├──► .with_subject(SubjectID(1))
    ├──► .with_predicate(PredicateID(0))
    └──► .with_confidence(0.5)
    │
    ▼
execute()
    │
    ├──► For each idx in 0..schema.len()
    │       ├──► Skip if tombstone deleted
    │       ├──► Check subject filter
    │       ├──► Check predicate filter
    │       ├──► Check object filter
    │       └──► Check confidence filter
    │
    └──► Return Vec<Fact>
```

### 5.3 Inference Flow

```
InferenceEngine::infer_forward_chaining(&mut schema)
    │
    ├──► For each iteration (0..max_iterations):
    │       ├──► For each enabled rule:
    │       │       ├──► find_pattern_matches()
    │       │       │       ├──► Triple pattern: scan columns
    │       │       │       ├──► And pattern: join matches
    │       │       │       ├──► Or pattern: union matches
    │       │       │       └──► Not pattern: exclude matches
    │       │       └──► Apply confidence_formula
    │       ├──► Filter by confidence_threshold
    │       ├──► Append derived facts to schema
    │       └──► If no new facts, break
    │
    └──► Return Vec<(Fact, RuleID)>
```

---

## 6. Concurrency Model

> Canonical concurrency model: AGENTS.md §Concurrency Model.

| Component | Mechanism | Protection | AGENTS.md Reference |
|-----------|-----------|------------|---------------------|
| KnowledgeDatabase | Arc<RwLock<Schema>> | Write lock for insert/update/delete; read lock for query snapshot | Schema — Arc<RwLock<Schema>> |
| SharedDictionary | Arc<RwLock<Dictionary>> | Write lock for insert; read lock for lookup | Dictionaries — Arc<RwLock<Dictionary>> |
| WriteAheadLog | Mutex<File> | Serialized writes | WAL — Mutex<File> |
| AuditLog | Mutex<Vec<AuditEvent>> | Serialized append | Audit Log — Mutex<Vec<AuditEvent>> |
| Metrics | AtomicU64 counters | Lock-free atomic operations | Metrics — AtomicU64 (10 counters) |
| TransactionCoordinator | Mutex<HashMap> | Serial transaction management | — |

---

## 7. Constraints

> Non-negotiable rules: AGENTS.md §Non-Negotiable Rules.

| Constraint | Rationale | AGENTS.md Reference |
|------------|-----------|---------------------|
| Single-process, single-writer | Simplifies concurrency; distributed mode is optional | — |
| Schema capacity pre-allocated | Avoids reallocation in hot path | — |
| WAL fsync on flush | Ensures crash recovery correctness | — |
| No circular crate dependencies | Enforces clean module boundaries | AGENTS.md §Dependency Policy |
| Foundation crate (kcm-core) has zero internal dependencies | Enables maximum reuse and testability | AGENTS.md §Crate Map |
| No panic!() in production code | System stability | AGENTS.md §Non-Negotiable Rules #3 |
| No TODO/FIXME/HACK in production code | Code completeness | AGENTS.md §Non-Negotiable Rules #4 |

---

## 8. References

- **Authoritative sources:** PRD.md §2 (Design Principles), PRD.md §4 (Data Structures), PRD.md §5 (Compute Engine), PRD.md §6 (Reasoning Engine), AGENTS.md §Crate Map, AGENTS.md §Dependency Flow, AGENTS.md §Concurrency Model
- **Depends on:** PRD.md (P4 authority), AGENTS.md (Engineering Constitution)
- **Parent specs:** KCM_SPECIFICATION
- **Related:** KCM_DATA_MODEL_SPEC, KCM_RUNTIME_SPEC, KCM_API_SPEC
