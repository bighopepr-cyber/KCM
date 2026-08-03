# Documentation Structure

| Field | Value |
|-------|-------|
| **Document ID** | KCM-REPO-007 |
| **Title** | Documentation Structure |
| **Version** | 1.0.0 |
| **Date** | 2026-08-03 |
| **Status** | Authoritative |
| **Authority** | Engineering Orchestrator (P1) |

---

## 1. Document Hierarchy

| Priority | Location | Purpose |
|----------|----------|---------|
| P0 | AGENTS.md | Engineering Constitution |
| P1 | PRD-TESTING.md | Testing/Benchmark authority |
| P2 | PRD3.md | Distributed/Security/Compliance authority |
| P3 | PRD2.md | Storage/Runtime/Interfaces authority |
| P4 | PRD.md | Core/Compute/Reasoning authority |
| P5 | docs/*.md | Derived specifications |

## 2. Document Categories

### Authoritative Specifications (docs/)
- PRD documents (4): PRD.md, PRD2.md, PRD3.md, PRD-TESTING.md
- Technical specs (17): KCM_*_SPEC.md files
- Reports (5): Audit, stability, performance, design review

### Repository Specifications (docs/specs/repository/)
- Architecture, workspace, folders, naming, ownership
- Documentation structure, versioning, dependencies
- Release, governance, evolution

### Ecosystem Specifications (docs/specs/ecosystem/)
- Developer, enterprise, SDK, CLI roadmaps
- Plugin, extension, deployment, cloud strategies
- Observability, integrations, community, vision

### Website Documentation (website/docs/)
- HTML versions of all technical specifications

## 3. Document Template

Every specification document must include:

```markdown
# Title

| Field | Value |
|-------|-------|
| **Document ID** | KCM-XXX-NNN |
| **Title** | Document Title |
| **Version** | 1.0.0 |
| **Date** | YYYY-MM-DD |
| **Status** | Authoritative/Draft/Deprecated |
| **Authority** | Skill Name (Priority) |

---
[Content sections]
```

## 4. Validation Rules

1. No document may contradict an authoritative PRD
2. All cross-references must be valid
3. All terminology must match the glossary
4. All diagrams must be consistent with architecture
