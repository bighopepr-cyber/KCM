---
name: kcm-documentation-guardian
description: Maintain documentation as the Single Source of Truth (SSOT) for KCM, ensuring all specifications are consistent, complete, and aligned with implementation
---

# Skill: Documentation Guardian

> Document ID: KCM-SKILL-011 | Version: 2.0.0 | Status: Active

## Overview

Maintain documentation as the Single Source of Truth (SSOT) for KCM, ensuring all specifications are consistent, complete, and aligned with implementation. Technical Writer / Specification Engineer role covering all documentation in docs/, all PRD files, README, and specification-code consistency.

## Mission

Every PRD requirement has a specification section, every specification matches its implementation, zero conflicting specifications, zero documentation duplication. Documentation is versioned, reviewed, and tested as code.

## Responsibilities

| # | Responsibility | Description |
|---|---------------|-------------|
| 1 | Specification Consistency | Ensure all specifications match PRD requirements and implementation |
| 2 | Conflict Detection | Identify and resolve conflicting specifications across documents |
| 3 | Gap Analysis | Find PRD requirements without specification coverage |
| 4 | Code-Spec Alignment | Verify type definitions, API signatures, and binary formats match code |
| 5 | Duplication Prevention | Ensure each fact appears in exactly one specification document |
| 6 | Metadata Validation | Ensure all documents have Document ID, Version, Status |
| 7 | Cross-Reference Validation | Verify all links resolve to existing files |
| 8 | Terminology Consistency | Ensure terminology matches KCM_GLOSSARY.md |

## Authority

| Priority | Authority Level | Blocking Authority | Approval Authority | Escalation |
|----------|----------------|-------------------|-------------------|------------|
| P11 | Documentation Authority | Can block undocumented changes | Documentation quality decisions | P1 (Orchestrator) |

## Scope

| In Scope | Out of Scope |
|----------|-------------|
| All documentation in docs/ directory | Writing production code (P10) |
| All PRD files (PRD, PRD2, PRD3, PRD-TESTING) | Architecture review (P5) |
| Specification-code consistency | Test writing (P9) |
| Documentation duplication detection | Security review (P7) |
| Conflict resolution between documents | Performance review (P8) |
| README accuracy | Code quality review (P10) |

## Non Goals

1. Write production implementation code
2. Review architecture or design patterns
3. Write test code
4. Implement security features
5. Create roadmap or marketing documents
6. Create unnecessary documentation

## Inputs

| Input | Source | Required |
|-------|--------|----------|
| All files in docs/ directory | Codebase | Yes |
| PRD files (PRD, PRD2, PRD3, PRD-TESTING) | `docs/specs/` | Yes |
| README.md | Root directory | Yes |
| Source code being documented | Codebase | Yes |
| Current audit status | `docs/KCM_DOCUMENT_AUDIT_REPORT.md` | Yes |

## Outputs

| Output | Format | Destination |
|--------|--------|-------------|
| Consistency report | Markdown | Engineering Report |
| Conflict resolution | Markdown | Engineering Report |
| Gap analysis | Table | Engineering Report |
| Documentation updates | Markdown | docs/ directory |

## Workflow

```
1. Read the PRD requirement
2. Locate the corresponding specification section
3. Verify the specification matches the PRD
4. Verify the implementation matches the specification
5. Verify tests validate the specification
6. Compare all PRD files for overlapping requirements
7. Compare PRD requirements with specification documents
8. Compare specification documents with implementation
9. Document any conflicts with source-of-truth resolution
10. List all PRD requirements and map to specs, code, and tests
11. Report missing mappings
```

## Decision Process

```
Documentation Change Requested
  ↓
Identify affected documents
  ↓
Check for conflicts with existing specs
  ├── Conflict found → Resolve by priority order
  │   1. PRD-TESTING-AND-BENCHMARK.md
  │   2. PRD3.md (Distributed, ML, security, compliance)
  │   3. PRD2.md (Storage, runtime, interfaces)
  │   4. PRD.md (Core types, storage, compute)
  │   5. SSOT.md (highest authority)
  └── No conflict ↓
Check for duplication
  ├── Duplication found → Consolidate into one document
  └── No duplication ↓
Check spec-code alignment
  ├── Mismatch found → Update spec to match code (or vice versa per SSOT)
  └── Aligned ↓
Update documentation
  ↓
Verify metadata (Document ID, Version, Status)
  ↓
PASS
```

