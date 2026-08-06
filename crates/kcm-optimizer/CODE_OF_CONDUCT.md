# kcm-optimizer Community Guidelines

> For project-wide community guidelines, refer to the [CODE_OF_CONDUCT.md](../../CODE_OF_CONDUCT.md) located in the repository root.

---

## Respect

All participants in the `kcm-optimizer` development community are expected to treat each other with respect and professionalism. This includes:

- Acknowledging diverse perspectives and experience levels
- Being considerate in language and tone
- Accepting constructive criticism gracefully
- Focusing on the technical problem, not the person
- Giving credit where it is due

## Professional Communication

Technical discussions about the optimizer should be conducted professionally:

- Use clear, precise technical language
- Provide evidence and reasoning for claims about optimizer behavior
- Reference specifications (PRD2.md §16) when discussing intended behavior
- Distinguish between facts, opinions, and hypotheses
- Acknowledge uncertainty rather than making unfounded assertions
- Use code examples and benchmarks to support arguments

## Code Review Etiquette

Code reviews for `kcm-optimizer` changes should follow these principles:

| Principle | Description |
|-----------|-------------|
| Be specific | Point to exact lines and explain the concern |
| Be constructive | Suggest improvements, not just identify problems |
| Be timely | Review within a reasonable timeframe |
| Be humble | Accept that you may have missed context |
| Be thorough | Check correctness, performance, security, and style |
| Reference specs | Cite PRD2.md §16 or other SSOT documents when applicable |

When reviewing optimizer code, pay particular attention to:

- Correctness of cost model calculations
- Validity of plan transformations
- Edge cases in statistics handling
- Performance implications of algorithmic changes
- Concurrency safety in shared state

## Collaboration

The optimizer is a critical component of the KCM engine. Collaborative practices include:

- Discussing design decisions before implementation
- Sharing benchmark results openly
- Reviewing optimizer changes with domain expertise
- Participating in architecture discussions
- Contributing to optimizer documentation and specifications
- Mentoring new contributors on optimizer internals

## Reporting Issues

Issues with optimizer behavior should be reported with:

1. **Reproduction steps**: Query and data that trigger the issue
2. **Expected behavior**: What the optimizer should produce
3. **Actual behavior**: What the optimizer actually produces
4. **Impact**: Performance degradation, incorrect results, or crash
5. **Environment**: KCM version, data size, schema complexity

Security issues should be reported through the project-wide security process defined in [SECURITY.md](../../SECURITY.md).

## Enforcement

Violations of these community guidelines may result in:

1. A private reminder from a maintainer
2. A formal warning with documentation
3. Temporary suspension from optimizer development
4. Permanent removal from the project

Enforcement decisions are made by the project maintainers in accordance with the project-wide [CODE_OF_CONDUCT.md](../../CODE_OF_CONDUCT.md).

## References

| Document | Scope |
|----------|-------|
| [Project CODE_OF_CONDUCT.md](../../CODE_OF_CONDUCT.md) | Project-wide community guidelines |
| [AGENTS.md](../../AGENTS.md) | Engineering constitution |
| [PRD2.md §16](../../docs/PRD2.md) | Optimizer specification |
