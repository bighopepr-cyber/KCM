<p align="center">
  <img src="https://drive.google.com/uc?export=view&id=1Mz77KUmqKnIUssu1jLQyfBSNviJB0Hbh" alt="KCM Logo" width="400">
</p>

<h1 align="center">KCM — Knowledge Columnar Model</h1>

<p align="center">
  <strong>A Rust-native columnar knowledge engine with persistent storage, query execution, and inference-capable runtime behavior.</strong>
</p>

<p align="center">
  <a href="#features">Features</a> •
  <a href="#architecture">Architecture</a> •
  <a href="#quick-start">Quick Start</a> •
  <a href="#api-reference">API</a> •
  <a href="#deployment">Deployment</a> •
  <a href="#contributing">Contributing</a>
</p>

---

## Overview

KCM is a self-contained columnar knowledge representation, storage, query, and reasoning engine implemented in Rust. It owns its core technology stack: storage, execution, query engine, compression, dictionary encoding, bitmap engine, optimizer, reasoning engine, transaction engine, recovery, benchmarking, testing, monitoring, and documentation.

### Design Principles

| Principle | Description |
|-----------|-------------|
| **Columnar Native** | All knowledge stored as independent typed columns |
| **Dictionary-Encoded** | All string/reference data mapped to integer IDs |
| **Deterministic** | Identical input always produces identical output |
| **Zero-Copy Access** | DenseVec provides direct slice access without allocation |
| **SIMD-Ready** | Data structures aligned for vector processing |
| **Production-Grade** | Full ACID, crash recovery, validation |

---

## Features

### Core Engine
- **Columnar Storage** — 10 physical columns with per-column encoding and compression
- **Dictionary Encoding** — Efficient string/reference storage via integer mapping
- **SIMD-Accelerated Compute** — AVX2-optimized query operators
- **Cost-Based Optimizer** — Filter pushdown, column pruning, join reordering
- **Forward-Chaining Inference** — Rule-based reasoning engine

### Data Operations
- **ACID Transactions** — Full transaction support with isolation levels
- **Batch Operations** — High-throughput bulk insert/update/delete
- **Crash Recovery** — WAL-based recovery with integrity verification
- **Schema Evolution** — Dynamic schema management with validation

### Security & Compliance
- **RBAC** — 5-level permission model (Read, Write, Delete, Execute, Admin)
- **AES-256-GCM Encryption** — Authenticated encryption at rest
- **Audit Logging** — Hash-chained tamper-evident audit trail
- **GDPR Compliance** — Consent management and data classification

### Interfaces
- **C FFI** — 18 functions for language interop
- **Python Bindings** — PyO3-based Python integration
- **REST API** — HTTP endpoints with OpenAPI spec
- **gRPC** — High-performance RPC interface
- **KQL** — Knowledge Query Language parser

### Infrastructure
- **Docker** — Multi-stage production builds
- **Kubernetes** — StatefulSet manifests
- **Distributed** — Hash/Range/ConsistentHash sharding with 2PC
- **ML Integration** — Learned indexes and confidence scoring

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                         │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │
│  │ C FFI    │  │ Python   │  │ REST API │  │ KQL      │   │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘   │
│       └──────────────┼──────────────┼──────────────┘        │
│                      ▼                                      │
│  ┌──────────────────────────────────────────────────────┐   │
│  │              kcm-runtime (Orchestration)              │   │
│  └──────────────────────┼───────────────────────────────┘   │
│                         ▼                                   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │              kcm-compute (Query Execution)            │   │
│  └──────────────────────┼───────────────────────────────┘   │
│                         ▼                                   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │              kcm-storage (Data Layer)                 │   │
│  └──────────────────────┼───────────────────────────────┘   │
│                         ▼                                   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │              kcm-core (Foundation)                    │   │
│  └──────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

### Crate Map

| Crate | Responsibility |
|-------|----------------|
| `kcm-core` | Types, DenseVec, Bitmap, Dictionary |
| `kcm-storage` | Columns, Codecs, WAL, FileFormat, Index, Recovery |
| `kcm-compute` | Relational algebra operators, SIMD acceleration |
| `kcm-reasoning` | Rule definitions, inference engine |
| `kcm-optimizer` | Cost model, query planner, statistics |
| `kcm-runtime` | KnowledgeDatabase, Transactions, Metrics |
| `kcm-interface` | C FFI, Python bindings, REST handlers, KQL parser |
| `kcm-distributed` | Sharding strategies, 2PC coordinator |
| `kcm-ml` | Learned index, confidence learner |
| `kcm-security` | RBAC, AES-256-GCM, audit log |
| `kcm-compliance` | GDPR, data classification |
| `kcm-testing` | Load, stress, security, recovery tests |
| `kcm-server` | HTTP (actix-web) + gRPC (tonic) |

