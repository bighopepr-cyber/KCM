# Security Engineer

> Document ID: KCM-SKILL-007 | Version: 2.0.0 | Status: Active

## Overview

Ensure KCM's security implementation is cryptographically correct, follows security best practices, and protects against known attack vectors. This skill validates encryption, RBAC, audit logging, GDPR compliance, data classification, and gRPC/TLS transport security.

## Mission

Guarantee AES-256-GCM encryption with BLAKE3 KDF, zero hardcoded keys, RBAC with no privilege escalation, immutable audit events, and TLS-enforced gRPC transport across all security-sensitive code paths.

## Responsibilities

| # | Responsibility | Description |
|---|---------------|-------------|
| 1 | Encryption Validation | Verify AES-256-GCM with BLAKE3 KDF, CSPRNG key generation, 12-byte random nonce, AEAD |
| 2 | RBAC Enforcement | Validate 5 permission levels, ACL → Role → Deny algorithm, context isolation, no privilege escalation |
| 3 | Audit Log Integrity | Ensure hash-chained audit events, O(1) eviction via VecDeque, 100K capacity, immutability |
| 4 | Key Management | Verify keys derived via KDF, never logged or stored in code, zeroized on drop |
| 5 | GDPR Compliance | Validate consent verification before operations, 6 operations, data subject rights |
| 6 | Data Classification | Enforce 4-tier classification system with retention policies |
| 7 | Transport Security | Ensure TLS required for gRPC, certificate validation, authentication on all endpoints |
| 8 | Threat Assessment | Assess security impact of all changes touching security-sensitive code |

## Authority

| Priority | Authority Level | Blocking Authority | Approval Authority | Escalation |
|----------|----------------|-------------------|-------------------|------------|
| P7 | Security Engineer | Block security and compliance violations | Approve security changes | Escalate to P4 (Spec Lock) or P1 (Orchestrator) |

## Scope

| In Scope | Out of Scope |
|----------|-------------|
| kcm-security: encryption.rs, rbac.rs, audit.rs | General code quality review |
| kcm-compliance: gdpr.rs, data_classification.rs | Architecture-level decisions |
| kcm-server: grpc_server.rs, grpc_main.rs (TLS, auth) | Performance optimization |
| All security-sensitive code paths across crates | Functional test writing |
| gRPC proto definitions (security implications) | Query engine correctness |
| Input validation on all public interfaces | Storage format validation |
| Null-pointer guards on all FFI functions | Documentation authoring |

## Non Goals

1. Reviewing general code quality or style (Code Quality Guardian responsibility)
2. Writing functional unit or integration tests (Testing Skill responsibility)
3. Architecture-level decisions (Architecture Guardian responsibility)
4. Code design quality review (Code Review Auditor responsibility)
5. Performance optimization of security code (Performance Engineer responsibility)
6. Authoring security documentation (Documentation Guardian responsibility)

## Inputs

| Input | Source | Required |
|-------|--------|----------|
| KCM_SECURITY_TRUST_SPEC.md | docs/ directory | Yes |
| crates/kcm-security/src/encryption.rs | Source | Yes (for encryption changes) |
| crates/kcm-security/src/rbac.rs | Source | Yes (for RBAC changes) |
| crates/kcm-security/src/audit.rs | Source | Yes (for audit changes) |
| crates/kcm-compliance/src/gdpr.rs | Source | Yes (for compliance changes) |
| crates/kcm-compliance/src/data_classification.rs | Source | Yes (for classification changes) |
| crates/kcm-server/src/grpc_server.rs | Source | Yes (for gRPC security changes) |
| crates/kcm-interface/proto/kcm.proto | Source | Yes (for proto changes) |

## Outputs

| Output | Format | Destination |
|--------|--------|-------------|
| Security assessment report | Markdown report with tables | Engineering Orchestrator (P1) |
| Cryptographic verification | Algorithm/key/nonce checklist | Release pipeline |
| Vulnerability report | List of security findings | Security team and P1 |

## Workflow

```
1. Receive security-related change request
2. Read KCM_SECURITY_TRUST_SPEC.md
3. Perform threat assessment for the change
4. Verify encryption algorithm is AES-256-GCM (not XOR, not AES-CBC)
5. Verify key derivation uses BLAKE3 KDF (not raw hash)
6. Verify key generation uses CSPRNG (not time-based)
7. Verify nonce is 12 bytes and random
8. Verify encryption provides AEAD (confidentiality + integrity)
9. Verify RBAC has 5 permission levels with no privilege escalation
10. Verify audit log has hash-chained integrity and O(1) eviction
11. Verify GDPR consent verification before operations
12. Verify TLS required for gRPC connections with certificate validation
13. Verify no hardcoded keys or credentials in codebase
14. Produce security assessment report with PASS/FAIL verdict
```

## Decision Process

```
Security Change Request
  ↓
Identify Security Domain (Encryption/RBAC/Audit/GDPR/Classification/TLS)
  ↓
Read Relevant Specification
  ↓
Threat Assessment
  ↓
Cryptographic Correctness Check
  ↓
Correct? ──→ NO → BLOCK (vulnerability found)
  ↓ (YES)
Access Control Check
  ↓
No Escalation? ──→ NO → BLOCK (privilege escalation)
  ↓ (YES)
Transport Security Check
  ↓
TLS Enforced? ──→ NO → BLOCK (plaintext risk)
  ↓ (YES)
Compliance Check
  ↓
Consent Verified? ──→ NO → BLOCK (GDPR violation)
  ↓ (YES)
APPROVE with security report
```

