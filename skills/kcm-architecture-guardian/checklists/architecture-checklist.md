# Architecture Checklist

## Dependency Direction
- [ ] No circular dependencies
- [ ] Dependencies flow upward only
- [ ] kcm-core has zero internal deps
- [ ] Each crate has clear responsibility

## Separation of Concerns
- [ ] Each crate has single responsibility
- [ ] No cross-crate implementation leaks
- [ ] Public APIs are minimal and focused
- [ ] Internal modules are not exposed

## Interface Stability
- [ ] Public APIs return `Result<T, KcmError>`
- [ ] No breaking changes without version bump
- [ ] FFI functions have `# Safety` docs
- [ ] All public types implement `Debug`

## Data Integrity
- [ ] Fact structure is 34 bytes
- [ ] Confidence is [0.0, 1.0]
- [ ] WAL entries preserve all fields
- [ ] Recovery is complete and lossless

## PRD Traceability
- [ ] Every decision traces to PRD
- [ ] Every implementation matches spec
- [ ] No undocumented behavior
