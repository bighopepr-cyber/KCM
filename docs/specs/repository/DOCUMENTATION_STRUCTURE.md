# Documentation Structure

| Field | Value |
|-------|-------|
| **Document ID** | KCM-REPO-007 |
| **Title** | Documentation Structure |
| **Version** | 2.0.0 |
| **Date** | 2026-08-05 |
| **Status** | Authoritative |
| **Owner** | Documentation Guardian (P11) |
| **Authority** | Engineering Orchestrator (P1) |

---

## 1. Authority Model

The documentation set is governed by a strict single-source-of-truth model.

| Priority | Location | Role |
|----------|----------|------|
| P0 | AGENTS.md | Engineering constitution and repository policy |
| P1 | PRD-TESTING& BRACHMARCK.md | Testing and benchmark authority |
| P2 | PRD3.md | Distributed, ML, security, and compliance authority |
| P3 | PRD2.md | Storage, runtime, and interface authority |
| P4 | PRD.md | Core type, compute, and reasoning authority |
| P5 | docs/*.md | Derived implementation and operational specifications |

## 2. Authoritative Categories

### Primary specification surface
- PRD documents: PRD.md, PRD2.md, PRD3.md, PRD-TESTING& BRACHMARCK.md
- Technical specifications: KCM_*_SPEC.md files
- Repository contract documents: docs/specs/repository/*

### Operational reference surface
- Guides, handbooks, tutorials, and cookbook material provide procedure-oriented context and must reference the authoritative specification set rather than restate it.

### Archive surface
- Audit, review, and retrospective reports are retained for historical traceability only and are not normative for implementation.

## 3. Structural Rules

1. One topic must have one authoritative document.
2. A derived document may reference the authoritative document; it must not redefine the same contract.
3. Navigation pages must only enumerate current implementation-relevant content.
4. Historical repository evolution material must not appear in the active implementation documentation path.

## 4. Validation Rules

1. No document may contradict an authoritative PRD.
2. All cross-references must resolve to an existing file or location.
3. All terminology must match the glossary.
4. All diagrams and interface examples must align with the codebase and the active API surface.
