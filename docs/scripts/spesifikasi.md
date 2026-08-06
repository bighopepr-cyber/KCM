# Scripts Technical Specification

## Overview

This document specifies the technical design of the `scripts/` directory — build automation, validation utilities, CLI tools, and their integration with the KCM codebase.

## Scope

Covers all files within `scripts/` including shell scripts, Python utilities, and the `kcm-cli/` Rust workspace of CLI tools.

## Responsibilities

| Area | Description |
|---|---|
| Build automation | Compilation, release packaging, CI pipeline orchestration |
| Validation | SSOT compliance, SDK API compliance, benchmark regression detection |
| CLI tools | Database management, inspection, migration, and administration via 17 Rust CLI tools |

## Technical Specification

### validate-ssot.sh

- Executes 13 automated compliance checks against the SSOT documentation.
- Checks include: API surface match, FFI function parity, REST endpoint parity, gRPC RPC parity, crate structure alignment, dependency policy compliance, error model conformance, storage model conformance, query model conformance, testing strategy alignment, benchmark coverage, documentation completeness, and architecture consistency.
- Exits `0` on full compliance, non-zero with a diagnostic report on failure.

### validate-sdk-api.sh

- Validates that public SDK APIs match the specification defined in `docs/PRD2.md §19` and `docs/PRD3.md`.
- Checks function signatures, return types, error variants, and backward compatibility.
- Produces a compliance report in machine-readable format.

### bench-regression.py

- Parses benchmark output from `cargo bench`.
- Compares current results against a baseline using configurable thresholds:
  - **5% threshold**: Warning — performance degradation detected.
  - **10% threshold**: Failure — unacceptable regression.
- Produces a regression report with per-benchmark deltas.

### kcm-cli

- 17 Rust CLI tools implemented as workspace members under `kcm-cli/`.
- Each tool is a standalone binary for database management, inspection, and administration.
- Tools integrate with `kcm-runtime` for database operations and `kcm-security` for access control.

## Architecture

```
scripts/
├── bench-regression.py        # Benchmark regression detection
├── validate-sdk-api.sh        # SDK API compliance validation
├── validate-ssot.sh           # SSOT compliance checks (13 automated checks)
├── kcm-cli/                   # 17 Rust CLI tools
│   ├── Cargo.toml
│   └── src/
│       └── bin/
│           ├── kcm-create/
│           ├── kcm-query/
│           ├── kcm-insert/
│           ├── kcm-delete/
│           ├── kcm-backup/
│           ├── kcm-restore/
│           ├── kcm-export/
│           ├── kcm-import/
│           ├── kcm-schema/
│           ├── kcm-index/
│           ├── kcm-wal/
│           ├── kcm-compact/
│           ├── kcm-reason/
│           ├── kcm-permissions/
│           ├── kcm-audit/
│           ├── kcm-metrics/
│           └── kcm-health/
├── SECURITY.md
├── CONTRIBUTING.md
├── CODE_OF_CONDUCT.md
└── README.md
```

## Internal Components

### Shell Scripts

| Script | Language | Purpose |
|---|---|---|
| `validate-ssot.sh` | Bash | 13 automated SSOT compliance checks |
| `validate-sdk-api.sh` | Bash | SDK API surface validation |

### Python Scripts

| Script | Language | Purpose |
|---|---|---|
| `bench-regression.py` | Python 3 | Benchmark regression detection with 5%/10% thresholds |

### CLI Tools (kcm-cli)

| Tool | Purpose |
|---|---|
| `kcm-create` | Create a new KCM database |
| `kcm-query` | Execute KQL queries against a database |
| `kcm-insert` | Insert facts into a database |
| `kcm-delete` | Delete facts from a database |
| `kcm-backup` | Create database backups |
| `kcm-restore` | Restore from backup |
| `kcm-export` | Export database to portable format |
| `kcm-import` | Import data into a database |
| `kcm-schema` | Inspect and manage database schema |
| `kcm-index` | Manage database indexes |
| `kcm-wal` | Inspect and manage WAL |
| `kcm-compact` | Trigger database compaction |
| `kcm-reason` | Execute inference rules |
| `kcm-permissions` | Manage RBAC permissions |
| `kcm-audit` | Query audit log |
| `kcm-metrics` | Display database metrics |
| `kcm-health` | Health check endpoint |

## Data Model

- Scripts produce structured output (JSON where possible) for CI consumption.
- CLI tools operate on KCM database files using the storage format defined in `docs/PRD2.md §15`.
- Benchmark regression data is persisted as JSON baseline files.

## Execution Flow

### CI Pipeline Flow

```
Push/PR
  │
  ├── cargo fmt --all -- --check
  ├── cargo clippy --workspace -- -D warnings
  ├── cargo build --workspace
  ├── cargo test --workspace
  ├── cargo bench --workspace
  │     └── bench-regression.py (compare against baseline)
  ├── bash scripts/validate-ssot.sh (13 checks)
  ├── bash scripts/validate-sdk-api.sh
  └── Quality Gate (all pass → merge allowed)
```

## Public API

CLI tools expose the following command patterns:

```
kcm-<tool> [OPTIONS] <SUBCOMMAND>

Options:
  --db <PATH>       Path to KCM database
  --format <FMT>    Output format (json, text, table)
  --verbose          Enable verbose output
  --help             Display usage information
```