---

## Quick Start

### Prerequisites

- Rust 1.85+
- Cargo

### Build

```bash
# Debug build
cargo build --workspace

# Release build (optimized)
cargo build --release --workspace
```

### Run Tests

```bash
# Run all tests
cargo test --workspace

# Run with output
cargo test --workspace -- --nocapture
```

### Start Server

```bash
# Start HTTP/gRPC server
./target/release/kcm-server

# With environment variables
RUST_LOG=info KCM_DATA_PATH=/data/kcm.db ./target/release/kcm-server
```

### Docker

```bash
# Build image
docker build -t kcm:latest .

# Run container
docker run -d \
  -p 8080:8080 \
  -v kcm_data:/data \
  -e RUST_LOG=info \
  kcm:latest
```

---

## API Reference

### REST Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/health` | Health check |
| `GET` | `/metrics` | Prometheus metrics |
| `GET` | `/openapi.json` | OpenAPI specification |
| `GET` | `/facts` | List facts |
| `GET` | `/facts/{id}` | Get fact by ID |
| `GET` | `/stats` | Database statistics |
| `POST` | `/api/v1/facts` | Insert fact |
| `POST` | `/api/v1/facts/batch` | Batch insert |
| `PUT` | `/api/v1/facts/{id}` | Update fact |
| `DELETE` | `/api/v1/facts/{id}` | Delete fact |

### C FFI

```c
#include <kcm_interface.h>

KCM_Database *db;
KCM_DatabaseNew(&db);

KCM_Fact fact = {
    .subject = 1,
    .predicate = 2,
    .object = 3,
    .confidence = 0.95,
    .evidence = 1,
    .timestamp = 1700000000000000000,
    .context = 1,
    .version = 1,
    .priority = 0,
    .owner = 1
};

KCM_DatabaseInsert(db, &fact);
```

### Rust API

```rust
use kcm_core::types::Fact;
use kcm_runtime::KnowledgeDatabase;

let db = KnowledgeDatabase::new()?;
let fact = Fact::new(subject, predicate, object, confidence)?;
let row_id = db.insert(&fact)?;
```

### Python

```python
import kcm

db = kcm.Database()
fact = kcm.Fact(subject=1, predicate=2, object=3, confidence=0.95)
row_id = db.insert(fact)
```

---

## Data Model

### Fact Structure

```rust
pub struct Fact {
    pub subject: SubjectID,      // u32 — dictionary-encoded
    pub predicate: PredicateID,  // u8  — dictionary-encoded
    pub object: ObjectID,        // u32 — dictionary-encoded
    pub confidence: f64,         // validated [0.0, 1.0]
    pub evidence: EvidenceID,    // u8
    pub timestamp: i64,          // nanoseconds since epoch
    pub context: ContextID,      // u8
    pub version: i32,            // monotonic on update
    pub priority: i8,            // -128..127
    pub owner: u16,              // dictionary-encoded
}
```

**Size:** 34 bytes uncompressed per fact.

### Column Storage

| Column | Type | Encoding | Compression |
|--------|------|----------|-------------|
| Subject | u32 | Dictionary | Zstd |
| Predicate | u8 | Dictionary | RLE |
| Object | u32 | Dictionary | Zstd |
| Confidence | f64 | Gorilla | Zstd |
| Evidence | u8 | Dictionary | RLE |
| Timestamp | i64 | Delta | Zstd |
| Context | u8 | Dictionary | RLE |
| Version | i32 | Delta | LZ4 |
| Priority | i8 | Identity | RLE |
| Owner | u16 | Dictionary | Zstd |

---

## Deployment

### Docker Compose

```yaml
version: '3.8'
services:
  kcm:
    build: .
    volumes:
      - kcm_data:/data
    environment:
      RUST_LOG: info
      KCM_DATA_PATH: /data/kcm.db
    ports:
      - "8080:8080"
    restart: unless-stopped
volumes:
  kcm_data:
    driver: local
```

### Kubernetes

