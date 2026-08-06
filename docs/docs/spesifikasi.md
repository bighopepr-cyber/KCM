# Documentation Technical Specification

## Overview

This document defines the technical specification for the KCM documentation system — how documentation is structured, versioned, maintained, and validated as the Single Source of Truth (SSOT) for the entire project.

## Scope

This specification covers:

- SSOT hierarchy and authority model
- ADR lifecycle and format
- Specification versioning and change control
- Documentation validation and integrity checks
- Integration with CI/CD and engineering governance

## Responsibilities

| Responsibility | Owner | Authority |
|---------------|-------|-----------|
| SSOT Management | Documentation Guardian (P11) | Maintains document hierarchy and prevents conflicts |
| ADR Tracking | Engineering Orchestrator (P1) | Ensures architectural decisions are recorded before implementation |
| Specification Maintenance | Specification Lock (P4) | Approves changes to frozen contracts |
| Documentation Validation | Code Quality Guardian (P10) | Enforces automated checks via `validate-ssot.sh` |

## Technical Specification

### SSOT Hierarchy

The SSOT hierarchy defines document authority when conflicts arise:

| Priority | Document | Scope | Override Authority |
|----------|----------|-------|-------------------|
| P1 | `SSOT.md` | Root engineering constitution | None — highest authority |
| P2 | `PRD-TESTING-AND-BENCHMARK.md` | Testing strategy, quality gates, benchmark methodology | P1 only |
| P3 | `PRD3.md` | Distributed architecture, ML integration, security, compliance | P1–P2 |
| P4 | `PRD2.md` | Persistence layer, optimizer, monitoring, interfaces | P1–P3 |
| P5 | `PRD.md` | Core types, storage engine, compute engine, reasoning engine | P1–P4 |
| P6 | `AGENTS.md` | Engineering constitution, governance rules | P1–P5 |

**Conflict Resolution**: When two SSOT documents contradict each other, the higher-priority document wins. All conflicts must be resolved by updating the lower-priority document to align with the higher-priority one.

### ADR Process

Architecture Decision Records follow a structured lifecycle:

```
Proposed → Accepted → (Deprecated | Superseded)
```

| Stage | Description |
|-------|-------------|
| Proposed | Draft ADR submitted for review |
| Accepted | ADR approved and merged; becomes binding |
| Deprecated | ADR no longer relevant but retained for history |
| Superseded | ADR replaced by a newer ADR (referenced by ID) |

**ADR Format**:

```markdown
# ADR-NNN: [Title]

## Status
Proposed | Accepted | Deprecated | Superseded

## Context
[Problem statement and constraints]

## Decision
[What was decided]

## Consequences
[Impact: positive, negative, neutral]

## Related ADRs
[Links to related decisions]
```

### Specification Versioning

Specifications are versioned to track evolution:

| Component | Versioning | Format |
|-----------|-----------|--------|
| SSOT documents | Content revision via git | Git commit hash |
| ADRs | Sequential numbering (ADR-001, ADR-002, ...) | Numeric ID |
| API specs | Semantic versioning (v1.0.0) | SemVer |
| Format specs | Major.Minor (v1.0) | Version header in document |

**Version Header Format** (for versioned specs):

```markdown
<!-- Version: 1.2.0 -->
<!-- Last Reviewed: 2026-08-06 -->
<!-- Status: Active -->
```

## Architecture

```
┌─────────────────────────────────────────────────┐
│                SSOT Hierarchy                    │
│  SSOT.md > PRD-TESTING > PRD3 > PRD2 > PRD     │
└──────────────────┬──────────────────────────────┘
                   │
         ┌─────────┴─────────┐
         │                   │
    ┌────▼────┐        ┌────▼────┐
    │  ADRs   │        │  Specs  │
    │ (adr/)  │        │(specs/) │
    └────┬────┘        └────┬────┘
         │                   │
    ┌────▼────┐        ┌────▼────┐
    │Handbook │        │   SDK   │
    │(handbook│        │  (sdk/) │
    └────┬────┘        └────┬────┘
         │                   │
    ┌────▼────┐        ┌────▼────┐
    │Runbooks │        │  CI/CD  │
    │(runbook)│        │validate │
    └─────────┘        └─────────┘
```

