# Specification Lock

> Document ID: KCM-SKILL-004 | Version: 2.0.0 | Status: Active

## Overview

The Specification Lock skill protects frozen technical contracts and binary formats from accidental modification. It acts as a change control gate for any code that touches protected specifications including binary file format, WAL entry format, C FFI signatures, gRPC proto definitions, error code enums, public API return types, and `#[repr(C)]` struct layouts. It has veto authority over contract changes.

## Mission

Ensure zero frozen contract violations by enforcing version bump requirements, backward compatibility, roundtrip test validation, and specification document updates for every protected change.

## Responsibilities

| # | Responsibility | Description |
|---|---------------|-------------|
| 1 | Contract Protection | Veto any change that violates frozen contracts |
| 2 | Version Enforcement | Ensure format changes include `DB_VERSION` increment |
| 3 | Backward Compatibility | Verify new format reads old format or migration tool exists |
| 4 | Roundtrip Validation | Ensure codec/compression changes pass encode→decode identity tests |
| 5 | FFI Synchronization | Verify C header matches Rust declarations |
| 6 | Proto Compatibility | Verify gRPC proto changes maintain backward compatibility |
| 7 | Error Code Preservation | Ensure error variants are never removed, only added |
| 8 | Spec Document Updates | Ensure specification documents match code changes |
| 9 | SSOT Alignment | Verify all changes align with SSOT specifications |

## Authority

| Attribute | Value |
|-----------|-------|
| Priority | P4 |
| Authority Level | Veto |
| Blocking Authority | Can VETO any format/API/FFI/proto change that violates frozen contracts |
| Approval Authority | Can approve or reject contract changes; requires version bump and spec update |
| Escalation | Engineering Orchestrator (P1) |

## Scope

| In Scope | Out of Scope |
|----------|-------------|
| Binary file format (magic bytes, header layout, column block format) | Internal algorithm design |
| WAL entry format (byte layout, field order, entry sizes) | Storage algorithm implementation |
| Codec encode/decode logic (DeltaCodec, RleCodec, GorillaCodec, DictCodec) | Query execution logic |
| Compression algorithm selection | Transaction logic |
| C FFI function signatures | Code quality review |
| gRPC proto definitions | Architecture decisions |
| Error code enum variants | Performance optimization |
| Public API return types | Test strategy design |
| `#[repr(C)]` struct layouts | |

## Non Goals

1. Implementing changes — domain specialists handle this
2. Reviewing architecture — Architecture Guardian (P5) handles this
3. Reviewing code quality — Code Quality Guardian (P10) handles this
4. Writing tests — Testing Verification (P9) handles this
5. Designing internal algorithms — Database Engine Specialist (P6) handles this

## Inputs

| Input | Source | Required |
|-------|--------|----------|
| Proposed protected change | Implementation skill | Yes |
| Binary format specification | `docs/KCM_COLUMNAR_FORMAT_SPEC.md` | Yes |
| Codec specification | `docs/KCM_COMPRESSION_SPEC.md` | Yes |
| API specification | `docs/KCM_API_SPEC.md` | Yes |
| Versioning specification | `docs/KCM_VERSIONING_SPEC.md` | Yes |
| File being modified | Codebase | Yes |
| Test files for modified component | Codebase | Yes |

## Outputs

| Output | Format | Destination |
|--------|--------|-------------|
| Lock Assessment Report | Structured markdown | Orchestrator / Implementing skill |
| Approval/Block decision | APPROVED/BLOCKED/NEEDS MIGRATION | Orchestrator (P1) |
| Version bump requirement | Version increment | Implementing skill |
| Spec update requirement | Document update list | Documentation Guardian (P11) |
| Migration requirement | Migration plan | Database Engine Specialist (P6) |

## Workflow

```
1. Detect change to protected component
2. Read relevant specification documents
3. Inspect the specific file being modified
4. Read all test files that exercise the modified component
5. Assess backward compatibility
6. Determine version bump requirement
7. Verify migration path if needed
8. Produce Lock Assessment Report
9. APPROVE, BLOCK, or REQUIRE MIGRATION PLAN
```

## Decision Process

```
Protected Change Detected → Spec Review → File Inspection → Backward Compatibility Check → Version Impact Assessment → Approval/Block
```

## Validation

| Check | Method | Pass Criteria |
|-------|--------|--------------|
| Binary format version bumped | `DB_VERSION` check | Version incremented if layout changed |
| WAL entry size matches spec | Size verification | 34 bytes Insert, 9 bytes Delete |
| Codec roundtrip tests pass | Test execution | encode→decode identity verified |
| Compression roundtrip tests pass | Test execution | encode→decode identity verified |
| C FFI signatures match spec | Header comparison | Rust declarations match C header |
| gRPC proto backward compatible | Proto review | No breaking changes or migration plan provided |
| Error codes unchanged or additive | Enum review | No removals, only additions |
| All existing tests still pass | Test execution | 100% pass rate |
| Specification document updated | Doc review | Code and spec are consistent |

## Quality Gates

- [ ] Binary format version bumped if layout changed
- [ ] WAL entry size matches specification (34 bytes Insert, 9 bytes Delete)
- [ ] Codec roundtrip tests pass
- [ ] Compression roundtrip tests pass
- [ ] C FFI signatures match specification
- [ ] gRPC proto changes backward compatible
- [ ] Error codes unchanged or documented as additive
- [ ] All existing tests still pass
- [ ] Specification document updated to match code

## Dependencies

