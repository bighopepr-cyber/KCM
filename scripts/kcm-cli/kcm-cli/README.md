# kcm-cli

Main command-line interface for KCM.

## Status: Implemented

## Commands

| Command | Description |
|---------|-------------|
| kcm create | Create a new database |
| kcm stats | Show database statistics |
| kcm benchmark | Run benchmark suite |
| kcm version | Show version |

## Options

| Option | Description |
|--------|-------------|
| --db <path> | Database path (default: kcm.db) |
| --format <fmt> | Output format (json, table, csv) |

## Installation

```bash
cargo install kcm-cli
```

## Usage

```bash
# Create database
kcm create

# Show statistics
kcm stats

# Run benchmarks
kcm benchmark
```
