# KCM Document Dependency Map

**Document ID:** DOC-DEPMAP-001
**Version:** 1.0.0
**Status:** Active

## Purpose
Shows the dependency relationships between all KCM specification documents. Changes to a parent document may impact all dependent documents.

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

When modifying a document, check these dependents:

| Document | Direct Dependents | Indirect Dependents |
|----------|------------------|-------------------|
| AGENTS.md | PRD.md, PRD2.md, PRD3.md, PRD-TESTING | All specs |
| PRD.md | KCM_DATA_MODEL_SPEC, KCM_ARCHITECTURE, KCM_ENGINEERING_RULES | All derived specs |
| PRD2.md | KCM_RUNTIME_SPEC, KCM_API_SPEC, KCM_DEPLOYMENT_SPEC | — |
| PRD3.md | KCM_SECURITY_TRUST_SPEC, KCM_INDEXING_SPEC | — |
| PRD-TESTING | KCM_TESTING_SPEC, KCM_PERFORMANCE_SPEC, KCM_BENCHMARK_REPORTING_SPEC | — |
| KCM_DATA_MODEL_SPEC | KCM_COLUMNAR_FORMAT_SPEC, KCM_QUERY_EXECUTION_SPEC, KCM_INDEXING_SPEC | KCM_COMPRESSION_SPEC |
| KCM_ARCHITECTURE | KCM_RUNTIME_SPEC, KCM_API_SPEC, KCM_DEPLOYMENT_SPEC | — |