| Skill | Dependency Type | Description |
|-------|----------------|-------------|
| kcm-database-engine-specialist (P6) | Peer | P6 implements changes; P4 validates contracts |
| kcm-architecture-guardian (P5) | Peer | P5 validates architecture; P4 validates contracts (P4 higher priority) |
| kcm-documentation-guardian (P11) | Downstream | P11 updates spec documents based on P4 requirements |
| kcm-engineering-orchestrator (P1) | Upstream | P1 coordinates P4 through engineering gates |
| kcm-testing-verification (P9) | Downstream | P9 runs roundtrip tests for codec changes |

## Related Skills

| Skill | Relationship |
|-------|-------------|
| kcm-database-engine-specialist (P6) | P6 implements storage changes; P4 gates contract compliance |
| kcm-architecture-guardian (P5) | P4 has higher priority on format changes; P5 validates architecture |
| kcm-engineering-orchestrator (P1) | P1 coordinates P4 in Gate 2 |
| kcm-documentation-guardian (P11) | P11 executes spec updates required by P4 |
| kcm-code-quality-guardian (P10) | P10 validates code quality after P4 approves |

## SSOT References

| Document | Section | Relevance |
|----------|---------|-----------|
| SSOT.md | — | Single Source of Truth |
| AGENTS.md | Section 5.3 | Immutable Contracts |
| AGENTS.md | Section 18 | API Stability Rules |
| AGENTS.md | Section 17 | Versioning Rules |
| AGENTS.md | Section 25 | Skill Governance |

## Failure Conditions

| Condition | Impact | Escalation |
|-----------|--------|------------|
| Frozen contract violated | Data format corruption | P4 VETO — blocks merge |
| Version not bumped | Incompatible format versions | P4 BLOCKS — requires version increment |
| Backward compatibility broken | Existing data unreadable | P4 BLOCKS — requires migration plan |
| Codec roundtrip fails | Data corruption | P4 BLOCKS — requires fix |
| FFI mismatch | Runtime crashes | P4 BLOCKS — requires header update |
| Error variant removed | API breaking change | P4 BLOCKS — requires additive-only change |
| Spec not updated | Documentation drift | P4 BLOCKS — requires spec update |

## Escalation

| Level | Path | SLA |
|-------|------|-----|
| 1 | Skill internal | 1 hour |
| 2 | Higher priority skill | 4 hours |
| 3 | Engineering Orchestrator (P1) | 24 hours |
| 4 | SSOT.md | Final authority |

## Protected Contracts

| Contract | Location | Protected Property |
|----------|----------|-------------------|
| Binary file format | `kcm-storage/src/file_format.rs` | DB_MAGIC, DB_VERSION, header layout |
| WAL entry format | `kcm-storage/src/wal.rs` | WAL_INSERT_SIZE, WAL_DELETE_SIZE |
| Codec encode/decode | `kcm-storage/src/codec.rs` | DeltaCodec, RleCodec, GorillaCodec |
| Dict codec | `kcm-storage/src/dict_codec.rs` | Dictionary encoding logic |
| Compression | `kcm-storage/src/compress.rs` | Algorithm selection |
| C FFI signatures | `kcm-interface/src/lib.rs` | 18 function signatures |
| gRPC proto | `kcm-interface/proto/kcm.proto` | Service and message definitions |
| Error codes | `kcm-core/src/types.rs` | KcmError enum variants |
| Public API | All crates | `Result<T, KcmError>` return types |
| `#[repr(C)]` structs | Various | Struct field layout |

## Operating Rules

1. **No format change without version bump** — If binary format changes, `DB_VERSION` must increment
2. **No API change without spec update** — If public API changes, corresponding spec document must update
3. **No codec change without roundtrip test** — Every codec modification must pass encode→decode identity test
4. **No FFI change without header update** — C header must match Rust declarations
5. **No proto change without backward compatibility** — gRPC proto changes must maintain backward compatibility or include migration plan
6. **No error code removal** — Error variants may be added but never removed
7. **Backward compatibility required** — New format must be able to read old format (or migration tool required)

## SSOT-First Lock Protocol

Every specification change MUST:

1. **Verify Authority** — Confirm this document is the authoritative source for the change
2. **Check Hierarchy** — Ensure no higher-priority document conflicts
3. **Backward Compatibility** — Assess if change breaks existing implementations
4. **Update All References** — Ensure all dependent documents are updated
5. **Version Bump** — Increment document version for any behavioral change
6. **Notify Stakeholders** — Alert affected skill owners of the change

## Activation Rules

**Activate when:**
- Any file in `crates/kcm-storage/src/file_format.rs` is modified
- Any file in `crates/kcm-storage/src/wal.rs` is modified
- Any file in `crates/kcm-storage/src/codec.rs` is modified
- Any file in `crates/kcm-storage/src/compress.rs` is modified
- Any file in `crates/kcm-storage/src/dict_codec.rs` is modified
- Any C FFI function signature changes in `crates/kcm-interface/src/lib.rs`
- Any error code enum changes in `crates/kcm-core/src/types.rs`
- Any public API signature changes across any crate
- Any `#[repr(C)]` struct changes
- Any gRPC proto definition changes in `crates/kcm-interface/proto/kcm.proto`

**Do NOT activate when:**
- Adding new functions (not modifying existing)
- Adding new tests
- Documentation-only changes
- Internal implementation changes that don't affect public API

## Examples

See [examples/](examples/) for usage examples.

## Checklist

See [checklists/](checklists/) for validation checklists.

## References

- [SSOT.md](../../SSOT.md)
- [AGENTS.md](../../AGENTS.md)
- [CONTRIBUTING.md](../../CONTRIBUTING.md)
- [SECURITY.md](../../SECURITY.md)
