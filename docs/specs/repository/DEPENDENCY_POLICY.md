# Dependency Policy

| Field | Value |
|-------|-------|
| **Document ID** | KCM-REPO-009 |
| **Title** | Dependency Policy |
| **Version** | 1.0.0 |
| **Date** | 2026-08-03 |
| **Status** | Authoritative |
| **Authority** | Engineering Orchestrator (P1) |

---

## 1. Allowed Dependencies

| Dependency | Version | Justification |
|------------|---------|---------------|
| parking_lot | 0.12 | 3-5x faster RwLock/Mutex |
| serde/serde_json | 1.0 | Serialization framework |
| zstd | 0.13 | Industry-standard compression |
| lz4 | 1.24 | Speed-optimized compression |
| blake3 | 1.5 | Fastest cryptographic hash |
| thiserror | 2.0 | Error derive macro |
| log | 0.4 | Logging facade |
| env_logger | 0.11 | Log output |
| rayon | 1.7 | Work-stealing parallelism |
| tokio | 1.35 | Async runtime |
| pyo3 | 0.22 | Python bindings (feature-gated) |
| actix-web | 4 | HTTP server |
| tonic | 0.12 | gRPC framework |
| prost | 0.13 | Protobuf encoding |
| aes-gcm | 0.10 | Authenticated encryption |
| getrandom | 0.2 | CSPRNG |
| tempfile | 3 | Temporary files |
| criterion | 0.5 | Benchmarking (dev) |
| proptest | 1.0 | Property testing (dev) |

## 2. Dependency Management

All dependencies are centralized via `[workspace.dependencies]` in root `Cargo.toml`.

## 3. Audit Requirements

- Run `cargo audit` before releases
- Run `cargo deny check` for license compliance
- No known vulnerabilities in dependency tree

## 4. License Compatibility

Allowed licenses: MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, Zlib

Prohibited: GPL, AGPL, SSPL, EUPL
