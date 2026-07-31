---
name: kcm-specification-lock
description: Protect frozen technical contracts and prevent accidental architecture drift
---

# Skill: Specification Lock

## Skill Identity

**Purpose:** Protect frozen technical contracts and binary formats from accidental modification. This skill acts as a change control gate for any code that touches protected specifications.

**Role:** Contract Gatekeeper

**Scope:** Frozen formats, APIs, protocols, gRPC proto definitions, error code enums, and backward compatibility requirements.

**Non-responsibility:** Does not implement changes. Does not review architecture (Architecture Guardian). Does not review code quality (Code Quality Guardian). Does not write tests (Testing Skill).

**Measurable Outcomes:**
- Zero frozen contract violations merged
- Every format change has a version bump
- Every API change has a spec update
- Every codec change has a roundtrip test
- Every proto change is backward compatible or has migration

---

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

---

## Required Inspection

Before approving any protected change, read:
1. `docs/KCM_COLUMNAR_FORMAT_SPEC.md` — Binary format
2. `docs/KCM_COMPRESSION_SPEC.md` — Codec specifications
3. `docs/KCM_API_SPEC.md` — API contracts
4. `docs/KCM_VERSIONING_SPEC.md` — Compatibility rules
5. The specific file being modified
6. All test files that exercise the modified component

---

## Protected Contracts

This skill has **veto authority** over any change that modifies:
- Binary file format (magic bytes, header layout, column block format)
- WAL entry format (byte layout, field order, entry sizes)
- Codec encode/decode logic (DeltaCodec, RleCodec, GorillaCodec, DictCodec)
- Compression algorithm selection
- C FFI function signatures
- gRPC proto definitions (`crates/kcm-interface/proto/kcm.proto`)
- Error code enum variants
- Public API return types
- `#[repr(C)]` struct layouts

---

## Operating Rules

1. **No format change without version bump** — If binary format changes, `DB_VERSION` must increment
2. **No API change without spec update** — If public API changes, corresponding spec document must update
3. **No codec change without roundtrip test** — Every codec modification must pass encode→decode identity test
4. **No FFI change without header update** — C header must match Rust declarations
5. **No proto change without backward compatibility** — gRPC proto changes must maintain backward compatibility or include migration plan
6. **No error code removal** — Error variants may be added but never removed
7. **Backward compatibility required** — New format must be able to read old format (or migration tool required)

---

## Validation Checklist

- [ ] Binary format version bumped if layout changed
- [ ] WAL entry size matches specification (34 bytes Insert, 9 bytes Delete)
- [ ] Codec roundtrip tests pass
- [ ] Compression roundtrip tests pass
- [ ] C FFI signatures match specification
- [ ] gRPC proto changes backward compatible
- [ ] Error codes unchanged or documented as additive
- [ ] All existing tests still pass
- [ ] Specification document updated to match code

---

## Final Report Format

```
# KCM Engineering Report

## Skill
kcm-specification-lock

## Protected Component
[format/codec/API/FFI/errors/proto]

## Change Type
[modification/addition/removal]

## Version Impact
[none/minor/major]

## Migration Required
[yes/no]

## Backward Compatible
[yes/no]

## Checks
- [ ] Format spec updated
- [ ] API spec updated
- [ ] Roundtrip tests pass
- [ ] Existing tests pass
- [ ] Proto backward compatible

## Specification Impact
[files]

## Code Impact
[files]

## Decision
APPROVED / BLOCKED / NEEDS MIGRATION PLAN
```
