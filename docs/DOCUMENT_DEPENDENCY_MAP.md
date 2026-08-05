# KCM Document Dependency Map

**Document ID:** DOC-DEPMAP-001
**Version:** 2.0.0
**Status:** Active
**Owner:** Documentation Guardian (P11)

## Purpose
Shows the dependency relationships between the authoritative KCM specification documents and the derived operational documents that reference them. This map clarifies role ownership and prevents duplicate authority on the same topic.

## Dependency Graph

```
AGENTS.md (Engineering Constitution)
  ├── PRD.md (P4 - Core Types)
  │     ├── KCM_DATA_MODEL_SPEC.md
  │     │     ├── KCM_COLUMNAR_FORMAT_SPEC.md
  │     │     │     └── KCM_COMPRESSION_SPEC.md
  │     │     ├── KCM_QUERY_EXECUTION_SPEC.md
  │     │     └── KCM_INDEXING_SPEC.md
  │     ├── KCM_ARCHITECTURE.md
  │     │     ├── KCM_RUNTIME_SPEC.md
  │     │     ├── KCM_API_SPEC.md
  │     │     └── KCM_DEPLOYMENT_SPEC.md
  │     └── KCM_ENGINEERING_RULES.md
  ├── PRD2.md (P3 - Storage/Runtime)
  │     └── (derives types from PRD.md)
  ├── PRD3.md (P2 - Distributed/Security)
  │     ├── KCM_SECURITY_TRUST_SPEC.md
  │     └── (derives types from PRD.md, runtime from PRD2.md)
  └── PRD-TESTING&BRACHMARCK.md (P1 - Testing)
        ├── KCM_TESTING_SPEC.md
        ├── KCM_PERFORMANCE_SPEC.md
        └── KCM_BENCHMARK_REPORTING_SPEC.md
```

## Impact Analysis

When modifying a document, check the document role first and then the dependents listed below.

| Canonical Topic | Authoritative Document | Derived Documents | Operational Documents |
|----------------|------------------------|-------------------|----------------------|
| Engineering policy | AGENTS.md | PRD.md, PRD2.md, PRD3.md, PRD-TESTING& BRACHMARCK.md | docs/specs/repository/* |
| Core architecture | PRD.md | KCM_ARCHITECTURE.md, KCM_DATA_MODEL_SPEC.md, KCM_ENGINEERING_RULES.md | docs/guides/*, docs/tutorials/* |
| Storage and runtime | PRD2.md | KCM_RUNTIME_SPEC.md, KCM_API_SPEC.md, KCM_DEPLOYMENT_SPEC.md | docs/cookbook/* |
| Security, compliance, distributed | PRD3.md | KCM_SECURITY_TRUST_SPEC.md | docs/handbook/* |
| Testing and benchmarks | PRD-TESTING& BRACHMARCK.md | KCM_TESTING_SPEC.md, KCM_PERFORMANCE_SPEC.md, KCM_BENCHMARK_REPORTING_SPEC.md | docs/guides/monitoring.md |
