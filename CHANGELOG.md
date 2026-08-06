# Changelog

All notable changes to the KCM project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-08-06

### Added
- **Core Engine**: Columnar storage with 10 physical columns, per-column encoding and compression
- **Dictionary Encoding**: Efficient string/reference storage via integer mapping
- **SIMD-Accelerated Compute**: AVX2-optimized query operators for columnar scan and filter
- **Cost-Based Optimizer**: Filter pushdown, column pruning, join reordering with statistics
- **Forward-Chaining Inference**: Rule-based reasoning engine with conflict resolution
- **ACID Transactions**: Full transaction support with isolation levels
- **Batch Operations**: High-throughput bulk insert/update/delete
- **Crash Recovery**: WAL-based recovery with BLAKE3 integrity verification
- **Schema Evolution**: Dynamic schema management with validation
- **RBAC**: 5-level permission model (Read, Write, Delete, Execute, Admin)
- **AES-256-GCM Encryption**: Authenticated encryption at rest with BLAKE3 KDF
- **Audit Logging**: Hash-chained tamper-evident audit trail (FIFO at 100K events)
- **GDPR Compliance**: Consent management and 4-tier data classification
- **C FFI**: 18 functions for language interop with null-pointer guards
- **Python Bindings**: PyO3-based Python integration
- **REST API**: HTTP endpoints with OpenAPI specification
- **gRPC**: High-performance RPC interface via tonic/prost
- **KQL**: Knowledge Query Language parser
- **CLI Tools**: 17 specialized CLI binaries for operations, diagnostics, and migration
- **Distributed**: Sharding strategies (Hash, Range, ConsistentHash) with 2PC coordinator
- **ML Engine**: Learned index (regression), confidence learner, rule discovery
- **13 Core Crates**: kcm-core, kcm-storage, kcm-compute, kcm-reasoning, kcm-optimizer, kcm-runtime, kcm-interface, kcm-distributed, kcm-ml, kcm-security, kcm-compliance, kcm-testing, kcm-server
- **9 SDKs**: Rust, C, C++, Python, JavaScript, TypeScript, Go, Java, .NET
- **Deployment**: Docker, Kubernetes, Helm, Prometheus, Grafana
- **Testing**: Unit, integration, property-based, security, stress, load, recovery tests
- **Benchmarks**: Criterion-based benchmarking with regression detection
- **Documentation**: Enterprise-grade specs, ADRs, operational runbooks, SDK guides

### Security
- AES-256-GCM encryption for data at rest
- BLAKE3 key derivation
- Hash-chained audit logging
- RBAC enforcement on all sensitive operations
- TLS for all network communication
- Input validation on all public interfaces
- Null-pointer guards on all FFI functions

### Engineering
- Semantic Versioning 2.0.0 with canonical VERSION file
- SSOT-first development with 16 AI engineering skills
- CI/CD pipeline with format, lint, build, test, SSOT validation
- Version governance with automated synchronization and verification
