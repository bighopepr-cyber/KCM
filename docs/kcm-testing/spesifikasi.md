# kcm-testing Technical Specification

## Overview

kcm-testing is the testing infrastructure crate for KCM. It provides the foundational testing capabilities that validate correctness, performance, security, and resilience of the entire KCM system. The crate generates deterministic, reproducible test data and exercises the system under controlled conditions ranging from normal load to extreme stress.

## Scope

kcm-testing covers:

- Load testing with configurable concurrent users and operation mixes
- Stress testing for resource exhaustion and graceful degradation
- Security testing for attack surface validation
- Chaos engineering with configurable fault injection
- Regression detection with configurable thresholds
- Real-time metrics collection and reporting
- Deterministic benchmark fixtures for reproducible performance testing

kcm-testing does **not** cover:

- Distributed system testing (covered by integration tests using kcm-distributed)
- Compliance testing (covered by integration tests using kcm-compliance)
- The actual production code of other crates (kcm-testing is a consumer, not a provider)

## Responsibilities

| Responsibility | Module | Description |
|---------------|--------|-------------|
| Load Testing | `load_tests` | Configurable concurrent load scenarios with QPS and latency measurement |
| Stress Testing | `stress_tests` | Resource exhaustion scenarios with graceful degradation validation |
| Security Testing | `security_tests` | Attack surface validation: injection, overflow, RBAC, timing, memory safety |
| Chaos Engineering | `chaos` | Fault injection via CSPRNG-driven packet loss, latency, and partition simulation |
| Regression Detection | `regression_detector` | Benchmark comparison with configurable thresholds and severity levels |
| Metrics Dashboard | `metrics_dashboard` | Real-time test metrics collection, aggregation, and reporting |
| Benchmark Fixtures | `bench_fixtures` | Standardized, deterministic dataset generators for all benchmark suites |

## Technical Specification

### LoadTests (`load_tests`)

Configurable concurrent load testing with measurement of throughput and latency.

```rust
pub struct LoadTestScenario {
    pub name: String,
    pub concurrent_users: usize,
    pub operations_per_user: u64,
    pub initial_facts: u64,
    pub expected_qps: f64,
    pub max_latency_p99_ms: f64,
}

pub struct LoadTestResults {
    pub scenario: String,
    pub total_operations: u64,
    pub failed_operations: u64,
    pub elapsed_secs: f64,
    pub actual_qps: f64,
    pub avg_latency_ms: f64,
    pub p99_latency_ms: f64,
}
```

- `run_load_test(scenario)` spawns `concurrent_users` threads, each executing `operations_per_user` operations (mix of inserts and queries).
- Measures actual QPS, average latency, and P99 latency.
- Pass criterion: `failed_operations <= total_operations / 1000` (0.1% failure rate).
- Pre-populates database with `initial_facts` before starting concurrent operations.
- Operation mix: 1/3 inserts, 2/3 queries (deterministic by thread + iteration index).

### StressTests (`stress_tests`)

Resource exhaustion scenarios with graceful degradation validation.

```rust
pub struct StressTestScenario {
    pub name: String,
    pub max_concurrent_users: usize,
    pub duration_secs: u64,
}

pub struct StressTestConfig {
    pub name: String,
    pub max_concurrent_users: usize,
    pub ops_per_user: u64,
    pub batch_size: usize,
}
```

- `run_stress_test(scenario)` runs for a fixed duration with `max_concurrent_users` threads operating continuously.
- `run_stress_test_config(config)` runs a bounded number of operations per user with configurable batch sizes.
- Graceful degradation criterion: `failure_rate < 10%`.
- Operation mix: 1/3 inserts, 2/3 queries (deterministic).

### SecurityTests (`security_tests`)

Attack surface validation for core KCM components. Module is `#[cfg(test)]` only.