## Internal Components

### adr/

Contains 10 Architecture Decision Records (ADR-001 through ADR-010). Each ADR captures:

- Problem context and constraints
- Options considered with trade-offs
- Final decision and rationale
- Consequences and impact on the system

### handbook/

Developer and contributor handbooks providing onboarding material, coding standards, and workflow guides.

### runbook/

Operational runbooks for:

- **DISASTER_RECOVERY.md**: Recovery procedures for data loss, corruption, or system failure
- **OPERATIONAL_RUNBOOK.md**: Day-to-day operational procedures, monitoring, and incident response

### sdk/

Language-specific SDK documentation for 9 supported languages plus compatibility matrix and technical specification.

### specs/

19 specification documents organized by priority:

| Category | Documents |
|----------|-----------|
| Core PRDs | PRD.md, PRD2.md, PRD3.md, PRD-TESTING-AND-BENCHMARK.md |
| Component Specs | KCM_API_SPEC.md, KCM_COLUMNAR_FORMAT_SPEC.md, KCM_COMPRESSION_SPEC.md, KCM_DATA_MODEL_SPEC.md, KCM_DEPLOYMENT_SPEC.md, KCM_INDEXING_SPEC.md, KCM_PERFORMANCE_SPEC.md, KCM_QUERY_EXECUTION_SPEC.md, KCM_RUNTIME_SPEC.md, KCM_SECURITY_TRUST_SPEC.md, KCM_SPECIFICATION.md, KCM_TESTING_SPEC.md, KCM_VERSIONING_SPEC.md |
| Reference | KCM_DOCUMENT_AUDIT_REPORT.md, KCM_GLOSSARY.md |

## Data Model

### ADR Record

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| id | string | Yes | ADR identifier (e.g., ADR-001) |
| title | string | Yes | Short descriptive title |
| status | enum | Yes | Proposed, Accepted, Deprecated, Superseded |
| context | markdown | Yes | Problem statement and constraints |
| decision | markdown | Yes | What was decided |
| consequences | markdown | Yes | Impact analysis |
| date | date | Yes | Date of decision |
| related | list | No | References to related ADRs |

### Spec Version Record

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| version | string | Yes | Semantic version (e.g., 1.2.0) |
| last_reviewed | date | Yes | Date of last review |
| status | enum | Yes | Active, Deprecated, Draft |
| author | string | Yes | Author or owning team |
| reviewed_by | string | No | Last reviewer |

## Execution Flow

### Documentation Update Flow

```
1. Identify Change
   ├── Check SSOT hierarchy
   ├── Verify no conflict with higher-priority docs
   └── Determine affected documents

2. Prepare Change
   ├── Update target document
   ├── Maintain version header (if applicable)
   ├── Update cross-references
   └── Update glossary if new terms introduced

3. Validate
   ├── Run scripts/validate-ssot.sh
   ├── Verify all internal links
   ├── Check for secrets/credentials
   └── Verify terminology consistency

4. Review
   ├── Documentation Guardian review
   ├── Specification Lock review (if frozen contract)
   └── Security Engineer review (if security-related)

5. Merge
   ├── CI validation passes
   ├── Required approvals obtained
   └── Changes merged to main
```

## Public API

The documentation system exposes no programmatic API. Access is via:

- Direct file reading (Markdown files)
- CI validation script (`scripts/validate-ssot.sh`)
- Git history for version tracking

## Configuration

| Configuration | Location | Description |
|---------------|----------|-------------|
| SSOT hierarchy | `AGENTS.md` §Document Hierarchy | Priority ordering |
| ADR format | `AGENTS.md` §Single Source of Truth | ADR structure requirements |
| Validation script | `scripts/validate-ssot.sh` | Automated checks |

