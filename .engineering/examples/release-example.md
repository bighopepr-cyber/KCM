# Release Example

> Document ID: KCM-EX-REL-001 | Version: 1.0.0

## Task

Release KCM version 1.0.0.

## Execution Plan

**Task ID:** KCM-2026-003
**Pipeline:** release.md
**Risk Level:** High

### Required Skills

| Skill | Phase | Responsibility |
|-------|-------|---------------|
| P12 | Release | Validate all gates |
| P1 | Orchestrator | Final approval |

## Execution Steps

### Step 1: Pre-Release Validation (P12)

```
- [ ] All CI jobs pass
- [ ] SSOT validation passes
- [ ] No regressions > 5%
- [ ] All APIs match SSOT
- [ ] All FFI matches SSOT
- [ ] All REST matches SSOT
- [ ] All gRPC matches SSOT
- [ ] Deployment configs valid
- [ ] Documentation complete
- [ ] Changelog updated
- [ ] Version bumped
```

### Step 2: Version Bump

```bash
# Update VERSION file
echo "1.0.0" > VERSION

# Sync versions
bash scripts/release/sync-version.sh

# Verify versions
bash scripts/release/verify-version.sh
```

### Step 3: Changelog Update

```markdown
## [1.0.0] - 2026-08-06

### Added
- Core engine with 13 crates
- 9 language SDKs
- 17 CLI tools
- Full documentation suite

### Security
- AES-256-GCM encryption
- RBAC with 5 permission levels
- Hash-chained audit logging
```

### Step 4: Git Operations

```bash
git add -A
git commit -m "release: v1.0.0"
git tag v1.0.0
git push origin main --tags
```

### Step 5: Post-Release

```
- Verify CI/CD triggers release
- Verify deployment configs
- Monitor for issues
```

## Completion Report

**Status:** COMPLETED
**Version:** 1.0.0
**Date:** 2026-08-06
**Git Tag:** v1.0.0
**All Gates:** PASS
