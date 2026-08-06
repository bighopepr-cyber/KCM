# Contract Validation Checklist

## Frozen Contracts
- [ ] Binary file format (DB_MAGIC, DB_VERSION, header layout)
- [ ] WAL entry format (WAL_INSERT_SIZE, WAL_DELETE_SIZE)
- [ ] C FFI signatures (18 functions)
- [ ] Error code enum (7 variants)
- [ ] Fact structure (34 bytes, 10 fields)
- [ ] gRPC proto definitions
- [ ] Public API return types (`Result<T, KcmError>`)
- [ ] `#[repr(C)]` struct layouts

## Change Validation
- [ ] Change is necessary (cannot be avoided)
- [ ] Change is backward compatible OR has migration path
- [ ] Version bump applied (major for breaking)
- [ ] All SDKs updated consistently
- [ ] All specs updated
- [ ] Roundtrip tests added for codec changes
- [ ] FFI safety documentation updated

## Approval
- [ ] P4 Specification Lock approved
- [ ] P7 Security Engineer approved (for FFI)
- [ ] P5 Architecture Guardian approved
- [ ] P11 Documentation Guardian updated specs
