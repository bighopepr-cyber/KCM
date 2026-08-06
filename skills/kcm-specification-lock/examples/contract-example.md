# Contract Validation Example

## Change: Add new REST endpoint

### Analysis
- **Type:** API change (new endpoint, additive)
- **Impact:** Non-breaking (additive change)
- **Frozen contract:** REST API surface

### Validation
1. New endpoint does not modify existing endpoints ✓
2. New endpoint follows existing patterns ✓
3. No version bump required (additive) ✓
4. Spec update required: KCM_API_SPEC.md ✓
5. SDK update required: All SDKs must expose new endpoint ✓

### Decision
- **APPROVED** — Additive change, non-breaking
- **Required:** Spec update, SDK updates, tests
- **Version:** Minor bump (0.x.0)