| Test | Validates |
|------|-----------|
| `test_injection_prevention` | SQL/XSS/path traversal injection via dictionary insert |
| `test_buffer_overflow_prevention` | DenseVec capacity boundary enforcement |
| `test_integer_overflow_subject_id` | SubjectID at u32::MAX boundary |
| `test_confidence_boundary_rejection` | Confidence rejects NaN, infinity, out-of-range values |
| `test_rbac_enforcement` | Role-based permission checking |
| `test_rbac_admin_role` | Admin role grants all permissions |
| `test_context_isolation` | Cross-context permission isolation |
| `test_timing_attack_mitigation` | Dictionary lookup timing ratio bounded |
| `test_memory_safety_no_use_after_free` | DenseVec slice access after push |
| `test_bitmap_boundary_access` | Bitmap access at boundaries and out-of-range |
| `test_large_fact_insertion` | 10K fact insertion correctness |
| `test_concurrent_fact_insertion_safety` | 4-thread concurrent insertion correctness |
| `test_dictionary_concurrent_access` | 4-thread concurrent dictionary operations |
| `test_negative_confidence_rejected` | Negative confidence values rejected |
| `test_fact_equality` | Fact field equality comparison |
| `test_error_handling_consistency` | Error returns for invalid operations |
| `test_dictionary_capacity_stress` | 50K dictionary entry capacity |
| `test_bitmap_large_scale` | 1M bitmap with sparse density |
| `test_query_after_delete_consistency` | Query results correct after deletion |

### Chaos Engineering (`chaos`)

Fault injection via CSPRNG-driven failure simulation.

```rust
pub struct ChaosConfig {
    pub packet_loss_percent: f64,
    pub latency_ms: u64,
    pub partition_duration: Duration,
    pub node_failure_count: usize,
}

pub struct ChaosMonkey {
    active: Arc<AtomicBool>,
    failure_count: Arc<AtomicU64>,
    config: ChaosConfig,
}
```

- `ChaosMonkey::new(config)` creates a fault injector.
- `activate()` / `deactivate()` control injection state.
- `should_inject_failure()` returns `true` when active and CSPRNG selects failure (based on `packet_loss_percent`).
- `inject_latency()` sleeps for `latency_ms` when active.
- `record_failure()` increments atomic failure counter.
- CSPRNG via `getrandom::getrandom` for cryptographically secure randomness.

### RegressionDetector (`regression_detector`)

Benchmark comparison with configurable thresholds and severity classification.

```rust
pub struct RegressionDetector {
    baselines: Vec<RegressionBaseline>,
    threshold: f64,  // default: 0.05 (5%)
}

pub enum Severity {
    Low,      // change > 5%
    Medium,   // change > 5% (above threshold)
    High,     // change > 10%
    Critical, // change > 20%
}
```

- `detect(current)` compares current metrics against the latest baseline.
- Default threshold: 5% (configurable via `with_threshold()`).
- Severity classification: Low (5-10%), Medium (5-10%), High (10-20%), Critical (>20%).
- Baselines can be loaded incrementally; detection uses the most recent baseline.
- Zero-valued baselines are ignored (avoids division by zero).

### MetricsDashboard (`metrics_dashboard`)

Real-time test metrics collection, aggregation, and reporting.

```rust
pub struct TestMetrics {
    pub total: u64,
    pub passed: u64,
    pub failed: u64,
    pub skipped: u64,
    pub execution_time_secs: f64,
}

pub struct MetricsCollector {
    test_metrics: HashMap<String, TestMetrics>,
    perf_metrics: HashMap<String, PerformanceMetrics>,
    start_time: Instant,
}
```

- `MetricsCollector::new()` starts the collection timer.
- `record_test_suite(name, metrics)` records per-suite results.
- `record_performance(name, metrics)` records performance metrics.
- `generate_report()` produces a formatted dashboard report.
- `overall_pass_rate()` computes aggregate pass rate across all suites.

### BenchFixtures (`bench_fixtures`)

Standardized, deterministic dataset generators for all benchmark suites.

