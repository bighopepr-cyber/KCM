# Workspace Layout

| Field | Value |
|-------|-------|
| **Document ID** | KCM-REPO-003 |
| **Title** | Workspace Layout |
| **Version** | 1.0.0 |
| **Date** | 2026-08-03 |
| **Status** | Authoritative |
| **Authority** | Engineering Orchestrator (P1) |

---

## 1. Cargo Workspace Configuration

```toml
[workspace]
members = [
    "crates/kcm-core",
    "crates/kcm-storage",
    "crates/kcm-compute",
    "crates/kcm-reasoning",
    "crates/kcm-optimizer",
    "crates/kcm-runtime",
    "crates/kcm-interface",
    "crates/kcm-distributed",
    "crates/kcm-ml",
    "crates/kcm-security",
    "crates/kcm-compliance",
    "crates/kcm-testing",
    "crates/kcm-server",
]
resolver = "2"
```

## 2. Centralized Dependencies

All shared dependencies are managed via `[workspace.dependencies]`:

```toml
[workspace.dependencies]
parking_lot = "0.12"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
zstd = "0.13"
lz4 = "1.24"
blake3 = "1.5"
thiserror = "2.0"
log = "0.4"
env_logger = "0.11"
rayon = "1.7"
tokio = { version = "1.35", features = ["full"] }
pyo3 = { version = "0.22", features = ["extension-module"] }
actix-web = "4"
tonic = "0.12"
prost = "0.13"
aes-gcm = "0.10"
getrandom = "0.2"
tempfile = "3"
criterion = { version = "0.5", features = ["html_reports"] }
proptest = "1.0"
```

Each crate references workspace dependencies as:

```toml
[dependencies]
kcm-core = { workspace = true }
parking_lot = { workspace = true }
```

## 3. Feature Gates

| Feature | Crate | Purpose |
|---------|-------|---------|
| `python` | kcm-interface | Enable PyO3 Python bindings |
| `serialization` | kcm-core | Enable serde support |

## 4. Build Profiles

| Profile | opt-level | lto | codegen-units | strip | Purpose |
|---------|-----------|-----|---------------|-------|---------|
| dev | 0 | no | 16 | no | Development |
| release | 3 | true | 1 | true | Production |
| bench | 3 | true | 1 | true | Benchmarking |
