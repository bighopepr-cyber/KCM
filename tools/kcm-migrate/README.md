# kcm-migrate

Schema migration tool for KCM.

## Status: Planned

> Not Yet Implemented. No migration files are read or applied.

## Commands

| Command | Description |
|---------|-------------|
| kcm-migrate status <db> | Show migration status |
| kcm-migrate up <db> | Apply pending migrations |
| kcm-migrate down <db> | Rollback last migration |
| kcm-migrate create <name> | Create new migration |

## Usage

```bash
# Check status
kcm-migrate status my_knowledge.db

# Apply migrations
kcm-migrate up my_knowledge.db
```