## Configuration

- CLI tools accept configuration via command-line arguments and environment variables.
- No configuration files are required for normal operation.
- Environment variables: `KCM_DB_PATH`, `KCM_LOG_LEVEL`, `KCM_FORMAT`.

## Dependencies

| Component | Dependencies |
|---|---|
| Shell scripts | `bash`, `coreutils`, `grep`, `awk` |
| Python scripts | `python3`, `json`, `sys`, `os` |
| CLI tools | `kcm-core`, `kcm-storage`, `kcm-runtime`, `kcm-security`, `clap` (argument parsing) |

## Error Handling

- All scripts exit with meaningful exit codes (`0` = success, `1` = general error, `2` = usage error).
- CLI tools return `Result<T, KcmError>` using the unified error model from `AGENTS.md`.
- Error messages are written to `stderr`.
- Scripts clean up temporary resources on failure via trap handlers.

## Performance Characteristics

| Component | Target |
|---|---|
| `validate-ssot.sh` | < 30 seconds |
| `validate-sdk-api.sh` | < 15 seconds |
| `bench-regression.py` | < 10 seconds |
| CLI tool cold start | < 100ms |
| CLI query execution | Depends on query complexity |

## Security Considerations

- No secrets in scripts or CLI source code.
- CLI tools integrate with `kcm-security` RBAC for privileged operations.
- File paths are validated and canonicalized before use.
- See [scripts/SECURITY.md](SECURITY.md) for full security policy.

## Integration

- Scripts integrate with the CI pipeline defined in `.github/workflows/`.
- CLI tools integrate with `kcm-runtime` for database operations.
- CLI tools integrate with `kcm-security` for access control and audit logging.
- Benchmark results integrate with `bench-regression.py` for regression detection.

## Sequence Diagram

```
CI Pipeline → validate-ssot.sh
  │
  ├── Read SSOT documents
  ├── Read source code
  ├── Compare API surfaces
  ├── Compare data structures
  ├── Check dependency policies
  ├── Verify test coverage
  ├── Check benchmark coverage
  ├── Verify documentation
  ├── Check architecture
  ├── Check error model
  ├── Check storage model
  ├── Check query model
  └── Report results (exit 0 or non-zero)

CI Pipeline → bench-regression.py
  │
  ├── Parse cargo bench output
  ├── Load baseline data
  ├── Compare per-benchmark
  ├── Flag 5%+ regressions (warning)
  ├── Flag 10%+ regressions (failure)
  └── Report results
```

## Architecture Diagram

```
┌─────────────────────────────────────────────────┐
│                 CI Pipeline                      │
├─────────────────────────────────────────────────┤
│  fmt │ clippy │ build │ test │ bench │ validate │
├─────────────────────────────────────────────────┤
│                                                  │
│  ┌──────────────┐  ┌──────────────────────────┐ │
│  │ Shell Scripts │  │ Python Scripts           │ │
│  │ validate-ssot │  │ bench-regression.py      │ │
│  │ validate-sdk  │  └──────────────────────────┘ │
│  └──────────────┘                               │
│                                                  │
│  ┌──────────────────────────────────────────────┐│
│  │ kcm-cli/ (17 Rust CLI tools)                ││
│  │  ┌─────────┐ ┌──────────┐ ┌───────────────┐ ││
│  │  │ create  │ │ query    │ │ insert/delete │ ││
│  │  │ backup  │ │ restore  │ │ export/import │ ││
│  │  │ schema  │ │ index    │ │ wal/compact   │ ││
│  │  │ reason  │ │ perms    │ │ audit/metrics │ ││
│  │  │ health  │ │          │ │               │ ││
│  │  └────┬────┘ └────┬─────┘ └──────┬────────┘ ││
│  └───────┼───────────┼──────────────┼───────────┘│
│          │           │              │            │
│  ┌───────▼───────────▼──────────────▼───────────┐│
│  │ kcm-runtime │ kcm-security │ kcm-core        ││
│  └──────────────────────────────────────────────┘│
└─────────────────────────────────────────────────┘
```

## References

- [AGENTS.md — Build and Test Commands](../AGENTS.md)
- [AGENTS.md — Non-Negotiable Rules](../AGENTS.md)
- [AGENTS.md — CI Pipeline Requirements](../AGENTS.md)
- [scripts/README.md](README.md)
- [scripts/SECURITY.md](SECURITY.md)
- [scripts/CONTRIBUTING.md](CONTRIBUTING.md)
- `docs/PRD.md §3` — Core types
- `docs/PRD2.md §15` — Storage format
- `docs/PRD2.md §19` — Interfaces
- `docs/PRD3.md` — Distributed, security, compliance

## SSOT Alignment

| Document | Section | Reference |
|---|---|---|
| SSOT.md | §SSOT compliance checks | `validate-ssot.sh` implements 13 automated checks |
| AGENTS.md | Engineering Gates | Gate 6 validation commands |
| AGENTS.md | CI Pipeline Requirements | Pipeline jobs and block-merge rules |
| AGENTS.md | Build and Test Commands | `cargo build`, `cargo test`, `cargo clippy`, `cargo fmt`, `cargo bench` |
| AGENTS.md | Non-Negotiable Rules | Rules 8–12 enforce validation |
| docs/PRD-TESTING | §4 Benchmarks | `bench-regression.py` threshold definitions |
