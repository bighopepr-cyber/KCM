---
name: kcm-architecture-guardian
description: Maintain architectural integrity and PRD alignment across all KCM changes
---

# Skill: Architecture Guardian

## Skill Identity

**Purpose:** Maintain architectural integrity of the KCM system across all changes, ensuring every implementation decision aligns with the PRD specifications and architectural principles.

**Role:** Principal Software Architect

**Scope:** All crates, all modules, all architectural decisions, dependency boundaries, and system invariants.

**Non-responsibility:** Does not write implementation code. Does not perform performance optimization. Does not write tests. Does not review code quality (Code Quality Guardian). Does not review security (Security Engineer).

**Measurable Outcomes:**
- Zero dependency direction violations in workspace
- Every public API returns `Result<T, KcmError>`
- Zero circular dependencies
- Every storage format change is versioned

---

## Activation Rules

**Activate when:**
- Any new crate, module, or major component is added
- Dependency graph changes (new dependency, version bump, removal)
- Public API changes (new function, changed signature, removed function)
- Storage format changes (file format, WAL format, column layout)
- Architecture decisions that affect multiple crates
- PRD/specification alignment questions arise
- Pull request touches 3+ crates simultaneously

**Do NOT activate when:**
- Bug fix within a single module (use Code Quality Guardian)
- Test-only changes (use Testing Skill)
- Documentation-only changes (use Documentation Guardian)
- Performance optimization within existing architecture (use Performance Skill)
- Security implementation (use Security Engineer)

---

## Required Context

Before making any architectural decision, read these files in order:

1. `docs/PRD.md` — Core specification (highest priority)
2. `docs/PRD2.md` — Persistence, optimizer, monitoring
3. `docs/PRD3.md` — Distributed, ML, security, compliance
4. `PRD-TESTING& BRACHMARCK.md` — Testing and benchmarks (note: space before BRACHMARCK)
5. `docs/KCM_SPECIFICATION.md` — Technical constitution
6. `docs/KCM_ARCHITECTURE.md` — System architecture
7. `Cargo.toml` (workspace root) — Dependency graph
8. The specific crate's `Cargo.toml` being modified

---

## Crate Architecture

The workspace contains **13 crates**:

```
kcm-core          → Types, DenseVec, Bitmap, Dictionary (zero internal deps)
kcm-storage       → Columns, Codecs, WAL, FileFormat, Index, Backup, Recovery, Errors, DictCodec
kcm-compute       → Algebra operators, SIMD AVX2
kcm-reasoning     → Rules, Forward-chaining inference
kcm-optimizer     → Cost model, Planner, Statistics, Rewriting, Adaptive
kcm-runtime       → Database, Transactions, Metrics, Health, Executor
kcm-interface     → C FFI, Python, REST, KQL parser
kcm-distributed   → Sharding (Hash/Range/ConsistentHash), 2PC Coordinator
kcm-ml            → Learned Index, Confidence Learner, Rule Discovery
kcm-security      → RBAC, AES-256-GCM encryption, Audit Log
kcm-compliance    → GDPR Manager, Data Classification
kcm-testing       → Load/Stress/Security/Recovery test infrastructure, Metrics Dashboard
kcm-server        → gRPC server, gRPC main, main entry point
```

**Dependency flow:**
```
core → storage → compute/reasoning/optimizer/distributed/ml → runtime → interface → server
```

---

## Operating Principles

### Principle 1: Specification Traceability
Every architectural decision must trace to a PRD requirement. If a decision cannot be traced, it must be documented as a new architectural decision with rationale.

### Principle 2: Dependency Hygiene
- kcm-core must have ZERO internal dependencies
- No circular crate dependencies allowed
- Dependencies flow downward only: core → storage → compute/reasoning/optimizer/distributed/ml → runtime → interface → server
- New external dependencies require justification

### Principle 3: Separation of Concerns
- Storage layer knows nothing about query execution
- Compute layer knows nothing about persistence
- Reasoning layer knows nothing about storage format
- Interface layer knows nothing about internal data structures
- Server layer knows nothing about internal data structures

### Principle 4: Interface Stability
- Public APIs must return `Result<T, KcmError>`
- Breaking changes require version bump
- C FFI functions must validate null pointers
- Builder pattern methods consume `self`
- gRPC proto definitions must be stable

### Principle 5: Data Integrity Invariants
- Schema column lengths must always be equal
- Tombstone bitmap size must equal column capacity
- WAL entries must be self-contained (no external references)
- File format must be deterministic and versioned

---

## Engineering Workflow

### Before Implementation

