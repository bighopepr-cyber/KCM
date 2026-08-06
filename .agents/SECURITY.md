# .agents/ Security Policy

> For project-wide security policies, refer to the [SECURITY.md](../SECURITY.md) located in the repository root.

## Overview

This document defines the security policy for the `.agents/` directory, which contains AI agent governance configuration for the KCM project. Security of this directory ensures that AI agents operate within defined governance boundaries and cannot bypass engineering standards.

## Security Scope

- **Skill instructions**: Integrity and authenticity of `SKILL.md` files defining agent governance rules
- **Agent permissions**: Ensuring AI agents respect authority hierarchy and cannot escalate privileges through modified skill definitions

## Threat Model

| Threat | Description | Risk Level |
|--------|-------------|------------|
| Skill modification to bypass governance | An adversary modifies `SKILL.md` files to remove quality gates or governance constraints | High |
| Unauthorized agent access | AI agents access skill definitions they should not have permissions for | Medium |
| Skill content injection | Malicious instructions injected into skill files to alter agent behavior | High |
| Structural desynchronization | `.agents/skills/` diverges from `skills/`, causing agents to use stale governance rules | Medium |

## Security Risks

| Risk | Impact | Mitigation |
|------|--------|------------|
| Modified skill file bypasses code quality checks | Substandard code reaches production | Integrity validation of skill files |
| Agent executes unauthorized governance escalation | Agent applies rules outside its authority | Authority hierarchy enforcement |
| Stale skill definitions applied | Agents use outdated governance rules | Structural consistency verification |
| Skill file contains sensitive data exposure | Secrets leaked through skill instructions | No secrets policy in skill files |

## Access Control

- **Skill files are read-only during execution**: AI agents may read `SKILL.md` files but must not modify them during task execution
- **Write access restricted**: Only authorized engineers may modify `.agents/skills/` content
- **Version control**: All changes to `.agents/` are tracked through git history

## RBAC Integration

The `.agents/` directory integrates with KCM's RBAC system through the authority hierarchy defined in each `SKILL.md`:

| Authority Level | Skill | Access Scope |
|----------------|-------|-------------|
| P1 | `kcm-engineering-orchestrator` | Master coordinator — overrides all |
| P2 | `kcm-task-planner` | Task planning authority |
| P3 | `kcm-change-impact-analysis` | Change impact assessment |
| P4 | `kcm-specification-lock` | Specification authority — can VETO |
| P5 | `kcm-architecture-guardian` | Architecture authority |
| P6 | `kcm-database-engine-specialist` | Database engine authority |
| P7 | `kcm-security-engineer` | Security authority |
| P8 | `kcm-performance-engineer` | Performance authority |
| P9 | `kcm-testing-verification` | Testing authority |
| P10 | `kcm-code-quality-guardian` | Code quality authority |
| P11 | `kcm-documentation-guardian` | Documentation authority |
| P12 | `kcm-release-readiness` | Release authority |
| P13 | `kcm-code-review-auditor` | Code review authority |
| P14 | `kcm-debugging-root-cause` | Debugging authority |
| P15 | `kcm-engineering-decision-record` | Decision record authority |
| P16 | `kcm-repository-intelligence` | Repository intelligence |

## Sensitive Assets

| Asset | Sensitivity | Protection |
|-------|-------------|------------|
| `SKILL.md` files | High | Integrity validation, version control |
| Skill authority definitions | High | Authority hierarchy enforcement |
| Governance rules | High | No modification without approval |

## Secret Management

- **No secrets in skill files**: `SKILL.md` files must not contain API keys, tokens, passwords, or any sensitive credentials
- **No hardcoded paths**: Skill files reference logical paths, not environment-specific paths
- **Environment separation**: Any configuration values used by skills must be provided at runtime, not embedded in skill definitions

## Secure Development Rules

### Skill Integrity Validation

All skill files must pass integrity validation before deployment:

```bash
# Verify skill file integrity
for skill in .agents/skills/*/SKILL.md; do
  if [ -f "$skill" ]; then
    echo "Validating: $skill"
    # Check for no secrets
    grep -qE "(password|secret|token|api_key|private_key)" "$skill" && \
      echo "SECURITY: Potential secret found in $skill" && exit 1
  fi
done
```

### No Secrets in Skill Files

Before any skill file is committed:

1. Scan for potential secrets (passwords, tokens, API keys)
2. Verify no environment-specific paths are hardcoded
3. Ensure no credentials are embedded in governance rules

### Audit Trail for Skill Changes

All modifications to `.agents/` files must be traceable:

1. Changes are committed with descriptive commit messages
2. Commit history provides complete audit trail
3. Code review required before merging skill changes

## Audit Logging

| Event | Log Level | Description |
|-------|-----------|-------------|
| Skill file modification | INFO | Record of any change to `.agents/skills/` |
| Integrity validation failure | ERROR | Detected attempt to modify skill files with invalid content |
| Unauthorized access attempt | WARN | Agent or user attempting to access restricted skill definitions |
| Structural desynchronization | WARN | Detected divergence between `skills/` and `.agents/skills/` |

## Validation Checklist

- [ ] No secrets in any `SKILL.md` file
- [ ] No hardcoded environment paths
- [ ] All 16 skills present in `.agents/skills/`
- [ ] Structural consistency with `skills/` directory verified
- [ ] No unauthorized authority escalation in skill definitions
- [ ] Skill files are read-only during agent execution
- [ ] Changes tracked in version control
- [ ] Code review completed for all skill modifications

## References

- `skills/` — Source directory for governance skill definitions
- `AGENTS.md` — Engineering constitution defining skill authority hierarchy
- `SECURITY.md` (repository root) — Project-wide security policy
- `docs/agents/spesifikasi.md` — Technical specification for agents configuration