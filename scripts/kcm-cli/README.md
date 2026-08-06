# KCM CLI Tools

Command-line tools for KCM database management, diagnostics, and operations.

## Available Tools

| Tool | Purpose | Key Commands | Status |
|------|---------|--------------|--------|
| kcm-cli | Main CLI | query, insert, serve | Planned |
| kcm-backup | Backup | create, list, verify | Planned |
| kcm-restore | Restore | from, list, verify | Planned |
| kcm-migrate | Migration | up, down, status | Planned |
| kcm-bench | Benchmarking | run, compare, report | Planned |
| kcm-inspect | Inspection | schema, data, stats | Planned |
| kcm-doctor | Health check | check, fix, report | Planned |
| kcm-profile | Profiling | start, report | Planned |
| kcm-snapshot | Snapshots | create, list, restore | Planned |
| kcm-import | Import | csv, json, parquet | Planned |
| kcm-export | Export | csv, json, parquet | Planned |
| kcm-compact | Compaction | run, status, analyze | Planned |
| kcm-diagnose | Diagnostics | full, perf, report | Planned |
| kcm-cluster | Cluster mgmt | status, rebalance | Planned |
| kcm-schema | Schema gen | generate, validate | Planned |
| kcm-docs | Doc gen | generate, serve | Planned |
| kcm-perf | Perf analyzer | analyze, baseline | Planned |

## Installation

```bash
cargo install kcm-cli
```

## Building

Each tool is a separate binary crate in its respective directory.