| Fixture | Purpose |
|---------|---------|
| `DatasetConfig` | Configuration for deterministic dataset generation with validation |
| `ColumnFixture` | Pre-computed `DenseVec<u32>` for column benchmarks |
| `U8ColumnFixture` | Pre-computed `DenseVec<u8>` for SIMD filter benchmarks |
| `BitmapFixture` | Pre-computed bitmap with deterministic density |
| `DenseVecU64Fixture` | Pre-computed `DenseVec<u64>` for memory benchmarks |
| `DictionaryFixture` | Pre-computed dictionary with deterministic entries |
| `SchemaFixture` | Pre-populated `Schema` with deterministic facts |
| `DatabaseFixture` | Pre-populated `KnowledgeDatabase` with deterministic facts |
| `CompressionFixture` | Pre-computed compressed data for codec benchmarks |
| `WalBenchmarkFixture` | Self-contained WAL lifecycle fixture with auto-cleanup |
| `FileFormatFixture` | Pre-computed file format save/load fixture |
| `RuleFixture` | Pre-computed rule registry for reasoning benchmarks |

All fixtures are deterministic: same config always produces same data. No randomness, no timestamps, no environmental dependencies.

Canonical dataset sizes: `1K, 10K, 100K, 1M` facts. Enterprise scale: `10M, 100M, 1B`.

## Architecture

kcm-testing is organized as a flat module structure with 7 source modules under `src/`:

```
kcm-testing
├── Cargo.toml
├── src/
│   ├── lib.rs                  — Crate root, module declarations
│   ├── bench_fixtures.rs       — Deterministic benchmark data generators
│   ├── chaos.rs                — Chaos engineering fault injection
│   ├── load_tests.rs           — Concurrent load testing
│   ├── metrics_dashboard.rs    — Test metrics collection and reporting
│   ├── regression_detector.rs  — Benchmark regression detection
│   ├── security_tests.rs       — Security attack surface validation
│   └── stress_tests.rs         — Stress testing with graceful degradation
└── tests/
    ├── test_concurrent_access.rs
    ├── test_crash_recovery.rs
    ├── test_distributed.rs
    ├── test_fuzz_kql.rs
    ├── test_gdpr.rs
    ├── test_integration_cli.rs
    ├── test_recovery.rs
    ├── test_soak.rs
    ├── test_stress_concurrent.rs
    ├── test_stress_scale.rs
    ├── test_additional.rs
    ├── test_wal_recovery.rs
    └── test_concurrent_access.rs
```

## Internal Components

### lib.rs

Crate root. Declares all public modules and gates `security_tests` with `#[cfg(test)]`. Applies `#[allow(clippy::unwrap_used, clippy::panic)]` crate-wide for test code.

### bench_fixtures.rs

Provides canonical, deterministic, and reproducible dataset generators. All fixtures use modular arithmetic for determinism. `DatasetConfig` validates parameters against KCM type constraints. `WalBenchmarkFixture` manages the full WAL lifecycle including creation, population, flush, validation, and cleanup via `TempDir`.

### chaos.rs

Implements `ChaosMonkey` for fault injection. Uses `Arc<AtomicBool>` for activation state and `Arc<AtomicU64>` for failure counting. CSPRNG via `getrandom::getrandom` for cryptographically secure fault probability.

### load_tests.rs

Implements `run_load_test()` for concurrent load testing. Spawns OS threads via `std::thread::spawn`. Measures QPS, average latency, and P99 latency. Uses `Arc<AtomicU64>` for thread-safe operation counting and `parking_lot::Mutex<Vec<f64>>` for latency collection.

### metrics_dashboard.rs

Implements `MetricsCollector` for aggregating test results across suites. Tracks per-suite `TestMetrics` and `PerformanceMetrics`. Generates formatted reports with pass rates, timing, and per-suite breakdowns.

### regression_detector.rs

Implements `RegressionDetector` for benchmark comparison. Loads baselines incrementally, compares against latest baseline, classifies severity. Zero-valued baselines are skipped to avoid division by zero.

### security_tests.rs

Gated with `#[cfg(test)]`. Contains 19 test functions validating injection prevention, buffer overflow, integer overflow, confidence boundaries, RBAC enforcement, context isolation, timing attacks, memory safety, concurrent access, and error handling consistency.

### stress_tests.rs

Implements `run_stress_test()` (duration-based) and `run_stress_test_config()` (operation-count-based). Both validate graceful degradation (< 10% failure rate).

## Data Model

### Test Data Flow

