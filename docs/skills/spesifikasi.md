# Skills Technical Specification

## Overview

This document specifies the technical design of the KCM engineering skills system — 16 AI engineering skills that enforce governance, quality, and authority boundaries across KCM development.

## Scope

| In Scope | Out of Scope |
|----------|-------------|
| Skill definitions and authority hierarchy | Implementation of individual skills |
| Execution flow and gate system | Runtime behavior of AI agents |
| Conflict resolution and priority ordering | SSOT document content |
| Skill invocation and integration | Tooling for skill management |

## Responsibilities

| Area | Description |
|------|-------------|
| AI governance | Enforcing rules for AI agent behavior during development |
| Quality enforcement | Preventing code that violates standards from being committed |
| Engineering gates | 6 mandatory gates at each development phase |
| Authority management | Resolving conflicts between skills by priority |

## Technical Specification

### Priority Levels

| Level | Skill | Override Authority |
|-------|-------|-------------------|
| P1 | kcm-engineering-orchestrator | Can override all lower-priority skills |
| P2 | kcm-task-planner | Can block implementation without plan |
| P3 | kcm-change-impact-analysis | Can block changes with unassessed impact |
| P4 | kcm-specification-lock | Can VETO format/API/FFI changes |
| P5 | kcm-architecture-guardian | Can block architecture violations |
| P6 | kcm-database-engine-specialist | Can block storage/query changes |
| P7 | kcm-security-engineer | Can block security/compliance violations |
| P8 | kcm-performance-engineer | Can block performance regressions |
| P9 | kcm-testing-verification | Can block changes without tests |
| P10 | kcm-code-quality-guardian | Can reject code quality issues |
| P11 | kcm-documentation-guardian | Can block undocumented changes |
| P12 | kcm-release-readiness | Can block releases |
| P13 | kcm-code-review-auditor | Provides review feedback |
| P14 | kcm-debugging-root-cause | Provides diagnostic analysis |
| P15 | kcm-engineering-decision-record | Documents decisions |
| P16 | kcm-repository-intelligence | Provides codebase understanding |

### Authority Hierarchy

```
P1 (orchestrator) ─── overrides all
  ├── P2 (task-planner)
  ├── P3 (change-impact)
  ├── P4 (specification-lock) ─── VETO on frozen contracts
  ├── P5 (architecture-guardian) ─── architecture ownership
  │     └── defers to P4 for format changes
  ├── P6 (database-engine) ─── storage/query ownership
  │     └── cannot change contracts (P4)
  ├── P7 (security-engineer) ─── security ownership
  │     └── no skill can override
  ├── P8 (performance-engineer)
  ├── P9 (testing-verification)
  ├── P10 (code-quality-guardian)
  ├── P11 (documentation-guardian)
  ├── P12 (release-readiness)
  ├── P13 (code-review-auditor)
  ├── P14 (debugging-root-cause)
  ├── P15 (engineering-decision-record)
  └── P16 (repository-intelligence)
```

### Execution Flow

```
Phase 1: Repository Understanding
  └── kcm-repository-intelligence (P16)
      Output: Affected file list, dependency map

Phase 2: Specification Validation
  ├── kcm-specification-lock (P4)
  │   Output: Frozen contract verification
  └── kcm-architecture-guardian (P5)
      Output: Architecture alignment check

Phase 3: Planning
  ├── kcm-task-planner (P2)
  │   Output: Implementation plan
  └── kcm-change-impact-analysis (P3)
      Output: Impact assessment

Phase 4: Implementation
  ├── kcm-database-engine-specialist (P6)
  ├── kcm-security-engineer (P7)
  └── kcm-performance-engineer (P8)

Phase 5: Verification
  ├── kcm-testing-verification (P9)
  ├── kcm-code-quality-guardian (P10)
  └── kcm-code-review-auditor (P13)

Phase 6: Release
  └── kcm-release-readiness (P12)
      Output: Release approval
```

### Gate System

| Gate | Name | Skills Involved | Blocks |
|------|------|-----------------|--------|
| Gate 1 | Repository Understanding | P16 | Missing context |
| Gate 2 | Specification Validation | P4, P5 | SSOT deviation |
| Gate 3 | Implementation Planning | P2, P3 | Unplanned changes |
| Gate 4 | Implementation Validation | P6, P7, P8 | Placeholders, unwrap, panic |
| Gate 5 | Domain Validation | P6, P7, P8 | Domain-specific failures |
| Gate 6 | Production Readiness | P9, P10, P12 | Failed quality gates |

## Architecture

### Skill Dependency Graph

```
kcm-core (zero deps)
  ↑
kcm-storage (core + compression + hashing)
  ↑
kcm-compute, kcm-reasoning, kcm-optimizer
  ↑
kcm-runtime (core + storage + concurrency)
  ↑
kcm-interface (core + storage + runtime + FFI)
  ↑
kcm-server (core + runtime + interface + web)
```

Skills reference these crates but do not depend on each other.

## Internal Components

