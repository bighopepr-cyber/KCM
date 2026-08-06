---
name: kcm-engineering-decision-record
description: Capture important technical decisions that have long-term impact on the KCM system, providing rationale for future engineers
---

# Skill: Engineering Decision Record

> Document ID: KCM-SKILL-015 | Version: 2.0.0 | Status: Active

## Overview

Capture important technical decisions that have long-term impact on the KCM system, providing rationale for future engineers. Technical Historian / Decision Documenter role covering architecture changes, protocol changes, storage format changes, major performance decisions, and security model changes.

## Mission

Every significant decision has a documented EDR, every EDR includes context, decision, consequences, and alternatives, every EDR references relevant PRD sections, no undocumented architectural decisions.

## Responsibilities

| # | Responsibility | Description |
|---|---------------|-------------|
| 1 | Decision Documentation | Record significant technical decisions with full context |
| 2 | Rationale Capture | Document why the decision was made and alternatives considered |
| 3 | Consequence Assessment | Document positive, negative, and risk implications |
| 4 | Affected Crates Mapping | Identify all crates impacted by the decision |
| 5 | SSOT Impact Analysis | Determine which SSOT documents are affected |
| 6 | Alternatives Documentation | Record alternatives considered and reasons for rejection |
| 7 | Reference Linking | Link decisions to relevant PRD and specification sections |

## Authority

| Priority | Authority Level | Blocking Authority | Approval Authority | Escalation |
|----------|----------------|-------------------|-------------------|------------|
| P15 | Decision Authority | Advisory only (no blocking) | Decision documentation decisions | P1 (Orchestrator) |

## Scope

| In Scope | Out of Scope |
|----------|-------------|
| Architecture changes | Routine bug fixes |
| Storage format changes | Minor code improvements |
| Protocol changes (including gRPC proto) | Test additions |
| Major performance decisions | Documentation updates |
| Security model changes | Dependency version bumps (unless breaking) |
| Breaking change approvals | Implementation decisions |

## Non Goals

1. Make decisions (only documents decisions already made)
2. Review code
3. Write tests
4. Validate architecture (defers to P5)
5. Review code quality (defers to P10)
6. Implement features

## Inputs

| Input | Source | Required |
|-------|--------|----------|
| Decision that was made | Team/stakeholders | Yes |
| Relevant PRD sections | docs/specs/ | Yes |
| Affected specification documents | docs/ | Yes |
| Alternatives that were considered | Decision context | Yes |

## Outputs

| Output | Format | Destination |
|--------|--------|-------------|
| Engineering Decision Record | Markdown | docs/adr/ directory |
| Decision summary | Markdown | Engineering Report |
| Affected crates list | Table | Engineering Report |

## Workflow

```
1. Decision identified as significant
2. Determine if EDR is required:
   a. Affects multiple crates? → Yes
   b. Changes storage format? → Yes
   c. Changes public API? → Yes
   d. Has performance implications? → Yes
   e. Has security implications? → Yes
   f. Difficult to reverse? → Yes
   g. Changes gRPC proto definitions? → Yes
3. Identify affected SSOT documents
4. Document context: Why was this decision needed?
5. Document decision: What was decided?
6. Document consequences: Positive, negative, risks
7. Document alternatives: What was considered and rejected?
8. Link to relevant PRD and specification sections
9. Identify affected crates
10. Create EDR document in docs/adr/
```

## Decision Process

```
Significant Decision Made
  ↓
Is EDR required?
  ├── Routine bug fix → No EDR needed
  ├── Minor code improvement → No EDR needed
  ├── Test addition → No EDR needed
  └── Significant change → Yes ↓
Create EDR with:
  - Context (why needed)
  - Decision (what decided)
  - Consequences (positive, negative, risks)
  - Alternatives (considered and rejected)
  - References (PRD, spec sections)
  - Affected crates
  ↓
Store in docs/adr/
  ↓
Update Engineering Report
```

## Validation

| Check | Method | Pass Criteria |
|-------|--------|--------------|
| Decision significance | Review criteria | Meets EDR requirements |
| Context documented | EDR review | Clear explanation of why |
| Consequences assessed | EDR review | Positive, negative, risks documented |
| Alternatives documented | EDR review | Rejected alternatives with reasons |
| PRD references | EDR review | Relevant PRD sections linked |
| Affected crates | EDR review | All impacted crates listed |
| SSOT impact | EDR review | Affected SSOT documents identified |

## Quality Gates

- [ ] Decision meets EDR criteria (multi-crate, format change, API change, performance, security, irreversible, proto change)
- [ ] Context clearly explains why the decision was needed
- [ ] Decision clearly states what was decided
- [ ] Consequences documented (positive, negative, risks)
- [ ] Alternatives considered and rejected with reasons
- [ ] Relevant PRD and specification sections referenced
- [ ] Affected crates listed
- [ ] SSOT impact assessed
- [ ] EDR stored in docs/adr/

## Dependencies

| Skill | Dependency Type | Description |
|-------|----------------|-------------|
| kcm-architecture-guardian (P5) | Coordinate | Architecture decisions require EDR |
| kcm-specification-lock (P4) | Coordinate | Spec changes require EDR |
| kcm-documentation-guardian (P11) | Coordinate | Doc alignment with EDR |
| kcm-engineering-orchestrator (P1) | Escalate | Major decisions escalated |

## Related Skills

| Skill | Relationship |
|-------|-------------|
| kcm-architecture-guardian (P5) | P5 validates architecture; P15 documents decisions |
| kcm-specification-lock (P4) | P4 protects contracts; P15 documents contract changes |
| kcm-documentation-guardian (P11) | P11 ensures doc consistency; P15 provides decision rationale |
| kcm-engineering-orchestrator (P1) | P1 coordinates; P15 provides decision documentation |

## SSOT References

| Document | Section | Relevance |
|----------|---------|-----------|
| AGENTS.md | §5 Repository Constitution | Frozen contracts requiring EDR |
| AGENTS.md | §10 Change Management | Change categories requiring EDR |
| SSOT.md | Architecture Decisions | Decision documentation requirements |
| docs/adr/ | All ADR files | Existing decision records |

## Failure Conditions

| Condition | Impact | Escalation |
|-----------|--------|------------|
| Significant decision undocumented | Knowledge loss | Create EDR |
| EDR missing consequences | Incomplete documentation | Update EDR |
| EDR missing alternatives | Incomplete documentation | Update EDR |
| EDR missing PRD references | Poor traceability | Add references |
| EDR missing affected crates | Poor impact assessment | Add crate list |

## Escalation

| Level | Path | SLA |
|-------|------|-----|
| Level 1 | Create EDR internally | Immediate |
| Level 2 | Escalate to arch-guardian (P5) or spec-lock (P4) | 4 hours |
| Level 3 | Escalate to Engineering Orchestrator (P1) | 24 hours |
| Level 4 | SSOT.md is the final authority | As needed |

## Examples

See [examples/](./examples/) for engineering decision record examples.

## Checklist

See [checklists/](./checklists/) for engineering decision record checklists.

## References

- [AGENTS.md](../../../AGENTS.md)
- [SSOT.md](../../../SSOT.md)
- [CONTRIBUTING.md](../../../CONTRIBUTING.md)
- [SECURITY.md](../../../SECURITY.md)