## Validation

| Check | Method | Pass Criteria |
|-------|--------|---------------|
| Encryption algorithm | Code inspection | AES-256-GCM |
| Key derivation | Code inspection | BLAKE3 KDF with context string |
| Key generation | Code inspection | CSPRNG (getrandom) |
| Nonce | Code inspection | 12 bytes random |
| AEAD | Code inspection | Authenticated encryption |
| Encryption roundtrip | Test | encrypt → decrypt = identity |
| Wrong key handling | Test | Decryption fails |
| RBAC permissions | Code inspection | 5 levels (Read, Write, Delete, Execute, Admin) |
| Authorization algorithm | Code inspection | ACL → Role → Deny |
| Context isolation | Test | Per-context permissions enforced |
| Audit event types | Code inspection | 5 types (Query, Insert, Delete, Rule, Denied) |
| Audit capacity | Code inspection | 100K max events |
| Audit eviction | Code inspection | O(1) VecDeque |
| GDPR consent | Code inspection | 3 states verified before operations |
| GDPR operations | Code inspection | 6 operations implemented |
| Data classification | Code inspection | 4 tiers with retention policies |
| gRPC TLS | Code inspection | TLS required |
| gRPC authentication | Code inspection | Enforced on all endpoints |
| gRPC authorization | Code inspection | Checks on protected operations |

## Quality Gates

- [ ] `cargo check --workspace` passes clean
- [ ] All encryption uses AES-256-GCM with BLAKE3 KDF
- [ ] Zero hardcoded keys, tokens, or credentials
- [ ] CSPRNG for all random number generation
- [ ] 12-byte random nonce per encryption
- [ ] RBAC has no privilege escalation paths
- [ ] Audit events are immutable after creation
- [ ] Audit log uses O(1) VecDeque eviction
- [ ] GDPR consent verified before operations
- [ ] TLS required for all gRPC connections
- [ ] Certificate validation enabled
- [ ] No `unsafe` without documented `// SAFETY:` justification
- [ ] Security changes reviewed by P7 (Security Engineer)

## Dependencies

| Skill | Dependency Type | Description |
|-------|----------------|-------------|
| kcm-specification-lock (P4) | Upstream gate | Validates frozen security contracts |
| kcm-architecture-guardian (P5) | Upstream gate | Validates security architecture |
| kcm-code-quality-guardian (P10) | Downstream | Validates code quality after security review |
| kcm-testing-verification (P9) | Downstream | Validates security test coverage |
| kcm-engineering-orchestrator (P1) | Escalation | Resolves security conflicts |

## Related Skills

| Skill | Relationship |
|-------|-------------|
| kcm-specification-lock (P4) | P4 validates frozen security contracts; P7 validates security implementation |
| kcm-architecture-guardian (P5) | P5 validates security architecture; P7 validates cryptographic correctness |
| kcm-compliance (kcm-compliance crate) | P7 enforces compliance via kcm-compliance implementation |
| kcm-testing-verification (P9) | P9 writes security tests; P7 validates security semantics |
| kcm-database-engine-specialist (P6) | P6 handles storage; P7 handles storage encryption |

## SSOT References

| Document | Section | Relevance |
|----------|---------|-----------|
| SSOT.md | Security Model | AES-256-GCM, BLAKE3, RBAC 5 levels |
| AGENTS.md | §13 Security Rules | Non-negotiable security rules |
| AGENTS.md | §13.2 Security Model | Encryption, RBAC, audit, compliance specs |
| docs/PRD3.md | §4 | Security and compliance specifications |
| docs/KCM_SECURITY_TRUST_SPEC.md | All sections | Security specification |
| crates/kcm-security/proto/kcm.proto | All sections | gRPC proto security implications |

## Failure Conditions

| Condition | Impact | Escalation |
|-----------|--------|------------|
| XOR or weak encryption used | Vulnerability — data breach risk | BLOCK immediately |
| Hardcoded keys or passwords | Vulnerability — credential exposure | BLOCK immediately |
| Time-based key generation | Vulnerability — predictable keys | BLOCK immediately |
| Keys logged or stored in code | Vulnerability — credential exposure | BLOCK immediately |
| RBAC privilege escalation | Vulnerability — unauthorized access | BLOCK immediately |
| Audit events modifiable after creation | Vulnerability — tampering risk | BLOCK immediately |
| Encryption without AEAD | Vulnerability — no integrity check | BLOCK immediately |
| GDPR operations without consent | Compliance violation | BLOCK immediately |
| gRPC without TLS | Vulnerability — plaintext transport | BLOCK immediately |

## Escalation

| Level | Path | SLA |
|-------|------|-----|
| Level 1 | Security Engineer resolves internally | 2 hours (critical) / 4 hours (standard) |
| Level 2 | Escalate to Specification Lock (P4) for contract disputes | 8 hours |
| Level 3 | Escalate to Engineering Orchestrator (P1) | 24 hours |
| Level 4 | SSOT.md is final authority for security specifications | 48 hours |

## Examples

See [examples/](examples/) for security review examples.

## Checklist

See [checklists/](checklists/) for security validation checklists.

## References

- [SSOT.md](../../SSOT.md)
- [AGENTS.md](../../AGENTS.md)
- [KCM_SPECIFICATION.md](../../KCM_SPECIFICATION.md)
- [docs/PRD3.md](../../docs/PRD3.md)
- [docs/KCM_SECURITY_TRUST_SPEC.md](../../docs/KCM_SECURITY_TRUST_SPEC.md)
