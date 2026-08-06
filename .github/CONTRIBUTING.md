# Contributing to .github/

> For core engine contribution rules, refer to the repository [CONTRIBUTING.md](../CONTRIBUTING.md).

## Overview

Guidelines for contributing to the `.github/` configuration, including CI/CD workflows, issue templates, and code ownership rules.

## Before Contributing

1. Understand the CI/CD pipeline structure
2. Review existing workflow files
3. Follow YAML formatting standards
4. Test workflow changes locally with `act`

## Coding Standards

| Standard | Description |
|----------|-------------|
| YAML formatting | Consistent indentation and spacing |
| Workflow best practices | Use reusable actions where possible |
| Action versions | Pin to specific SHA or version |
| Comments | Document non-obvious workflow steps |

## Module Architecture Rules

| Rule | Description |
|------|-------------|
| Independent workflows | Each workflow handles a specific concern |
| No cross-workflow dependencies | Workflows should be self-contained |
| Secret isolation | Each workflow uses only required secrets |
| Least-privilege | Minimum required permissions per workflow |

## Documentation Rules

| Rule | Description |
|------|-------------|
| Workflow names | Clear, descriptive workflow names |
| Job descriptions | Document each job's purpose |
| Step comments | Explain non-obvious steps |
| Template consistency | Consistent issue/PR template formatting |

## Testing Requirements

| Method | Description |
|--------|-------------|
| act | Local GitHub Actions testing |
| actionlint | Workflow syntax validation |
| yamllint | YAML formatting validation |
| Manual review | Visual inspection of workflow logic |

```bash
# Test workflow locally
act push
act pull_request

# Validate workflow syntax
actionlint .github/workflows/*.yml

# Validate YAML formatting
yamllint .github/workflows/*.yml
```

## Performance Rules

| Rule | Description |
|------|-------------|
| Workflow duration | Keep workflows under 30 minutes |
| Parallel jobs | Use parallel jobs where possible |
| Caching | Cache dependencies and build artifacts |
| Conditional execution | Skip jobs when not needed |

## Review Checklist

- [ ] YAML formatting is correct
- [ ] Actions are pinned to specific versions
- [ ] Permissions follow least-privilege
- [ ] Secrets are not exposed in logs
- [ ] Workflow logic is clear and documented
- [ ] No hardcoded values in workflow files
- [ ] Branch protection rules are not bypassed

## Pull Request Requirements

| Requirement | Description |
|-------------|-------------|
| Title format | `fix:`, `feat:`, `chore:` prefix |
| Description | Clear description of workflow changes |
| Testing evidence | Proof of local testing with `act` |
| Security review | Verify no secret exposure |

## References

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Repository CONTRIBUTING.md](../CONTRIBUTING.md)
- [CODEOWNERS](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners)
