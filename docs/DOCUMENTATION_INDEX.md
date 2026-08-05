0# KCM Documentation Index

**Document ID:** DOC-INDEX-001
**Version:** 2.0.0
**Status:** Canonical
**Owner:** Documentation Guardian (P11)
**Last Updated:** 2026-08-05

## Purpose

This index is the repository-level navigation map for the current documentation set. It intentionally lists only documents that are part of the active Single Source of Truth surface. Historical, retrospective, and audit artifacts are retained only as archive references and are not authoritative for implementation.

## Canonical Documentation Hierarchy

| Priority | Document | Authority | Role |
|----------|----------|-----------|------|
| P1 | PRD-TESTING& BRACHMARCK.md | Testing strategy, benchmark and validation requirements | Authoritative testing contract |
| P2 | PRD3.md | Distributed, ML, security, and compliance | Authoritative advanced-systems contract |
| P3 | PRD2.md | Storage, runtime, and interface contracts | Authoritative runtime/storage contract |
| P4 | PRD.md | Core types, storage, compute, and reasoning | Authoritative core architecture contract |
| P5 | AGENTS.md | Engineering constitution and repository policy | Repository governance and non-negotiable rules |

## Primary Specification Set

| Document | Scope | Role |
|----------|-------|------|
| KCM_SPECIFICATION.md | Technical constitution and contract overview | Derived specification overview |
| KCM_ARCHITECTURE.md | Derived architectural overview | Implementation architecture reference |
| KCM_DATA_MODEL_SPEC.md | Data model and type contract | Canonical data-model reference |
| KCM_COLUMNAR_FORMAT_SPEC.md | Binary format and WAL contract | Storage format reference |
| KCM_QUERY_EXECUTION_SPEC.md | Query pipeline and KQL contract | Execution contract reference |
| KCM_COMPRESSION_SPEC.md | Compression and codec model | Codec selection reference |
| KCM_INDEXING_SPEC.md | Indexing and access path design | Indexing reference |
| KCM_SECURITY_TRUST_SPEC.md | Security, RBAC, encryption, and trust posture | Security contract reference |
| KCM_API_SPEC.md | Public API contract for FFI, REST, and gRPC | API contract reference |
| KCM_RUNTIME_SPEC.md | Runtime lifecycle, concurrency, metrics, and health | Runtime operational contract |
| KCM_PERFORMANCE_SPEC.md | Performance requirements and targets | Performance contract |
| KCM_TESTING_SPEC.md | Test policy and expected quality gates | Verification contract |
| KCM_ENGINEERING_RULES.md | Engineering rules and validation contract | Quality and governance reference |
| KCM_VERSIONING_SPEC.md | Versioning and compatibility policy | Release and compatibility reference |
| KCM_DEPLOYMENT_SPEC.md | Deployment posture and operational configuration | Deployment artifact reference |
| KCM_BENCHMARK_REPORTING_SPEC.md | Benchmark artifact and reporting requirements | Benchmark reporting contract |
| KCM_GLOSSARY.md | Canonical terminology | Shared vocabulary source |

## Documentation Dependency Graph

```text
AGENTS.md (policy)
  └─ PRD.md (core architecture, types, compute, reasoning)
      ├─ PRD2.md (storage/runtime/interface contract)
      ├─ PRD3.md (distributed/security/compliance/ML contract)
      └─ PRD-TESTING& BRACHMARCK.md (testing/benchmark contract)

PRD.md, PRD2.md, PRD3.md, PRD-TESTING& BRACHMARCK.md
  └─ Derived KCM_*_SPEC.md documents
       ├─ API, runtime, deployment, performance, testing, glossary
       └─ repository and ecosystem reference docs

Derived KCM_*_SPEC.md and repository/ecosystem docs
  └─ Operational guides, tutorials, handbooks, and cookbooks
       └─ Reference the authoritative spec tree instead of redefining it
```

## Derived Reference Index

| Area | Role | Canonical Path |
|------|------|----------------|
| Guides | Operational procedure | docs/guides/ |
| Handbooks | Contributor and maintainer workflow | docs/handbook/ |
| Tutorials | Executable learning path | docs/tutorials/ |
| Cookbook | Current runtime examples | docs/cookbook/ |
| ADRs | Design decision log | docs/adr/ |
| Repository specs | Repository contract | docs/specs/repository/ |
| Ecosystem specs | Product and integration direction | docs/specs/ecosystem/ |
| Runtime and interface implementations | Code-backed reference | crates/kcm-runtime/, crates/kcm-interface/, crates/kcm-server/ |

## Archive and Historical References

The following files are retained for traceability and review history only. They do not define current behavior or implementation contracts:

| Document | Use |
|----------|-----|
| KCM_DOCUMENT_AUDIT_REPORT.md | Historical documentation audit |
| DOCUMENTATION_CONSISTENCY_REPORT.md | Historical consistency review |
| DOCUMENTATION_CONSISTENCY_REPORT_V2.md | Historical remediation review |
| KCM_STABILITY_READINESS_REPORT.md | Historical stability assessment |
| KCM_PERFORMANCE_ENGINEERING_REPORT.md | Historical performance review |
| PROJECT_COMPLIANCE_REPORT.md | Historical compliance review |
| DESIGN_REVIEW_REPORT.md | Historical design review |
| CODEBASE_AUDIT_REPORT.md | Historical codebase review |

## Relationship Rules

1. The PRD documents are the root authoritative contracts.
2. Derived specifications explain the implementation contract in one location only.
3. Guides, handbooks, tutorials, and cookbook material provide operational context and may reference the primary specification set rather than restating it.
4. Archive reports are non-normative and must not be used as the basis for new implementation changes.