```
DatasetConfig → deterministic_fact() → Fact → KnowledgeDatabase / Schema
```

All test data flows through `DatasetConfig` to ensure determinism and reproducibility.

### Metrics Data Flow

```
Test Execution → TestMetrics / PerformanceMetrics → MetricsCollector → Report
```

### Regression Data Flow

```
Baseline (HashMap<String, f64>) → RegressionDetector → RegressionAlert → Report
```

## Execution Flow

### Test Execution Pipeline

```
1. Configure
   └── Create LoadTestScenario / StressTestScenario / ChaosConfig / DatasetConfig

2. Setup
   └── Create KnowledgeDatabase / Schema
   └── Pre-populate with deterministic facts (optional)

3. Execute
   ├── LoadTest: Spawn threads → Execute operations → Collect metrics
   ├── StressTest: Spawn threads → Run for duration → Stop threads → Collect metrics
   ├── SecurityTest: Exercise attack vectors → Assert expected behavior
   └── ChaosTest: Activate monkey → Inject faults → Deactivate → Collect results

4. Measure
   ├── Collect QPS, latency, failure rates
   ├── Record to MetricsCollector
   └── Compare against RegressionDetector baselines

5. Report
   ├── Generate test report
   ├── Generate performance report
   └── Check regression thresholds
```

## Public API

### Load Testing

```rust
pub fn run_load_test(scenario: &LoadTestScenario) -> Result<LoadTestResults, KcmError>
```

### Stress Testing

```rust
pub fn run_stress_test(scenario: &StressTestScenario) -> Result<StressTestResults, KcmError>
pub fn run_stress_test_config(config: &StressTestConfig) -> Result<StressTestConfigResults, KcmError>
```

### Chaos Engineering

```rust
pub fn ChaosMonkey::new(config: ChaosConfig) -> Self
pub fn ChaosMonkey::activate(&self)
pub fn ChaosMonkey::deactivate(&self)
pub fn ChaosMonkey::should_inject_failure(&self) -> bool
pub fn ChaosMonkey::record_failure(&self)
pub fn ChaosMonkey::failure_count(&self) -> u64
pub fn ChaosMonkey::inject_latency(&self)
```

### Regression Detection

```rust
pub fn RegressionDetector::new() -> Self
pub fn RegressionDetector::with_threshold(self, threshold: f64) -> Self
pub fn RegressionDetector::load_baseline(&mut self, baseline: RegressionBaseline)
pub fn RegressionDetector::detect(&self, current: &HashMap<String, f64>) -> Vec<RegressionAlert>
```

### Metrics Dashboard

```rust
pub fn MetricsCollector::new() -> Self
pub fn MetricsCollector::record_test_suite(&mut self, name: &str, metrics: TestMetrics)
pub fn MetricsCollector::record_performance(&mut self, name: &str, metrics: PerformanceMetrics)
pub fn MetricsCollector::generate_report(&self) -> String
pub fn MetricsCollector::overall_pass_rate(&self) -> f64
```

### Benchmark Fixtures

```rust
pub fn DatasetConfig::for_count(fact_count: usize) -> Self
pub fn DatasetConfig::validate(&self) -> Result<(), String>
pub fn deterministic_fact(index: usize, config: &DatasetConfig) -> Fact
pub fn ColumnFixture::new(size: usize) -> Self
pub fn BitmapFixture::new(size: usize, step: usize) -> Self
pub fn DictionaryFixture::new(size: usize) -> Self
pub fn SchemaFixture::new(config: &DatasetConfig) -> Self
pub fn DatabaseFixture::new(config: &DatasetConfig) -> Self
pub fn WalBenchmarkFixture::new(config: &DatasetConfig) -> Self
pub fn FileFormatFixture::new(config: &DatasetConfig) -> Self
```

## Configuration

### DatasetConfig Defaults

| Field | Default | Valid Range |
|-------|---------|-------------|
| fact_count | (required) | > 0 |
| subject_range | 100 | [1, u32::MAX] |
| predicate_range | 10 | [1, 256] |
| object_range | 200 | [1, u32::MAX] |
| base_confidence | 0.5 | [0.0, 1.0) |
| confidence_step | 0.0001 | >= 0.0 |

