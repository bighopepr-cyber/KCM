# Agents Configuration Technical Specification

## Overview

This document provides the technical specification for the `.agents/` directory configuration in the KCM project. It defines the structural mirror of the `skills/` directory for AI agent governance, including skill definitions, authority hierarchy, and execution flow.

## Scope

The `.agents/` directory configuration encompasses:

- Structural mirror of the `skills/` directory for AI agent consumption
- 16 governance skill definitions with priority-based authority hierarchy
- Integration points for AI agent systems
- Governance enforcement mechanisms

## Responsibilities

| Responsibility | Description |
|----------------|-------------|
| AI agent skill delivery | Deliver standardized governance skill instructions to AI agent systems |
| Governance enforcement | Enforce KCM engineering standards through AI-readable skill definitions |
| Structural consistency | Maintain exact structural mirror of `skills/` directory |
| Authority hierarchy enforcement | Ensure agents respect priority-based skill authority levels |

## Technical Specification

### Skills Mirroring skills/

The `.agents/skills/` directory contains 16 skill definitions that mirror the `skills/` directory. Each skill is defined in a `SKILL.md` file within its respective directory.

### Priority Levels P1-P16

| Priority | Skill | Authority Description |
|----------|-------|----------------------|
| P1 | `kcm-engineering-orchestrator` | Master coordinator — overrides all |
| P2 | `kcm-task-planner` | Can block implementation without plan |
| P3 | `kcm-change-impact-analysis` | Can block changes with unassessed impact |
| P4 | `kcm-specification-lock` | Can veto format/API/FFI changes |
| P5 | `kcm-architecture-guardian` | Can block architecture violations |
| P6 | `kcm-database-engine-specialist` | Can block storage/query changes |
| P7 | `kcm-security-engineer` | Can block security/compliance violations |
| P8 | `kcm-performance-engineer` | Can block performance regressions |
| P9 | `kcm-testing-verification` | Can block changes without tests |
| P10 | `kcm-code-quality-guardian` | Can reject code quality issues |
| P11 | `kcm-documentation-guardian` | Can block undocumented changes |
| P12 | `kcm-release-readiness` | Can block releases |
| P13 | `kcm-code-review-auditor` | Provides review feedback |
| P14 | `kcm-debugging-root-cause` | Provides diagnostic analysis |
| P15 | `kcm-engineering-decision-record` | Documents decisions |
| P16 | `kcm-repository-intelligence` | Provides codebase understanding |

### Authority Hierarchy

The authority hierarchy defines the priority ordering for skill execution:

```
P1 (Master Coordinator)
├── P2 (Task Planning)
│   └── P3 (Change Impact)
├── P4 (Specification Lock) — VETO power
├── P5 (Architecture Guardian)
├── P6 (Database Engine Specialist)
├── P7 (Security Engineer)
├── P8 (Performance Engineer)
├── P9 (Testing Verification)
├── P10 (Code Quality Guardian)
├── P11 (Documentation Guardian)
├── P12 (Release Readiness)
├── P13 (Code Review Auditor)
├── P14 (Debugging Root Cause)
├── P15 (Engineering Decision Record)
└── P16 (Repository Intelligence)
```

### Execution Flow

