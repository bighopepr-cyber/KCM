# .cargo/ Community Guidelines

> For project-wide community guidelines, refer to the [CODE_OF_CONDUCT.md](../CODE_OF_CONDUCT.md) located in the repository root.

## Respect

All contributors to the Cargo configuration must treat each other with respect:

- Acknowledge that build configuration affects all developers
- Accept feedback on configuration choices gracefully
- Consider the impact on different platforms and environments
- Value reproducibility and stability over convenience

## Professional Communication

- Explain the rationale behind configuration changes clearly
- Reference documentation or benchmarks when proposing optimizations
- Use precise technical language when discussing compiler flags
- Provide context for platform-specific settings

## Code Review Etiquette

When reviewing `.cargo/` configuration changes:

- Focus on correctness, security, and portability
- Question settings that reduce reproducibility
- Acknowledge well-documented configuration
- Verify that comments match actual behavior
- Consider the impact on CI/CD pipelines

## Collaboration

- Discuss significant configuration changes before implementing
- Share benchmark results when proposing performance-related flags
- Document platform-specific requirements
- Update documentation when configuration changes
- Coordinate with CI maintainers for pipeline compatibility

## Reporting Issues

Configuration issues should be reported via:

- **Build failures**: Issue with error output and platform details
- **Performance regression**: Issue with benchmark comparison
- **Reproducibility issues**: Issue with environment details
- **Security concerns**: Direct message to security team, not public issue

## Enforcement

Configuration quality standards are enforced through:

| Mechanism | Scope |
|-----------|-------|
| Code review | All config changes require maintainer approval |
| CI validation | Build and test verification on every push |
| Security review | Security-relevant flags require security engineer review |
| Platform testing | Changes tested on Linux and macOS |

Violations may result in:

1. Request for revision
2. Discussion in PR review
3. Escalation to Engineering Orchestrator
4. Revert of merged changes if standards were not met

## References

- [CODE_OF_CONDUCT.md](../CODE_OF_CONDUCT.md) — Repository root community guidelines
- `AGENTS.md` — Engineering constitution
- [Cargo Configuration Reference](https://doc.rust-lang.org/cargo/reference/config.html)
- `.cargo/config.toml` — Current configuration
