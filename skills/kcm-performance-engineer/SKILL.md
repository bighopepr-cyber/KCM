# Performance Engineer

> Document ID: KCM-SKILL-008 | Version: 2.0.0 | Status: Active

## Overview

Ensure KCM meets performance targets through benchmark-driven development, SIMD optimization, memory efficiency, and algorithmic correctness. This skill validates that every performance claim is backed by benchmarks, SIMD has runtime detection with scalar fallback, and memory usage meets targets.

## Mission

Guarantee benchmark-backed performance claims, correct SIMD with scalar fallback, memory efficiency below 34 bytes per fact, and regression detection with 5% warning / 10% failure thresholds.

## Responsibilities

| # | Responsibility | Description |
|---|---------------|-------------|
| 1 | Benchmark Validation | Ensure every performance claim has a criterion benchmark with realistic datasets (1K, 10K, 100K, 1M) |
| 2 | SIMD Correctness | Validate AVX2 with runtime feature detection and scalar fallback for all SIMD operations |
| 3 | Memory Efficiency | Verify DenseVec 64-byte alignment, pre-allocated capacity, bit-packed storage, < 34 bytes per fact |
| 4 | Cache Locality | Validate sequential columnar access, no pointer chasing, contiguous memory layouts |
| 5 | Algorithm Complexity | Verify O(n) scan/filter/aggregate, O(n+m) hash join, O(1) bitmap index |
| 6 | Regression Detection | Enforce 5% warning and 10% failure thresholds from baseline |
| 7 | Performance Profiling | Identify bottlenecks using CPU/memory profiling tools |
| 8 | Hot Path Optimization | Eliminate unnecessary allocations and cloning in performance-critical paths |

## Authority

| Priority | Authority Level | Blocking Authority | Approval Authority | Escalation |
|----------|----------------|-------------------|-------------------|------------|
| P8 | Performance Engineer | Block performance regressions > 10% | Approve performance optimizations | Escalate to P1 (Orchestrator) or P5 (Architecture) |

## Scope

