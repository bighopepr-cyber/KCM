# Impact Analysis Example

## Change: Modify Fact structure (add field)

### Direct Impact
- **Crate:** kcm-core
- **Files:** types.rs, lib.rs
- **API:** Fact struct layout changes
- **Behavior:** Fact size increases from 34 bytes

### Indirect Impact
- **kcm-storage:** Column storage must accommodate new field
- **kcm-compute:** All operators must handle new field
- **kcm-interface:** FFI struct layout changes
- **kcm-reasoning:** Rule matching may be affected
- **All SDKs:** Language bindings must expose new field

### Specification Impact
- PRD.md: Fact structure must be updated
- KCM_DATA_MODEL_SPEC.md: Data model must be updated
- KCM_API_SPEC.md: API must be updated
- All SDK docs must be updated

### Test Impact
- All unit tests using Fact must be updated
- All property tests for Fact must be updated
- All integration tests must be updated
- All SDK tests must be updated

### Compatibility Impact
- **BREAKING:** Storage format change requires version bump
- **BREAKING:** FFI change requires SDK version bump
- **BREAKING:** API change requires major version bump

### Migration Required
- Existing databases must be migrated
- Existing FFI consumers must update code
- Existing SDK users must update code
