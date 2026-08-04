---
name: kcm-change-impact-analysis
description: Analyze the impact of proposed changes before implementation, identifying affected modules, specifications, compatibility requirements, and testing needs.
---

# Skill: Change Impact Analysis

## Skill Identity

**Purpose:** Analyze the impact of proposed changes before implementation, identifying affected modules, specifications, compatibility requirements, and testing needs.

**Role:** Senior Architect / Impact Analyst

**Scope:** Pre-implementation analysis of all code changes, identifying ripple effects across the system.

**Non-responsibility:** Does not implement changes. Does not write tests. Does not make go/no-go decisions (defers to Architecture Guardian). Does not review code quality (Code Quality Guardian). Does not review security (Security Engineer).

**Measurable Outcomes:**
- Every proposed change has a complete impact matrix
- All affected files identified before implementation
- All specification documents identified for updates
- All breaking changes catalogued with migration paths

---

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

---

## Required Context

1. The proposed change description
2. The crate dependency graph (workspace Cargo.toml) — **13 crates**
3. The specification documents for affected components
4. The existing test coverage for affected components

---

## Crate Awareness

The workspace contains **13 crates** organized as:

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

---

## Operating Principles

### Impact Categories

**Direct Impact:** Files and modules directly changed
**Indirect Impact:** Files and modules that depend on changed code
**Specification Impact:** Specification documents that need updating
**Test Impact:** Tests that need updating or creation
**Compatibility Impact:** Backward compatibility implications
**Migration Impact:** Data migration requirements

### Impact Analysis Matrix

```
Change → Direct Impact → Indirect Impact → Spec Impact → Test Impact → Compatibility → Migration
```

---

## Engineering Workflow

### Analysis Process

```
1. Understand the proposed change
2. Identify directly affected files
3. Trace dependencies to find indirectly affected files
4. Check specification documents for alignment
5. Identify tests that need updating
6. Assess backward compatibility
7. Determine migration requirements
8. Estimate effort
9. Report findings
```

---

## Files Validated

All source files across all 13 crates are within scope for impact analysis.

---

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

1. **Identify SSOT Requirements**: Map change to all affected SSOT requirements
2. **Check Document Hierarchy**: Verify no higher-priority document conflicts
3. **Assess Backward Compatibility**: Identify breaking changes
4. **Map Affected Components**: List all crates, modules, APIs, tests affected
5. **Estimate Risk**: Rate impact as Low/Medium/High/Critical
6. **Recommend Mitigation**: Suggest how to minimize impact