| Skill | Domain | Key Responsibility |
|-------|--------|-------------------|
| kcm-engineering-orchestrator | Governance | Master coordinator, conflict resolution |
| kcm-task-planner | Planning | Task decomposition, implementation plan |
| kcm-change-impact-analysis | Planning | Impact assessment, risk identification |
| kcm-specification-lock | Contracts | Frozen contract protection, VETO authority |
| kcm-architecture-guardian | Architecture | System architecture integrity |
| kcm-database-engine-specialist | Storage | Storage engine, query engine, transactions |
| kcm-security-engineer | Security | Cryptographic correctness, RBAC, audit |
| kcm-performance-engineer | Performance | Benchmarks, SIMD, memory efficiency |
| kcm-testing-verification | Quality | Test strategy, coverage, correctness |
| kcm-code-quality-guardian | Quality | Code standards, no placeholders |
| kcm-documentation-guardian | Docs | SSOT alignment, documentation completeness |
| kcm-release-readiness | Release | Release validation, version management |
| kcm-code-review-auditor | Review | Code review, architectural risk assessment |
| kcm-debugging-root-cause | Debugging | Root cause analysis, fix verification |
| kcm-engineering-decision-record | Decisions | Decision documentation, rationale |
| kcm-repository-intelligence | Context | Codebase understanding, dependency mapping |

## Data Model

| Field | Type | Description |
|-------|------|-------------|
| priority | u8 (1–16) | Skill priority level |
| authority_level | enum (L1–L5) | RBAC permission level |
| scope | string[] | Domains the skill covers |
| ssot_reference | string | Authoritative SSOT document |
| can_veto | bool | Whether skill can VETO changes |
| can_block | bool | Whether skill can block changes |

## Execution Flow (Detailed)

### Phase 1: Repository Understanding

```
Input: Proposed change description
Agent: kcm-repository-intelligence (P16)
Output:
  - Affected crates and modules
  - Dependency graph
  - Existing implementations
  - Affected tests and benchmarks
```

### Phase 2: Specification Validation

```
Input: Repository understanding output
Agents:
  - kcm-specification-lock (P4): Verify frozen contracts
  - kcm-architecture-guardian (P5): Verify architecture alignment
Output:
  - Frozen contract status
  - Architecture compliance
  - SSOT alignment check
```

### Phase 3: Planning

```
Input: Specification validation output
Agents:
  - kcm-task-planner (P2): Create implementation plan
  - kcm-change-impact-analysis (P3): Assess impact
Output:
  - Step-by-step plan
  - Affected files list
  - Impact assessment
  - Risk mitigation
```

### Phase 4: Implementation

```
Input: Implementation plan
Agents:
  - kcm-database-engine-specialist (P6): Storage changes
  - kcm-security-engineer (P7): Security review
  - kcm-performance-engineer (P8): Performance validation
Output:
  - Implemented code
  - Security validation
  - Performance benchmarks
```

### Phase 5: Verification

```
Input: Implementation output
Agents:
  - kcm-testing-verification (P9): Test execution
  - kcm-code-quality-guardian (P10): Quality check
  - kcm-code-review-auditor (P13): Code review
Output:
  - Test results
  - Quality report
  - Review feedback
```

### Phase 6: Release

```
Input: Verification output
Agent: kcm-release-readiness (P12)
Output:
  - Release approval
  - Version bump recommendation
  - Changelog update
```

## Public API

### Skill Invocation

Skills are invoked via the `skill` tool:

```
skill(name: "<skill-name>")
```

Example:

```
skill(name: "kcm-database-engine-specialist")
```

### Skill Loading

| Parameter | Type | Description |
|-----------|------|-------------|
| name | string | Skill directory name (kebab-case) |
| Output | Skill instructions | Markdown content loaded into context |

## Configuration

| Setting | Value | Source |
|---------|-------|--------|
| Skill directory | `skills/` | Repository structure |
| Skill file | `SKILL.md` | Per-skill convention |
| Priority levels | P1–P16 | `AGENTS.md` |
| Authority hierarchy | Fixed | `AGENTS.md` |
| SSOT documents | `docs/PRD*.md` | Repository |

## Dependencies

| Dependency | Type | Description |
|------------|------|-------------|
| AGENTS.md | Reference | Authority hierarchy definition |
| SSOT documents | Reference | Specification alignment |
| cargo build | Build | Code compilation |
| cargo test | Test | Test execution |
| cargo clippy | Lint | Code quality |
| validate-ssot.sh | Script | SSOT compliance |

## Error Handling

### Skill Conflict Resolution

| Conflict Type | Resolution |
|---------------|------------|
| Two skills disagree | Higher-priority skill wins |
| P4 vs P5 on format | P4 (specification-lock) wins |
| P7 blocks security change | P7 (security-engineer) wins; no override |
| P1 vs any | P1 (engineering-orchestrator) wins |
| P13 vs P10 on quality | P10 (code-quality-guardian) wins (automated prevention runs first) |

### Error States

