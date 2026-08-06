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
cargo build --workspace           # Debug build
cargo build --release --workspace # Release build (optimized)
```

### Run Tests

```bash
cargo test --workspace
cargo test --workspace -- --nocapture
```

### Start Server

```bash
./target/release/kcm-server
RUST_LOG=info KCM_DATA_PATH=/data/kcm.db ./target/release/kcm-server
```

### Docker

```bash
docker build -t kcm:latest .
docker run -d -p 8080:8080 -v kcm_data:/data -e RUST_LOG=info kcm:latest
```

---

## API Reference

### REST Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/health` | Health check |
| `GET` | `/api/stats` | Database statistics |
| `POST` | `/api/facts` | Insert fact |
| `GET` | `/api/facts` | List facts |
| `GET` | `/api/facts/:id` | Get fact by ID |
| `DELETE` | `/api/facts/:id` | Delete fact |
| `POST` | `/api/query` | Query facts |
| `POST` | `/api/transactions/begin` | Begin transaction |

### C FFI (18 functions)

```c
#include <kcm_interface.h>

KCM_Database *db;
KCM_DatabaseNew(&db);

KCM_Fact fact = {
    .subject = 1, .predicate = 2, .object = 3,
    .confidence = 0.95, .evidence = 1,
    .timestamp = 1700000000000000000,
    .context = 1, .version = 1, .priority = 0, .owner = 1
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

### Fact Structure (34 bytes uncompressed)

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

```bash
kubectl apply -f deployment/k8s/deployment.yaml
```

### Helm

```bash
helm install kcm deployment/helm/kcm
```

---

## Quality Gates

| Gate | Command | Blocks Merge |
|------|---------|-------------|
| Format | `cargo fmt --all -- --check` | Yes |
| Clippy | `cargo clippy --workspace -- -D warnings` | Yes |
| Build | `cargo build --workspace` | Yes |
| Unit Tests | `cargo test --lib --all` | Yes |
| Integration Tests | `cargo test --test '*' --all` | Yes |
| SSOT Validation | `bash scripts/validate-ssot.sh` | Yes |

---

## Project Structure

```
KCM/
├── crates/                    # 13 core Rust crates
├── scripts/                   # Build, test, CLI tools
│   └── kcm-cli/              # CLI tool binaries
├── docs/                      # Documentation (SSOT v2.0)
│   ├── adr/                   # Architecture Decision Records
│   ├── specs/                 # PRDs and specifications
│   └── handbook/              # Engineering handbook
├── deployment/                # Docker, K8s, Helm, Terraform
├── tests/                     # Integration & security tests
├── sdk/                       # Multi-language SDKs
├── assets/                    # Logo and static assets
├── benchmark-results/         # Benchmark baselines and reports
├── skills/                    # AI engineering skills
└── .github/workflows/         # CI/CD pipelines
```

---

## Documentation

| Document | Scope | Priority |
|----------|-------|----------|
| [SSOT.md](SSOT.md) | Single Source of Truth | Root |
| [KCM_SPECIFICATION.md](KCM_SPECIFICATION.md) | Technical constitution | Root |
| [ROADMAP.md](ROADMAP.md) | Release plan | Root |
| [ARCHITECTURE_CONSISTENCY_MATRIX.md](ARCHITECTURE_CONSISTENCY_MATRIX.md) | Component registry | Root |
| [PRD.md](docs/specs/PRD.md) | Core types, storage, compute, reasoning | P4 |
| [PRD2.md](docs/specs/PRD2.md) | Storage, runtime, interfaces | P3 |
| [PRD3.md](docs/specs/PRD3.md) | Distributed, ML, security, compliance | P2 |
| [PRD-TESTING](docs/specs/PRD-TESTING-AND-BENCHMARK.md) | Testing, benchmarks | P1 |
| [handbook.md](docs/handbook/handbook.md) | Development guide | — |

---

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Find the SSOT requirement in PRD docs
4. Implement matching specification
5. Write tests validating implementation
6. Run quality gates: `cargo build --workspace && cargo test --workspace && cargo clippy --workspace -- -D warnings && cargo fmt --all -- --check && bash scripts/validate-ssot.sh`
7. Open a Pull Request with SSOT requirement reference

---

## License

MIT License — see [LICENSE](LICENSE) for details.

---

<p align="center">
  <sub>Built with care by the KCM Engineering Team</sub>
</p>
