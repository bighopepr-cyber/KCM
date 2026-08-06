# KCM Engineering Skills

## Overview

KCM defines 16 AI engineering skills, each in its own subdirectory with a `SKILL.md` file. Every skill has a defined authority boundary, priority level, and scope of responsibility within the KCM development process.

## Purpose

Skills enforce governance and quality standards across all KCM development activities. They ensure architectural integrity, specification compliance, security correctness, performance targets, and production readiness.

## Responsibilities

| Area | Description |
|------|-------------|
| Skill governance | Managing authority hierarchy and priority ordering |
| Authority enforcement | Ensuring skills operate within defined boundaries |
| Engineering gates | Enforcing mandatory gates at each development phase |
| Quality assurance | Preventing code that violates standards from being committed |
| Security enforcement | Ensuring cryptographic correctness and RBAC compliance |

## Skill Registry

| Priority | Skill | Authority |
|----------|-------|-----------|
| P1 | kcm-engineering-orchestrator | Master coordinator — overrides all |
| P2 | kcm-task-planner | Can block implementation without plan |
| P3 | kcm-change-impact-analysis | Can block changes with unassessed impact |
| P4 | kcm-specification-lock | Can veto format/API/FFI changes |
| P5 | kcm-architecture-guardian | Can block architecture violations |
| P6 | kcm-database-engine-specialist | Can block storage/query changes |
| P7 | kcm-security-engineer | Can block security/compliance violations |
| P8 | kcm-performance-engineer | Can block performance regressions |
| P9 | kcm-testing-verification | Can block changes without tests |
| P10 | kcm-code-quality-guardian | Can reject code quality issues |
| P11 | kcm-documentation-guardian | Can block undocumented changes |
| P12 | kcm-release-readiness | Can block releases |
| P13 | kcm-code-review-auditor | Provides review feedback |
| P14 | kcm-debugging-root-cause | Provides diagnostic analysis |
| P15 | kcm-engineering-decision-record | Documents decisions |
| P16 | kcm-repository-intelligence | Provides codebase understanding |

## Execution Flow

```
1. Repository Understanding    → kcm-repository-intelligence (P16)
2. Specification Validation    → kcm-specification-lock (P4), kcm-architecture-guardian (P5)
3. Planning                    → kcm-task-planner (P2), kcm-change-impact-analysis (P3)
4. Implementation              → Domain skills (P6, P7, P8)
5. Verification                → kcm-testing-verification (P9), kcm-code-quality-guardian (P10), kcm-code-review-auditor (P13)
6. Release                     → kcm-release-readiness (P12)
```

## Authority Boundaries

- **Specification Lock (P4)**: Owns frozen contracts (format, API, FFI). Can VETO changes that deviate from SSOT.
- **Architecture Guardian (P5)**: Owns system architecture. Defers to P4 for format changes.
- **Database Engine Specialist (P6)**: Owns storage/query implementation. Cannot change contracts.
- **Security Engineer (P7)**: Owns security rules. No skill can override security decisions.
- **Task Planner (P2)**: Answers "What should be done?" before implementation begins.
- **Change Impact Analysis (P3)**: Answers "What will break?" before changes are made.

## Dependencies

All skills reference the SSOT (Single Source of Truth) documents:

| Document | Priority | Skills That Reference It |
|----------|----------|--------------------------|
| `docs/PRD-TESTING& BRACHMARCK.md` | P1 | kcm-testing-verification, kcm-performance-engineer, kcm-release-readiness |
| `docs/PRD3.md` | P2 | kcm-security-engineer, kcm-architecture-guardian |
| `docs/PRD2.md` | P3 | kcm-database-engine-specialist, kcm-release-readiness |
| `docs/PRD.md` | P4 | kcm-database-engine-specialist, kcm-architecture-guardian |
| `AGENTS.md` | P5 | All skills |

## Integration

Skills are invoked by AI agents during development. Each skill provides domain-specific instructions and workflows. Skills are loaded via the `skill` tool when a task matches their description.

## Build

Skills are Markdown documents and require no compilation. Validation is performed through:

```bash
bash scripts/validate-ssot.sh
```

## Run

Skills are loaded at runtime by AI agents. No separate execution step is required.

## Test

Skills are tested via agent invocation. Each skill's correctness is validated by:

1. Agent follows skill instructions during development
2. `cargo build --workspace` passes
3. `cargo test --workspace` passes
4. `cargo clippy --workspace -- -D warnings` passes
5. `bash scripts/validate-ssot.sh` passes

## Examples

### Invoking a skill for storage changes

```bash
# Agent loads kcm-database-engine-specialist before modifying storage code
```

### Invoking security review

```bash
# Agent loads kcm-security-engineer before implementing encryption changes
```

## References

- `AGENTS.md` — Engineering constitution and authority hierarchy
- `docs/PRD-TESTING& BRACHMARCK.md` — Testing strategy and quality gates
- `docs/PRD.md` through `docs/PRD3.md` — SSOT specification documents