```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: kcm-server
spec:
  serviceName: kcm-service
  replicas: 1
  selector:
    matchLabels:
      app: kcm-server
  template:
    spec:
      containers:
      - name: kcm-server
        image: kcm:latest
        ports:
        - containerPort: 8080
        env:
        - name: RUST_LOG
          value: "info"
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "1Gi"
            cpu: "1000m"
```

---

## Quality Gates

Every change must pass 6 mandatory gates:

| Gate | Validation |
|------|------------|
| **Build** | `cargo build --workspace` |
| **Tests** | `cargo test --workspace` |
| **Clippy** | `cargo clippy --workspace -- -D warnings` |
| **Format** | `cargo fmt --all -- --check` |
| **SSOT** | `bash scripts/validate-ssot.sh` |
| **Security** | No unwrap in production code |

---

## CI/CD Pipeline

| Job | Trigger | What it validates |
|-----|---------|-------------------|
| Format Check | Every push | `cargo fmt --all -- --check` |
| Build | Every push | `cargo build --workspace` |
| Clippy | Every push | `cargo clippy --workspace -- -D warnings` |
| Unit Tests | Every push | `cargo test --lib --all` |
| Integration Tests | Every push | `cargo test --test '*' --all` |
| Security Tests | After unit tests | `cargo test security_tests --all` |
| Property Tests | Every push | `cargo test property_tests --all` |
| Benchmarks | After unit tests | `cargo bench --workspace --no-run` |
| Quality Gate | All above pass | Final merge decision |

---

## Engineering Governance

KCM uses a 16-skill engineering system enforced by AI agents:

| Priority | Skill | Role |
|----------|-------|------|
| P1 | Engineering Orchestrator | Master coordinator |
| P2 | Task Planner | Implementation planning |
| P3 | Change Impact Analysis | Pre-change assessment |
| P4 | Specification Lock | Frozen contract protection |
| P5 | Architecture Guardian | Architecture integrity |
| P6 | Database Engine Specialist | Storage/query correctness |
| P7 | Security Engineer | Security and compliance |
| P8 | Performance Engineer | Performance validation |
| P9 | Testing Verification | Test coverage |
| P10 | Code Quality Guardian | Rust code quality |
| P11 | Documentation Guardian | Spec consistency |
| P12 | Release Readiness | Production validation |
| P13 | Code Review Auditor | Senior review |
| P14 | Debugging Root Cause | Bug investigation |
| P15 | Engineering Decision Record | Decision documentation |
| P16 | Repository Intelligence | Codebase understanding |

---

## Project Structure

```
kcm/
├── assets/              # Logo and static assets
├── crates/              # Rust workspace crates
│   ├── kcm-core/        # Core types and memory structures
│   ├── kcm-storage/     # Storage formats, WAL, codecs
│   ├── kcm-compute/     # Query operators and execution
│   ├── kcm-reasoning/   # Inference and rule execution
│   ├── kcm-optimizer/   # Cost model, query planner
│   ├── kcm-runtime/     # Database lifecycle, transactions
│   ├── kcm-interface/   # FFI, REST, gRPC, KQL parser
│   ├── kcm-distributed/ # Sharding, 2PC
│   ├── kcm-ml/          # Learned indexes, confidence
│   ├── kcm-security/    # RBAC, encryption, audit
│   ├── kcm-compliance/  # GDPR, classification
│   ├── kcm-testing/     # Load, stress, security tests
│   └── kcm-server/      # HTTP + gRPC server
├── docs/                # Documentation (SSOT)
├── scripts/             # Build and validation scripts
├── tests/               # Integration tests
└── benches/             # Benchmarks
```

---

## Documentation

All specifications are maintained as Single Source of Truth (SSOT):

| Document | Scope | Priority |
|----------|-------|----------|
| [PRD.md](docs/PRD.md) | Core types, storage, compute, reasoning | P4 |
| [PRD2.md](docs/PRD2.md) | Storage, runtime, interfaces | P3 |
| [PRD3.md](docs/PRD3.md) | Distributed, ML, security, compliance | P2 |
| [PRD-TESTING](docs/PRD-TESTING&%20BRACHMARCK.md) | Testing, benchmarks, quality gates | P1 |
| [DOCUMENTATION_INDEX.md](docs/DOCUMENTATION_INDEX.md) | Repository navigation | — |

---

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Workflow

```bash
# Run quality gates
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check
bash scripts/validate-ssot.sh
```

---

## License

MIT License — see [LICENSE](LICENSE) for details.

---

<p align="center">
  <sub>Built with care by the KCM Engineering Team</sub>
</p>
