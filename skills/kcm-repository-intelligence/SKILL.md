---
name: kcm-repository-intelligence
description: Help AI agents understand the complete repository structure before making changes
---

# Skill: Repository Intelligence

## Skill Identity

**Purpose:** Before any code change, the agent must understand where the change belongs in the repository. This skill provides structured analysis of the KCM codebase to prevent misplaced code, duplicated implementations, and incorrect dependency usage.

**Role:** Codebase Intelligence Analyst

**Scope:** Repository structure analysis, dependency graph understanding, module ownership identification, existing implementation discovery, test location mapping.

**Non-responsibility:** Does not implement code. Does not write tests. Does not review architecture (Architecture Guardian). Does not review code quality (Code Quality Guardian).

**Measurable Outcomes:**
- Every change targets the correct crate
- No duplicated implementations exist
- Dependencies flow in correct direction
- Tests are in the correct location
- All 13 crates and their files are accurately mapped

---

## Activation Rules

**Activate when:**
- Agent is unfamiliar with the codebase
- Change affects multiple crates
- Agent needs to locate where a feature should be implemented
- Agent needs to understand dependency relationships
- Agent needs to find existing implementations before writing new code

**Do NOT activate when:**
- Agent already knows the specific file and line to change
- Change is a simple bug fix in a known location
- Change is test-only in a known test file

---

## Required Inspection

When activated, analyze these in order:

### 1. Workspace Structure
Read `Cargo.toml` at workspace root. The workspace contains **13 crates**.

### 2. Crate Map
For each crate, identify:
- Purpose (from `lib.rs` module declarations)
- Public API surface (from `pub` exports)
- Test locations (`tests/` directory or `#[cfg(test)]` modules)

### 3. Dependency Graph
```
kcm-core (zero internal deps)
  ↑
kcm-storage
  ↑
kcm-compute, kcm-reasoning, kcm-optimizer, kcm-distributed, kcm-ml
  ↑
kcm-runtime
  ↑
kcm-interface, kcm-testing, kcm-server
```

### 4. Module Ownership

| Module | Crate | Responsibility |
|--------|-------|---------------|
| types.rs | kcm-core | RowID, SubjectID, Fact, KcmError |
| vec.rs | kcm-core | DenseVec (SIMD-aligned) |
| bitmap.rs | kcm-core | Bitmap (64-bit word ops) |
| dictionary.rs | kcm-core | String→u32 mapping |
| column.rs | kcm-storage | Column<T>, Schema |
| codec.rs | kcm-storage | Delta, RLE, Gorilla codecs |
| compress.rs | kcm-storage | Zstd, LZ4, RLE compressors |
| file_format.rs | kcm-storage | Binary DB format |
| wal.rs | kcm-storage | Write-Ahead Log |
| index.rs | kcm-storage | BitmapIndex, ZoneMap, BloomFilter, CompositeIndex |
| dict_codec.rs | kcm-storage | Dictionary encoding |
| errors.rs | kcm-storage | Storage-specific error types |
| backup.rs | kcm-storage | Backup and restore |
| recovery.rs | kcm-storage | Crash recovery |
| algebra.rs | kcm-compute | Scan, Filter, Project, Join, Aggregate operators |
| simd.rs | kcm-compute | AVX2 SIMD operations |
| rule.rs | kcm-reasoning | Rule, RulePattern, RuleRegistry |
| inference.rs | kcm-reasoning | Forward-chaining inference |
| cost_model.rs | kcm-optimizer | Cost estimation |
| planner.rs | kcm-optimizer | Query planner |
| statistics.rs | kcm-optimizer | Column statistics |
| rewriting.rs | kcm-optimizer | Optimizer rules |
| adaptive.rs | kcm-optimizer | Adaptive execution |
| database.rs | kcm-runtime | KnowledgeDatabase |
| transaction.rs | kcm-runtime | Transaction, VersionStore |
| executor.rs | kcm-runtime | Rayon thread pool |
| async_executor.rs | kcm-runtime | Tokio async bridge |
| metrics.rs | kcm-runtime | AtomicU64 metrics |
| health.rs | kcm-runtime | Health checks |
| logging.rs | kcm-runtime | Structured logging |
| lib.rs | kcm-interface | C FFI (18 functions) |
| rest_api.rs | kcm-interface | REST handlers |
| kql_parser.rs | kcm-interface | KQL lexer/parser |
| python.rs | kcm-interface | PyO3 bindings |
| sharding.rs | kcm-distributed | Hash/Range/ConsistentHash |
| coordinator.rs | kcm-distributed | 2PC coordinator |
| learned_index.rs | kcm-ml | Regression-based index |
| confidence_learner.rs | kcm-ml | Accuracy tracking (NOT in kcm-reasoning) |
| rule_discovery.rs | kcm-ml | Pattern mining |
| rbac.rs | kcm-security | Role-based access control |
| encryption.rs | kcm-security | AES-256-GCM |
| audit.rs | kcm-security | Audit logging |
| gdpr.rs | kcm-compliance | GDPR data subject management |
| data_classification.rs | kcm-compliance | 4-tier classification |
| security_tests.rs | kcm-testing | Security test infrastructure |
| load_tests.rs | kcm-testing | Load test infrastructure |
| stress_tests.rs | kcm-testing | Stress test infrastructure |
| regression_detector.rs | kcm-testing | Performance regression detection |
| metrics_dashboard.rs | kcm-testing | Metrics dashboard |
| grpc_server.rs | kcm-server | gRPC server implementation |
| grpc_main.rs | kcm-server | gRPC main entry point |
| main.rs | kcm-server | Main entry point |

---

## Operating Rules

1. **Search before creating** — Before writing new code, search for existing implementations
2. **Respect crate boundaries** — Don't put storage logic in compute, don't put API logic in core
3. **Follow dependency direction** — core → storage → compute/reasoning/optimizer/distributed/ml → runtime → interface → server
4. **Check public API** — Use existing public APIs instead of reaching into internals
5. **Find test location** — Tests go in the crate that owns the code being tested
6. **confidence_learner.rs is in kcm-ml** — NOT in kcm-reasoning

---

## Validation Checklist

- [ ] Change goes in the correct crate
- [ ] No duplicated implementation exists
- [ ] Dependencies flow in correct direction
- [ ] Public API used correctly
- [ ] Tests in correct location
- [ ] All 13 crates recognized

---

## Final Report Format

```
# KCM Engineering Report

## Skill
kcm-repository-intelligence

## Repository Intelligence Report

Change Request: [description]
Target Crate: [crate name]
Target Module: [module name]
Dependencies Used: [list]
Existing Implementations Found: [list or none]
Test Location: [file path]

Validation:
- [ ] Correct crate
- [ ] No duplication
- [ ] Correct dependencies
- [ ] Tests located

## Specification Impact
[files]

## Code Impact
[files]
```
