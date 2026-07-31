---
name: kcm-task-planner
description: Prevent uncontrolled coding by requiring structured task analysis before implementation
---

# Skill: Task Planner

## Purpose

Prevent uncontrolled coding. Before any implementation, the agent must produce a structured task plan that identifies requirements, affected files, specifications, risks, and testing strategy.

## Activation Rules

**Activate when:**
- Any new feature is requested
- Any bug fix that affects more than one file
- Any refactoring task
- Any performance optimization
- Any security-related change

**Do NOT activate when:**
- Single-line typo fix
- Comment-only changes
- Formatting-only changes
- Running existing commands

## Responsibilities

This skill controls:
- Task decomposition
- Requirement analysis
- File impact identification
- Specification mapping
- Risk assessment
- Testing strategy

## Required Inspection

Before producing a plan:
1. Read the user's request completely
2. Search the codebase for related existing code
3. Read relevant specification documents
4. Identify all files that will be affected
5. Identify all tests that will need updating

## Operating Rules

1. **No code before plan** — Never write implementation code before producing a task plan
2. **Plan must be specific** — Vague plans like "fix the bug" are not acceptable
3. **Plan must identify files** — List every file that will be modified
4. **Plan must identify specs** — List every specification document relevant to the change
5. **Plan must identify tests** — List every test that needs to be added or modified
6. **Plan must identify risks** — List every risk (compatibility, performance, security)

## Validation Checklist

- [ ] Requirement clearly stated
- [ ] All affected files listed
- [ ] All relevant specs identified
- [ ] Implementation strategy defined
- [ ] Testing strategy defined
- [ ] Risks identified with mitigations

## Final Report Format

```
## Task Planning Report

Task: [description]
Plan Status: COMPLETE / INCOMPLETE
Files Affected: [count]
Specs Referenced: [count]
Tests Planned: [count]
Risks Identified: [count]

Ready for Implementation: YES / NO
```
