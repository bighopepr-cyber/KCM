# Contract Change Request

**Change:** {{CHANGE_DESCRIPTION}}
**Date:** {{DATE}}
**Requester:** {{REQUESTER}}

## Contract Type

- [ ] Binary file format
- [ ] WAL entry format
- [ ] C FFI signature
- [ ] Error code enum
- [ ] Fact structure
- [ ] gRPC proto
- [ ] Public API return type
- [ ] `#[repr(C)]` struct layout

## Change Description

{{CHANGE_DESCRIPTION}}

## Backward Compatibility

- [ ] Fully backward compatible
- [ ] Breaking change with migration path
- [ ] Breaking change without migration path

## Version Impact

- [ ] No version bump needed
- [ ] Patch bump (0.0.x)
- [ ] Minor bump (0.x.0)
- [ ] Major bump (x.0.0)

## Required Updates

| Component | Update Required |
|-----------|----------------|
| Specs | {{SPECS}} |
| SDKs | {{SDKS}} |
| Tests | {{TESTS}} |
| Documentation | {{DOCS}} |

## Approval

- [ ] P4 Specification Lock
- [ ] P7 Security Engineer (if FFI)
- [ ] P5 Architecture Guardian