### Canonical Dataset Sizes

| Constant | Values |
|----------|--------|
| COLUMN_SIZES | 1K, 10K, 100K, 1M |
| BITMAP_SIZES | 10K, 100K, 1M |
| DICTIONARY_SIZES | 1K, 10K, 100K, 1M |
| DATABASE_SIZES | 100, 1K, 10K, 100K |
| WAL_SIZES | 1K, 10K, 100K, 1M |
| SCALE_SIZES | 10M, 100M, 1B |
| COMPRESSION_SIZES | 1K, 10K, 100K, 1M |
| TRANSACTION_SIZES | 100, 1K, 10K, 100K, 1M |

## Dependencies

| Dependency | Usage | Justification |
|------------|-------|---------------|
| kcm-core | Types, DenseVec, Bitmap, Dictionary | Core data structures under test |
| kcm-storage | Schema, WAL, FileFormat, Compressor | Storage layer under test |
| kcm-runtime | KnowledgeDatabase | Runtime layer under test |
| kcm-reasoning | RulePattern | Rule fixtures for reasoning benchmarks |
| kcm-security | ACLManager | RBAC security test validation |
| parking_lot | Mutex, RwLock | Faster synchronization than std |
| tempfile | TempDir | Auto-cleaned temporary files for WAL/file fixtures |
| getrandom | CSPRNG | Chaotic fault injection probability |

### Dev Dependencies

| Dependency | Usage |
|------------|-------|
| kcm-distributed | Integration tests for distributed scenarios |
| kcm-compliance | Integration tests for GDPR compliance |

## Error Handling

- Public functions return `Result<T, KcmError>`.
- Fixture constructors may panic with descriptive messages (validated config, deterministic setup).
- Thread panics during load/stress tests are caught via `join()` and converted to `KcmError::Io`.
- No `unwrap()` in non-test public code paths.
- No `panic!()` in non-test public code paths.

## Performance Characteristics

| Metric | Target |
|--------|--------|
| Unit test execution | < 100ms per test |
| Integration test execution | 1s-5s per test |
| Load test (light: 4 users, 50 ops) | < 5s |
| Load test (heavy: 100 users, 20 ops) | < 15s |
| Stress test (8 users, 2s duration) | < 10s |
| Security test suite | < 2s |
| Fixture generation (100K facts) | < 1s |
| Regression detection (100 metrics) | < 10ms |

## Security Considerations

- All test data is synthetic — no production data is used.
- Temporary files use `TempDir` for automatic cleanup.
- Chaos injection requires explicit activation via `ChaosMonkey::activate()`.
- `security_tests` module is gated with `#[cfg(test)]`.
- No secrets, keys, or credentials in test code.
- Concurrent tests use `Arc` + atomics for thread safety.
- Timing attack tests verify bounded ratios, not exact times.

## Integration

kcm-testing integrates with the KCM ecosystem as a test dependency:

```
kcm-core          ← used for types, DenseVec, Bitmap, Dictionary
kcm-storage       ← used for Schema, WAL, FileFormat, Compressor
kcm-runtime       ← used for KnowledgeDatabase
kcm-reasoning     ← used for RulePattern (benchmarks)
kcm-security      ← used for ACLManager (security tests)
kcm-distributed   ← used in integration tests (dev-dependency)
kcm-compliance    ← used in integration tests (dev-dependency)
```

## Sequence Diagram

### Load Test Execution

```
┌──────────────┐     ┌──────────────────┐     ┌────────────────────┐
│  Test Caller  │────▶│  run_load_test() │────▶│ KnowledgeDatabase  │
└──────────────┘     └──────────────────┘     └────────────────────┘
                           │
                    ┌──────┴──────┐
                    │  Spawn N    │
                    │  threads    │
                    └──────┬──────┘
                           │
                    ┌──────┴──────┐
                    │  Per thread: │
                    │  - insert    │
                    │  - query     │
                    │  - measure   │
                    └──────┬──────┘
                           │
                    ┌──────┴──────┐
                    │  Aggregate  │
                    │  results    │
                    └──────┬──────┘
                           │
                    ┌──────┴──────┐
                    │  Return     │
                    │  Results    │
                    └─────────────┘
```

