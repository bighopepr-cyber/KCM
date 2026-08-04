# Platform Roadmap

| Field | Value |
|-------|-------|
| **Document ID** | KCM-ROADMAP-001 |
| **Title** | Platform Roadmap |
| **Version** | 1.0.0 |
| **Date** | 2026-08-04 |
| **Status** | Authoritative |
| **Authority** | Engineering Orchestrator (P1) |

---

## 1. Vision

KCM becomes a long-term platform with enterprise-grade quality, comprehensive ecosystem, and sustainable community.

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

## 3. LTS Policy

| Version | Support Level | End of Life |
|---------|--------------|-------------|
| 1.0.x | Full support | 2030-06 |
| 1.1.x | Full support | 2030-09 |
| 2.0.x | Full support | 2031-06 |

### LTS Guarantees

- Security patches for 3 years
- Bug fixes for 2 years
- No feature additions after EOL
- Migration guides for major versions

## 4. Deprecation Policy

| Step | Timeline | Action |
|------|----------|--------|
| 1. Announce | Release N | Mark as deprecated in docs |
| 2. Warn | Release N+1 | Add runtime warnings |
| 3. Remove | Release N+2 | Remove from codebase |

## 5. Compatibility Policy

### Semantic Versioning

- MAJOR: Breaking API changes
- MINOR: New features, backward-compatible
- PATCH: Bug fixes, backward-compatible

### Breaking Change Process

1. Write ADR explaining the change
2. Get Architecture Guardian approval
3. Add migration guide
4. Deprecate old API first
5. Remove in next MAJOR

## 6. Ecosystem Roadmap

### SDKs (9 languages)

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

### CLI Tools (17 tools)

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

### Integrations (15 integrations)

| Integration | Priority | Timeline | Status |
|------------|----------|----------|--------|
| gRPC | P0 | Current | Stable |
| REST | P0 | Current | Stable |
| Apache Arrow | P1 | Q1 2027 | Planned |
| Apache Parquet | P1 | Q1 2027 | Planned |
| Pandas | P1 | Q2 2027 | Planned |
| Apache Kafka | P1 | Q2 2027 | Planned |
| DataFusion | P2 | Q3 2027 | Planned |
| Polars | P2 | Q3 2027 | Planned |
| DuckDB | P2 | Q3 2027 | Planned |
| Arrow Flight | P2 | Q4 2027 | Planned |
| MQTT | P2 | Q4 2027 | Planned |
| NATS | P2 | Q4 2027 | Planned |
| Apache Iceberg | P3 | Q1 2028 | Planned |
| Delta Lake | P3 | Q1 2028 | Planned |
| MCP | P3 | Q2 2028 | Planned |

## 7. Community Model

### Contribution Levels

| Level | Criteria | Privileges |
|-------|----------|-----------|
| User | Uses KCM | Issues, discussions |
| Contributor | 1+ merged PR | PR reviews |
| Collaborator | 5+ merged PRs | Triage issues |
| Maintainer | 20+ merged PRs | Merge PRs |
| Core Team | Sustained | Architecture decisions |

### Governance

- RFC process for major changes
- Monthly community calls
- Transparent decision-making
- ADR for architectural decisions
