# Feature Implementation Example

> Document ID: KCM-EX-FEAT-001 | Version: 1.0.0

## Task

Implement a new `kcm_snapshot()` function in `kcm-interface` that creates a point-in-time snapshot of the database.

## Execution Plan

**Task ID:** KCM-2026-001
**Pipeline:** feature.md
**Risk Level:** Medium

### Affected Components

| Component | Files | Change Type | Risk |
|-----------|-------|-------------|------|
| kcm-interface | `src/lib.rs` | New FFI function | Medium |
| kcm-storage | `src/file_format.rs` | Snapshot support | Low |
| kcm-runtime | `src/database.rs` | Snapshot orchestration | Low |

### Required Skills

| Skill | Phase | Responsibility |
|-------|-------|---------------|
| P16 | Intelligence | Map codebase |
| P2 | Planning | Create plan |
| P3 | Impact | Assess impact |
| P4 | Spec | Validate FFI contract |
| P5 | Arch | Validate architecture |
| P6 | Domain | Implement snapshot |
| P10 | Quality | Code quality |
| P9 | Testing | Test validation |
| P8 | Benchmark | Performance baseline |
| P11 | Docs | Update docs |
| P13 | Review | Code review |
| P12 | Release | Release gate |
| P1 | Orchestrator | Final approval |

## Execution Steps

### Step 1: Intelligence (P16)

```
- Read kcm-interface/src/lib.rs
- Read kcm-storage/src/file_format.rs
- Read kcm-runtime/src/database.rs
- Map existing snapshot patterns
- Identify test locations
```

### Step 2: Planning (P2)

```
- Create execution plan
- Identify all affected files
- Map dependencies
- Estimate effort: 4 hours
```

### Step 3: Impact Analysis (P3)

```
- Direct: kcm-interface, kcm-storage, kcm-runtime
- Indirect: kcm-core (types), kcm-testing (fixtures)
- Spec: KCM_API_SPEC.md (FFI), KCM_RUNTIME_SPEC.md
- Tests: integration tests, FFI tests
- SDK: All SDKs (FFI change)
```

### Step 4: Spec Validation (P4)

```
- Validate FFI signature follows conventions
- Validate error handling pattern
- Validate memory management
- Approve spec change
```

### Step 5: Implementation

```rust
// In kcm-interface/src/lib.rs
#[no_mangle]
pub extern "C" fn kcm_snapshot(
    db: *mut KCM_Database,
    snapshot_path: *const c_char,
) -> KCM_Error {
    // SAFETY: db is validated non-null, snapshot_path is validated non-null
    // ... implementation
}
```

### Step 6: Quality (P10)

```
- cargo fmt --check ✓
- cargo clippy -- -D warnings ✓
- No unwrap/panic/TODO ✓
- Error handling complete ✓
```

### Step 7: Testing (P9)

```
- Unit test: kcm_snapshot_basic
- Unit test: kcm_snapshot_null_db
- Unit test: kcm_snapshot_null_path
- Integration test: snapshot_and_restore
- FFI test: cross-language snapshot
```

### Step 8: Documentation (P11)

```
- Update KCM_API_SPEC.md (new FFI function)
- Update docs/kcm-interface/spesifikasi.md
- Update docs/kcm-runtime/spesifikasi.md
- Update CHANGELOG.md
```

## Completion Report

**Status:** COMPLETED
**Duration:** 4 hours
**Files Changed:** 5
**Tests Added:** 5
**Quality Gates:** 10/10 PASS

## Lessons Learned

- Snapshot requires careful file handle management
- FFI boundary needs explicit path validation
- Integration test coverage is critical for FFI functions
