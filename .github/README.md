# KCM GitHub Configuration

CI/CD workflows, templates, and GitHub-specific configuration.

## Workflows

| Workflow | File | Trigger | Purpose |
|----------|------|---------|---------|
| CI Pipeline | ci.yml | Push/PR | Build, test, lint, quality gate |
| Benchmarks | benchmark.yml | Push/PR/Weekly | Performance regression detection |
| Website | deploy-website.yml | Push to main | Deploy documentation site |

## Templates

| Template | Purpose |
|----------|---------|
| PULL_REQUEST_TEMPLATE.md | PR description and checklist |
| ISSUE_TEMPLATE/bug_report.md | Bug report form |
| ISSUE_TEMPLATE/feature_request.md | Feature request form |

## CODEOWNERS

Defines code ownership for automatic review assignment:
- @kcm/core-team: crates/kcm-core/
- @kcm/storage-team: crates/kcm-storage/
- @kcm/security-team: crates/kcm-security/
- @kcm/devops-team: deployment/, .github/, scripts/

## Branch Protection

| Branch | Rules |
|--------|-------|
| main | 2 approvals, CI required, no force push |
| develop | 1 approval, CI required |
| release/* | 1 approval, CI required |
