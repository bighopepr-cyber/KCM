# Repository Architecture

| Field | Value |
|-------|-------|
| **Document ID** | KCM-REPO-002 |
| **Title** | Repository Architecture |
| **Version** | 1.0.0 |
| **Date** | 2026-08-03 |
| **Status** | Authoritative |
| **Authority** | Engineering Orchestrator (P1) |

---

## 1. Philosophy

KCM follows **Documentation First Development (DFD)**:

1. Design > Document > Validate > Implement > Verify
2. No code changes without approved documentation
3. Documentation is the Single Source of Truth (SSOT)

## 2. Repository as Platform

The repository is structured as a **platform**, not just an engine:

```
+---------------------------------------------+
|            Users & Integrations              |
+---------------------------------------------+
|  SDKs (Python, JS, Go, Java, .NET, C++)     |
+---------------------------------------------+
|  Tools (CLI, Backup, Restore, Inspector)    |
+---------------------------------------------+
|  IDE (VS Code, JetBrains, LSP)              |
+---------------------------------------------+
|  Enterprise (Docker, K8s, Helm, Terraform)  |
+---------------------------------------------+
|  Integrations (Arrow, Parquet, Kafka, MCP)  |
+---------------------------------------------+
|  KCM Engine (13 crates)                     |
+---------------------------------------------+
|  Core (types, storage, compute, reasoning)  |
+---------------------------------------------+
```

## 3. Engine Architecture

### Layered Architecture

```
Layer 4: kcm-server      > HTTP/gRPC binaries
Layer 3: kcm-interface   > C FFI, Python, REST, KQL
Layer 2: kcm-runtime     > Database, Transactions, Metrics
Layer 1: kcm-compute     > Query operators, SIMD
         kcm-reasoning   > Rules, Inference
         kcm-optimizer   > Cost model, Planner
         kcm-distributed > Sharding, 2PC
         kcm-ml          > Learned index, Confidence
Layer 0: kcm-storage     > Columns, WAL, FileFormat, Index
Layer -1: kcm-core       > Types, DenseVec, Bitmap, Dictionary
```

### Cross-Cutting Concerns

```
kcm-security   > RBAC, Encryption, Audit (across all layers)
kcm-compliance > GDPR, Classification (across all layers)
kcm-testing    > Load, Stress, Security, Recovery (across all layers)
```

## 4. Documentation Hierarchy

| Priority | Document | Authority |
|----------|----------|-----------|
| P0 | AGENTS.md | Engineering Constitution |
| P1 | PRD-TESTING.md | Testing/Benchmark |
| P2 | PRD3.md | Distributed/Security/Compliance |
| P3 | PRD2.md | Storage/Runtime/Interfaces |
| P4 | PRD.md | Core/Compute/Reasoning |
| P5 | docs/*.md | Derived specifications |

## 5. Quality Gates

Every change must pass 6 gates:

1. **Repository Understanding** - Know where the change belongs
2. **Specification Validation** - Confirm contract compliance
3. **Implementation Planning** - Define strategy and impact
4. **Implementation Validation** - No placeholders, complete errors
5. **Domain Validation** - Expert review for the domain
6. **Production Readiness** - Build, test, clippy, fmt all pass

## 6. Repository Health Metrics

| Metric | Target | Current |
|--------|--------|---------|
| Test coverage | >=95% | Unknown |
| Clippy warnings | 0 | 0 |
| Documentation contradictions | 0 | 0 (after fixes) |
| Technical debt items | <10 | 20 (after fixes) |
| Enterprise readiness | >=8/10 | 5/10 (after fixes) |