## Validation

| Check | Method | Pass Criteria |
|-------|--------|--------------|
| PRD Traceability | Manual review | Every PRD requirement has spec section |
| Spec-Code Consistency | Manual review | Spec matches implementation |
| No Duplication | Grep/diff analysis | Each fact in exactly one document |
| No Conflicts | Cross-reference check | All documents agree |
| Coverage | Manual review | Every public API documented |
| Accuracy | Type comparison | Type definitions match code |
| Metadata | Document scan | All documents have ID, Version, Status |
| Cross-references | Link check | All links resolve |

## Quality Gates

- [ ] Every PRD requirement has a corresponding specification section
- [ ] Every specification matches its implementation
- [ ] Zero conflicting specifications
- [ ] Zero documentation duplication
- [ ] All public APIs documented
- [ ] All type definitions match between spec and code
- [ ] All documents have Document ID, Version, Status
- [ ] All cross-references resolve
- [ ] Terminology matches KCM_GLOSSARY.md
- [ ] No roadmap or marketing documents created

## Dependencies

| Skill | Dependency Type | Description |
|-------|----------------|-------------|
| kcm-architecture-guardian (P5) | Escalate | Architecture questions escalated |
| kcm-specification-lock (P4) | Coordinate | Spec lock validates contract changes |
| kcm-code-quality-guardian (P10) | Coordinate | Code quality review before doc update |
| kcm-engineering-decision-record (P15) | Escalate | Major decisions require EDR |

## Related Skills

| Skill | Relationship |
|-------|-------------|
| kcm-specification-lock (P4) | P4 protects frozen contracts; P11 ensures doc alignment |
| kcm-architecture-guardian (P5) | P5 validates architecture; P11 ensures doc consistency |
| kcm-release-readiness (P12) | P12 gates release; P11 validates documentation completeness |
| kcm-engineering-decision-record (P15) | P15 documents decisions; P11 ensures decision documentation |

## SSOT References

| Document | Section | Relevance |
|----------|---------|-----------|
| AGENTS.md | §8 Documentation Hierarchy | Document priority and authority |
| AGENTS.md | §16 Documentation Rules | Documentation requirements and standards |
| SSOT.md | Single Source of Truth | Highest authority for all documentation |
| docs/specs/PRD.md | Core Types | Primary specification document |
| docs/specs/PRD2.md | Storage, Runtime | Secondary specification document |
| docs/specs/PRD3.md | Distributed, ML, Security | Tertiary specification document |
| docs/specs/PRD-TESTING-AND-BENCHMARK.md | Testing Targets | Testing specification document |

## Failure Conditions

| Condition | Impact | Escalation |
|-----------|--------|------------|
| Specification-code mismatch | Blocks merge | Update spec or code |
| Conflicting specifications | Blocks merge | Resolve by priority order |
| Documentation duplication | Blocks merge | Consolidate documents |
| PRD requirement without spec | Blocks merge | Write specification |
| Public API without documentation | Blocks merge | Document API |
| Invalid cross-reference | Blocks merge | Fix or remove reference |

## Escalation

| Level | Path | SLA |
|-------|------|-----|
| Level 1 | Fix documentation issues internally | Immediate |
| Level 2 | Escalate to spec-lock (P4) or arch-guardian (P5) | 4 hours |
| Level 3 | Escalate to Engineering Orchestrator (P1) | 24 hours |
| Level 4 | SSOT.md is the final authority | As needed |

## Examples

See [examples/](./examples/) for documentation implementation examples.

## Checklist

See [checklists/](./checklists/) for documentation validation checklists.

## References

- [AGENTS.md](../../../AGENTS.md)
- [SSOT.md](../../../SSOT.md)
- [CONTRIBUTING.md](../../../CONTRIBUTING.md)
- [SECURITY.md](../../../SECURITY.md)
