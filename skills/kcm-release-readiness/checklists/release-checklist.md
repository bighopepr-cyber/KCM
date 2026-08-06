# Release Checklist

## Build
- [ ] cargo build --release --workspace passes
- [ ] All 13 crates build
- [ ] No build warnings

## Tests
- [ ] cargo test --workspace passes
- [ ] 100% test pass rate
- [ ] >= 372 tests

## Quality
- [ ] clippy clean
- [ ] fmt clean
- [ ] No unwrap/panic/TODO

## Performance
- [ ] No regression > 5%
- [ ] Benchmarks within baseline

## Security
- [ ] No hardcoded keys
- [ ] FFI safety docs present
- [ ] RBAC enforced

## Documentation
- [ ] All READMEs updated
- [ ] All spesifikasi updated
- [ ] Changelog updated
- [ ] Version bumped

## SSOT
- [ ] All APIs match SSOT
- [ ] All FFI matches SSOT
- [ ] All REST matches SSOT
- [ ] All gRPC matches SSOT