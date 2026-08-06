# deployment/ Community Guidelines

> For project-wide community guidelines, refer to the [CODE_OF_CONDUCT.md](../CODE_OF_CONDUCT.md) located in the repository root.

## Respect

All contributors to the deployment infrastructure must treat each other with respect and professionalism. This includes:

- Respecting diverse perspectives and experience levels
- Acknowledging that deployment infrastructure is a shared responsibility
- Recognizing that different contributors have different areas of expertise
- Valuing constructive feedback over criticism

## Professional Communication

### Code Review

- Review deployment changes promptly and constructively
- Focus on the technical merits of the change, not the author
- Provide specific, actionable feedback with references to documentation or security policies
- Acknowledge good work when it is warranted

### Issue Reporting

- Report deployment issues with reproducible steps
- Include relevant logs, configuration snippets, and environment details
- Prioritize security issues through the responsible disclosure process
- Follow up on reported issues with additional context when available

### Discussion

- Use clear, concise language when discussing deployment decisions
- Reference the relevant SSOT documents when making technical arguments
- Avoid making deployment changes without discussion for significant modifications
- Document decisions in GitHub issues or pull request discussions

## Code Review Etiquette

- Reviewers must check compliance with [deployment/SECURITY.md](SECURITY.md) for all changes
- Authors must not self-merge deployment PRs without at least one approval
- Feedback must be addressed before merging; dismissals require justification
- Deployment reviews should be completed within 48 hours of request
- Focus reviews on security impact, operational correctness, and maintainability

## Collaboration

- Share knowledge about deployment patterns and anti-patterns
- Mentor less experienced contributors on infrastructure best practices
- Cross-review deployment changes across different targets (Docker, Kubernetes, Terraform, etc.)
- Contribute to deployment documentation when discovering undocumented behavior

## Reporting Issues

### Security Vulnerabilities

Report security issues in deployment configurations through private disclosure. Do not open public issues for security vulnerabilities. Include:

- Description of the vulnerability
- Affected deployment component
- Potential impact
- Suggested remediation

### Operational Issues

Report operational issues (broken deployments, misconfigurations) as GitHub issues with:

- Steps to reproduce
- Expected behavior
- Actual behavior
- Relevant logs and configuration

## Enforcement

Violations of these community guidelines may result in:

1. **First offense**: Private communication and reminder of guidelines
2. **Second offense**: Temporary restriction from deployment-related PRs
3. **Third offense**: Escalation to the repository maintainers

The repository maintainers reserve the right to take immediate action for security-related violations or harassment.

## References

- [CNCF Code of Conduct](https://github.com/cncf/foundation/blob/main/code-of-conduct.md)
- [Kubernetes Community Guidelines](https://github.com/kubernetes/community/blob/master/governance/community-values.md)
- [SSOT: docs/PRD3.md §33](../PRD3.md) — Deployment architecture
