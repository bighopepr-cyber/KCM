# Developer Ecosystem

| Field | Value |
|-------|-------|
| **Document ID** | KCM-ECO-001 |
| **Title** | Developer Ecosystem |
| **Version** | 1.0.0 |
| **Date** | 2026-08-03 |
| **Status** | Authoritative |
| **Authority** | Engineering Orchestrator (P1) |

---

## 1. Developer Journey

```
Install > Quick Start > Tutorial > Production
   |           |            |           |
   v           v            v           v
  SDK       Examples     Docs      Enterprise
```

## 2. SDK Strategy

| Language | Status | Package | Priority |
|----------|--------|---------|----------|
| Rust | Stable | kcm-core | P0 |
| Python | Planned | kcm (PyPI) | P1 |
| JavaScript | Planned | @kcm/js (npm) | P1 |
| TypeScript | Planned | @kcm/ts (npm) | P1 |
| Go | Planned | github.com/kcm/go-sdk | P2 |
| Java | Planned | io.kcm:sdk (Maven) | P2 |
| .NET | Planned | Kcm.Sdk (NuGet) | P2 |
| C | Stable | FFI via kcm-interface | P0 |
| C++ | Planned | libkcm | P2 |

## 3. CLI Tool Strategy

17 tools covering:
- Database management (cli, backup, restore, migrate)
- Performance (bench, profile, perf)
- Operations (doctor, diagnose, snapshot, compact)
- Data (import, export, inspect)
- Cluster (cluster, schema, docs)

## 4. IDE Integration Strategy

- VS Code extension with KQL support
- JetBrains plugin
- Language Server Protocol (LSP)
- Syntax highlighting, autocomplete, query explain

## 5. Documentation Strategy

- Getting Started guide
- Tutorials (beginner, intermediate, advanced)
- API reference
- Architecture deep-dives
- Examples for each language

## 6. Community Strategy

- GitHub Discussions
- Discord server
- Monthly blog posts
- Conference talks
- Open source governance
