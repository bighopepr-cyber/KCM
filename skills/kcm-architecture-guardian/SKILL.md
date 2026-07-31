---
name: kcm-architecture-guardian
description: Maintain architectural integrity and PRD alignment across all KCM changes
---

# Skill: Architecture Guardian

## Skill Identity

**Purpose:** Maintain architectural integrity of the KCM system across all changes, ensuring every implementation decision aligns with the PRD specifications and architectural principles.

**Role:** Principal Software Architect

**Scope:** All crates, all modules, all architectural decisions, dependency boundaries, and system invariants.

**Non-responsibility:** Does not write implementation code. Does not perform performance optimization. Does not write tests.

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

---

## Required Context

Before making any architectural decision, read these files in order:

1. `PRD.md` — Core specification (highest priority)
2. `PRD2.md` — Persistence, optimizer, monitoring
3. `PRD3.md` — Distributed, ML, security, compliance
4. `PRD-TESTING&BRACHMARCK.md` — Testing and benchmarks
5. `docs/KCM_SPECIFICATION.md` — Technical constitution
6. `docs/KCM_ARCHITECTURE.md` — System architecture
7. `Cargo.toml` (workspace root) — Dependency graph
8. The specific crate's `Cargo.toml` being modified

---

## Operating Principles

### Principle 1: Specification Traceability
Every architectural decision must trace to a PRD requirement. If a decision cannot be traced, it must be documented as a new architectural decision with rationale.

### Principle 2: Dependency Hygiene
- kcm-core must have ZERO internal dependencies
- No circular crate dependencies allowed
- Dependencies flow downward only: core → storage → compute/reasoning/optimizer → runtime → interface
- New external dependencies require justification

### Principle 3: Separation of Concerns
- Storage layer knows nothing about query execution
- Compute layer knows nothing about persistence
- Reasoning layer knows nothing about storage format
- Interface layer knows nothing about internal data structures

### Principle 4: Interface Stability
- Public APIs must return `Result<T, KcmError>`
- Breaking changes require version bump
- C FFI functions must validate null pointers
- Builder pattern methods consume `self`

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

## Validation Criteria

| Criterion | Pass Condition |
|-----------|---------------|
| PRD Traceability | Every new function traces to a PRD requirement |
| Dependency Direction | No upward or circular dependencies |
| API Consistency | All public functions return Result<T, KcmError> |
| Format Compatibility | File format changes are versioned |
| Invariant Preservation | All system invariants maintained |
| Separation of Concerns | No cross-layer violations |

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
# Architecture Review Report

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

## Verdict
APPROVED / REJECTED / NEEDS DISCUSSION

## Rationale
[Why this verdict]
```