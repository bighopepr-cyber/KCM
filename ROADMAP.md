# KCM Roadmap

**Document ID:** KCM-ROADMAP-001  
**Version:** 2.0.0  
**Status:** Active  
**Owner:** Engineering Orchestrator (P1)

---

## 1. Vision

KCM becomes a long-term enterprise platform with columnar knowledge storage, deterministic reasoning, and a sustainable multi-language SDK ecosystem.

## 2. Release Train

| Version | Target Date | Theme | Key Deliverables |
|---------|-------------|-------|-----------------|
| 0.1.0 | 2026-08-03 | Initial release | Core engine, C FFI, REST API |
| 0.2.0 | 2026-10 | Python SDK | PyO3 bindings, pip package |
| 0.3.0 | 2026-12 | CLI & Tools | kcm-cli, kcm-backup, kcm-doctor |
| 0.4.0 | 2027-02 | JavaScript SDK | N-API bindings, npm package |
| 0.5.0 | 2027-04 | IDE Support | VS Code extension, LSP |
| 1.0.0 | 2027-06 | Stable API | API freeze, LTS begins |
| 1.1.0 | 2027-09 | Enterprise | Helm, Terraform, monitoring |
| 1.2.0 | 2027-12 | Integrations | Arrow, Parquet, Kafka |
| 2.0.0 | 2028-06 | Distributed | Multi-node, sharding |

## 3. SDK Roadmap

| Language | Priority | Timeline | Status |
|----------|----------|----------|--------|
| Rust | P0 | Current | Stable |
| C | P0 | Current | Stable |
| Python | P1 | Q4 2026 | Planned |
| JavaScript | P1 | Q4 2026 | Planned |
| TypeScript | P1 | Q1 2027 | Planned |
| Go | P2 | Q2 2027 | Planned |
| Java | P2 | Q2 2027 | Planned |
| .NET | P2 | Q3 2027 | Planned |
| C++ | P2 | Q3 2027 | Planned |

## 4. CLI Tools Roadmap

| Tool | Priority | Timeline | Status |
|------|----------|----------|--------|
| kcm-cli | P1 | Q4 2026 | Planned |
| kcm-backup | P1 | Q4 2026 | Planned |
| kcm-restore | P1 | Q4 2026 | Planned |
| kcm-doctor | P1 | Q1 2027 | Planned |
| kcm-bench | P1 | Q1 2027 | Planned |
| kcm-import | P1 | Q1 2027 | Planned |
| kcm-export | P1 | Q1 2027 | Planned |
| kcm-inspect | P2 | Q2 2027 | Planned |
| kcm-migrate | P2 | Q2 2027 | Planned |
| kcm-profile | P2 | Q2 2027 | Planned |
| kcm-snapshot | P2 | Q2 2027 | Planned |
| kcm-compact | P2 | Q3 2027 | Planned |
| kcm-diagnose | P2 | Q3 2027 | Planned |
| kcm-schema | P2 | Q3 2027 | Planned |
| kcm-perf | P2 | Q3 2027 | Planned |
| kcm-cluster | P3 | Q4 2027 | Planned |
| kcm-docs | P3 | Q4 2027 | Planned |

## 5. LTS Policy

| Version | Support Level | End of Life |
|---------|--------------|-------------|
| 1.0.x | Full support | 2030-06 |
| 1.1.x | Full support | 2030-09 |
| 2.0.x | Full support | 2031-06 |

- Security patches for 3 years
- Bug fixes for 2 years
- No feature additions after EOL

## 6. Deprecation Policy

| Step | Timeline | Action |
|------|----------|--------|
| 1. Announce | Release N | Mark as deprecated in docs |
| 2. Warn | Release N+1 | Add runtime warnings |
| 3. Remove | Release N+2 | Remove from codebase |

## 7. Semantic Versioning

- MAJOR: Breaking API changes
- MINOR: New features, backward-compatible
- PATCH: Bug fixes, backward-compatible
