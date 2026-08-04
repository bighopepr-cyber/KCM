# Maintainer Handbook

**Document ID:** HANDBOOK-MAINTAINER-001
**Version:** 1.0.0
**Status:** Active
**Owner:** KCM Engineering Team
**Last Reviewed:** 2026-08-04
**Depends on:** KCM_SPECIFICATION

## Purpose

This handbook provides guidelines and procedures for KCM maintainers, including review processes, release management, and issue triage.

## Responsibilities

- Review and merge PRs
- Triage issues
- Release management
- Community support
- Architecture decisions

## Review Process

### PR Review Checklist

- [ ] CI passes
- [ ] Tests pass
- [ ] Documentation updated
- [ ] No security issues
- [ ] Performance acceptable
- [ ] Code style consistent

### Merge Requirements

- 2 approvals for critical paths
- 1 approval for standard changes
- CI green
- No unresolved conversations

## Release Process

### Version Bump

```bash
# Update version in Cargo.toml
cargo release version patch  # or minor, major
```

### Create Release

```bash
# Tag release
git tag v0.2.0
git push origin v0.2.0

# Create GitHub release
gh release create v0.2.0 --generate-notes
```

### Publish to crates.io

```bash
cargo publish -p kcm-core
cargo publish -p kcm-storage
# ... etc
```

### Docker Image

```bash
docker build -t kcm:0.2.0 .
docker push kcm:0.2.0
docker tag kcm:0.2.0 kcm:latest
docker push kcm:latest
```

## Issue Triage

### Priority Levels

| Priority | Description | Response Time |
|----------|-------------|---------------|
| P0 | Critical bug, data loss | Immediate |
| P1 | High impact, workaround exists | 24 hours |
| P2 | Medium impact, enhancement | 1 week |
| P3 | Low impact, nice-to-have | Backlog |

### Labels

| Label | Description |
|-------|-------------|
| bug | Bug report |
| enhancement | Feature request |
| documentation | Documentation issue |
| good-first-issue | Good for newcomers |
| help-wanted | Needs community help |
| priority/P0 | Critical |
| priority/P1 | High |
| priority/P2 | Medium |
| priority/P3 | Low |

## Architecture Decisions

- Write ADR for significant changes
- Get approval from Architecture Guardian
- Update relevant documentation
- Communicate to community

## Security

- Review security implications
- Run security tests
- Check for vulnerabilities
- Update dependencies regularly

## Communication

- Monthly community call
- Weekly maintainer sync
- Daily standup (async)
- Release announcements
