# CLI Roadmap

| Field | Value |
|-------|-------|
| **Document ID** | KCM-ECO-004 |
| **Title** | CLI Roadmap |
| **Version** | 1.0.0 |
| **Date** | 2026-08-03 |
| **Status** | Authoritative |
| **Authority** | Engineering Orchestrator (P1) |

---

## 1. Tool Registry

| # | Tool | Purpose | Key Commands | Priority |
|---|------|---------|--------------|----------|
| 1 | kcm-cli | Main CLI | query, insert, serve | P1 |
| 2 | kcm-backup | Backup | create, list, verify | P1 |
| 3 | kcm-restore | Restore | from, list, verify | P1 |
| 4 | kcm-migrate | Migration | up, down, status | P2 |
| 5 | kcm-bench | Benchmarking | run, compare, report | P1 |
| 6 | kcm-inspect | Inspection | schema, data, stats | P2 |
| 7 | kcm-doctor | Health check | check, fix, report | P1 |
| 8 | kcm-profile | Profiling | start, report | P2 |
| 9 | kcm-snapshot | Snapshots | create, list, restore | P2 |
| 10 | kcm-import | Import | csv, json, parquet | P1 |
| 11 | kcm-export | Export | csv, json, parquet | P1 |
| 12 | kcm-compact | Compaction | run, status, analyze | P2 |
| 13 | kcm-diagnose | Diagnostics | full, perf, report | P2 |
| 14 | kcm-cluster | Cluster mgmt | status, rebalance | P3 |
| 15 | kcm-schema | Schema gen | generate, validate | P2 |
| 16 | kcm-docs | Doc gen | generate, serve | P3 |
| 17 | kcm-perf | Perf analyzer | analyze, baseline | P2 |

## 2. Installation

```bash
# Via cargo
cargo install kcm-cli

# Via homebrew (planned)
brew install kcm

# Via Docker
docker run kcm/kcm-cli
```

## 3. Command Structure

```
kcm <command> [subcommand] [options]

Global options:
  --db <path>      Database path
  --format <fmt>   Output format (json, table, csv)
  --verbose        Verbose output
  --quiet          Suppress output
```