```
1. Repository Understanding    → kcm-repository-intelligence (P16)
2. Specification Validation    → kcm-specification-lock (P4), kcm-architecture-guardian (P5)
3. Planning                    → kcm-task-planner (P2), kcm-change-impact-analysis (P3)
4. Implementation              → Domain skills (P6, P7, P8)
5. Verification                → kcm-testing-verification (P9), kcm-code-quality-guardian (P10), kcm-code-review-auditor (P13)
6. Release                     → kcm-release-readiness (P12)
```

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                  AI Agent System                     │
│                                                     │
│  ┌───────────────────────────────────────────────┐  │
│  │           Skill Loader                        │  │
│  │  - Scans .agents/skills/                      │  │
│  │  - Loads SKILL.md files                       │  │
│  │  - Applies authority hierarchy                │  │
│  └───────────────────────────────────────────────┘  │
│                       │                             │
│                       ▼                             │
│  ┌───────────────────────────────────────────────┐  │
│  │         Governance Engine                     │  │
│  │  - Enforces quality gates                     │  │
│  │  - Validates specifications                   │  │
│  │  - Manages authority levels                   │  │
│  └───────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────┐
│               .agents/ Directory                    │
│                                                     │
│  ┌───────────────────────────────────────────────┐  │
│  │           skills/ (Mirror)                    │  │
│  │  ├── kcm-engineering-orchestrator/SKILL.md   │  │
│  │  ├── kcm-task-planner/SKILL.md               │  │
│  │  ├── kcm-change-impact-analysis/SKILL.md     │  │
│  │  ├── kcm-specification-lock/SKILL.md         │  │
│  │  ├── kcm-architecture-guardian/SKILL.md      │  │
│  │  ├── kcm-database-engine-specialist/SKILL.md │  │
│  │  ├── kcm-security-engineer/SKILL.md          │  │
│  │  ├── kcm-performance-engineer/SKILL.md       │  │
│  │  ├── kcm-testing-verification/SKILL.md       │  │
│  │  ├── kcm-code-quality-guardian/SKILL.md      │  │
│  │  ├── kcm-documentation-guardian/SKILL.md     │  │
│  │  ├── kcm-release-readiness/SKILL.md          │  │
│  │  ├── kcm-code-review-auditor/SKILL.md        │  │
│  │  ├── kcm-debugging-root-cause/SKILL.md       │  │
│  │  ├── kcm-engineering-decision-record/SKILL.md│  │
│  │  └── kcm-repository-intelligence/SKILL.md    │  │
│  └───────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────┐
│              skills/ (Source)                       │
│  (Authoritative source for governance definitions)  │
└─────────────────────────────────────────────────────┘
```

## Internal Components

### Skill Mirror Structure

Each skill in `.agents/skills/` mirrors its `skills/` counterpart:

```
.agents/skills/<skill-name>/
└── SKILL.md
    ├── # Skill Name
    ├── ## Overview
    ├── ## Authority Level
    ├── ## Responsibilities
    ├── ## Execution Rules
    └── ## References
```

### File Naming Convention

- Directory names: `kcm-<skill-function>` (lowercase, hyphen-separated)
- Skill files: `SKILL.md` (uppercase, Markdown format)

## Data Model

### Skill Definition

```yaml
Skill:
  name: string          # Skill identifier (e.g., "kcm-code-quality-guardian")
  priority: integer     # Authority level (P1-P16)
  authority: string     # Authority description
  responsibilities: list[string]  # What the skill is responsible for
  execution_rules: list[string]   # Rules the skill enforces
  references: list[string]        # Related documents
```

### Authority Level

```yaml
AuthorityLevel:
  P1: Master Coordinator    # Overrides all
  P2: Task Planning         # Can block implementation
  P3: Change Impact         # Can block unassessed changes
  P4: Specification Lock    # Can VETO
  P5: Architecture          # Can block architecture violations
  P6: Database Engine       # Can block storage/query changes
  P7: Security              # Can block security violations
  P8: Performance           # Can block performance regressions
  P9: Testing               # Can block changes without tests
  P10: Code Quality         # Can reject quality issues
  P11: Documentation        # Can block undocumented changes
  P12: Release Readiness    # Can block releases
  P13: Code Review          # Provides review feedback
  P14: Debugging            # Provides diagnostic analysis
  P15: Decision Records     # Documents decisions
  P16: Repository Intel     # Provides codebase understanding
```

## Execution Flow

### Agent Loads Skill → Executes Instructions

```
┌──────────────┐     ┌──────────────────┐     ┌─────────────────┐
│  AI Agent    │────▶│  Scan .agents/   │────▶│  Load SKILL.md  │
│  System      │     │  skills/         │     │  files          │
└──────────────┘     └──────────────────┘     └─────────────────┘
                                                          │
                                                          ▼
