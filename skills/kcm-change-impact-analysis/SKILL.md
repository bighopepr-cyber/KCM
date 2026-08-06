# Change Impact Analysis

> Document ID: KCM-SKILL-003 | Version: 2.0.0 | Status: Active

## Overview

The Change Impact Analysis skill analyzes the impact of proposed changes before implementation, identifying all affected modules, specifications, compatibility requirements, and testing needs. It produces a complete impact matrix covering direct, indirect, specification, test, compatibility, and migration impacts.

## Mission

Produce comprehensive impact assessments for every proposed change, ensuring all ripple effects are identified before implementation begins, preventing unexpected breakage across the 13-crate workspace.

## Responsibilities

| # | Responsibility | Description |
|---|---------------|-------------|
| 1 | Direct Impact Analysis | Identify all files and modules directly changed |
| 2 | Indirect Impact Analysis | Trace dependencies to find indirectly affected files |
| 3 | Specification Impact | Identify specification documents requiring updates |
| 4 | Test Impact | Identify tests that need updating or creation |
| 5 | Compatibility Impact | Assess backward compatibility implications |
| 6 | Migration Impact | Determine data migration requirements |
| 7 | Risk Assessment | Rate impacts as Low/Medium/High/Critical |
| 8 | Effort Estimation | Estimate implementation, testing, and documentation effort |
| 9 | SSOT Impact | Map changes to all affected SSOT requirements |

## Authority

| Attribute | Value |
|-----------|-------|
| Priority | P3 |
| Authority Level | Block |
| Blocking Authority | Can block changes with unassessed or high-risk impact |
| Approval Authority | Can approve impact assessments and recommend proceed/needs-analysis/blocked |
| Escalation | Engineering Orchestrator (P1) |

## Scope

| In Scope | Out of Scope |
|----------|-------------|
| All code changes across 13 crates | Making go/no-go decisions (Architecture Guardian decides) |
| Dependency tracing across crate boundaries | Implementing changes |
| Specification impact assessment | Writing tests |
| Backward compatibility analysis | Reviewing code quality |
| Migration requirement identification | Security review (Security Engineer handles) |
| Effort estimation | Architecture decisions |

## Non Goals

1. Implementing changes — domain specialists handle this
2. Writing tests — Testing Verification (P9) handles this
3. Making go/no-go decisions — Architecture Guardian (P5) handles this
4. Reviewing code quality — Code Quality Guardian (P10) handles this
5. Reviewing security — Security Engineer (P7) handles this
6. Designing architecture — Architecture Guardian (P5) handles this

## Inputs

| Input | Source | Required |
|-------|--------|----------|
| Proposed change description | User / Task Planner (P2) | Yes |
| Crate dependency graph | Workspace Cargo.toml | Yes |
| Specification documents | Specification Lock (P4) | Yes |
| Existing test coverage | Testing Verification (P9) | Yes |
| Codebase structure | Repository Intelligence (P16) | Yes |

## Outputs

| Output | Format | Destination |
|--------|--------|-------------|
| Impact Assessment Report | Structured markdown | Orchestrator / Task Planner |
| Direct impact matrix | File table | Implementation skills |
| Indirect impact matrix | Dependency table | Architecture Guardian (P5) |
| Specification impact list | Document table | Documentation Guardian (P11) |
| Test impact list | Test table | Testing Verification (P9) |
| Compatibility assessment | YES/NO with details | Specification Lock (P4) |
| Risk assessment | Risk matrix | Orchestrator (P1) |
| Effort estimate | Hour breakdown | Orchestrator (P1) |

## Workflow

```
1. Understand the proposed change
2. Identify directly affected files
3. Trace dependencies to find indirectly affected files
4. Check specification documents for alignment
5. Identify tests that need updating
6. Assess backward compatibility
7. Determine migration requirements
8. Estimate effort
9. Rate risk level
10. Produce Impact Assessment Report
```

## Decision Process

```
Change Proposed → File Identification → Dependency Tracing → Spec Check → Test Check → Compatibility Assessment → Risk Rating → Report
```

## Validation

| Check | Method | Pass Criteria |
|-------|--------|--------------|
| Direct impact complete | File scan | All directly affected files listed |
| Indirect impact traced | Dependency graph | All downstream dependents identified |
| Specification impact assessed | Spec search | All affected specs listed with required updates |
| Test impact assessed | Test scan | All affected tests identified |
| Compatibility evaluated | API review | Breaking changes catalogued with migration paths |
| Migration requirements identified | Format review | Data migration needs documented |
| Risk rated | Risk matrix | All significant risks rated |
| Effort estimated | Breakdown | Implementation, testing, doc hours estimated |

## Quality Gates

- [ ] All directly affected files identified
- [ ] All indirectly affected files traced through dependencies
- [ ] All affected specification documents listed
- [ ] All affected tests identified
- [ ] Backward compatibility assessed
- [ ] Migration requirements documented
- [ ] Risk assessment complete with ratings
- [ ] Effort estimate provided
- [ ] SSOT requirement mapping complete
- [ ] Impact assessment approved before implementation begins

## Dependencies

| Skill | Dependency Type | Description |
|-------|----------------|-------------|
| kcm-task-planner (P2) | Upstream | Provides the task plan with proposed changes |
| kcm-repository-intelligence (P16) | Upstream | Provides codebase structure and dependencies |
| kcm-specification-lock (P4) | Upstream | Provides specification documents for alignment check |
| kcm-architecture-guardian (P5) | Downstream | Receives impact assessment for architecture validation |
| kcm-testing-verification (P9) | Downstream | Receives test impact list for test planning |
| kcm-documentation-guardian (P11) | Downstream | Receives specification impact list for doc updates |