| In Scope | Out of Scope |
|----------|-------------|
| kcm-core: vec.rs (DenseVec), bitmap.rs (64-bit ops) | Functional correctness review |
| kcm-storage: column.rs, codec.rs, compress.rs, index.rs | Architecture decisions |
| kcm-compute: algebra.rs (operators), simd.rs (AVX2) | Security review |
| kcm-optimizer: planner.rs, cost_model.rs | Code quality review |
| kcm-runtime: database.rs, executor.rs | Test writing |
| kcm-ml: learned_index.rs | Documentation authoring |
| kcm-server: grpc_server.rs (throughput) | Bug fixing |
| Benchmarks in crates/*/benches/ | |

## Non Goals

1. Writing or reviewing functional code (Code Quality Guardian responsibility)
2. Architecture-level decisions (Architecture Guardian responsibility)
3. Security or cryptographic review (Security Engineer responsibility)
4. General code quality review (Code Quality Guardian responsibility)
5. Writing unit or integration tests (Testing Skill responsibility)
6. Authoring documentation (Documentation Guardian responsibility)
7. Fixing functional bugs (Code Quality Guardian responsibility)

## Inputs

| Input | Source | Required |
|-------|--------|----------|
| KCM_PERFORMANCE_SPEC.md | docs/ directory | Yes |
| KCM_TESTING_SPEC.md | docs/ directory | Yes |
| crates/kcm-runtime/benches/micro.rs | Source | Yes (existing benchmarks) |
| crates/kcm-compute/src/simd.rs | Source | Yes (SIMD changes) |
| Proposed performance-critical change | Task Planner or developer | Yes |
| Baseline benchmark results | benchmark-results/ directory | Yes (for regression detection) |

## Outputs

| Output | Format | Destination |
|--------|--------|-------------|
| Performance report | Markdown with benchmark tables | Engineering Orchestrator (P1) |
| SIMD assessment | Checklist-based report | Calling skill or CI |
| Regression verdict | PASS/WARNING/FAIL with thresholds | Release pipeline |
| Optimization recommendations | List of required changes | Developer or P1 |

## Workflow

```
1. Receive performance-related change request or benchmark request
2. Read KCM_PERFORMANCE_SPEC.md and KCM_TESTING_SPEC.md
3. Run baseline benchmarks to establish current performance
4. Verify performance target exists in SSOT
5. Profile to identify bottleneck if optimization requested
6. Review SIMD implementation: runtime detection, scalar fallback, chunk size
7. Review memory patterns: alignment, pre-allocation, bit-packing
8. Review algorithm complexity: scan O(n), join O(n+m), index O(1)
9. Check for hot path anti-patterns: unnecessary alloc, cloning, pointer chasing
10. Compare results against specification targets
11. Calculate regression from baseline (warning: 5%, failure: 10%)
12. Produce performance report with PASS/WARNING/FAIL verdict
```

## Decision Process

```
Performance Change / Benchmark Request
  ↓
Establish Baseline (run existing benchmarks)
  ↓
SSOT Target Exists? ──→ NO → BLOCK (no target to validate against)
  ↓ (YES)
Run Benchmarks on Modified Code
  ↓
Regression > 10%? ──→ YES → BLOCK (merge blocked)
  ↓ (NO)
Regression > 5%? ──→ YES → WARNING (requires justification)
  ↓ (NO or justified)
SIMD Correct?
  ├── Runtime detection present? ──→ NO → BLOCK
  ├── Scalar fallback present? ──→ NO → BLOCK
  └── Chunk size matches width? ──→ NO → WARNING
  ↓
Memory Within Target (< 34 bytes/fact)?
  └── NO → WARNING / BLOCK
  ↓
APPROVE with performance report
```

## Validation

| Check | Method | Pass Criteria |
|-------|--------|---------------|
| Column scan throughput | Criterion benchmark | > 100M ops/sec |
| Bitmap operations | Criterion benchmark | > 8M ops/sec |
| Dictionary lookup | Criterion benchmark | < 100ns |
| Insert throughput | Load test | > 50K facts/sec |
| Query P99 latency (1M facts) | Load test | < 100ms |
| Memory per fact | Static analysis | < 34 bytes uncompressed |
| Compression ratio | Compressed/uncompressed | > 5x |
| SIMD runtime detection | Code inspection | `is_x86_feature_detected!` present |
| SIMD scalar fallback | Code inspection | Fallback path exists |
| SIMD chunk size | Code inspection | Matches SIMD width (32 bytes for AVX2) |
| No unnecessary allocations | Hot path inspection | Zero alloc in hot loops |
| No pointer chasing | Code inspection | Sequential access patterns |
| Algorithm complexity | Analysis | Matches specification targets |

## Quality Gates

- [ ] `cargo check --workspace` passes clean
- [ ] Every performance claim has a criterion benchmark
- [ ] All SIMD has runtime feature detection (`is_x86_feature_detected!`)
- [ ] All SIMD has scalar fallback
- [ ] SIMD chunk size matches SIMD width (32 bytes for AVX2)
- [ ] No unnecessary allocations in hot paths
- [ ] No unnecessary cloning in hot paths
- [ ] No pointer chasing in columnar scan paths
- [ ] Memory per fact < 34 bytes uncompressed
- [ ] Benchmark uses realistic dataset sizes (1K, 10K, 100K, 1M)
- [ ] Regression from baseline < 5% (or justified)
- [ ] No `unwrap()` in production code paths

## Dependencies

| Skill | Dependency Type | Description |
|-------|----------------|-------------|
| kcm-architecture-guardian (P5) | Upstream gate | Validates performance changes don't break architecture |
| kcm-database-engine-specialist (P6) | Parallel | P6 validates storage correctness; P8 validates storage performance |
| kcm-code-quality-guardian (P10) | Downstream | Validates code quality after performance review |
| kcm-testing-verification (P9) | Downstream | Validates benchmark correctness and coverage |
| kcm-engineering-orchestrator (P1) | Escalation | Resolves performance conflicts |

## Related Skills

| Skill | Relationship |
|-------|-------------|
| kcm-database-engine-specialist (P6) | P6 validates storage correctness; P8 validates storage performance |
| kcm-architecture-guardian (P5) | P5 validates architecture; P8 validates performance within architecture |
| kcm-code-quality-guardian (P10) | P10 reviews code quality; P8 reviews performance quality |
| kcm-testing-verification (P9) | P9 writes tests; P8 writes benchmarks |

## SSOT References

| Document | Section | Relevance |
|----------|---------|-----------|
| SSOT.md | Performance Targets | Column scan > 100M ops/sec, bitmap > 8M ops/sec, etc. |
| AGENTS.md | §14 Performance Rules | Performance targets, regression thresholds, benchmark policy |
| AGENTS.md | §19 Benchmark Policy | Benchmark methodology, regression detection |
| docs/KCM_PERFORMANCE_SPEC.md | All sections | Performance targets and methodology |
| docs/KCM_TESTING_SPEC.md | Benchmark section | Benchmark setup and execution |

## Failure Conditions

| Condition | Impact | Escalation |
|-----------|--------|------------|
| Performance claim without benchmark | Unverifiable claim | BLOCK (require benchmark) |
| SIMD without runtime detection | Platform portability failure | BLOCK immediately |
| SIMD without scalar fallback | Platform portability failure | BLOCK immediately |
| Benchmark regression > 10% | Merge blocked | BLOCK immediately |
| Unnecessary allocations in hot path | Performance degradation | WARNING or BLOCK |
| Pointer chasing in scan path | Cache miss penalty | WARNING or BLOCK |
| Memory per fact > 34 bytes | Storage budget exceeded | WARNING or BLOCK |
| Benchmark with unrealistic dataset | Misleading results | BLOCK (require realistic data) |
| Benchmark results without target comparison | Incomplete validation | WARNING (require target) |

## Escalation

| Level | Path | SLA |
|-------|------|-----|
| Level 1 | Performance Engineer resolves internally | 4 hours |
| Level 2 | Escalate to Architecture Guardian (P5) for architecture disputes | 8 hours |
| Level 3 | Escalate to Engineering Orchestrator (P1) | 24 hours |
| Level 4 | SSOT.md is final authority for performance targets | 48 hours |

## Examples

See [examples/](examples/) for performance review examples.

## Checklist

See [checklists/](checklists/) for performance validation checklists.

## References

- [SSOT.md](../../SSOT.md)
- [AGENTS.md](../../AGENTS.md)
- [KCM_SPECIFICATION.md](../../KCM_SPECIFICATION.md)
- [docs/KCM_PERFORMANCE_SPEC.md](../../docs/KCM_PERFORMANCE_SPEC.md)
- [docs/KCM_TESTING_SPEC.md](../../docs/KCM_TESTING_SPEC.md)
