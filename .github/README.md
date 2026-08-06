# .github/ Configuration

## Overview

GitHub-specific configuration for CI/CD, issue templates, and code ownership.

## Purpose

- CI pipeline definitions
- Issue/PR templates
- Code ownership

## Responsibilities

- CI enforcement
- Contribution templates
- Code review assignment

## Folder Structure

| File/Directory | Description |
|----------------|-------------|
| CODEOWNERS | Code review assignment rules |
| PULL_REQUEST_TEMPLATE.md | PR template for contributors |
| ISSUE_TEMPLATE/ | Bug report and feature request templates |
| workflows/ | CI/CD workflow definitions |

## Public API

| Workflow | Trigger | Description |
|----------|---------|-------------|
| ci.yml | push, pull_request | Main CI pipeline |
| ci-full.yml | push to main | Full CI with security and benchmarks |
| sdk-ci.yml | SDK changes | Per-language SDK checks |
| sdk-publish.yml | release | SDK publish pipeline |
| benchmark.yml | schedule, manual | Performance benchmarks |

## Internal Components

| Component | File | Description |
|-----------|------|-------------|
| CODEOWNERS | CODEOWNERS | Defines code review assignment rules |
| PR Template | PULL_REQUEST_TEMPLATE.md | Standardized PR description |
| Bug Template | ISSUE_TEMPLATE/bug_report.md | Bug report template |
| Feature Template | ISSUE_TEMPLATE/feature_request.md | Feature request template |
| CI Pipeline | workflows/ci.yml | Format, clippy, build, test |
| Full CI | workflows/ci-full.yml | CI + security + benchmarks + SSOT |
| SDK CI | workflows/sdk-ci.yml | SDK-specific checks |
| SDK Publish | workflows/sdk-publish.yml | SDK release pipeline |
| Benchmarks | workflows/benchmark.yml | Performance benchmarking |

## Dependencies

- Workspace `Cargo.toml` for Rust toolchain configuration
- GitHub Actions runner environments

## Integration

CI runs on every push and pull request to enforce code quality gates.

## Build

```bash
# Validate workflow YAML syntax
yamllint .github/workflows/*.yml
```

## Run

```bash
# Local CI simulation with act
act push
act pull_request
```

## Test

```bash
# Validate workflow files
actionlint .github/workflows/*.yml
```

## Examples

See workflow files in `workflows/` directory for CI/CD configuration examples.

## References

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [CODEOWNERS](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners)
- Workspace `Cargo.toml`
