# KCM Engineering Orchestrator

> Document ID: KCM-ENG-ORCH-001 | Version: 1.0.0 | Status: Active

## Overview

The KCM Engineering Orchestrator is the central coordination system for all AI Engineering activities in the KCM repository. It transforms tasks into structured, validated, documented, and auditable engineering workflows.

## Purpose

The Orchestrator ensures every engineering task follows a deterministic process governed by:
- AGENTS.md (Engineering Constitution)
- Authority System
- Decision Matrix
- Workflow definitions
- Skills Governance
- SSOT (Single Source of Truth)
- Documentation Governance
- Repository Structure
- CI/CD Rules
- Validation Rules

## Architecture

```
.engineering/
├── README.md                    # This file
├── ENGINE.md                    # Master orchestration engine specification
├── orchestrator/                # Core engine components
│   ├── README.md
│   ├── routing.md               # Skill routing logic
│   ├── execution-engine.md      # Task execution engine
│   ├── planning-engine.md       # Execution planning
│   ├── approval-engine.md       # Approval workflow
│   ├── conflict-engine.md       # Conflict resolution
│   ├── escalation-engine.md     # Escalation paths
│   ├── quality-engine.md        # Quality gates
│   ├── reporting-engine.md      # Report generation
│   └── state-machine.md         # Task state machine
├── pipelines/                   # Workflow pipelines
│   ├── standard.md
│   ├── feature.md
│   ├── bugfix.md
│   ├── optimization.md
│   ├── refactor.md
│   ├── documentation.md
│   ├── release.md
│   └── emergency.md
├── templates/                   # Output templates
├── examples/                    # Usage examples
├── checklists/                  # Validation checklists
└── validators/                  # Automated validators
```

## Quick Reference

### Task Types

| Type | Pipeline | Key Skills |
|------|----------|-----------|
| Feature | feature.md | P2,P3,P4,P5,P6/P7/P8,P9,P10,P11,P12 |
| Bug Fix | bugfix.md | P14,P10,P9,P12 |
| Optimization | optimization.md | P8,P6,P10,P9,P12 |
| Security Patch | emergency.md | P7,P4,P9,P12 |
| Documentation | documentation.md | P11,P4,P12 |
| Refactoring | refactor.md | P5,P10,P9,P11,P12 |
| Release | release.md | P12,P1 |

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

## References

- [AGENTS.md](../../AGENTS.md) — Engineering Constitution
- [ENGINE.md](ENGINE.md) — Master orchestration specification
- [skills/AUTHORITY-SYSTEM.md](../../skills/AUTHORITY-SYSTEM.md) — Authority system
- [skills/DECISION-MATRIX.md](../../skills/DECISION-MATRIX.md) — Decision matrix
- [skills/WORKFLOW.md](../../skills/WORKFLOW.md) — Workflow definitions
- [SSOT.md](../../SSOT.md) — Single Source of Truth
