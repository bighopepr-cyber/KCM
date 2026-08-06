# Release Pipeline

> Document ID: KCM-PIPE-REL-001 | Version: 1.0.0

## Overview

Pipeline for version releases.

## Pipeline

```
1. P12 Release Readiness (validate all gates)
2. P1 Engineering Orchestrator (final approval)
3. Version bump
4. Changelog update
5. Git tag
6. CI/CD triggers release
```

## Release Checklist

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

## Version Rules

| Change Type | Bump | Example |
|-------------|------|---------|
| Bug fix | Patch | 1.0.0 → 1.0.1 |
| New feature | Minor | 1.0.0 → 1.1.0 |
| Breaking change | Major | 1.0.0 → 2.0.0 |