## Related Skills

| Skill | Relationship |
|-------|-------------|
| kcm-task-planner (P2) | P2 produces the plan; P3 assesses its impact |
| kcm-architecture-guardian (P5) | P5 uses impact assessment for architecture decisions |
| kcm-engineering-orchestrator (P1) | P1 coordinates impact analysis through gates |
| kcm-specification-lock (P4) | P4 provides specs and receives compatibility assessment |
| kcm-testing-verification (P9) | P9 receives test impact list |

## SSOT References

| Document | Section | Relevance |
|----------|---------|-----------|
| SSOT.md | — | Single Source of Truth |
| AGENTS.md | Section 6 | Repository Structure Rules |
| AGENTS.md | Section 10 | Change Management |
| AGENTS.md | Section 11 | Engineering Workflow |
| AGENTS.md | Section 25 | Skill Governance |

## Failure Conditions

| Condition | Impact | Escalation |
|-----------|--------|------------|
| Incomplete impact matrix | Unexpected breakage | Orchestrator (P1) blocks implementation |
| Missing indirect impacts | Cascading failures | Re-analysis required |
| Incomplete spec impact | Documentation drift | Specification Lock (P4) intervenes |
| Underestimated risk | Production issues | Orchestrator (P1) escalates to higher review |
| Missing migration path | Data loss risk | Database Engine Specialist (P6) intervenes |

## Escalation

| Level | Path | SLA |
|-------|------|-----|
| 1 | Skill internal | 1 hour |
| 2 | Higher priority skill | 4 hours |
| 3 | Engineering Orchestrator (P1) | 24 hours |
| 4 | SSOT.md | Final authority |

## Crate Awareness

```
kcm-core          → Types, DenseVec, Bitmap, Dictionary
kcm-storage       → Columns, Codecs, WAL, FileFormat, Index, Backup, Recovery, Errors, DictCodec
kcm-compute       → Algebra operators, SIMD AVX2
kcm-reasoning     → Rules, Forward-chaining inference
kcm-optimizer     → Cost model, Planner, Statistics, Rewriting, Adaptive
kcm-runtime       → Database, Transactions, Metrics, Health, Executor
kcm-interface     → C FFI, Python, REST, KQL parser
kcm-distributed   → Sharding, 2PC Coordinator
kcm-ml            → Learned Index, Confidence Learner, Rule Discovery
kcm-security      → RBAC, AES-256-GCM encryption, Audit Log
kcm-compliance    → GDPR Manager, Data Classification
kcm-testing       → Load/Stress/Security/Recovery test infrastructure, Metrics Dashboard
kcm-server        → gRPC server, gRPC main, main entry point
```

**Dependency flow:**
```
core → storage → compute/reasoning/optimizer/distributed/ml → runtime → interface → server
```

## Impact Categories

| Category | Description |
|----------|-------------|
| Direct Impact | Files and modules directly changed |
| Indirect Impact | Files and modules that depend on changed code |
| Specification Impact | Specification documents that need updating |
| Test Impact | Tests that need updating or creation |
| Compatibility Impact | Backward compatibility implications |
| Migration Impact | Data migration requirements |

## Final Report Format

```
# KCM Engineering Report

## Skill
kcm-change-impact-analysis

## Proposed Change
[Description of the change]

## Direct Impact
| File | Change Type | Description |
|------|-------------|-------------|
| ... | Modify/Add/Delete | ... |

## Indirect Impact
| File | Reason | Required Change |
|------|--------|-----------------|
| ... | Depends on ... | ... |

## Specification Impact
| Document | Section | Required Update |
|----------|---------|-----------------|
| ... | ... | ... |

## Test Impact
| Test File | Required Change |
|-----------|-----------------|
| ... | ... |

## Compatibility Impact
- Backward compatible: YES/NO
- Breaking changes: [list]
- Migration required: YES/NO

## Effort Estimate
- Implementation: [hours]
- Testing: [hours]
- Documentation: [hours]
- Total: [hours]

## Risk Assessment
| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| ... | Low/Med/High | Low/Med/High | ... |

## Recommendation
PROCEED / NEEDS MORE ANALYSIS / BLOCKED
```

## SSOT-First Impact Protocol

Every change impact analysis MUST:

1. **Identify SSOT Requirements** — Map change to all affected SSOT requirements
2. **Check Document Hierarchy** — Verify no higher-priority document conflicts
3. **Assess Backward Compatibility** — Identify breaking changes
4. **Map Affected Components** — List all crates, modules, APIs, tests affected
5. **Estimate Risk** — Rate impact as Low/Medium/High/Critical
6. **Recommend Mitigation** — Suggest how to minimize impact

## Activation Rules

**Activate when:**
- Major feature is planned
- Storage format change is proposed
- Public API change is proposed
- Cross-crate change is planned
- Breaking change is considered

**Do NOT activate when:**
- Bug fix within single module (use Code Quality Guardian)
- Test-only changes (use Testing Skill)
- Documentation-only changes (use Documentation Guardian)
- Architecture decisions needed (use Architecture Guardian)

## Examples

See [examples/](examples/) for usage examples.

## Checklist

See [checklists/](checklists/) for validation checklists.

## References

- [SSOT.md](../../SSOT.md)
- [AGENTS.md](../../AGENTS.md)
- [CONTRIBUTING.md](../../CONTRIBUTING.md)
- [SECURITY.md](../../SECURITY.md)
