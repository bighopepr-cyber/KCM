# benchmark-results/ Community Guidelines

> For project-wide community guidelines, refer to the [CODE_OF_CONDUCT.md](../CODE_OF_CONDUCT.md) located in the repository root.

## Respect

All contributors to the `benchmark-results/` module must treat others with respect and professionalism. This includes:

- Respecting different perspectives on performance optimization
- Acknowledging that benchmark results may vary across environments
- Valuing contributions from all experience levels
- Providing constructive feedback on performance claims

## Professional Communication

When discussing benchmark results and performance data:

- Present data objectively without embellishment
- Acknowledge limitations and variability in measurements
- Use precise terminology for performance metrics
- Cite sources when referencing external benchmarks
- Avoid making unsubstantiated performance claims

## Code Review Etiquette

When reviewing benchmark-related changes:

- Focus on data accuracy and methodology
- Verify that benchmark configurations are appropriate
- Check that regression thresholds are correctly applied
- Ensure report formats are consistent
- Provide specific, actionable feedback

### Review Guidelines

| Do | Don't |
|----|-------|
| Ask for clarification on methodology | Dismiss results without explanation |
| Suggest improvements to measurement approach | Make personal comments about contributors |
| Verify data against known baselines | Accept unverified performance claims |
| Check for environmental factors | Ignore potential confounding variables |

## Collaboration

Contributors are encouraged to:

- Share knowledge about benchmarking best practices
- Collaborate on performance investigation
- Review each other's benchmark configurations
- Discuss performance trade-offs openly
- Document performance decisions and rationale

## Reporting Issues

When reporting issues with benchmark results:

1. Include environment details (OS, CPU, memory)
2. Provide the exact commands used to generate results
3. Include relevant metadata files
4. Describe expected vs actual behavior
5. Suggest potential causes if known

### Issue Template

```markdown
## Benchmark Issue

### Environment
- OS: [e.g., Ubuntu 22.04]
- CPU: [e.g., Intel i7-12700K]
- Memory: [e.g., 32GB DDR5]

### Description
[Describe the issue]

### Expected behavior
[What should happen]

### Actual behavior
[What actually happened]

### Steps to reproduce
1. [Step 1]
2. [Step 2]
```

## Enforcement

Violations of these guidelines may result in:

1. **First occurrence**: Private discussion with contributor
2. **Second occurrence**: Written warning
3. **Third occurrence**: Temporary suspension from contributing
4. **Severe cases**: Permanent removal from the project

For enforcement inquiries, contact the project maintainers through the repository's issue tracker.

## References

- [Repository CODE_OF_CONDUCT.md](../CODE_OF_CONDUCT.md) - Project-wide guidelines
- [Repository CONTRIBUTING.md](../CONTRIBUTING.md) - Contribution rules
- [Benchmark Results README](README.md) - Module overview
