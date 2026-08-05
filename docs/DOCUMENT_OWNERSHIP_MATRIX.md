# KCM Document Ownership Matrix

**Document ID:** DOC-OWN-001
**Version:** 2.0.0
**Status:** Active
**Owner:** Documentation Guardian (P11)

## Purpose
Defines ownership and review responsibility for the active documentation surface. Each topic must have exactly one owner and one authoritative document.

## Ownership Rules
- Each active topic has exactly one Owner.
- The Owner is responsible for correctness, update cadence, and cross-reference review.
- Derived and operational documents must reference the authoritative source rather than redefine the same topic.
- Archive and report documents are historical and do not participate in the active ownership model.

## Ownership Matrix

| Topic | Authoritative Document | Owner (Skill) | Reviewer (Skill) | Role |
|-------|------------------------|--------------|-------------------|------|
| Engineering policy | AGENTS.md | Engineering Orchestrator (P1) | All skills | Governance |
| Core architecture | PRD.md | Specification Lock (P4) | Architecture Guardian (P5) | Core contract |
| Storage and runtime | PRD2.md | Specification Lock (P4) | Architecture Guardian (P5) | Runtime contract |
| Distributed/security/compliance | PRD3.md | Specification Lock (P4) | Security Engineer (P7) | Advanced-systems contract |
| Testing and benchmarking | PRD-TESTING& BRACHMARCK.md | Specification Lock (P4) | Testing Verification (P9) | Verification contract |
| System overview | KCM_SPECIFICATION.md | Specification Lock (P4) | Architecture Guardian (P5) | Derived overview |
| Architecture detail | KCM_ARCHITECTURE.md | Architecture Guardian (P5) | Specification Lock (P4) | Derived architecture reference |
| Data model | KCM_DATA_MODEL_SPEC.md | Database Engine Specialist (P6) | Specification Lock (P4) | Data contract |
| Binary layout | KCM_COLUMNAR_FORMAT_SPEC.md | Database Engine Specialist (P6) | Specification Lock (P4) | Storage format |
| Query execution | KCM_QUERY_EXECUTION_SPEC.md | Database Engine Specialist (P6) | Performance Engineer (P8) | Query runtime contract |
| Compression | KCM_COMPRESSION_SPEC.md | Database Engine Specialist (P6) | Performance Engineer (P8) | Codec contract |
| Indexing | KCM_INDEXING_SPEC.md | Database Engine Specialist (P6) | Performance Engineer (P8) | Indexing contract |
| Security posture | KCM_SECURITY_TRUST_SPEC.md | Security Engineer (P7) | Specification Lock (P4) | Security contract |
| API surfaces | KCM_API_SPEC.md | Specification Lock (P4) | Database Engine Specialist (P6) | Interface contract |
| Runtime lifecycle | KCM_RUNTIME_SPEC.md | Database Engine Specialist (P6) | Performance Engineer (P8) | Runtime operational contract |
| Performance targets | KCM_PERFORMANCE_SPEC.md | Performance Engineer (P8) | Testing Verification (P9) | Performance contract |
| Validation policy | KCM_TESTING_SPEC.md | Testing Verification (P9) | Code Quality Guardian (P10) | Quality gate contract |
| Engineering rules | KCM_ENGINEERING_RULES.md | Code Quality Guardian (P10) | Specification Lock (P4) | Quality policy |
| Versioning | KCM_VERSIONING_SPEC.md | Specification Lock (P4) | Release Readiness (P12) | Compatibility contract |
| Deployment | KCM_DEPLOYMENT_SPEC.md | Release Readiness (P12) | Architecture Guardian (P5) | Deployment contract |
| Benchmark reporting | KCM_BENCHMARK_REPORTING_SPEC.md | Performance Engineer (P8) | Testing Verification (P9) | Benchmark reporting |
| Glossary | KCM_GLOSSARY.md | Documentation Guardian (P11) | Specification Lock (P4) | Terminology authority |
| Docs navigation | DOCUMENTATION_INDEX.md | Documentation Guardian (P11) | — | Navigation index |
| Dependency map | DOCUMENT_DEPENDENCY_MAP.md | Documentation Guardian (P11) | Architecture Guardian (P5) | Dependency graph |
| Ownership model | DOCUMENT_OWNERSHIP_MATRIX.md | Documentation Guardian (P11) | Engineering Orchestrator (P1) | Ownership registry |
| Guides and procedures | docs/guides/* | Platform Operations Owner | Runtime/Deployment Owner | Operational procedures |
| Tutorials | docs/tutorials/* | Documentation Guardian (P11) | Runtime/Interface Owner | Learning path |
| Cookbook examples | docs/cookbook/* | Runtime/Interface Owner | Documentation Guardian (P11) | Current example recipes |
| Handbook | docs/handbook/* | Documentation Guardian (P11) | Engineering Orchestrator (P1) | Contributor workflow |
| Repository specs | docs/specs/repository/* | Engineering Orchestrator (P1) | Specification Lock (P4) | Repository contract |
| Ecosystem specs | docs/specs/ecosystem/* | Product/Platform Owner | Architecture Guardian (P5) | Integration/product context |
