# KCM Engineering Authority System

> Document ID: KCM-AUTH-001 | Version: 2.0.0 | Status: Active

## Overview

The KCM Engineering Authority System defines the decision-making authority, blocking power, approval rights, and escalation paths for all 16 engineering skills. This system ensures deterministic, auditable, and SSOT-compliant governance.

## Authority Types

| Type | Description | Skills |
|------|-------------|--------|
| Override | Can override any skill decision | P1 |
| Veto | Can block contract changes | P4 |
| Block | Can block specific categories | P2,P3,P5,P6,P7,P8,P9,P10,P11,P12 |
| Feedback | Advisory only, no blocking power | P13,P14,P15,P16 |

## Authority Matrix

| Priority | Skill | Authority | Can Block | Can Approve | Escalates To |
|----------|-------|-----------|-----------|-------------|-------------|
| P1 | Engineering Orchestrator | Override | Everything | Everything | SSOT.md |
| P2 | Task Planner | Block | Implementation without plan | Plans | P1 |
| P3 | Change Impact Analysis | Block | Unassessed changes | Impact reports | P1 |
| P4 | Specification Lock | Veto | Format/API/FFI changes | Contract changes | P1 |
| P5 | Architecture Guardian | Block | Architecture violations | Architecture decisions | P1 |
| P6 | Database Specialist | Block | Storage/query changes | Storage decisions | P5 |
| P7 | Security Engineer | Block | Security/compliance violations | Security decisions | P1 |
| P8 | Performance Engineer | Block | Performance regressions | Performance decisions | P1 |
| P9 | Testing Verification | Block | Changes without tests | Test decisions | P10 |
| P10 | Code Quality Guardian | Block | Code quality issues | Quality decisions | P1 |
| P11 | Documentation Guardian | Block | Undocumented changes | Doc decisions | P1 |
| P12 | Release Readiness | Block | Releases | Release decisions | P1 |
| P13 | Code Review Auditor | Feedback | — | — | P1 |
| P14 | Debugging Root Cause | Feedback | — | — | P1 |
| P15 | Engineering Decision Record | Feedback | — | — | P1 |
| P16 | Repository Intelligence | Feedback | — | — | P1 |

## Conflict Resolution Rules

| Scenario | Resolution |
|----------|-----------|
| Two skills disagree | Higher priority wins |
| Same priority, different domain | Domain authority wins |
| Same priority, same domain | Engineering Orchestrator (P1) decides |
| Security vs Performance | Security wins (P7 > P8) |
| Security vs Functionality | Security wins (P7 > any feature) |
| Performance vs Correctness | Correctness wins (per philosophy) |

## Escalation Paths

### Level 1: Internal Resolution
- Skill resolves within its domain
- SLA: 1 hour

### Level 2: Higher Priority
- Escalate to next higher priority skill
- SLA: 4 hours

### Level 3: Orchestrator
- Escalate to Engineering Orchestrator (P1)
- SLA: 24 hours

### Level 4: SSOT
- SSOT.md is the final authority
- No SLA — definitive resolution

## Decision Recording

All decisions must be recorded with:
- Decision ID
- Date
- Skill(s) involved
- Decision made
- Rationale
- SSOT reference
- Escalation path used (if any)
