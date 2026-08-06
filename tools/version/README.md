# KCM Version Tooling

## Overview

This directory contains version management tools for the KCM repository.

## Canonical Version Source

The single source of truth for the KCM version is:

```
VERSION
```

located at the repository root. This file contains the version string in plain text format:

```
1.0.0
```

## Version Governance Rules

| Rule | Description |
|------|-------------|
| VERSION-01 | `VERSION` is the single source of truth for all version references |
| VERSION-02 | All `Cargo.toml` workspace packages must use `workspace.package.version` |
| VERSION-03 | All SDKs must match the VERSION file version |
| VERSION-04 | All deployment manifests must match the VERSION file version |
| VERSION-05 | All documentation badges must match the VERSION file version |
| VERSION-06 | No hardcoded version strings may differ from VERSION |
| VERSION-07 | Version changes require running `sync-version.sh` |
| VERSION-08 | CI validates version consistency on every push/PR |

## Usage

### Synchronize Versions

Update all version references to match the VERSION file:

```bash
# Sync to version in VERSION file
bash scripts/release/sync-version.sh

# Sync to a specific version
bash scripts/release/sync-version.sh 1.1.0
```

### Verify Versions

Validate that all version references are consistent:

```bash
bash scripts/release/verify-version.sh
```

### Bump Version

To bump the version:

1. Update the `VERSION` file
2. Run `bash scripts/release/sync-version.sh`
3. Run `bash scripts/release/verify-version.sh`
4. Update `CHANGELOG.md`
5. Commit all changes
6. Create a git tag: `git tag v<new-version>`

## Components Governed

| Component | File | Version Field |
|-----------|------|---------------|
| Workspace | `Cargo.toml` | `workspace.package.version` |
| Rust SDK | `sdk/rust/Cargo.toml` | `package.version` |
| Python SDK | `sdk/python/pyproject.toml` | `project.version` |
| JavaScript SDK | `sdk/javascript/package.json` | `version` |
| TypeScript SDK | `sdk/typescript/package.json` | `version` |
| Java SDK | `sdk/java/pom.xml` | `project.version` |
| .NET SDK | `sdk/dotnet/Kcm.Sdk.csproj` | `Version` |
| C++ SDK | `sdk/cpp/CMakeLists.txt` | `project(... VERSION ...)` |
| Helm Chart | `deployment/helm/kcm/Chart.yaml` | `version` + `appVersion` |
| Examples | `examples/rust/Cargo.toml` | `package.version` |
| CHANGELOG | `CHANGELOG.md` | `## [version]` heading |

## CI/CD Integration

The `.github/workflows/version.yml` workflow validates version consistency on every push and PR that touches version-related files.

## References

- `docs/specs/KCM_VERSIONING_SPEC.md` - Versioning specification
- `docs/specs/KCM_DEPLOYMENT_SPEC.md` - Deployment versioning
- `SSOT.md` - Single Source of Truth