| State | Description | Recovery |
|-------|-------------|----------|
| SSOT divergence | Skill instructions conflict with SSOT | Fix skill, not SSOT |
| Authority violation | Skill operates outside its authority | Block operation, escalate to P1 |
| Missing skill | Required skill not found | Create skill per CONTRIBUTING.md |
| Gate failure | Mandatory gate fails | Fix issues, re-run gate |

## Performance Characteristics

| Metric | Target |
|--------|--------|
| Skill loading | < 100ms |
| Skill size | < 50KB per SKILL.md |
| Gate execution | < 5s per gate |
| Full flow | < 30s total |
| Runtime overhead | Zero (instruction-only) |

## Security Considerations

| Consideration | Description |
|---------------|-------------|
| Skill immutability | Skills cannot be modified during execution |
| Authority boundaries | Fixed priority hierarchy prevents escalation |
| Security override | No skill can override kcm-security-engineer |
| SSOT protection | Only kcm-specification-lock can modify frozen contracts |
| Audit trail | All skill invocations are tracked |

## Integration

### Agent Integration

```
AI Agent
  ├── Loads skill via skill tool
  ├── Follows skill instructions
  ├── Respects authority hierarchy
  └── Reports to engineering-orchestrator
```

### CI/CD Integration

```
CI Pipeline
  ├── format-check
  ├── clippy-lint
  ├── build
  ├── unit-tests
  ├── integration-tests
  ├── property-tests
  ├── security-tests
  ├── benchmarks
  ├── ssot-validation
  └── quality-gate (all above pass)
```

## Sequence Diagram

```
User → Agent: "Modify storage engine"
Agent → P16: Repository Understanding
P16 → Agent: Affected files, dependencies
Agent → P4: Specification Validation
P4 → Agent: Frozen contract status
Agent → P5: Architecture Validation
P5 → Agent: Architecture compliance
Agent → P2: Task Planning
P2 → Agent: Implementation plan
Agent → P3: Impact Analysis
P3 → Agent: Impact assessment
Agent → P6: Implementation (storage)
P6 → Agent: Implemented code
Agent → P7: Security Review
P7 → Agent: Security validation
Agent → P9: Test Execution
P9 → Agent: Test results
Agent → P10: Quality Check
P10 → Agent: Quality report
Agent → P13: Code Review
P13 → Agent: Review feedback
Agent → P12: Release Readiness
P12 → Agent: Release approval
Agent → User: Change complete
```

## Architecture Diagram

```
┌─────────────────────────────────────────────────┐
│              kcm-engineering-orchestrator (P1)   │
│              Master Coordinator                  │
├─────────┬─────────┬─────────┬───────────────────┤
│ P2      │ P3      │ P4      │ P5                │
│ Task    │ Change  │ Spec    │ Architecture      │
│ Planner │ Impact  │ Lock    │ Guardian          │
├─────────┼─────────┼─────────┼───────────────────┤
│ P6      │ P7      │ P8      │ P9                │
│ DB      │ Security│ Perf    │ Testing           │
│ Engine  │ Engineer│ Engineer│ Verification      │
├─────────┼─────────┼─────────┼───────────────────┤
│ P10     │ P11     │ P12     │ P13               │
│ Code    │ Doc     │ Release │ Code              │
│ Quality │ Guard   │ Ready   │ Review            │
├─────────┼─────────┼─────────┼───────────────────┤
│ P14     │ P15     │ P16     │                   │
│ Debug   │ Decision│ Repo    │                   │
│ Root    │ Record  │ Intel   │                   │
└─────────┴─────────┴─────────┴───────────────────┘
```

## References

- `AGENTS.md` — Engineering constitution and authority hierarchy
- `docs/PRD.md` — Core types, storage, compute, reasoning
- `docs/PRD2.md` — Storage, runtime, interfaces
- `docs/PRD3.md` — Distributed, ML, security, compliance
- `docs/PRD-TESTING& BRACHMARCK.md` — Testing and benchmarks
- `skills/README.md` — Skills registry and execution flow

## SSOT Alignment

| SSOT Document | Skill | Alignment |
|---------------|-------|-----------|
| PRD.md §3 | kcm-database-engine-specialist | Core types definition |
| PRD.md §5 | kcm-database-engine-specialist | Query engine operators |
| PRD2.md §15 | kcm-database-engine-specialist | Storage format |
| PRD2.md §16 | kcm-performance-engineer | Optimizer cost model |
| PRD2.md §18 | kcm-database-engine-specialist | Runtime, transactions |
| PRD2.md §19 | kcm-database-engine-specialist | Interfaces (FFI, REST, gRPC) |
| PRD3.md §27 | kcm-architecture-guardian | Distributed architecture |
| PRD3.md §30 | kcm-security-engineer | Security (RBAC, encryption) |
| PRD3.md §32 | kcm-security-engineer | Compliance (GDPR) |
| PRD-TESTING§1-8 | kcm-testing-verification | Test strategy, quality gates |
| PRD-TESTING§4 | kcm-performance-engineer | Benchmark suite |
