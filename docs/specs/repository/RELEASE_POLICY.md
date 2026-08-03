# Release Policy

| Field | Value |
|-------|-------|
| **Document ID** | KCM-REPO-010 |
| **Title** | Release Policy |
| **Version** | 1.0.0 |
| **Date** | 2026-08-03 |
| **Status** | Authoritative |
| **Authority** | Engineering Orchestrator (P1) |

---

## 1. Release Process

### Pre-Release Checklist

1. All tests pass (`cargo test --workspace`)
2. Zero clippy warnings (`cargo clippy --workspace -- -D warnings`)
3. Format check passes (`cargo fmt --all -- --check`)
4. CHANGELOG.md updated
5. Version numbers bumped in Cargo.toml
6. Documentation reviewed

### Release Steps

1. Create release branch: `release/vX.Y.Z`
2. Update CHANGELOG.md with release date
3. Create git tag: `vX.Y.Z`
4. Push tag: `git push origin vX.Y.Z`
5. Create GitHub Release with notes
6. Publish to crates.io (if applicable)
7. Build and push Docker image

## 2. Changelog Format

Follows [Keep a Changelog](https://keepachangelog.com/):

```markdown
## [X.Y.Z] - YYYY-MM-DD

### Added
- New features

### Changed
- Changes to existing features

### Fixed
- Bug fixes

### Removed
- Removed features
```

## 3. Git Tagging

Format: `vMAJOR.MINOR.PATCH`

Examples: v0.1.0, v1.0.0, v1.2.3

## 4. Docker Image Tagging

Format: `kcm:MAJOR.MINOR.PATCH`

Examples: kcm:0.1.0, kcm:1.0.0

Also tagged as `kcm:latest` for latest stable.
