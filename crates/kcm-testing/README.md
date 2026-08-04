# kcm-testing

Testing infrastructure for KCM: load, stress, security, recovery, and regression testing with metrics dashboard.

## Purpose

Provides comprehensive testing frameworks that validate KCM's correctness, performance, security, and reliability under various conditions.

## Modules

| Module | Purpose |
|--------|---------|
| `load_tests` | Sustained workload testing |
| `stress_tests` | Extreme load and scale testing |
| `security_tests` | Security attack surface validation |
| `regression_detector` | Performance regression detection |
| `metrics_dashboard` | Test metrics collection and reporting |
| `bench_fixtures` | Benchmark data generators |

## Dependencies

| Dependency | Type | Purpose |
|------------|------|---------|
| `kcm-core` | dependency | Core types |
| `kcm-storage` | dependency | Storage layer testing |
| `kcm-runtime` | dependency | Runtime testing |
| `kcm-reasoning` | dependency | Reasoning engine testing |
| `kcm-security` | dependency | Security testing |
| `kcm-distributed` | dev-dependency | Distributed testing |
| `kcm-compliance` | dev-dependency | Compliance testing |
| `parking_lot` | dependency | Concurrent access |
| `tempfile` | dependency | Temporary test databases |

## Test Categories

### Load Tests

Sustained throughput testing:
- Concurrent read/write operations
- Mixed workload (80% read / 20% write)
- Configurable duration and thread count

### Stress Tests

Extreme conditions:
- Memory pressure testing
- Connection exhaustion
- Disk full scenarios
- Maximum concurrent transactions

### Security Tests

Attack surface validation:
- Input fuzzing (malformed queries)
- Buffer overflow attempts
- SQL injection in KQL
- Encryption key handling
- RBAC bypass attempts
- Audit log tampering

### Recovery Tests

Crash recovery validation:
- WAL replay correctness
- Partial commit recovery
- Backup/restore integrity
- Corruption detection

### Regression Detection

Performance baseline comparison:
- Metric collection during test runs
- Baseline storage and comparison
- Threshold-based regression alerts

## Running Tests

```bash
# All testing suite
cargo test --workspace -p kcm-testing

# Load tests only
cargo test load_tests --workspace -- --nocapture

# Stress tests only
cargo test stress_tests --workspace -- --nocapture

# Security tests only
cargo test security_tests --workspace -- --nocapture

# Recovery tests only
cargo test recovery --workspace -- --nocapture
```

## Metrics Dashboard

Collects and reports:
- Throughput (ops/sec)
- Latency (p50, p95, p99)
- Memory usage
- CPU utilization
- Error rates
- Recovery times
