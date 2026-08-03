# kcm-doctor

Health check tool for KCM.

## Status: Planned

## Commands

| Command | Description |
|---------|-------------|
| kcm-doctor check <db> | Run full health check |
| kcm-doctor integrity <db> | Verify data integrity |
| kcm-doctor wal <db> | Check WAL consistency |
| kcm-doctor repair <db> | Attempt automatic repair |

## Usage

```bash
# Run health check
kcm-doctor check my_knowledge.db

# Repair if needed
kcm-doctor repair my_knowledge.db
```
