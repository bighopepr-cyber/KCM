# KCM Engineering Handbook

**Document ID:** KCM-HB-001  
**Version:** 2.0.0  
**Status:** Active  
**Owner:** Code Quality Guardian (P10)

---

## 1. Purpose

Single consolidated engineering guide for KCM development. Replaces the previous 40+ scattered documentation files.

## 2. Development Setup

### Prerequisites

- Rust 1.85+ (see `rust-toolchain.toml`)
- Docker (for deployment testing)

### Build Commands

```bash
cargo build --workspace           # Debug build
cargo build --release --workspace # Release build (optimized, LTO, stripped)
```

### Test Commands

```bash
cargo test --workspace            # All tests
cargo test --workspace -- --nocapture  # With output
cargo test -p kcm-core            # Single crate
```

### Quality Gate Commands

```bash
cargo fmt --all -- --check        # Format check
cargo clippy --workspace -- -D warnings  # Lint
cargo bench --workspace --no-run  # Benchmark compile
bash scripts/validate-ssot.sh     # SSOT validation
```

## 3. Coding Standards

### Error Handling

- All public APIs return `Result<T, KcmError>`
- `KcmError` variants: NotFound, OutOfMemory, InvalidArgument, Io, Corrupted, Conflict, TransactionAborted
- No `unwrap()` in production code paths
- No `panic!()` in production code
- No TODO/FIXME/HACK in production code

### Concurrency

| Component | Mechanism |
|-----------|-----------|
| Schema | `Arc<RwLock<Schema>>` (parking_lot) |
| Dictionaries | `Arc<RwLock<Dictionary>>` (parking_lot) |
| WAL | `Mutex<File>` (parking_lot) |
| Audit Log | `Mutex<VecDeque<AuditEvent>>` (parking_lot) |
| Metrics | `AtomicU64` (lock-free) |
| Thread Pool | rayon ThreadPool |
| Async | tokio Runtime |

### Rust Idioms

- Use `parking_lot` for mutexes/rwlocks (not std)
- Use `Send + Sync` bounds on all shared types
- Use `is_some_and` over `map_or(false, ...)`
- Use `div_ceil` instead of manual ceiling division
- Use `clamp` instead of chained `min/max`
- Use `or_default()` instead of `or_insert_with(Vec::new)`

## 4. Architecture Rules

- No circular crate dependencies
- kcm-core has zero internal dependencies
- All inter-crate communication through public API only
- New modules must have corresponding tests
- Feature-gated dependencies must have `#[cfg(feature)]`

## 5. Testing Rules

| Rule | Description |
|------|-------------|
| TR-001 | Every PR must pass `cargo test --workspace` |
| TR-002 | Every PR must pass `cargo clippy --workspace` |
| TR-003 | Every PR must pass `cargo fmt --check` |
| TR-004 | New code must have ≥ 95% test coverage |
| TR-005 | Security code must have security tests |
| TR-006 | Performance-critical code must have benchmarks |
| TR-007 | Property tests required for arithmetic operations |

## 6. Performance Rules

| Rule | Threshold |
|------|-----------|
| Benchmark regression | < 5% from baseline |
| Memory usage | < 100 bytes/fact |
| Compression ratio | > 5x |
| Query latency (1M facts) | P99 < 100ms |

## 7. Documentation Rules

- Architecture changes update `KCM_ARCHITECTURE.md` (in docs/specs/)
- API changes update `KCM_API_SPEC.md` (in docs/specs/)
- All changes must pass `bash scripts/validate-ssot.sh`

## 8. Security Rules

- No hardcoded secrets or keys
- Encryption must use AEAD (AES-256-GCM)
- Key generation must use CSPRNG
- All user input must be validated
- Audit logging for all write operations

## 9. Git Workflow

1. Fork and clone
2. Create feature branch: `git checkout -b feature/name`
3. Find SSOT requirement in PRD docs
4. Implement matching specification
5. Write tests validating implementation
6. Run quality gates
7. Submit PR with SSOT requirement reference
8. Code review + CI pass + merge

## 10. Deployment

### Docker

```bash
docker build -t kcm:latest .
docker run -d -p 8080:8080 -v kcm_data:/data kcm:latest
```

### Kubernetes

```bash
kubectl apply -f deployment/k8s/deployment.yaml
```

### Helm

```bash
helm install kcm deployment/helm/kcm
```

## 11. Glossary

| Term | Definition |
|------|-----------|
| Fact | A (subject, predicate, object, confidence) tuple |
| Column | Independent typed array for one attribute |
| DenseVec | Aligned, growable vector with zero-copy slice access |
| Bitmap | Bit-level set for fast intersection/union |
| Dictionary | String-to-integer mapping for encoding |
| WAL | Write-Ahead Log for crash recovery |
| KQL | Knowledge Query Language |
| FFI | Foreign Function Interface (C ABI) |
| RBAC | Role-Based Access Control |
| 2PC | Two-Phase Commit protocol |
| SSOT | Single Source of Truth |
