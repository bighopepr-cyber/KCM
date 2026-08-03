# Repository Governance

| Field | Value |
|-------|-------|
| **Document ID** | KCM-REPO-011 |
| **Title** | Repository Governance |
| **Version** | 1.0.0 |
| **Date** | 2026-08-03 |
| **Status** | Authoritative |
| **Authority** | Engineering Orchestrator (P1) |

---

## 1. Decision-Making Process

| Decision Type | Authority | Approval Required |
|---------------|-----------|-------------------|
| Architecture changes | Architecture Guardian (P5) | 2 approvals |
| Specification changes | Specification Lock (P4) | 2 approvals |
| Code changes | Code Quality Guardian (P10) | 1 approval |
| Security changes | Security Engineer (P7) | 2 approvals |
| Release decisions | Release Readiness (P12) | 3 approvals |

## 2. PR Review Requirements

- **Critical paths** (core, storage, security): 2 approvals required
- **Standard paths** (tools, tests, docs): 1 approval required
- **CODEOWNERS** file determines required reviewers

## 3. Merge Criteria

All of the following must pass before merge:

1. `cargo build --workspace` passes
2. `cargo test --workspace` passes
3. `cargo clippy --workspace -- -D warnings` passes
4. `cargo fmt --all -- --check` passes
5. Required reviews obtained
6. No unresolved conversations
7. CI pipeline green

## 4. Branch Protection

| Branch | Protection Level |
|--------|-----------------|
| main | Full protection (2 approvals, CI required) |
| develop | Standard protection (1 approval, CI required) |
| release/* | Standard protection |
| feature/* | No protection |

## 5. Contribution Guidelines

1. Fork the repository
2. Create feature branch from `develop`
3. Make changes following coding standards
4. Write tests for new functionality
5. Update documentation if needed
6. Submit PR with clear description
7. Address review feedback
8. Merge after approval
