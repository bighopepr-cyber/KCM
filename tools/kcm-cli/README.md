# kcm-cli

Main command-line interface for KCM.

## Status: Planned

## Commands

| Command | Description |
|---------|-------------|
| kcm serve | Start HTTP server |
| kcm query <kql> | Execute KQL query |
| kcm insert <fact> | Insert a fact |
| kcm schema | Show database schema |
| kcm stats | Show database statistics |
| kcm version | Show version |

## Options

| Option | Description |
|--------|-------------|
| --db <path> | Database path (default: kcm.db) |
| --port <port> | Server port (default: 8080) |
| --format <fmt> | Output format (json, table, csv) |

## Installation

```bash
cargo install kcm-cli
```

## Usage

```bash
# Start server
kcm serve --db my_knowledge.db

# Execute query
kcm query "SELECT * FROM facts WHERE subject = 'planet'"

# Insert fact
kcm insert --subject planet --predicate orbits --object sun --confidence 0.99
```
