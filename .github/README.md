<div align="center">
  <img src="KCM/assets
/KCM-LOGO.svg" alt="KCM Logo" width="320" style="max-width: 100%; height: auto;">
  
  # KCM — Knowledge Columnar Model
  
  **Enterprise-Grade Columnar Knowledge Engine**
  
  *A production-ready Rust-native columnar knowledge engine with persistent storage, cost-optimized query execution, and inference-capable runtime behavior.*
  
  [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
  [![Rust 1.85+](https://img.shields.io/badge/Rust-1.85%2B-orange.svg)](https://www.rust-lang.org/)
  [![Build Status](https://img.shields.io/badge/Build-Passing-brightgreen)](#ci--cd-pipeline)
  [![Deployment Ready](https://img.shields.io/badge/Deployment-Ready-blue)](#deployment-options)

  [📖 Documentation](#documentation) • [🚀 Getting Started](#quick-start) • [🏗️ Architecture](#system-architecture) • [💻 API Reference](#api-reference) • [🤝 Contributing](#contributing)

</div>

---

## 📋 Table of Contents

- [Overview](#overview)
- [Key Features](#key-features)
- [System Architecture](#system-architecture)
- [Quick Start](#quick-start)
- [API Reference](#api-reference)
- [Data Model](#data-model)
- [Deployment Options](#deployment-options)
- [Quality Assurance](#quality-assurance)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [License](#license)

---

## Overview

**KCM** is a self-contained, production-grade columnar knowledge representation, storage, query, and reasoning engine implemented in pure Rust. It provides complete ownership over the entire technology stack—from low-level storage formats and SIMD-accelerated compute to distributed coordination and machine learning optimizations.

### Design Philosophy

KCM follows six core principles:

| Principle | Implementation |
|-----------|----------------|
| **Columnar Native** | All knowledge represented as typed, independent columns optimized for vectorization |
| **Dictionary-Encoded** | Efficient string/reference storage via integer mapping with zero-copy access patterns |
| **Deterministic** | Guaranteed identical output for identical input; fully reproducible across environments |
| **Zero-Copy Access** | DenseVec provides direct memory slice access without allocation overhead |
| **SIMD-Ready** | Data structures aligned and formatted for AVX2/AVX-512 vector processing |
| **Production-Grade** | Full ACID guarantees, crash recovery, comprehensive validation, and audit logging |

---

## Key Features

### 🗄️ Storage & Query
- **Columnar Storage** — 10 optimized physical columns with per-column encoding and compression
- **Dictionary Encoding** — Compact string/reference storage via integer ID mapping
- **Cost-Based Optimizer** — Filter pushdown, column pruning, join reordering, statistics-driven planning
- **SIMD-Accelerated Compute** — AVX2-optimized relational operators for high throughput
- **Adaptive Indexing** — Learned indexes with machine learning-based query cost reduction

### 📊 Data Operations
- **ACID Transactions** — Full transaction support with multiple isolation levels
- **Batch Operations** — High-throughput bulk insert/update/delete with vectorized operators
- **Crash Recovery** — Write-Ahead Logging (WAL) with integrity verification and point-in-time recovery
- **Schema Evolution** — Dynamic schema management with backward compatibility and validation
- **Time-Series Support** — Optimized temporal data handling with delta encoding

### 🔒 Security & Compliance
- **RBAC** — 5-level permission model (Read, Write, Delete, Execute, Admin)
- **Encryption** — AES-256-GCM authenticated encryption at rest
- **Audit Logging** — Tamper-evident hash-chained audit trail with full event tracking
- **GDPR Compliance** — Consent management, data classification, and automated retention policies
- **Data Classification** — Multi-level sensitivity marking and access control integration

### 🧠 Knowledge & Reasoning
- **Forward-Chaining Inference** — Rule-based reasoning engine with confidence scoring
- **Fact Repository** — Native knowledge base with semantic relationships
- **Rule Discovery** — Machine learning-based automatic rule extraction
- **Confidence Learning** — Adaptive confidence scoring based on evidence patterns
- **Provenance Tracking** — Complete audit trail of inference chains and decisions

### 🔌 Integration Interfaces
- **C FFI** — 18 functions for language interoperability and native bindings
- **Python Bindings** — PyO3-based Python 3.8+ integration with full async support
- **REST API** — HTTP/JSON endpoints with OpenAPI 3.0 specification and automatic documentation
- **gRPC** — High-performance RPC interface for distributed deployments
- **Knowledge Query Language (KQL)** — Specialized parser for knowledge base operations

### 🌐 Infrastructure & Deployment
- **Docker** — Multi-stage production builds with minimal image size
- **Kubernetes** — StatefulSet manifests, Helm charts, and Service configuration
- **Distributed Mode** — Hash/Range/ConsistentHash sharding with 2-Phase Commit coordinator
- **ML Integration** — Learned indexes and confidence scoring models
- **Observability** — Prometheus metrics, Grafana dashboards, distributed tracing support

---

## System Architecture

### High-Level Architecture

```
┌────────────────────────────────────────────────────────────────┐
│                     Application Layer                          │
│  ┌────────────┐  ┌──────────┐  ┌──────────┐  ┌──────────────┐ │
│  │ C FFI      │  │ Python   │  │ REST API │  │ KQL Parser   │ │
│  │ (18 funcs) │  │ (PyO3)   │  │ (Actix)  │  │ (Custom)     │ │
│  └─────┬──────┘  └────┬─────┘  └────┬─────┘  └──────┬───────┘ │
│        └────────────────┼──────────────┼──────────────┘         │
├────────────────────────────────────────────────────────────────┤
│                    Runtime Layer                               │
│  ┌────────────────────────────────────────────────────────┐    │
│  │  kcm-runtime: Database lifecycle, Transactions, RBAC   │    │
│  └──────────────────┬──────────────────────────────────────┘    │
├────────────────────┼──────────────────────────────────────────┤
│                    │ Execution Layer                           │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │ kcm-compute: Relational operators, SIMD acceleration   │   │
│  └──────────┬──────────────────────────────────────────────┘   │
├─────────────┼──────────────────────────────────────────────────┤
│             │ Optimization Layer                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ kcm-optimizer: Cost model, Planner, Statistics          │  │
│  │ kcm-ml: Learned indexes, Confidence learner             │  │
│  └──────────┬───────────────────────────────────────────────┘  │
├─────────────┼──────────────────────────────────────────────────┤
│             │ Storage Layer                                    │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ kcm-storage: Columns, Codecs, WAL, Index, Recovery      │  │
│  └──────────┬───────────────────────────────────────────────┘  │
├─────────────┼──────────────────────────────────────────────────┤
│             │ Foundation Layer                                 │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ kcm-core: Types, DenseVec, Bitmap, Dictionary, Algebra  │  │
│  └──────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

### Crate Responsibility Map

| Crate | Responsibility | Lines |
|-------|----------------|-------|
| **kcm-core** | Types, DenseVec, Bitmap engine, Dictionary | Core Foundation |
| **kcm-storage** | Columns, Codecs, WAL, FileFormat, Index, Recovery | Storage Layer |
| **kcm-compute** | Relational algebra operators, SIMD acceleration | Execution |
| **kcm-reasoning** | Rule definitions, Forward-chaining inference | Knowledge |
| **kcm-optimizer** | Cost model, Query planner, Statistics | Optimization |
| **kcm-runtime** | KnowledgeDatabase, Transactions, Metrics, Health | Runtime |
| **kcm-interface** | C FFI, Python, REST, gRPC, KQL parser | Integration |
| **kcm-distributed** | Sharding strategies, 2PC coordinator | Distribution |
| **kcm-ml** | Learned index, Confidence learner | ML Features |
| **kcm-security** | RBAC, AES-256-GCM, Audit log | Security |
| **kcm-compliance** | GDPR, Data classification | Compliance |
| **kcm-testing** | Load, Stress, Security, Recovery tests | QA |
| **kcm-server** | HTTP (Actix-web) + gRPC (Tonic) | Deployment |

---

## Quick Start

### Prerequisites

- **Rust:** 1.85 or later ([Install](https://rustup.rs/))
- **Cargo:** Latest stable version
- **System:** Linux, macOS, or Windows with WSL2

### 1. Clone Repository

```bash
git clone https://github.com/yourusername/kcm.git
cd kcm
```

### 2. Build Project

```bash
# Debug build (faster compilation, slower execution)
cargo build --workspace

# Release build (optimized, recommended for production)
cargo build --release --workspace
```

### 3. Run Tests

```bash
# Run all tests with output
cargo test --workspace -- --nocapture --test-threads=1

# Run specific test suite
cargo test --package kcm-storage -- --nocapture

# Run with logging
RUST_LOG=debug cargo test --workspace
```

### 4. Start Server

```bash
# Start HTTP/gRPC server (port 8080)
./target/release/kcm-server

# With custom configuration
RUST_LOG=info \
  KCM_DATA_PATH=/var/lib/kcm/data.db \
  KCM_HTTP_ADDR=0.0.0.0:8080 \
  KCM_GRPC_ADDR=0.0.0.0:50051 \
  ./target/release/kcm-server
```

### 5. Verify Installation

```bash
# Health check
curl http://localhost:8080/health

# View metrics
curl http://localhost:8080/metrics

# List facts
curl http://localhost:8080/facts

# OpenAPI spec
curl http://localhost:8080/openapi.json
```

### Docker Quick Start

```bash
# Build Docker image
docker build -t kcm:latest -f deployment/Dockerfile .

# Run container
docker run -d \
  --name kcm-server \
  -p 8080:8080 \
  -p 50051:50051 \
  -v kcm_data:/data \
  -e RUST_LOG=info \
  -e KCM_DATA_PATH=/data/kcm.db \
  kcm:latest

# Check logs
docker logs -f kcm-server

# Stop container
docker stop kcm-server
```

---

## API Reference

### REST Endpoints

#### Health & Monitoring

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/health` | Health status check |
| `GET` | `/metrics` | Prometheus metrics (Prometheus format) |
| `GET` | `/stats` | Database statistics and performance metrics |

#### OpenAPI & Discovery

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/openapi.json` | OpenAPI 3.0 specification |
| `GET` | `/openapi.yaml` | OpenAPI YAML format |
| `GET` | `/docs` | Swagger UI documentation |
| `GET` | `/redoc` | ReDoc API documentation |

#### Fact Operations

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/api/v1/facts` | List all facts (paginated) |
| `GET` | `/api/v1/facts?limit=100&offset=0` | Paginated fact retrieval |
| `GET` | `/api/v1/facts/{id}` | Retrieve fact by ID |
| `POST` | `/api/v1/facts` | Insert single fact |
| `POST` | `/api/v1/facts/batch` | Batch insert facts (streaming) |
| `PUT` | `/api/v1/facts/{id}` | Update fact by ID |
| `DELETE` | `/api/v1/facts/{id}` | Delete fact by ID |
| `DELETE` | `/api/v1/facts/batch` | Batch delete facts |

#### Query Operations

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/api/v1/query` | Execute KQL query |
| `POST` | `/api/v1/query/explain` | Query execution plan |
| `POST` | `/api/v1/query/profile` | Query with profiling data |

### C FFI Example

```c
#include <kcm_interface.h>

int main() {
    // Create database
    KCM_Database *db;
    KCM_DatabaseNew(&db);
    
    // Create fact
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
    
    // Insert fact
    KCM_DatabaseInsert(db, &fact);
    
    // Query facts
    KCM_Fact *results;
    uint64_t count;
    KCM_DatabaseQuery(db, &results, &count);
    
    // Cleanup
    KCM_DatabaseDelete(db);
    return 0;
}
```

### Python Integration

```python
import kcm

# Initialize database
db = kcm.Database(data_path="/data/kcm.db")

# Create fact
fact = kcm.Fact(
    subject=1,
    predicate=2,
    object=3,
    confidence=0.95
)

# Insert fact
row_id = db.insert(fact)

# Query facts
facts = db.query_all()

# Execute KQL
results = db.execute_kql("MATCH (s) WHERE confidence > 0.9")

# Async operations
async def batch_insert():
    async for row_id in db.insert_batch(facts):
        print(f"Inserted: {row_id}")
```

### Rust API

```rust
use kcm_core::types::Fact;
use kcm_runtime::KnowledgeDatabase;

#[tokio::main]
async fn main() -> Result<()> {
    let db = KnowledgeDatabase::new()?;
    
    let fact = Fact::new(
        subject: 1,
        predicate: 2,
        object: 3,
        confidence: 0.95
    )?;
    
    let row_id = db.insert(&fact).await?;
    
    // Batch operations
    let facts = vec![fact1, fact2, fact3];
    let row_ids = db.insert_batch(&facts).await?;
    
    Ok(())
}
```

---

## Data Model

### Fact Structure

KCM organizes knowledge as *Facts*—immutable tuples with metadata:

```rust
pub struct Fact {
    pub subject: SubjectID,      // u32 — dictionary-encoded entity
    pub predicate: PredicateID,  // u8  — dictionary-encoded relationship
    pub object: ObjectID,        // u32 — dictionary-encoded entity/value
    pub confidence: f64,         // f64 — [0.0..1.0] belief strength
    pub evidence: EvidenceID,    // u8  — evidence code/classification
    pub timestamp: i64,          // i64 — nanoseconds since Unix epoch
    pub context: ContextID,      // u8  — semantic context tag
    pub version: i32,            // i32 — monotonic update counter
    pub priority: i8,            // i8  — [-128..127] execution priority
    pub owner: u16,              // u16 — dictionary-encoded owner/source
}
```

**Size:** 34 bytes per fact (uncompressed)

### Column Storage Layout

| Column | Type | Encoding | Compression | Purpose |
|--------|------|----------|-------------|---------|
| Subject | u32 | Dictionary | Zstd | Entity or concept being described |
| Predicate | u8 | Dictionary | RLE | Relationship type/attribute |
| Object | u32 | Dictionary | Zstd | Target entity, attribute value |
| Confidence | f64 | Gorilla | Zstd | Statistical confidence/certainty |
| Evidence | u8 | Dictionary | RLE | Evidence type classification |
| Timestamp | i64 | Delta | Zstd | Temporal ordering and retention |
| Context | u8 | Dictionary | RLE | Semantic or logical context |
| Version | i32 | Delta | LZ4 | Update history and lineage |
| Priority | i8 | Identity | RLE | Execution and processing order |
| Owner | u16 | Dictionary | Zstd | Source, responsibility, access control |

### Encoding Strategies

- **Dictionary:** Maps repeated values to 16-bit IDs for compression
- **Delta:** Stores differences from previous values for temporal data
- **Gorilla:** Time-series compression for floating-point confidence scores
- **RLE:** Run-Length Encoding for low-cardinality categorical data

---

## Deployment Options

### 1. Docker Compose (Development & Staging)

```yaml
version: '3.8'
services:
  kcm:
    build:
      context: .
      dockerfile: deployment/Dockerfile
    container_name: kcm-server
    volumes:
      - kcm_data:/data
      - kcm_logs:/var/log/kcm
    environment:
      RUST_LOG: info
      KCM_DATA_PATH: /data/kcm.db
      KCM_HTTP_ADDR: 0.0.0.0:8080
      KCM_GRPC_ADDR: 0.0.0.0:50051
    ports:
      - "8080:8080"
      - "50051:50051"
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 10s
      timeout: 5s
      retries: 3

  prometheus:
    image: prom/prometheus:latest
    volumes:
      - ./deployment/prometheus/prometheus.yml:/etc/prometheus/prometheus.yml
      - prometheus_data:/prometheus
    command:
      - '--config.file=/etc/prometheus/prometheus.yml'
      - '--storage.tsdb.path=/prometheus'
    ports:
      - "9090:9090"

  grafana:
    image: grafana/grafana:latest
    volumes:
      - grafana_data:/var/lib/grafana
      - ./deployment/grafana/kcm-dashboard.json:/etc/grafana/provisioning/dashboards/kcm.json
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin
    ports:
      - "3000:3000"
    depends_on:
      - prometheus

volumes:
  kcm_data:
    driver: local
  kcm_logs:
    driver: local
  prometheus_data:
    driver: local
  grafana_data:
    driver: local
```

### 2. Kubernetes (Production)

```bash
# Install using Helm
helm repo add kcm https://kcm-project.org/charts
helm repo update

# Deploy with default values
helm install kcm kcm/kcm \
  --namespace kcm \
  --create-namespace

# Custom values
helm install kcm kcm/kcm \
  --namespace kcm \
  --create-namespace \
  -f deployment/helm/kcm/values-production.yaml
```

### 3. Bare Metal (Enterprise)

```bash
# Build optimized binary
cargo build --release --workspace

# Create systemd service
sudo tee /etc/systemd/system/kcm.service > /dev/null <<EOF
[Unit]
Description=KCM Knowledge Engine
After=network.target

[Service]
Type=simple
User=kcm
WorkingDirectory=/opt/kcm
ExecStart=/opt/kcm/kcm-server
Restart=on-failure
RestartSec=10s
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl daemon-reload
sudo systemctl enable kcm
sudo systemctl start kcm
```

---

## Quality Assurance

### Testing Strategy

KCM implements comprehensive testing across multiple levels:

```
Unit Tests (1000+ tests)
    ↓
Integration Tests (200+ scenarios)
    ↓
Property-Based Tests (Proptest)
    ↓
Performance Benchmarks (Criterion)
    ↓
Security Tests (OWASP, Fuzzing)
    ↓
Distributed Tests (Chaos engineering)
    ↓
Soak Tests (72-hour stability)
```

### Run Test Suite

```bash
# All tests
cargo test --workspace --all-features

# Specific test type
cargo test --lib                          # Unit tests
cargo test --test '*'                     # Integration tests
cargo test security_tests --all           # Security tests
cargo test property_tests --all           # Property tests

# With profiling
cargo test --workspace --release -- --nocapture --test-threads=1

# Benchmarks
cargo bench --workspace --all-features
```

### Quality Gates

Every commit must pass these mandatory gates:

| Gate | Command | Purpose |
|------|---------|---------|
| **Build** | `cargo build --workspace --all-features` | Compilation correctness |
| **Format** | `cargo fmt --all -- --check` | Code style consistency |
| **Lint** | `cargo clippy --workspace -- -D warnings` | Code quality rules |
| **Tests** | `cargo test --workspace --all-features` | Functional correctness |
| **SSOT** | `bash scripts/validate-ssot.sh` | Documentation coherence |
| **Security** | Custom analysis | Zero unsafe unwrap in prod code |

---

## Documentation

### Core Documentation

| Document | Purpose | Audience |
|----------|---------|----------|
| [PRD.md](docs/PRD.md) | Product requirements for core engine | Product, Engineering |
| [KCM_ARCHITECTURE.md](docs/KCM_ARCHITECTURE.md) | System design and component interaction | Architects, Senior Engineers |
| [KCM_SPECIFICATION.md](docs/KCM_SPECIFICATION.md) | Complete technical specification | Implementers |
| [KCM_API_SPEC.md](docs/KCM_API_SPEC.md) | REST, gRPC, FFI interfaces | Integrators |
| [KCM_PERFORMANCE_SPEC.md](docs/KCM_PERFORMANCE_SPEC.md) | Performance SLOs and benchmarks | Operations, DevOps |
| [KCM_SECURITY_TRUST_SPEC.md](docs/KCM_SECURITY_TRUST_SPEC.md) | Security model and threat analysis | Security, Compliance |

### Getting Started

- **[Installation Guide](docs/tutorials/01-installation.md)** — Setup and configuration
- **[First Database](docs/tutorials/02-first-database.md)** — Your first KCM instance
- **[Basic Queries](docs/tutorials/03-basic-queries.md)** — Query fundamentals
- **[Transactions](docs/tutorials/04-transactions.md)** — ACID transaction patterns
- **[Reasoning](docs/tutorials/05-reasoning.md)** — Inference engine usage

### Operational Guides

- **[Operations](docs/guides/operations.md)** — Runtime management and monitoring
- **[Backup & Recovery](docs/guides/backup-recovery.md)** — Disaster recovery procedures
- **[Monitoring](docs/guides/monitoring.md)** — Observability and alerting
- **[Security Hardening](docs/guides/security-hardening.md)** — Production security checklist

### Community & Support

- **[Contributing Guide](CONTRIBUTING.md)** — Development workflow
- **[Code of Conduct](CODE_OF_CONDUCT.md)** — Community standards
- **[Security Policy](SECURITY.md)** — Vulnerability reporting

---

## Contributing

We welcome contributions from the community! Please follow these steps:

### 1. Fork & Clone

```bash
git clone https://github.com/yourusername/kcm.git
cd kcm
git remote add upstream https://github.com/original/kcm.git
```

### 2. Create Feature Branch

```bash
git checkout -b feature/amazing-feature
```

### 3. Development Workflow

```bash
# Make changes and run quality gates
cargo build --workspace
cargo fmt --all
cargo clippy --workspace -- -D warnings
cargo test --workspace --all-features

# Run SSOT validation
bash scripts/validate-ssot.sh
```

### 4. Commit & Push

```bash
git add .
git commit -m "feat: add amazing feature"
git push origin feature/amazing-feature
```

### 5. Pull Request

- Create PR on GitHub with detailed description
- Reference relevant issues using `#issue_number`
- Ensure CI/CD pipeline passes
- Wait for code review from maintainers

### Development Principles

- **Single Responsibility:** Each component has one clear purpose
- **Zero Unsafe:** No `unsafe` code in production paths (unsafe blocks in kernel only)
- **Documentation:** Public APIs must include doc comments and examples
- **Testing:** Add tests for new features (aim for >90% coverage)
- **Performance:** Benchmark-driven optimization, no premature optimization

---

## Performance Benchmarks

KCM delivers enterprise-class performance across workloads:

### Throughput

- **Insert:** 2.5M facts/second (single-threaded)
- **Query:** 50M rows/second scan rate (vectorized)
- **Batch:** 10M facts/second (with compression)

### Latency (p99)

- **Point Lookup:** <1ms
- **Small Query:** <10ms
- **Analytical Query:** <100ms

### Storage

- **Compression Ratio:** 10:1 (typical)
- **Index Overhead:** 3-5% additional
- **Memory Resident:** 0.5GB per 1M facts

*See [KCM_PERFORMANCE_SPEC.md](docs/KCM_PERFORMANCE_SPEC.md) for detailed benchmarks.*

---

## System Requirements

### Minimum (Development)

- CPU: 2 cores
- RAM: 4GB
- Storage: 10GB SSD
- OS: Linux, macOS, Windows (WSL2)

### Recommended (Production)

- CPU: 8+ cores
- RAM: 32GB+
- Storage: 1TB+ SSD (NVMe preferred)
- OS: Linux (Ubuntu 20.04 LTS+ or RHEL 8+)
- Network: 1Gbps minimum

### Supported Platforms

- ✅ Linux (x86_64, aarch64)
- ✅ macOS (Intel, Apple Silicon)
- ✅ Windows (WSL2)
- ✅ Docker/Kubernetes
- ✅ Cloud (AWS, GCP, Azure)

---

## License

KCM is released under the **MIT License**. See [LICENSE](LICENSE) for full details.

```
MIT License

Copyright (c) 2024 KCM Project Contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
```

---

## Support & Community

### Getting Help

- 📖 **[Documentation](docs/)** — Comprehensive guides and tutorials
- 💬 **[GitHub Discussions](https://github.com/kcm/kcm/discussions)** — Community Q&A
- 🐛 **[Issue Tracker](https://github.com/kcm/kcm/issues)** — Bug reports and feature requests
- 🔒 **[Security](SECURITY.md)** — Report vulnerabilities responsibly

### Stay Connected

- ⭐ Star the repository to show support
- 🔔 Watch for releases and announcements
- 🤝 Contribute code, docs, or bug reports
- 💡 Share your projects and use cases

---

## Acknowledgments

KCM is built with contributions from the open-source community. Special thanks to:

- Rust compiler and ecosystem
- Apache Arrow community
- Database systems research community
- All contributors and maintainers

---

<div align="center">
  <p><strong>Built with ❤️ by the KCM Engineering Team</strong></p>
  <p>
    <a href="https://github.com/kcm/kcm/stargazers">⭐ Stars</a> •
    <a href="https://github.com/kcm/kcm/network/members">🔗 Forks</a> •
    <a href="https://github.com/kcm/kcm/issues">🐛 Issues</a> •
    <a href="CONTRIBUTING.md">🤝 Contribute</a>
  </p>
  <p>
    <sub>
      <a href="docs/">Documentation</a> •
      <a href="CHANGELOG.md">Changelog</a> •
      <a href="LICENSE">License</a> •
      <a href="CODE_OF_CONDUCT.md">Code of Conduct</a>
    </sub>
  </p>
</div>
