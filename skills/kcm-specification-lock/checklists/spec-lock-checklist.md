# Specification Lock Checklist

## Change Detection

- [ ] Proposed change has been identified
- [ ] Change type has been classified (format/API/FFI/protocol/data model)
- [ ] Frozen contract impact has been assessed
- [ ] SSOT document impact has been assessed

## Contract Validation

- [ ] Binary file format contracts are not violated
- [ ] WAL entry format contracts are not violated
- [ ] C FFI signature contracts are not violated
- [ ] Error code enum contracts are not violated
- [ ] Fact structure contracts are not violated
- [ ] gRPC proto definition contracts are not violated
- [ ] Public API return type contracts are not violated
- [ ] `#[repr(C)]` struct layout contracts are not violated

## SSOT Alignment

- [ ] Implementation matches SSOT specification exactly
- [ ] No code deviates from SSOT without approved SSOT update
- [ ] SSOT update has been proposed (if needed)
- [ ] SSOT update follows governance process

## Drift Detection

- [ ] Code-specification divergence has been checked
- [ ] API documentation matches implementation
- [ ] FFI signatures match specification
- [ ] Test coverage matches specification

## Approval

- [ ] P4 (Specification Lock) approval has been obtained
- [ ] Contract compatibility analysis has been completed
- [ ] Breaking change analysis has been documented
- [ ] Version bump requirements have been identified

## Documentation

- [ ] Change rationale has been documented
- [ ] Contract impact has been documented
- [ ] Migration path has been documented (if breaking)
- [ ] Changelog entry has been prepared
