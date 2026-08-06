# KCM Documentation

## Overview

The `docs/` directory is the central documentation hub for the KCM project. It houses all Single Source of Truth (SSOT) specifications, Architecture Decision Records (ADRs), operational handbooks, runbooks, and SDK documentation.

## Purpose

The documentation directory exists to:

- House all SSOT specifications that define the KCM system
- Maintain ADRs capturing key architectural decisions
- Provide operational handbooks and runbooks for day-to-day use
- Deliver SDK documentation for all supported language bindings
- Serve as the authoritative reference for all KCM components

## Responsibilities

| Responsibility | Owner | Description |
|---------------|-------|-------------|
| Specification Ownership | Documentation Guardian | SSOT documents are authoritative sources for implementation |
| ADR Management | Engineering Orchestrator | New architectural decisions require ADRs before implementation |
| Documentation Maintenance | All Contributors | Docs must stay current with implementation changes |
| Version Control | Specification Lock | Frozen contracts must not change without approval |

## Folder Structure

| Folder | Contents | Purpose |
|--------|----------|---------|
| `adr/` | ADR-001 through ADR-010 | Architecture Decision Records |
| `handbook/` | `handbook.md` | Developer and contributor handbooks |
| `runbook/` | `DISASTER_RECOVERY.md`, `OPERATIONAL_RUNBOOK.md` | Operational procedures |
| `sdk/` | 11 language-specific docs | SDK usage guides for C, C++, C#, Go, Java, JavaScript, Python, Rust, TypeScript, plus compatibility and specification |
| `specs/` | PRD.md, PRD2.md, PRD3.md, PRD-TESTING-AND-BENCHMARK.md, 15 KCM_*_SPEC.md files | SSOT specifications and technical specs |

## Public API

The documentation structure is consumed by:

- **Engineers**: Reference specs during implementation
- **Reviewers**: Validate changes against SSOT
- **CI/CD**: Automated SSOT validation via `scripts/validate-ssot.sh`
- **SDK Users**: Language-specific integration guides

## Internal Components

### adr/

Architecture Decision Records capture significant technical decisions with context, options considered, and rationale. Each ADR follows a standard format:

- **Status**: Proposed | Accepted | Deprecated | Superseded
- **Context**: Problem being addressed
- **Decision**: What was decided
- **Consequences**: Impact of the decision

### handbook/

The handbook provides developer onboarding material, coding standards, and workflow guides for contributors.

### runbook/

Operational runbooks document procedures for disaster recovery, system operations, and incident response. These contain operational credentials references and should be treated as sensitive.

### sdk/

Language-specific SDK documentation covers:

| Language | File | Binding Type |
|----------|------|-------------|
| C | `c.md` | FFI |
| C++ | `cpp.md` | FFI wrapper |
| C# / .NET | `dotnet.md` | P/Invoke |
| Go | `go.md` | cgo |
| Java | `java.md` | JNI |
| JavaScript | `javascript.md` | N-API |
| Python | `python.md` | PyO3 |
| Rust | `rust.md` | Native |
| TypeScript | `typescript.md` | N-API |
| Compatibility | `compatibility.md` | Cross-platform matrix |
| Specification | `spesifikasi.md` | Technical spec |

### specs/

SSOT specifications organized by priority:

| Priority | Document | Scope |
|----------|----------|-------|
| P1 | `PRD-TESTING-AND-BENCHMARK.md` | Testing strategy, benchmarks, quality gates |
| P2 | `PRD3.md` | Distributed, ML, security, compliance |
| P3 | `PRD2.md` | Storage, runtime, interfaces |
| P4 | `PRD.md` | Core types, storage, compute, reasoning |
| P5 | `KCM_*_SPEC.md` (15 files) | Component-level specifications |

## Dependencies

All documentation references the root SSOT documents:

- `AGENTS.md` (repository root) — Engineering constitution
- `docs/specs/PRD.md` — Core specification
- `docs/specs/PRD2.md` — Persistence specification
- `docs/specs/PRD3.md` — Distributed specification
- `docs/specs/PRD-TESTING-AND-BENCHMARK.md` — Testing specification

When documents conflict, the higher-priority document wins per the SSOT hierarchy.

## Integration

Documentation is referenced by all crates in the workspace:

- `kcm-core` references type definitions in `PRD.md`
- `kcm-storage` references format specs in `PRD2.md`
- `kcm-compute` references query specs in `PRD.md`
- `kcm-reasoning` references inference specs in `PRD.md`
- `kcm-optimizer` references planner specs in `PRD2.md`
- `kcm-runtime` references database specs in `PRD2.md`
- `kcm-interface` references FFI and API specs
- `kcm-distributed` references sharding specs in `PRD3.md`
- `kcm-security` references security specs in `PRD3.md`
- `kcm-compliance` references compliance specs in `PRD3.md`

## Build

Documentation is static Markdown. No build step is required.

```bash
# Validate documentation structure
ls -la docs/
ls -la docs/adr/ docs/handbook/ docs/runbook/ docs/sdk/ docs/specs/
```

## Run

Documentation is read directly. No runtime process is needed.

## Test

Documentation validity is verified through:

```bash
# Automated SSOT validation
bash scripts/validate-ssot.sh

# Check for broken internal links
grep -r '\[.*\](.*\.md)' docs/ --include="*.md"
```

## Examples

Refer to `docs/sdk/` for language-specific integration examples and `docs/handbook/` for developer workflow examples.

## References

- `AGENTS.md` — Engineering constitution
- `docs/specs/PRD.md` — Core specification
- `docs/specs/PRD2.md` — Persistence specification
- `docs/specs/PRD3.md` — Distributed specification
- `docs/specs/PRD-TESTING-AND-BENCHMARK.md` — Testing specification
- `scripts/validate-ssot.sh` — Automated SSOT validation
