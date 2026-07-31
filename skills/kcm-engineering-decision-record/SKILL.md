---
name: kcm-engineering-decision-record
description: Capture important technical decisions that have long-term impact on the KCM system, providing rationale for future engineers.
---

# Skill: Engineering Decision Record

## Skill Identity

**Purpose:** Capture important technical decisions that have long-term impact on the KCM system, providing rationale for future engineers.

**Role:** Technical Historian / Decision Documenter

**Scope:** Architecture changes, protocol changes, storage format changes, major performance decisions, and security model changes.

**Non-responsibility:** Does not make decisions. Does not review code. Does not write tests. Only documents decisions that have already been made. Does not validate architecture (Architecture Guardian). Does not review code quality (Code Quality Guardian).

**Measurable Outcomes:**
- Every significant decision has a documented EDR
- Every EDR includes context, decision, consequences, and alternatives
- Every EDR references relevant PRD sections
- No undocumented architectural decisions

---

## Activation Rules

**Activate when:**
- Architecture change is made
- Storage format change is made
- Protocol change is made (including gRPC proto changes)
- Major performance decision is made
- Security model change is made
- Breaking change is approved

**Do NOT activate when:**
- Routine bug fixes
- Minor code improvements
- Test additions
- Documentation updates
- Dependency version bumps (unless breaking)

---

## Required Context

1. The decision that was made
2. The PRD sections relevant to the decision
3. The specification documents affected
4. The alternatives that were considered

---

## Crate Awareness

Decisions may affect any of the **13 crates**: kcm-core, kcm-storage, kcm-compute, kcm-reasoning, kcm-optimizer, kcm-runtime, kcm-interface, kcm-distributed, kcm-ml, kcm-security, kcm-compliance, kcm-testing, kcm-server.

---

## Operating Principles

### When to Create a Record

Only create records for decisions that:
- Affect multiple crates
- Change the storage format
- Change the public API
- Have performance implications
- Have security implications
- Are difficult to reverse
- Change gRPC proto definitions

### When NOT to Create a Record

Do NOT create records for:
- Routine implementation choices
- Bug fixes
- Test additions
- Minor refactoring
- Dependency updates

### Record Format

Each record must include:
- Context: Why was this decision needed?
- Decision: What was decided?
- Consequences: What are the implications?
- Alternatives: What was considered and rejected?

---

## Final Report Format

```
# Engineering Decision Record: [Title]

**Date:** YYYY-MM-DD
**Status:** Accepted
**Deciders:** [who was involved]

## Context
[Why was this decision needed?]

## Decision
[What was decided?]

## Consequences
### Positive
- [benefit 1]
- [benefit 2]

### Negative
- [tradeoff 1]
- [tradeoff 2]

### Risks
- [risk 1]
- [risk 2]

## Alternatives Considered
### Alternative 1: [name]
[Why was it rejected?]

### Alternative 2: [name]
[Why was it rejected?]

## References
- [PRD section]
- [Specification section]

## Affected Crates
- [list of crates affected]
```
