# Orchestrator Engines

> Document ID: KCM-ORCH-README-001 | Version: 2.0.0

Core engine components of the KCM Engineering Orchestrator.

## Engines

| Engine | File | Purpose | Authority |
|--------|------|---------|-----------|
| Skill Router | `routing.md` | Selects skills based on change type | — |
| Execution Engine | `execution-engine.md` | Executes engineering workflow | — |
| Planning Engine | `planning-engine.md` | Creates execution plans | P2 |
| Approval Engine | `approval-engine.md` | Manages approval chains | P1 |
| Conflict Engine | `conflict-engine.md` | Resolves skill conflicts | P1 |
| Escalation Engine | `escalation-engine.md` | Handles escalations | P1 |
| Quality Engine | `quality-engine.md` | Enforces quality gates | P10 |
| Reporting Engine | `reporting-engine.md` | Generates reports | — |
| State Machine | `state-machine.md` | Manages task states | — |
| Documentation Engine | `documentation-engine.md` | Manages doc updates | P11 |

## Engine Interaction

```
Task Input
    ↓
[Routing] → Selects pipeline and skills
    ↓
[Planning] → Creates execution plan
    ↓
[State Machine] → Tracks state: NEW → PLANNED
    ↓
[Approval] → Manages approval chain
    ↓
[State Machine] → Tracks state: PLANNED → ANALYZED → APPROVED
    ↓
[Execution] → Executes workflow phases
    ↓
[Quality] → Enforces quality gates
    ↓
[Documentation] → Manages doc updates
    ↓
[Reporting] → Generates reports
    ↓
[State Machine] → Tracks state: IMPLEMENTING → TESTING → ... → COMPLETED
```

## References

- [ENGINE.md](../ENGINE.md) — Master specification
- [AGENTS.md](../../AGENTS.md) — Engineering Constitution
- [skills/WORKFLOW.md](../../skills/WORKFLOW.md) — Workflow definitions
