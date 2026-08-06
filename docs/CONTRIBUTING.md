# Contributing to docs/

> For core engine contribution rules, refer to the repository [CONTRIBUTING.md](../CONTRIBUTING.md).

## Overview

This guide covers how to contribute to the KCM documentation. All documentation changes must maintain SSOT integrity and follow the established hierarchy.

## Before Contributing

1. Identify which documentation category your change belongs to (specs, ADRs, runbooks, SDK, handbook)
2. Check if an existing document already covers your topic
3. Review the SSOT hierarchy to ensure your change doesn't conflict with higher-priority documents
4. Open an issue or discussion for significant documentation changes

## Coding Standards

### Markdown Style

- Use ATX-style headers (`#`, `##`, `###`)
- Use fenced code blocks with language identifiers
- Use tables for structured data
- Keep lines under 120 characters where practical
- Use consistent terminology per `specs/KCM_GLOSSARY.md`

### Heading Conventions

- One `H1` (`#`) per document — the document title
- `H2` (`##`) for major sections
- `H3` (`###`) for subsections
- Do not skip heading levels

### Table of Contents

- Include a table of contents for documents longer than 200 lines
- Place TOC after the title and overview section
- Link to all `H2` and `H3` sections

## Module Architecture Rules

Documentation follows the SSOT hierarchy defined in `AGENTS.md`:

| Priority | Document | Authority |
|----------|----------|-----------|
| P1 | `PRD-TESTING-AND-BENCHMARK.md` | Testing strategy, quality gates |
| P2 | `PRD3.md` | Distributed, ML, security, compliance |
| P3 | `PRD2.md` | Storage, runtime, interfaces |
| P4 | `PRD.md` | Core types, storage, compute, reasoning |
| P5 | `KCM_*_SPEC.md` files | Component specifications |
| P6 | `AGENTS.md` | Engineering constitution |

When documents conflict, the higher-priority document wins.

## Documentation Rules

| Rule | Description |
|------|-------------|
| SSOT compliance | All specifications must trace to an SSOT requirement |
| No duplication | Same information must not appear in multiple documents with different wording |
| Cross-references | Use relative links to reference other documents |
| Version awareness | Note which version of the system a document applies to |
| Accuracy | All code examples must be tested and working |
| Completeness | Document all public APIs, not just selected ones |

### SSOT Traceability

Every specification document must include a traceability section mapping requirements to implementations:

```
Requirement ID → Specification Section → Implementation File → Test File
```

### Cross-Reference Format

```markdown
See [PRD.md §5](specs/PRD.md#section-5) for query engine details.
See [ADR-003](adr/ADR-003.md) for the decision rationale.
```

## Testing Requirements

| Requirement | Validation |
|-------------|-----------|
| All internal links valid | Automated link checker |
| Code examples compile | `cargo build` verification |
| SSOT hierarchy maintained | `scripts/validate-ssot.sh` |
| No broken references | Grep for dead links |
| Glossary consistency | Cross-reference `KCM_GLOSSARY.md` |

## Performance Rules

- Keep documentation concise; avoid unnecessary verbosity
- Use tables and lists over long paragraphs
- Place detailed content in subsections, summaries at section top
- Reference rather than duplicate — link to existing specs instead of re-stating

## Review Checklist

Before submitting a documentation PR:

- [ ] Follows Markdown style conventions
- [ ] One H1 per document
- [ ] No skipped heading levels
- [ ] All code blocks have language identifiers
- [ ] All internal links are valid
- [ ] No duplicated content across documents
- [ ] SSOT hierarchy is maintained
- [ ] Terminology matches `KCM_GLOSSARY.md`
- [ ] No secrets or credentials
- [ ] `scripts/validate-ssot.sh` passes

## Pull Request Requirements

| Requirement | Description |
|-------------|-------------|
| Descriptive title | Clearly state what documentation changed and why |
| Summary | Explain the motivation and scope of changes |
| Link to issue | Reference related issues or discussions |
| SSOT impact | Note which SSOT documents are affected |
| Reviewer assignment | Tag Documentation Guardian for specs, domain expert for component docs |
| CI passing | `validate-ssot.sh` and link checks must pass |

## References

- [CONTRIBUTING.md](../CONTRIBUTING.md) — Repository root contribution rules
- `AGENTS.md` — Engineering constitution
- `specs/KCM_GLOSSARY.md` — Project terminology
- `docs/handbook/handbook.md` — Developer handbook
- `scripts/validate-ssot.sh` — SSOT validation