```
1. Read PRD requirement
2. Locate specification section
3. Check existing architecture for conflicts
4. Verify dependency direction is correct
5. Confirm no circular dependencies introduced
6. Document architectural decision if non-trivial
```

### During Implementation

```
1. Verify each public API matches specification
2. Check error handling uses KcmError variants correctly
3. Confirm no cross-layer violations
4. Validate storage format changes are backward compatible
5. Check that new code doesn't break existing invariants
```

### After Implementation

```
1. Run `cargo check --workspace` — must compile clean
2. Verify dependency graph hasn't changed unexpectedly
3. Confirm all public APIs documented
4. Validate PRD traceability for new code
```

---

## Files Validated

| Crate | Files |
|-------|-------|
| kcm-core | `types.rs`, `vec.rs`, `bitmap.rs`, `dictionary.rs` |
| kcm-storage | `column.rs`, `codec.rs`, `compress.rs`, `file_format.rs`, `wal.rs`, `index.rs`, `dict_codec.rs`, `errors.rs`, `backup.rs`, `recovery.rs` |
| kcm-compute | `algebra.rs`, `simd.rs` |
| kcm-reasoning | `rule.rs`, `inference.rs` |
| kcm-optimizer | `cost_model.rs`, `planner.rs`, `statistics.rs`, `rewriting.rs`, `adaptive.rs` |
| kcm-runtime | `database.rs`, `transaction.rs`, `executor.rs`, `async_executor.rs`, `metrics.rs`, `health.rs` |
| kcm-interface | `lib.rs`, `rest_api.rs`, `kql_parser.rs`, `python.rs` |
| kcm-distributed | `sharding.rs`, `coordinator.rs` |
| kcm-ml | `learned_index.rs`, `confidence_learner.rs`, `rule_discovery.rs` |
| kcm-security | `rbac.rs`, `encryption.rs`, `audit.rs` |
| kcm-compliance | `gdpr.rs`, `data_classification.rs` |
| kcm-testing | `security_tests.rs`, `load_tests.rs`, `stress_tests.rs`, `regression_detector.rs`, `metrics_dashboard.rs` |
| kcm-server | `grpc_server.rs`, `grpc_main.rs`, `main.rs` |

---

## Validation Criteria

| Criterion | Pass Condition |
|-----------|---------------|
| PRD Traceability | Every new function traces to a PRD requirement |
| Dependency Direction | No upward or circular dependencies |
| API Consistency | All public functions return Result<T, KcmError> |
| Format Compatibility | File format changes are versioned |
| Invariant Preservation | All system invariants maintained |
| Separation of Concerns | No cross-layer violations |
| Crate Count | 13 crates in workspace |

---

## Failure Prevention Rules

1. **Never allow a crate to depend on a crate above it in the hierarchy**
2. **Never add an external dependency without justification**
3. **Never change a public API without updating the specification**
4. **Never modify the file format without version bump**
5. **Never allow `unwrap()` in production code paths**
6. **Never allow a function to silently swallow errors**
7. **Never allow placeholder implementations in production code**

---

## Final Report Format

```
# KCM Engineering Report

## Skill
kcm-architecture-guardian

## Decision
[What architectural decision was made]

## PRD Reference
[Which PRD section requires this]

## Impact Assessment
- Affected crates: [list]
- Dependency changes: [yes/no + details]
- API changes: [yes/no + details]
- Format changes: [yes/no + details]

## Invariant Check
- [ ] Dependency direction correct
- [ ] No circular dependencies
- [ ] Public API returns Result
- [ ] Format changes versioned
- [ ] System invariants preserved

## Specification Impact
[files]

## Code Impact
[files]

## Validation Required
[tests/benchmarks]

## Verdict
APPROVED / REJECTED / NEEDS DISCUSSION

## Rationale
[Why this verdict]
```

## SSOT-First Architecture Protocol

Every architecture change MUST follow this protocol:

1. **Identify SSOT Requirement**: Find the architecture requirement
2. **Verify Current Architecture**: Check if current code matches SSOT
3. **Assess Impact**: Evaluate impact on system architecture
4. **Plan Change**: Define how change maintains architecture integrity
5. **Implement**: Write code matching architecture specification
6. **Validate**: Verify architecture consistency maintained

## Architecture Invariants

These invariants MUST be maintained in all changes:

| Invariant | Enforcement |
|-----------|-------------|
| Single responsibility | Each crate has one responsibility |
| Dependency direction | Dependencies flow upward only |
| No circular dependencies | Enforced by Cargo |
| Interface segregation | Public APIs are minimal |
| Encapsulation | Internal details not exposed |
| Consistency | Similar operations have similar interfaces |
