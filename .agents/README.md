# .agents/ Configuration

## Overview

The `.agents/` directory contains AI agent governance configuration for the KCM project. It is a structural mirror of the `skills/` folder, providing AI agents with skill instructions for KCM development governance.

## Purpose

Provide AI agents with skill instructions for KCM development governance. This directory ensures that AI agent systems can load and execute governance instructions consistently, enabling automated compliance with KCM engineering standards.

## Responsibilities

- **Skill delivery to AI agents**: Deliver standardized skill instructions to AI agent systems for governance enforcement
- **Governance enforcement**: Enforce KCM engineering standards, coding practices, and quality gates through AI-readable skill definitions
- **Structural mirroring**: Maintain an exact structural mirror of the `skills/` directory for consistency

## Folder Structure

```
.agents/
├── README.md                  # This file
├── SECURITY.md                # Security policy for AI agent configuration
├── CONTRIBUTING.md            # Contribution guidelines for .agents/
├── CODE_OF_CONDUCT.md         # Community guidelines for .agents/
└── skills/                    # Mirror of skills/ folder
    ├── kcm-architecture-guardian/
    │   └── SKILL.md
    ├── kcm-change-impact-analysis/
    │   └── SKILL.md
    ├── kcm-code-quality-guardian/
    │   └── SKILL.md
    ├── kcm-code-review-auditor/
    │   └── SKILL.md
    ├── kcm-database-engine-specialist/
    │   └── SKILL.md
    ├── kcm-debugging-root-cause/
    │   └── SKILL.md
    ├── kcm-documentation-guardian/
    │   └── SKILL.md
    ├── kcm-engineering-decision-record/
    │   └── SKILL.md
    ├── kcm-engineering-orchestrator/
    │   └── SKILL.md
    ├── kcm-performance-engineer/
    │   └── SKILL.md
    ├── kcm-release-readiness/
    │   └── SKILL.md
    ├── kcm-repository-intelligence/
    │   └── SKILL.md
    ├── kcm-security-engineer/
    │   └── SKILL.md
    ├── kcm-specification-lock/
    │   └── SKILL.md
    ├── kcm-task-planner/
    │   └── SKILL.md
    └── kcm-testing-verification/
        └── SKILL.md
```

## Public API

The `.agents/` directory exposes its content through the following file paths:

| Path | Description |
|------|-------------|
| `.agents/skills/<skill-name>/SKILL.md` | Skill instruction file for a specific governance skill |
| `.agents/README.md` | Configuration documentation |
| `.agents/SECURITY.md` | Security policy |
| `.agents/CONTRIBUTING.md` | Contribution guidelines |
| `.agents/CODE_OF_CONDUCT.md` | Community guidelines |

## Internal Components

16 skill definitions mirroring the `skills/` directory:

| # | Skill Directory | Authority Level |
|---|----------------|-----------------|
| 1 | `kcm-engineering-orchestrator` | P1 — Master coordinator |
| 2 | `kcm-task-planner` | P2 — Task planning |
| 3 | `kcm-change-impact-analysis` | P3 — Change impact |
| 4 | `kcm-specification-lock` | P4 — Specification authority |
| 5 | `kcm-architecture-guardian` | P5 — Architecture authority |
| 6 | `kcm-database-engine-specialist` | P6 — Database engine |
| 7 | `kcm-security-engineer` | P7 — Security |
| 8 | `kcm-performance-engineer` | P8 — Performance |
| 9 | `kcm-testing-verification` | P9 — Testing |
| 10 | `kcm-code-quality-guardian` | P10 — Code quality |
| 11 | `kcm-documentation-guardian` | P11 — Documentation |
| 12 | `kcm-release-readiness` | P12 — Release readiness |
| 13 | `kcm-code-review-auditor` | P13 — Code review |
| 14 | `kcm-debugging-root-cause` | P14 — Debugging |
| 15 | `kcm-engineering-decision-record` | P15 — Decision records |
| 16 | `kcm-repository-intelligence` | P16 — Repository intelligence |

## Dependencies

- **`skills/` directory**: The `.agents/skills/` folder must mirror the `skills/` directory exactly in structure and content
- **AI agent systems**: Consumers of `.agents/` content for loading skill instructions

## Integration

The `.agents/` directory is loaded by AI agent systems as follows:

1. AI agent system detects KCM project structure
2. Agent system scans `.agents/skills/` for available skill definitions
3. Agent loads relevant `SKILL.md` files based on task context
4. Agent executes instructions from loaded skills
5. Agent enforces governance rules during code generation and modification

## Build

No build step required. The `.agents/` directory is a static configuration directory.

## Run

No runtime execution. AI agent systems read skill files on-demand during development tasks.

## Test

Validation of `.agents/` content is performed through:

1. Structural consistency check against `skills/` directory
2. Content synchronization verification
3. Skill file integrity validation

```bash
# Verify structural consistency
diff -rq skills/ .agents/skills/
```

## Examples

### Loading a skill in an AI agent system

```python
# Example: AI agent loading kcm-code-quality-guardian skill
skill_path = ".agents/skills/kcm-code-quality-guardian/SKILL.md"
with open(skill_path, "r") as f:
    skill_instructions = f.read()
# Agent applies these instructions to code quality enforcement
```

### Verifying skill mirror consistency

```bash
# Verify all 16 skills are present
ls .agents/skills/ | wc -l
# Expected output: 16

# Verify content matches
diff -rq skills/ .agents/skills/
# Expected: no differences
```

## References

- `skills/` — Source directory that `.agents/skills/` mirrors
- `AGENTS.md` — Engineering constitution defining skill authority hierarchy
- `docs/agents/spesifikasi.md` — Technical specification for agents configuration