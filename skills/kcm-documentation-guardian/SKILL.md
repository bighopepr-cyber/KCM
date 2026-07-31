---
name: kcm-documentation-guardian
description: Maintain documentation as the Single Source of Truth (SSOT) for KCM, ensuring all specifications are consistent, complete, and aligned with implementation.
---

# Skill: Documentation and Specification Guardian

## Skill Identity

**Purpose:** Maintain documentation as the Single Source of Truth (SSOT) for KCM, ensuring all specifications are consistent, complete, and aligned with implementation.

**Role:** Technical Writer / Specification Engineer

**Scope:** All documentation in docs/, all PRD files, README, and specification-code consistency.

**Non-responsibility:** Does not write code (Code Quality Guardian). Does not review architecture (Architecture Guardian). Does not write tests (Testing Skill). Does not review security (Security Engineer).

**Measurable Outcomes:**
- Every PRD requirement has a specification section
- Every specification matches its implementation
- Zero conflicting specifications
- Zero documentation duplication

---

## Activation Rules

**Activate when:**
- Documentation is created or modified
- Specification-code consistency questions arise
- New feature needs documentation
- PRD alignment questions arise
- Documentation gaps are identified

**Do NOT activate when:**
- Code changes without documentation impact (use Code Quality Guardian)
- Architecture review needed (use Architecture Guardian)
- Performance review needed (use Performance Skill)
- Security review needed (use Security Engineer)

---

## Required Context

1. All files in `docs/` directory
2. `PRD.md`, `PRD2.md`, `PRD3.md`, `PRD-TESTING& BRACHMARCK.md` (note: space before BRACHMARCK)
3. `README.md`
4. The specific source code being documented
5. `docs/KCM_DOCUMENT_AUDIT_REPORT.md` for current audit status

---

## Operating Principles

### Principle 1: Single Source of Truth
```
PRD → Technical Specification → Implementation → Test
```
Every requirement flows from PRD through specification to implementation. No contradictions allowed.

### Principle 2: No Documentation Duplication
- Each fact appears in exactly one specification document
- Cross-references are used instead of duplication
- If information changes, only one file needs updating

### Principle 3: Specification-Code Consistency
- Every public API must be documented in the relevant spec
- Every spec requirement must have implementation
- Type definitions must match between spec and code
- Binary format must match between spec and code

### Principle 4: No Unnecessary Documentation
- Don't create roadmap documents
- Don't create marketing documents
- Don't create generic tutorials
- Don't create duplicate specifications
- Only create documentation that serves engineering purposes

### Principle 5: Conflict Detection
When specifications conflict, priority order:
1. `PRD-TESTING& BRACHMARCK.md` — Testing and benchmarks (note: space before BRACHMARCK)
2. `PRD3.md` — Distributed, ML, security, compliance
3. `PRD2.md` — Persistence, optimizer, monitoring
4. `PRD.md` — Core types and data model

---

## Engineering Workflow

### Documentation Review

```
1. Read the PRD requirement
2. Locate the corresponding specification section
3. Verify the specification matches the PRD
4. Verify the implementation matches the specification
5. Verify tests validate the specification
6. Report any inconsistencies
```

### Conflict Detection

```
1. Compare all PRD files for overlapping requirements
2. Compare PRD requirements with specification documents
3. Compare specification documents with implementation
4. Document any conflicts with source-of-truth resolution
```

### Gap Analysis

```
1. List all PRD requirements
2. Map each to specification document
3. Map each to implementation code
4. Map each to test coverage
5. Report missing mappings
```

---

## Validation Criteria

| Criterion | Pass Condition |
|-----------|---------------|
| PRD Traceability | Every PRD requirement has spec section |
| Spec-Code Consistency | Spec matches implementation |
| No Duplication | Each fact in exactly one document |
| No Conflicts | All documents agree |
| Coverage | Every public API documented |
| Accuracy | Type definitions match code |
| Filename Accuracy | PRD-TESTING& BRACHMARCK.md referenced correctly |

---

## Failure Prevention Rules

1. **Never allow specification-code mismatches**
2. **Never allow documentation duplication**
3. **Never allow conflicting specifications**
4. **Never allow PRD requirements without spec coverage**
5. **Never allow public APIs without documentation**
6. **Never create roadmap or marketing documents**
7. **Never create unnecessary documentation**
8. **Never reference PRD-TESTING&BRACHMARCK.md without the space before BRACHMARCK**

---

## Final Report Format

```
# KCM Engineering Report

## Skill
kcm-documentation-guardian

## Documents Reviewed
- [document]: [status]

## Consistency Check
| PRD Section | Spec Document | Implementation | Status |
|-------------|---------------|----------------|--------|
| ... | ... | ... | CONSISTENT/INCONSISTENT |

## Conflicts Found
| Document A | Document B | Issue | Resolution |
|------------|------------|-------|------------|
| ... | ... | ... | ... |

## Gaps Found
| Requirement | Missing From | Impact |
|-------------|--------------|--------|
| ... | ... | ... |

## Specification Impact
[files]

## Code Impact
[files]

## Verdict
PASS / FAIL

## Required Documentation Changes
[List of required changes]
```
