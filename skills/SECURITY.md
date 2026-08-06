# skills/ Security Policy

> For project-wide security policies, refer to the [SECURITY.md](../SECURITY.md) located in the repository root.

## Overview

The `skills/` directory contains 16 AI engineering skills that define security governance, authority boundaries, and quality enforcement for KCM development. Skills themselves must be protected from tampering and misuse.

## Security Scope

The `kcm-security-engineer` skill defines all security rules for the KCM project. This policy covers the security of the skills infrastructure itself.

| Component | Security Concern |
|-----------|-----------------|
| Skill definitions | Unauthorized modification of skill instructions |
| Authority hierarchy | Privilege escalation through skill manipulation |
| Engineering gates | Bypassing security gates via skill tampering |
| Skill invocation | Injection of malicious instructions through skill loading |

## Threat Model

| Threat | Severity | Mitigation |
|--------|----------|------------|
| Skill modification to bypass security gates | Critical | Skills are read-only during execution; changes require review |
| Authority escalation via skill priority manipulation | High | Priority levels are fixed in AGENTS.md; not modifiable by skills |
| Injection of malicious instructions through skill loading | High | Skills are loaded from verified filesystem locations |
| Skill conflict resolution bypass | Medium | kcm-engineering-orchestrator (P1) resolves all conflicts |

## Security Risks

| Risk | Description | Mitigation |
|------|-------------|------------|
| Skill tampering | Modifying SKILL.md to bypass security checks | Git history tracking, code review required |
| Authority boundary violations | Skill operating outside its defined authority | P1 orchestrator enforces boundaries |
| SSOT divergence | Skill instructions diverging from SSOT specifications | Automated validation via `validate-ssot.sh` |

## Access Control

| Action | Requirement |
|--------|-------------|
| Read skill definitions | No restriction |
| Modify skill definitions | Requires code review and approval |
| Delete skill definitions | Prohibited without engineering-orchestrator approval |
| Bypass skill authority | Prohibited — authority hierarchy is immutable |

## RBAC Integration

The `kcm-security-engineer` skill enforces RBAC rules across all KCM operations:

| Permission Level | Skill Authority | Description |
|-----------------|-----------------|-------------|
| L1 (Read) | P16, P15, P14 | Read-only access to codebase understanding |
| L2 (Write) | P13, P11, P10 | Code review, documentation, quality enforcement |
| L3 (Execute) | P9, P8, P7 | Testing, performance, security enforcement |
| L4 (Admin) | P6, P5, P4, P3, P2 | Storage, architecture, specification, impact, planning |
| L5 (Override) | P1 | Engineering orchestrator — master coordinator |

## Sensitive Assets

| Asset | Sensitivity | Protection |
|-------|-------------|------------|
| Skill definitions (SKILL.md) | High | Git tracking, review requirement |
| Authority hierarchy | Critical | Fixed in AGENTS.md, immutable by skills |
| SSOT documents | Critical | Specification Lock (P4) owns all changes |
| Security rules | Critical | Security Engineer (P7) owns all changes |

## Secret Management

Skills do not store secrets. All secrets (encryption keys, credentials) are managed by the `kcm-security-engineer` skill and the `kcm-security` crate.

## Secure Development Rules

| Rule | Description |
|------|-------------|
| Skill modifications require review | All changes to SKILL.md files must go through code review |
| No skill can override security-engineer | The `kcm-security-engineer` skill has authority over all security decisions |
| No skill can modify frozen contracts | Only `kcm-specification-lock` (P4) can modify SSOT specifications |
| Authority hierarchy is immutable | Priority levels cannot be changed by any skill |
| Skills are read-only during execution | AI agents load skills but cannot modify them at runtime |

## Audit Logging

| Event | Logged By | Retention |
|-------|-----------|-----------|
| Skill modification | Git history | Permanent |
| Authority boundary violation | kcm-engineering-orchestrator | Session |
| Security gate bypass attempt | kcm-security-engineer | Permanent |
| SSOT divergence detected | kcm-specification-lock | Session |

## Validation Checklist

- [ ] All skill definitions are tracked in git
- [ ] No skill can bypass security-engineer authority
- [ ] Authority hierarchy matches AGENTS.md
- [ ] No secrets stored in skill definitions
- [ ] Skill modifications require code review
- [ ] `validate-ssot.sh` passes for all skills
- [ ] No unwrap() or panic!() in any skill-related code

## References

- `AGENTS.md` — Engineering constitution and authority hierarchy
- `SECURITY.md` — Project-wide security policy
- `kcm-security-engineer/SKILL.md` — Security skill definition
- `kcm-specification-lock/SKILL.md` — Specification lock skill definition