┌──────────────┐     ┌──────────────────┐     ┌─────────────────┐
│  Enforce     │◀────│  Apply Authority  │◀────│  Parse Skill    │
│  Governance  │     │  Hierarchy       │     │  Instructions   │
└──────────────┘     └──────────────────┘     └─────────────────┘
       │
       ▼
┌─────────────────────────────────────────────────────────────┐
│  Execute Task with Governance Rules Applied                 │
│  - Quality gates enforced                                   │
│  - Specification validation performed                       │
│  - Authority levels respected                               │
│  - Audit trail maintained                                   │
└─────────────────────────────────────────────────────────────┘
```

## Public API

| Path | Method | Description |
|------|--------|-------------|
| `.agents/skills/<skill>/SKILL.md` | READ | Load skill definition |
| `.agents/README.md` | READ | Configuration documentation |
| `.agents/SECURITY.md` | READ | Security policy |
| `.agents/CONTRIBUTING.md` | READ | Contribution guidelines |
| `.agents/CODE_OF_CONDUCT.md` | READ | Community guidelines |

## Configuration

### Skill Loading Configuration

```yaml
skill_loading:
  source_directory: ".agents/skills/"
  file_pattern: "*/SKILL.md"
  load_order: "priority_descending"  # P1 first
  cache_enabled: true
  cache_ttl: "1h"
```

### Authority Enforcement Configuration

```yaml
authority_enforcement:
  enabled: true
  veto_enabled: true  # P4 can veto
  block_enabled: true  # P2-P12 can block
  feedback_enabled: true  # P13-P16 provide feedback
```

## Dependencies

| Dependency | Type | Description |
|------------|------|-------------|
| `skills/` | Source | Authoritative source for governance skill definitions |
| `AGENTS.md` | Reference | Engineering constitution defining skill authority hierarchy |
| AI agent system | Consumer | Reads and applies skill instructions |

## Error Handling

| Error | Condition | Resolution |
|-------|-----------|------------|
| `SkillNotFound` | Requested skill directory missing in `.agents/skills/` | Verify structural mirror with `skills/` |
| `SkillFileCorrupted` | `SKILL.md` file unreadable or malformed | Restore from `skills/` source |
| `AuthorityViolation` | Agent attempts to exceed its authority level | Enforce authority hierarchy |
| `StructuralDesync` | `.agents/skills/` differs from `skills/` | Synchronize directories |

## Performance Characteristics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Skill load time | < 10ms per skill | Time to parse `SKILL.md` |
| Total scan time | < 100ms | Time to scan all 16 skills |
| Memory footprint | < 1MB | Memory used by loaded skills |
| Authority check latency | < 1ms | Time to verify authority level |

## Security Considerations

### Skill Integrity

- All `SKILL.md` files must pass integrity validation
- No secrets or credentials allowed in skill files
- Changes to skill files require code review and version control tracking

### Access Control

- Skill files are read-only during agent execution
- Write access restricted to authorized engineers
- All modifications tracked in git history

## Integration

### AI Agent System Integration

```
┌─────────────────────────────────────────────────────────┐
│                   KCM Repository                        │
│                                                         │
│  .agents/                                               │
│  ├── README.md          ──────────────────────────────┐ │
│  ├── SECURITY.md        ──────────────────────────────┤ │
│  ├── CONTRIBUTING.md    ──────────────────────────────┤ │
│  ├── CODE_OF_CONDUCT.md ─────────────────────────────┤ │
│  └── skills/            ─────────────────────────────┤ │
│      └── <16 skills>/SKILL.md ───────────────────────┤ │
└───────────────────────────────────────────────────────┤ │
                                                        │ │
