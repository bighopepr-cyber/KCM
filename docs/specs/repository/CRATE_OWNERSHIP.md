# Crate Ownership

| Field | Value |
|-------|-------|
| **Document ID** | KCM-REPO-006 |
| **Title** | Crate Ownership |
| **Version** | 1.0.0 |
| **Date** | 2026-08-03 |
| **Status** | Authoritative |
| **Authority** | Engineering Orchestrator (P1) |

---

## Crate Registry

| Crate | Owner | Purpose | Key Dependencies |
|-------|-------|---------|-----------------|
| kcm-core | Core Team | Foundation types, DenseVec, Bitmap, Dictionary | parking_lot |
| kcm-storage | Storage Team | Columnar storage, codecs, WAL, file format | core + zstd + lz4 + blake3 + thiserror |
| kcm-compute | Compute Team | Query operators, SIMD acceleration | core + storage |
| kcm-reasoning | Reasoning Team | Rules, forward-chaining inference | core + storage |
| kcm-optimizer | Optimizer Team | Cost model, planner, statistics | core + storage |
| kcm-runtime | Runtime Team | Database, transactions, metrics | core + storage + rayon + tokio |
| kcm-interface | Interface Team | C FFI, Python, REST, KQL | core + storage + runtime |
| kcm-distributed | Distributed Team | Sharding, 2PC coordinator | core |
| kcm-ml | ML Team | Learned index, confidence learner | core + reasoning |
| kcm-security | Security Team | RBAC, AES-256-GCM, audit log | core + blake3 + aes-gcm |
| kcm-compliance | Compliance Team | GDPR, data classification | core |
| kcm-testing | QA Team | Load, stress, security, recovery tests | core + storage + runtime |
| kcm-server | Platform Team | HTTP/gRPC server binaries | core + runtime + interface |