### Regression Detection

```
┌──────────────────┐     ┌────────────────────┐     ┌─────────────────┐
│ load_baseline()  │────▶│ RegressionDetector  │◀────│ detect(current) │
└──────────────────┘     └────────────────────┘     └────────┬────────┘
                                                              │
                                                     ┌────────┴────────┐
                                                     │ Compare metrics  │
                                                     │ Classify severity│
                                                     └────────┬────────┘
                                                              │
                                                     ┌────────┴────────┐
                                                     │ Return alerts    │
                                                     └─────────────────┘
```

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                        kcm-testing                               │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────┐  │
│  │  load_tests   │  │ stress_tests │  │   security_tests     │  │
│  │  (public)     │  │  (public)    │  │   (#[cfg(test)])     │  │
│  └──────┬───────┘  └──────┬───────┘  └──────────┬───────────┘  │
│         │                 │                      │               │
│         └────────┬────────┘                      │               │
│                  │                               │               │
│         ┌────────┴────────┐                      │               │
│         │    chaos         │                      │               │
│         │    (public)      │                      │               │
│         └────────┬────────┘                      │               │
│                  │                               │               │
│         ┌────────┴──────────────────────────────┴───────────┐   │
│         │              bench_fixtures                         │   │
│         │  (DatasetConfig, ColumnFixture, BitmapFixture,     │   │
│         │   DictionaryFixture, SchemaFixture,                 │   │
│         │   DatabaseFixture, WalBenchmarkFixture,            │   │
│         │   FileFormatFixture, CompressionFixture,           │   │
│         │   RuleFixture)                                      │   │
│         └────────┬──────────────────────────────┬───────────┘   │
│                  │                               │               │
│         ┌────────┴────────┐             ┌────────┴────────┐    │
│         │  regression_     │             │  metrics_        │    │
│         │  detector        │             │  dashboard       │    │
│         └────────┬────────┘             └────────┬────────┘    │
│                  │                               │               │
└──────────────────┼───────────────────────────────┼───────────────┘
                   │                               │
         ┌─────────┴───────────────────────────────┴─────────┐
         │                    Dependencies                     │
         │  kcm-core    kcm-storage    kcm-runtime            │
         │  kcm-reasoning    kcm-security                      │
         │  parking_lot    tempfile    getrandom                │
         └────────────────────────────────────────────────────┘
```

## References

- [PRD-TESTING §1-8 — Testing Strategy](../PRD-TESTING%26%20BRACHMARCK.md)
- [AGENTS.md](../../AGENTS.md)
- [SSOT.md](../../SSOT.md)
- [PRD.md §3 — Core Types](../PRD.md)
- [PRD2.md §18 — Runtime](../PRD2.md)
- [PRD3.md §30 — Security](../PRD3.md)

## SSOT Alignment

This specification is aligned with the following SSOT documents:

| SSOT Requirement | Document | Section | Alignment |
|-----------------|----------|---------|-----------|
| Test pyramid (4-tier) | PRD-TESTING | §1 | kcm-testing provides unit, integration, and security test infrastructure |
| Quality gates | PRD-TESTING | §2 | Regression detection with 5% warn / 10% fail thresholds |
| Benchmark suite | PRD-TESTING | §4 | BenchFixtures provide deterministic dataset generators |
| Performance targets | PRD-TESTING | §4 | Load/stress test targets documented in Performance Characteristics |
| Security testing | PRD-TESTING | §6 | SecurityTests validate attack surfaces |
| Chaos engineering | PRD3.md | §27 | ChaosMonkey provides fault injection |
| RBAC validation | PRD3.md | §30 | Security tests validate ACLManager |
| Error model | AGENTS.md | Error Model | All public APIs return `Result<T, KcmError>` |
| Concurrency model | AGENTS.md | Concurrency Model | Arc + atomics + parking_lot for thread safety |
| No unwrap in production | AGENTS.md | Non-Negotiable Rules | #[cfg(test)] gate on security module |
