# Release Checklist

> Document ID: KCM-CHK-REL-001 | Version: 1.0.0

## Pre-Release Validation

- [ ] All CI jobs pass (ci.yml, ci-full.yml)
- [ ] SSOT validation passes (validate-ssot.sh)
- [ ] No performance regressions > 5% (bench-compare.py)
- [ ] All APIs match SSOT
- [ ] All FFI functions match SSOT (18 functions)
- [ ] All REST endpoints match SSOT
- [ ] All gRPC RPCs match SSOT

## Version Management

- [ ] VERSION file updated
- [ ] All Cargo.toml versions synced (verify-version.sh)
- [ ] All SDK versions synced
- [ ] All deployment versions synced
- [ ] All documentation badges synced

## Documentation

- [ ] CHANGELOG.md updated with version entry
- [ ] All spesifikasi.md files current
- [ ] All README files current
- [ ] All SDK documentation current
- [ ] Documentation validation passes (validate-docs.sh)

## Deployment

- [ ] Dockerfile builds successfully
- [ ] Helm chart version updated
- [ ] Kubernetes manifests valid
- [ ] Prometheus/Grafana configs valid

## Quality Gates

- [ ] Format: cargo fmt --check passes
- [ ] Lint: cargo clippy -- -D warnings passes
- [ ] Build: cargo build --workspace passes
- [ ] Unit Tests: cargo test --lib passes
- [ ] Integration: cargo test --test passes
- [ ] Property Tests: cargo test property passes
- [ ] Security: cargo audit passes
- [ ] Coverage: 100%

## Git Operations

- [ ] All changes committed
- [ ] No uncommitted changes
- [ ] Version commit created
- [ ] Git tag created (v{VERSION})
- [ ] Pushed to remote

## Post-Release

- [ ] CI/CD triggered release
- [ ] Release artifacts built
- [ ] Documentation published
- [ ] Monitoring configured
- [ ] Rollback plan documented
