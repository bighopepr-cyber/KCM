# Documentation Governance

> **Document Version:** 1.0.0
> **Status:** Active
> **Last Updated:** 2026-08-06
> **Owner:** Engineering Orchestrator (P1)
> **Reviewers:** All Guardians (P4-P13)
> **SSOT Authority:** AGENTS.md § Engineering Constitution

---

## Table of Contents

- [1. Overview](#1-overview)
- [2. Ownership Model](#2-ownership-model)
- [3. Review Workflow](#3-review-workflow)
- [4. Approval Workflow](#4-approval-workflow)
- [5. Documentation Lifecycle](#5-documentation-lifecycle)
- [6. Deprecation Policy](#6-deprecation-policy)
- [7. Update Policy](#7-update-policy)
- [8. Version Policy](#8-version-policy)
- [9. Release Policy](#9-release-policy)
- [10. Quality Standards](#10-quality-standards)
- [11. Enforcement](#11-enforcement)
- [12. Metrics and Reporting](#12-metrics-and-reporting)
- [13. References](#13-references)

---

## 1. Overview

The KCM Documentation Governance System ensures all documentation remains consistent, synchronized with source code, SSOT-compliant, automatically validated, auditable, enterprise-ready, open-source ready, and free from documentation drift. Documentation is treated as first-class source code.

### 1.1 Principles

| Principle | Description |
|-----------|-------------|
| Documentation as Code | All documentation is version controlled, reviewed, and tested like source code |
| Single Source of Truth | One authoritative source for every piece of information |
| SSOT Alignment | Every document traces to an SSOT requirement |
| Automated Validation | CI enforces documentation quality gates |
| Auditability | All documentation changes are traceable and auditable |
| Enterprise-grade | Professional, complete, accurate, and maintainable |
| Open Source Ready | Clear, accessible, welcoming to contributors |
| Zero Drift | Documentation never diverges from implementation |

### 1.2 Scope

This governance document applies to:

- All SSOT documents (`docs/PRD*.md`)
- Crate-level documentation (`README.md`, `SECURITY.md`, `CONTRIBUTING.md`)
- Crate specifications (`*SPEC.md`)
- SDK documentation
- Deployment and CI/CD documentation
- Architecture Decision Records (ADRs)
- Documentation templates
- The Documentation Index (`docs/INDEX.md`)

### 1.3 Compliance

All KCM contributors **MUST** comply with this governance document. Non-compliant documentation changes will be rejected by the CI pipeline and review process.

---

## 2. Ownership Model

### 2.1 Document Ownership Matrix

| Document Type | Owner | Reviewer | Approver |
|--------------|-------|----------|----------|
| SSOT (`SSOT.md`) | Specification Lock (P4) | Architecture Guardian (P5) | Engineering Orchestrator (P1) |
| PRD Documents | Specification Lock (P4) | Architecture Guardian (P5) | Engineering Orchestrator (P1) |
| Crate README | Crate Owner | Code Quality Guardian (P10) | Code Review Auditor (P13) |
| Crate SECURITY | Security Engineer (P7) | Specification Lock (P4) | Engineering Orchestrator (P1) |
| Crate CONTRIBUTING | Code Quality Guardian (P10) | Documentation Guardian (P11) | Release Readiness (P12) |
| Crate Specifications | Documentation Guardian (P11) | Domain Specialist | Architecture Guardian (P5) |
| SDK Docs | SDK Owner | Testing Verification (P9) | Documentation Guardian (P11) |
| Deployment Docs | DevOps Engineer | Security Engineer (P7) | Release Readiness (P12) |
| CI/CD Docs | CI/CD Engineer | DevOps Engineer | Release Readiness (P12) |
| ADR | Decision Record Owner | Architecture Guardian (P5) | Engineering Orchestrator (P1) |
| Templates | Documentation Guardian (P11) | All Guardians | Engineering Orchestrator (P1) |

### 2.2 Ownership Responsibilities

| Responsibility | Description |
|---------------|-------------|
| Maintain accuracy | Ensure document reflects current implementation at all times |
| Review changes | Review all changes to owned documents within SLA |
| Approve releases | Approve document for production use |
| Track SSOT alignment | Ensure document traces to SSOT requirements |
| Deprecate outdated | Mark and handle deprecated documents per deprecation policy |
| Version management | Manage document versioning per version policy |
| Cross-reference integrity | Ensure all internal links and references are valid |
| Stakeholder communication | Notify affected parties of significant changes |

### 2.3 Ownership Transfer

When an owner leaves or changes role:

1. Current owner identifies successor
2. Successor is briefed on owned documents
3. Ownership is transferred in the document metadata
4. Transfer is logged in the audit trail
5. Engineering Orchestrator (P1) approves transfer

---

## 3. Review Workflow

### 3.1 Standard Review Process

```
1. Author creates/updates document
2. Self-review: headings, links, formatting
3. Automated validation: CI pipeline
4. Peer review: domain expert
5. SSOT review: specification alignment
6. Security review: if security-relevant
7. Final approval: owner
8. Merge
```

### 3.2 Review SLA

| Document Type | Review SLA | Approval SLA |
|--------------|-----------|-------------|
| SSOT changes | 48 hours | 72 hours |
| Security docs | 24 hours | 48 hours |
| API docs | 24 hours | 48 hours |
| README | 12 hours | 24 hours |
| Templates | 48 hours | 72 hours |
| ADR | 48 hours | 72 hours |
| Other | 24 hours | 48 hours |

### 3.3 Review Criteria

Every documentation review **MUST** evaluate:

| Criterion | Description |
|-----------|-------------|
| Accuracy | Content matches current implementation |
| Completeness | All required sections are present |
| Clarity | Content is clear and unambiguous |
| SSOT alignment | Content aligns with SSOT specifications |
| Link integrity | All internal and external links are valid |
| Formatting | Follows documentation style guide |
| Metadata | Frontmatter is complete and accurate |
| Cross-references | Related documents are properly referenced |

### 3.4 Review Assignments

Reviewers are assigned based on the ownership matrix:

- **Automatic assignment**: CI assigns the default reviewer from the ownership matrix
- **Manual assignment**: Author may request additional reviewers
- **Escalation**: If review SLA is breached, escalate to Engineering Orchestrator (P1)

---

## 4. Approval Workflow

### 4.1 Approval Matrix

| Change Type | Required Approvals | Auto-merge |
|------------|-------------------|------------|
| Typo fix | 1 reviewer | No |
| Content update | 1 reviewer + owner | No |
| New document | 2 reviewers + owner | No |
| SSOT change | Specification Lock + Architecture Guardian | No |
| Security doc | Security Engineer + Specification Lock | No |
| Template change | Documentation Guardian + 1 reviewer | No |
| Breaking change | All guardians | No |

### 4.2 Approval Process

1. All required reviewers have approved
2. All CI checks pass
3. No unresolved review comments
4. Owner confirms approval
5. Documentation Guardian (P11) verifies standards compliance
6. Release Readiness (P12) confirms release gate (for release-blocking docs)

### 4.3 Approval Authority

| Approver | Authority |
|----------|-----------|
| Engineering Orchestrator (P1) | Final authority on all documentation |
| Specification Lock (P4) | Authority on SSOT and specification changes |
| Architecture Guardian (P5) | Authority on architectural documentation |
| Documentation Guardian (P11) | Authority on documentation standards |
| Release Readiness (P12) | Authority on release-blocking documentation |

---

## 5. Documentation Lifecycle

### 5.1 Lifecycle Stages

```
Draft → Review → Approved → Published → Maintained → Deprecated → Archived
```

| Stage | Description | Quality Gate |
|-------|-------------|-------------|
| Draft | Initial creation, work in progress | None |
| Review | Under review by assigned reviewers | CI passes |
| Approved | Approved for use, awaiting merge | All reviews pass |
| Published | Available to users, merged to main | Release process |
| Maintained | Actively maintained and updated | Regular updates |
| Deprecated | No longer maintained, replacement exists | Deprecation notice |
| Archived | Historical reference only | Moved to archive |

### 5.2 Lifecycle Transitions

| From | To | Trigger |
|------|-----|---------|
| Draft | Review | Author submits PR |
| Review | Approved | All reviews pass |
| Review | Draft | Reviewer requests changes |
| Approved | Published | Merge to main |
| Published | Maintained | Regular updates applied |
| Maintained | Deprecated | Replacement identified and approved |
| Deprecated | Archived | 6 months after deprecation |

### 5.3 Stage Requirements

| Stage | Requirements |
|-------|-------------|
| Draft | Title, owner, initial content |
| Review | Complete content, metadata, self-review passed |
| Approved | All reviews approved, CI passes |
| Published | Release process completed, index updated |
| Maintained | Regular update schedule, accuracy verified |
| Deprecated | Deprecation banner, replacement link, migration guide |
| Archived | Archived location, no active references |

---

## 6. Deprecation Policy

### 6.1 Deprecation Process

1. Mark document as deprecated with `DEPRECATED` banner
2. Add deprecation notice with replacement reference
3. Update all references to point to replacement
4. Keep deprecated document for 6 months
5. Archive deprecated document
6. Remove from active index

### 6.2 Deprecation Template

```markdown
> **DEPRECATED** — This document is deprecated as of [DATE].
> Replacement: [LINK]
> Migration guide: [LINK]
> This document will be archived on [DATE + 6 months].
```

### 6.3 Deprecation Requirements

| Requirement | Description |
|-------------|-------------|
| Notice period | Minimum 6 months before archival |
| Replacement | Must identify a replacement document |
| Migration guide | Must provide migration guidance |
| Reference update | All references must be updated |
| Index removal | Must be removed from active index |
| Audit log | Deprecation must be logged in audit trail |

---

## 7. Update Policy

### 7.1 Update Triggers

| Trigger | Action | Priority |
|---------|--------|----------|
| Source code change | Update affected docs | High |
| API change | Update API docs | Critical |
| New feature | Create/update docs | High |
| Bug fix | Update if behavior changed | Medium |
| Dependency change | Update dependency docs | Medium |
| Security fix | Update security docs | Critical |
| SSOT change | Update all downstream | Critical |

### 7.2 Update SLA

| Change Impact | Update SLA |
|--------------|-----------|
| Critical (security, API breaking) | 24 hours |
| High (new feature, behavior change) | 48 hours |
| Medium (dependency, config) | 1 week |
| Low (typo, formatting) | Next release |

### 7.3 Update Validation

Every documentation update must pass:

1. **Automated validation**: CI pipeline checks
2. **Link validation**: All links must be valid
3. **SSOT alignment**: Content must align with SSOT
4. **Formatting check**: Must follow style guide
5. **Metadata check**: Frontmatter must be complete

---

## 8. Version Policy

### 8.1 Versioning Rules

| Change Type | Version Bump | Example |
|-------------|-------------|---------|
| Typo fix | No bump | — |
| Content update | Patch (0.0.x) | 1.0.0 → 1.0.1 |
| New section | Minor (0.x.0) | 1.0.0 → 1.1.0 |
| Restructure | Minor (0.x.0) | 1.0.0 → 1.1.0 |
| Breaking change | Major (x.0.0) | 1.0.0 → 2.0.0 |
| SSOT update | Major (x.0.0) | — |

### 8.2 Version Metadata

Every document must include:

```yaml
---
title: Document Title
version: 1.0.0
status: Active|Deprecated|Draft
last_updated: YYYY-MM-DD
maintainer: @owner
owner: Team/Individual
category: Security|API|Architecture|...
audience: Contributors|Users|Operators
dependencies: [list of dependent docs]
related: [list of related docs]
ssot_authority: SSOT requirement ID
---
```

### 8.3 Version History

Every document must maintain a version history:

```markdown
## Version History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0.0 | 2026-08-06 | @author | Initial release |
| 1.1.0 | 2026-08-13 | @author | Added new section |
```

---

## 9. Release Policy

### 9.1 Release Process

1. All CI checks pass
2. All reviews approved
3. Version bumped if needed
4. Changelog updated
5. Index updated
6. Metrics updated
7. Merge to main
8. Post-merge validation

### 9.2 Release Validation

Before any documentation release:

- [ ] Documentation validation script passes
- [ ] No broken links
- [ ] SSOT alignment verified
- [ ] Coverage meets threshold
- [ ] All templates followed
- [ ] Metadata is complete
- [ ] Version history is updated
- [ ] Changelog is updated

### 9.3 Release Blocking

The following documentation issues **BLOCK** releases:

| Issue | Severity | Blocking |
|-------|----------|----------|
| SSOT misalignment | Critical | Yes |
| Broken links | High | Yes |
| Missing metadata | Medium | Yes |
| Style violations | Low | No |
| Outdated content | Medium | Conditional |

---

## 10. Quality Standards

### 10.1 Documentation Quality Matrix

| Standard | Requirement | Enforcement |
|----------|------------|-------------|
| Enterprise-grade | Professional, complete, accurate | Review |
| Open Source Ready | Clear, accessible, welcoming | Review |
| Rust Foundation Style | Idiomatic Rust documentation | Lint |
| CNCF Style | Kubernetes-style documentation | Lint |
| Diátaxis Framework | Tutorials, how-to, reference, explanation | Review |
| Docs as Code | Version controlled, reviewed, tested | CI |
| SSOT | Single source of truth, no duplication | Validation |

### 10.2 Quality Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Link validity | 100% | Automated link checker |
| Metadata completeness | 100% | Automated validation |
| SSOT alignment | 100% | SSOT validation script |
| Review coverage | 100% | PR review tracking |
| Update freshness | < 30 days | Last updated date |
| Template compliance | 100% | Automated validation |

### 10.3 Quality Gates

Every documentation PR must pass these quality gates:

1. **Format gate**: `cargo fmt --all -- --check` (for Rust docs)
2. **Lint gate**: `cargo clippy --workspace -- -D warnings` (for Rust docs)
3. **Link gate**: All links valid
4. **Metadata gate**: All required metadata present
5. **SSOT gate**: SSOT alignment verified
6. **Template gate**: Template compliance verified
7. **Review gate**: All required reviews approved

---

## 11. Enforcement

### 11.1 Automated Enforcement

| Mechanism | Scope | Action |
|-----------|-------|--------|
| CI pipeline | All PRs | Validates all documentation rules |
| Pre-commit hooks | Local development | Checks formatting before commit |
| Link checker | All PRs | Validates all links |
| Coverage checker | All PRs | Blocks merge below threshold |
| SSOT validator | All PRs | Checks SSOT alignment |
| Template validator | All PRs | Validates template compliance |
| Metadata validator | All PRs | Validates frontmatter |

### 11.2 Manual Enforcement

| Enforcer | Scope | Action |
|----------|-------|--------|
| Code owners | Documentation changes | Review and approve changes |
| Documentation Guardian (P11) | All documentation | Enforce standards |
| Release Readiness (P12) | Release-blocking docs | Gate documentation quality |
| Engineering Orchestrator (P1) | All documentation | Final authority |

### 11.3 Violation Handling

| Violation | Severity | Action |
|-----------|----------|--------|
| Missing metadata | Medium | Block merge, require fix |
| Broken link | High | Block merge, require fix |
| SSOT misalignment | Critical | Block merge, require fix |
| Style violation | Low | Warn, fix in next release |
| Missing review | High | Block merge, require review |
| Template non-compliance | Medium | Block merge, require compliance |

---

## 12. Metrics and Reporting

### 12.1 Documentation Metrics

| Metric | Description | Target |
|--------|-------------|--------|
| Documentation coverage | % of crates with complete documentation | 100% |
| Link validity | % of valid links | 100% |
| SSOT alignment | % of docs aligned with SSOT | 100% |
| Update freshness | Average days since last update | < 30 days |
| Review compliance | % of changes reviewed within SLA | 100% |
| Template compliance | % of docs following templates | 100% |

### 12.2 Reporting

| Report | Frequency | Owner |
|--------|-----------|-------|
| Documentation health | Weekly | Documentation Guardian (P11) |
| SSOT compliance | Every PR | CI pipeline |
| Link validity | Every PR | CI pipeline |
| Update freshness | Monthly | Documentation Guardian (P11) |
| Quality metrics | Monthly | Engineering Orchestrator (P1) |

### 12.3 Dashboards

- **Documentation Health Dashboard**: Real-time documentation quality metrics
- **SSOT Compliance Dashboard**: SSOT alignment status
- **Link Validity Dashboard**: Link checker results
- **Update Freshness Dashboard**: Documentation update recency

---

## 13. References

- [SSOT.md](../../SSOT.md) — Single Source of Truth
- [AGENTS.md](../../AGENTS.md) — Engineering constitution
- [CONTRIBUTING.md](../../CONTRIBUTING.md) — Contribution guidelines
- [docs/INDEX.md](../INDEX.md) — Documentation index
- [kcm-documentation-guardian](../../skills/kcm-documentation-guardian/SKILL.md) — Documentation Guardian skill
- [kcm-specification-lock](../../skills/kcm-specification-lock/SKILL.md) — Specification Lock skill
- [kcm-architecture-guardian](../../skills/kcm-architecture-guardian/SKILL.md) — Architecture Guardian skill
- [kcm-release-readiness](../../skills/kcm-release-readiness/SKILL.md) — Release Readiness skill

---

*This document is maintained by the Documentation Guardian (P11) and approved by the Engineering Orchestrator (P1). All changes must follow the governance process defined herein.*
