# KCM Engineering Orchestrator

> Document ID: KCM-ENG-ORCH-001 | Version: 2.0.0 | Status: Active

## Overview

The KCM Engineering Orchestrator is the central coordination system for all AI Engineering activities in the KCM repository. It transforms tasks into structured, validated, documented, and auditable engineering workflows governed by AGENTS.md.

## Purpose

The Orchestrator ensures every engineering task follows a deterministic process governed by:
- **AGENTS.md** — Engineering Constitution
- **Authority System** — Skill authority and blocking power
- **Decision Matrix** — Skill routing by change type
- **Workflow** — Standard engineering process
- **Skills** — 16 AI engineering skills (P1-P16)
- **SSOT** — Single Source of Truth
- **Documentation Governance** — Documentation standards
- **Repository Structure** — File organization
- **CI/CD Rules** — Automation gates
- **Validation Rules** — Quality enforcement

## Architecture

```
.engineering/
├── README.md                    # This file — entry point
├── ENGINE.md                    # Master orchestration engine specification
├── orchestrator/                # Core engine components (10 engines)
│   ├── routing.md               # Skill routing logic
│   ├── execution-engine.md      # Task execution engine
│   ├── planning-engine.md       # Execution planning
│   ├── approval-engine.md       # Approval workflow
│   ├── conflict-engine.md       # Conflict resolution
│   ├── escalation-engine.md     # Escalation paths
│   ├── quality-engine.md        # Quality gates
│   ├── reporting-engine.md      # Report generation
│   ├── state-machine.md         # Task state machine
│   └── documentation-engine.md  # Documentation rules
├── pipelines/                   # Workflow pipelines (8 types)
│   ├── standard.md              # General tasks
│   ├── feature.md               # New features
│   ├── bugfix.md                # Bug fixes
│   ├── optimization.md          # Performance optimization
│   ├── refactor.md              # Code refactoring
│   ├── documentation.md         # Documentation changes
│   ├── release.md               # Version releases
│   └── emergency.md             # Critical fixes
├── templates/                   # Output templates (6)
│   ├── execution-plan-template.md
│   ├── impact-analysis-template.md
│   ├── quality-report-template.md
│   ├── completion-report-template.md
│   ├── approval-record-template.md
│   └── conflict-record-template.md
├── examples/                    # Usage examples (3)
│   ├── feature-example.md
│   ├── bugfix-example.md
│   └── release-example.md
├── checklists/                  # Validation checklists (4)
│   ├── feature-checklist.md
│   ├── bugfix-checklist.md
│   ├── release-checklist.md
│   └── security-checklist.md
└── validators/                  # Automated validators (2)
    ├── orchestrator-validator.md
    └── workflow-validator.md
```

## Quick Reference

### Task Types → Pipelines

| Task Type | Pipeline | Primary Skill | Key Skills |
|-----------|----------|---------------|------------|
| Feature | `feature.md` | P2 Planning | P3,P4,P5,P6/P7/P8,P9,P10,P11,P12 |
| Bug Fix | `bugfix.md` | P14 Debugging | P10,P9,P12 |
| Optimization | `optimization.md` | P8 Performance | P6,P10,P9,P12 |
| Security Patch | `emergency.md` | P7 Security | P4,P9,P12 |
| Documentation | `documentation.md` | P11 Docs | P4,P12 |
| Refactoring | `refactor.md` | P5 Architecture | P10,P9,P11,P12 |
| Release | `release.md` | P12 Release | P1 |
| Emergency | `emergency.md` | P14 Debugging | P10,P9,P12 |

### State Machine

```
NEW → PLANNED → ANALYZED → APPROVED → IMPLEMENTING → TESTING
  → BENCHMARKING → DOCUMENTING → VALIDATING → READY → COMPLETED
```

### Authority Flow

```
Task → P16 (Intelligence) → P2 (Planning) → P3 (Impact) → P4 (Spec)
  → P5 (Arch) → Domain → P10 (Quality) → P9 (Testing) → P8 (Perf)
  → P11 (Docs) → P13 (Review) → P12 (Release) → P1 (Orchestrator)
```

### Quality Gates

| Gate | Validator | Blocking |
|------|-----------|----------|
| Format | `cargo fmt --check` | Yes |
| Lint | `cargo clippy` | Yes |
| Build | `cargo build` | Yes |
| Unit Tests | `cargo test --lib` | Yes |
| Integration | `cargo test --test` | Yes |
| Property | `cargo test property` | Yes |
| Security | `cargo audit` | Yes |
| SSOT | `validate-ssot.sh` | Yes |
| Doc Coverage | `calculate-coverage.sh` | Yes |
| Doc Validation | `validate-docs.sh` | Yes |

## Usage

### For AI Agents

When receiving a task:

1. **Classify** the task type (Feature/BugFix/Security/Performance/Docs/Refactor/Release/Emergency)
2. **Route** to the correct pipeline using `orchestrator/routing.md`
3. **Plan** the execution using `orchestrator/planning-engine.md`
4. **Execute** the workflow using the selected pipeline
5. **Validate** using `orchestrator/quality-engine.md`
6. **Report** using `orchestrator/reporting-engine.md`

### For Human Engineers

1. Read `ENGINE.md` for the complete specification
2. Select the appropriate pipeline from `pipelines/`
3. Follow the checklist from `checklists/`
4. Use templates from `templates/` for reports
5. Reference examples from `examples/` for guidance

## References

- [AGENTS.md](../../AGENTS.md) — Engineering Constitution
- [ENGINE.md](ENGINE.md) — Master orchestration specification
- [skills/AUTHORITY-SYSTEM.md](../../skills/AUTHORITY-SYSTEM.md) — Authority system
- [skills/DECISION-MATRIX.md](../../skills/DECISION-MATRIX.md) — Decision matrix
- [skills/WORKFLOW.md](../../skills/WORKFLOW.md) — Workflow definitions
- [SSOT.md](../../SSOT.md) — Single Source of Truth