## Dependencies

| Dependency | Type | Description |
|-----------|------|-------------|
| Git | Infrastructure | Version control and history |
| `scripts/validate-ssot.sh` | Tool | Automated SSOT validation |
| `AGENTS.md` | Reference | Engineering constitution and hierarchy definition |

## Error Handling

| Error | Handling |
|-------|----------|
| SSOT conflict detected | Block merge; escalate to Specification Lock |
| Broken internal link | Block merge; require fix |
| Secret detected in docs | Block merge; require redaction |
| Outdated specification | Flag for review; create issue |
| Missing version header | Block merge for versioned specs |

## Performance Characteristics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Validation script execution | < 5 seconds | CI pipeline timing |
| Documentation search | < 1 second | Manual grep timing |
| Cross-reference resolution | < 10 links per document | Document review |

## Security Considerations

| Consideration | Implementation |
|--------------|----------------|
| No secrets in docs | Grep-based secret scanning in CI |
| Sensitive runbooks | Access control via RBAC |
| Security specs review | Mandatory security engineer approval |
| Audit trail | Git history for all changes |

## Integration

The documentation system integrates with:

| Component | Integration Point |
|-----------|------------------|
| `kcm-security` | RBAC controls documentation access |
| CI/CD pipeline | `validate-ssot.sh` runs on every PR |
| Engineering skills | P4 (Specification Lock) gates contract changes |
| All crates | Reference SSOT specs for implementation |
| `scripts/` | Automated validation tooling |

## Sequence Diagram

```
Contributor          Documentation        CI Pipeline
     │                Guardian                 │
     │                    │                     │
     ├── Propose Change ──►                     │
     │                    │                     │
     │                    ├── Review Change ────►
     │                    │                     │
     │                    │                  Validate
     │                    │                  SSOT.sh
     │                    │                     │
     │                    │◄── Validation Result─┤
     │                    │                     │
     │◄── Approval/Reject─┤                     │
     │                    │                     │
     ├── Merge ───────────────────────────────►│
     │                    │                     │
```

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                    KCM Documentation System                  │
│                                                             │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  │
│  │ SSOT     │  │   ADR    │  │  Specs   │  │   SDK    │  │
│  │ Hierarchy│  │ Registry │  │ Registry │  │  Docs    │  │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘  │
│       │              │              │              │         │
│       └──────────────┴──────┬───────┴──────────────┘         │
│                             │                                │
│                    ┌────────▼────────┐                       │
│                    │ validate-ssot.sh│                       │
│                    └────────┬────────┘                       │
│                             │                                │
│                    ┌────────▼────────┐                       │
│                    │   CI Pipeline   │                       │
│                    └─────────────────┘                       │
└─────────────────────────────────────────────────────────────┘
```

## References

- `AGENTS.md` — Engineering constitution and SSOT hierarchy
- `docs/README.md` — Documentation structure overview
- `docs/SECURITY.md` — Documentation security policy
- `docs/CONTRIBUTING.md` — Documentation contribution guide
- `scripts/validate-ssot.sh` — Automated SSOT validation
- `docs/specs/KCM_GLOSSARY.md` — Project terminology
- `docs/specs/KCM_SPECIFICATION.md` — System specification

## SSOT Alignment

This document aligns with the following SSOT requirements:

| SSOT Requirement | Section | Alignment |
|-----------------|---------|-----------|
| SSOT-01 | SSOT Hierarchy | Defines document authority ordering |
| SSOT-02 | Technical Specification | Specifies implementation must match SSOT |
| SSOT-03 | Execution Flow | Defines update process requiring SSOT alignment |
| SSOT-05 | Conflict Resolution | Higher-priority documents win in conflicts |
| SSOT-06 | Traceability | ADR and spec records provide requirement tracing |
| SSOT-07 | ADR Process | New features require specification before implementation |
| SSOT-08 | Versioning | Specification versioning tracks evolution |