┌───────────────────────────────────────────────────────┤ │
│               AI Agent System                         │ │
│                                                       │ │
│  ┌─────────────────────────────────────────────────┐  │ │
│  │  1. Detect KCM project structure                │◄─┘ │
│  │  2. Scan .agents/skills/ for available skills   │    │
│  │  3. Load relevant SKILL.md files                │    │
│  │  4. Execute instructions from loaded skills     │    │
│  │  5. Enforce governance rules                    │    │
│  └─────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────┘
```

## Sequence Diagram

```
Agent System          .agents/skills/         skills/
     │                       │                    │
     │  Scan directory       │                    │
     │──────────────────────▶│                    │
     │  Return skill list    │                    │
     │◀──────────────────────│                    │
     │                       │                    │
     │  Load SKILL.md        │                    │
     │──────────────────────▶│                    │
     │  Return skill content │                    │
     │◀──────────────────────│                    │
     │                       │                    │
     │  Verify consistency   │                    │
     │───────────────────────────────────────────▶│
     │  Return comparison    │                    │
     │◀───────────────────────────────────────────│
     │                       │                    │
     │  Apply governance     │                    │
     │  rules to task        │                    │
     │  (internal)           │                    │
     │                       │                    │
     │  Execute task         │                    │
     │  with governance      │                    │
     │  enforced             │                    │
     │                       │                    │
```

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                    KCM Governance Architecture                   │
│                                                                 │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                    Source Layer                           │  │
│  │  ┌─────────────────────────────────────────────────────┐  │  │
│  │  │  skills/ (Authoritative)                            │  │  │
│  │  │  - 16 governance skill definitions                  │  │  │
│  │  │  - Priority-based authority hierarchy               │  │  │
│  │  │  - Engineering rules and standards                  │  │  │
│  │  └─────────────────────────────────────────────────────┘  │  │
│  └───────────────────────────────────────────────────────────┘  │
│                              │                                  │
│                              ▼                                  │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                    Mirror Layer                           │  │
│  │  ┌─────────────────────────────────────────────────────┐  │  │
│  │  │  .agents/skills/ (Mirror)                           │  │  │
│  │  │  - Exact structural mirror of skills/               │  │  │
│  │  │  - AI agent-readable format                         │  │  │
│  │  │  - Same 16 skill definitions                        │  │  │
│  │  └─────────────────────────────────────────────────────┘  │  │
│  └───────────────────────────────────────────────────────────┘  │
│                              │                                  │
│                              ▼                                  │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                    Agent Layer                            │  │
│  │  ┌─────────────────────────────────────────────────────┐  │  │
│  │  │  AI Agent System                                    │  │  │
│  │  │  - Loads skills from .agents/skills/                │  │  │
│  │  │  - Applies authority hierarchy                      │  │  │
│  │  │  - Enforces governance rules                        │  │  │
│  │  │  - Executes tasks with governance applied           │  │  │
│  │  └─────────────────────────────────────────────────────┘  │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

## References

| Document | Description |
|----------|-------------|
| `skills/` | Source directory for governance skill definitions |
| `AGENTS.md` | Engineering constitution defining skill authority hierarchy |
| `SECURITY.md` (repository root) | Project-wide security policy |
| `CONTRIBUTING.md` (repository root) | Core engine contribution rules |
| `CODE_OF_CONDUCT.md` (repository root) | Project-wide community guidelines |
| `docs/PRD.md` | Core types, storage, compute, reasoning |
| `docs/PRD2.md` | Storage, runtime, interfaces |
| `docs/PRD3.md` | Distributed, ML, security, compliance |

## SSOT Alignment

This specification aligns with the following SSOT documents:

| SSOT Document | Alignment |
|---------------|-----------|
| `AGENTS.md` | Skill authority hierarchy (P1-P16), execution flow, governance rules |
| `docs/PRD-TESTING& BRACHMARCK.md` | Testing strategy for skill consistency validation |
| `SECURITY.md` (repository root) | Security policies for skill integrity and access control |

All behavioral specifications in this document must match the authoritative definitions in the SSOT documents. When conflicts arise, the SSOT document takes precedence.