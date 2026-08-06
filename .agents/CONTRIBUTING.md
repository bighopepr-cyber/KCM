# Contributing to .agents/

> For core engine contribution rules, refer to the repository [CONTRIBUTING.md](../CONTRIBUTING.md).

## Overview

This document provides guidelines for contributing to the `.agents/` directory, which contains AI agent governance configuration mirroring the `skills/` directory. Contributors must ensure structural and content consistency with the source `skills/` directory.

## Before Contributing

1. Review the repository [CONTRIBUTING.md](../CONTRIBUTING.md) for general contribution rules
2. Understand the purpose of `.agents/` as a mirror of `skills/` for AI agent governance
3. Verify your change is necessary and does not duplicate existing content
4. Check that the corresponding `skills/` directory entry exists or is being updated simultaneously

## Coding Standards

### Markdown Formatting

- Use standard Markdown syntax throughout all files
- Use `#` for top-level headings, `##` for sections, `###` for subsections
- Use code blocks with language identifiers for code examples
- Use tables for structured data presentation
- Maintain consistent indentation (2 spaces)

### Consistent Structure

- All `SKILL.md` files must follow the same structural pattern as their `skills/` counterparts
- Section headings must be consistent across all skill files
- File paths must be relative and consistent

## Module Architecture Rules

### Mirrors skills/ Structure Exactly

The `.agents/skills/` directory must maintain an exact structural mirror of the `skills/` directory:

```
.agents/skills/
├── kcm-architecture-guardian/SKILL.md        ← mirrors skills/kcm-architecture-guardian/SKILL.md
├── kcm-change-impact-analysis/SKILL.md       ← mirrors skills/kcm-change-impact-analysis/SKILL.md
├── kcm-code-quality-guardian/SKILL.md        ← mirrors skills/kcm-code-quality-guardian/SKILL.md
├── kcm-code-review-auditor/SKILL.md          ← mirrors skills/kcm-code-review-auditor/SKILL.md
├── kcm-database-engine-specialist/SKILL.md   ← mirrors skills/kcm-database-engine-specialist/SKILL.md
├── kcm-debugging-root-cause/SKILL.md         ← mirrors skills/kcm-debugging-root-cause/SKILL.md
├── kcm-documentation-guardian/SKILL.md       ← mirrors skills/kcm-documentation-guardian/SKILL.md
├── kcm-engineering-decision-record/SKILL.md  ← mirrors skills/kcm-engineering-decision-record/SKILL.md
├── kcm-engineering-orchestrator/SKILL.md     ← mirrors skills/kcm-engineering-orchestrator/SKILL.md
├── kcm-performance-engineer/SKILL.md         ← mirrors skills/kcm-performance-engineer/SKILL.md
├── kcm-release-readiness/SKILL.md            ← mirrors skills/kcm-release-readiness/SKILL.md
├── kcm-repository-intelligence/SKILL.md      ← mirrors skills/kcm-repository-intelligence/SKILL.md
├── kcm-security-engineer/SKILL.md            ← mirrors skills/kcm-security-engineer/SKILL.md
├── kcm-specification-lock/SKILL.md           ← mirrors skills/kcm-specification-lock/SKILL.md
├── kcm-task-planner/SKILL.md                 ← mirrors skills/kcm-task-planner/SKILL.md
└── kcm-testing-verification/SKILL.md         ← mirrors skills/kcm-testing-verification/SKILL.md
```

### No Additions Without skills/ Mirror

- Never add a skill to `.agents/skills/` without a corresponding entry in `skills/`
- Never remove a skill from `.agents/skills/` without a corresponding removal from `skills/`
- Always update both directories simultaneously

## Documentation Rules

- All files must include proper section headings
- References to external documents must use relative paths
- Code examples must include language identifiers
- Tables must use consistent column alignment

## Testing Requirements

### Skill Consistency with skills/

Before submitting changes, verify consistency:

```bash
# Verify structural consistency
diff -rq skills/ .agents/skills/

# Verify file count matches
echo "skills/: $(ls skills/ | wc -l) directories"
echo ".agents/skills/: $(ls .agents/skills/ | wc -l) directories"
# Both should output 16

# Verify all SKILL.md files exist
for skill in skills/*/; do
  skill_name=$(basename "$skill")
  if [ ! -f ".agents/skills/$skill_name/SKILL.md" ]; then
    echo "MISSING: .agents/skills/$skill_name/SKILL.md"
  fi
done
```

### Content Validation

- Verify no secrets or credentials in skill files
- Ensure all file paths are relative and valid
- Check that authority levels are consistent with AGENTS.md

## Performance Rules

- Skill files should be concise and focused
- Avoid unnecessary verbosity in governance rules
- Use structured data (tables, lists) over prose where possible

## Review Checklist

Before submitting a pull request:

- [ ] Corresponding `skills/` directory entry exists or is updated simultaneously
- [ ] Structural consistency with `skills/` verified
- [ ] No secrets or credentials in any file
- [ ] Markdown formatting follows standards
- [ ] Section headings are consistent
- [ ] References use relative paths
- [ ] Code examples include language identifiers
- [ ] All 16 skills present (if modifying structure)

## Pull Request Requirements

1. PR title clearly describes the change to `.agents/`
2. PR description explains why the change is needed
3. PR includes verification that `skills/` and `.agents/skills/` remain in sync
4. PR passes all CI checks
5. PR receives approval from at least one reviewer

## References

- `skills/` — Source directory that `.agents/skills/` mirrors
- `CONTRIBUTING.md` (repository root) — Core engine contribution rules
- `AGENTS.md` — Engineering constitution
- `docs/agents/spesifikasi.md` — Technical specification for agents configuration