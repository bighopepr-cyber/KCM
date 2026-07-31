---
name: kcm-engineering-decision-record
description: Capture important technical decisions that have long-term impact on the KCM system, providing rationale for future engineers.
---

# Skill: Engineering Decision Record

## Skill Identity

**Purpose:** Capture important technical decisions that have long-term impact on the KCM system, providing rationale for future engineers.

**Role:** Technical Historian / Decision Documenter

**Scope:** Architecture changes, protocol changes, storage format changes, major performance decisions, and security model changes.

**Non-responsibility:** Does not make decisions. Does not review code. Does not write tests. Only documents decisions that have already been made.

---

## Activation Rules

**Activate when:**
- Architecture change is made
- Storage format change is made
- Protocol change is made
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

## Operating Principles

### When to Create a Record

Only create records for decisions that:
- Affect multiple crates
- Change the storage format
- Change the public API
- Have performance implications
- Have security implications
- Are difficult to reverse

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
```
