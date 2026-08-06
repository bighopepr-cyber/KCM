# kcm-storage Community Guidelines

Community guidelines specific to contributions in the `kcm-storage` crate.

> For project-wide community guidelines, refer to the [CODE_OF_CONDUCT.md](../../CODE_OF_CONDUCT.md) located in the repository root.

## Respect

- Treat all contributors with respect regardless of experience level
- Acknowledge that `kcm-storage` manages persistent data — errors here have lasting consequences
- Be patient with newcomers learning Rust storage and compression concepts
- Value correctness discussions over premature optimization

## Professional Communication

- Use clear, technical language in issues and PRs
- Provide evidence for claims about compression ratios, WAL performance, or recovery correctness
- Reference SSOT specifications when discussing storage format or codec design decisions
- Avoid emotional reactions to review feedback

## Code Review Etiquette

- Review `kcm-storage` changes with extra care — they affect data persistence and crash recovery
- Focus on WAL integrity, file format correctness, compression safety, and recovery edge cases
- Suggest alternatives rather than just rejecting
- Acknowledge good design decisions

## Collaboration

- Coordinate with other contributors on WAL format or file format changes
- Share benchmark results when claiming compression or WAL performance improvements
- Document design decisions in ADRs when appropriate
- Help maintain backward compatibility for database files

## Reporting Issues

- Include reproduction steps for correctness issues (especially WAL corruption or recovery failures)
- Include benchmarks for performance regressions
- Reference the affected module (column, compress, dict_codec, file_format, index, recovery, wal)
- Check if the issue affects downstream crates (kcm-compute, kcm-runtime, kcm-interface)

## Enforcement

Violations of these guidelines are handled per the project-wide [CODE_OF_CONDUCT.md](../../CODE_OF_CONDUCT.md). Contact: security@kcm.dev.

## References

- [CODE_OF_CONDUCT.md](../../CODE_OF_CONDUCT.md) — Project-wide community guidelines
- [CONTRIBUTING.md](../../CONTRIBUTING.md) — Contribution guidelines
- [AGENTS.md](../../AGENTS.md) — Engineering constitution
