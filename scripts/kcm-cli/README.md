# KCM CLI Tools

Command-line tools for KCM database management, diagnostics, and operations.

## Available Tools

| Tool | Purpose | Key Commands | Status |
|------|---------|--------------|--------|
| kcm-cli | Main CLI | query, insert, serve | Implemented |
| kcm-backup | Backup | create, list, verify | Implemented |
| kcm-restore | Restore | from, list, verify | Implemented |
| kcm-migrate | Migration | up, down, status, create, validate, history | Implemented |
| kcm-bench | Benchmarking | run, compare, report | Implemented |
| kcm-inspect | Inspection | schema, data, stats | Implemented |
| kcm-doctor | Health check | check, fix, report | Implemented |
| kcm-profile | Profiling | start, report | Implemented |
| kcm-snapshot | Snapshots | create, list, restore | Implemented |
| kcm-import | Import | csv, json, parquet | Implemented |
| kcm-export | Export | csv, json, parquet | Implemented |
| kcm-compact | Compaction | run, status, analyze | Implemented |
| kcm-diagnose | Diagnostics | full, perf, report | Implemented |
| kcm-cluster | Cluster mgmt | status, rebalance | Implemented |
| kcm-schema | Schema gen | generate, validate | Implemented |
| kcm-docs | Doc gen | generate, serve | Implemented |
| kcm-perf | Perf analyzer | analyze, baseline | Implemented |

## Installation

```bash
cargo install kcm-cli
```

## Building

Each tool is a separate binary crate in its respective directory.
