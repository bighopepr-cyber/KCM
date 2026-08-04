# kcm-doctor

Health check tool for KCM.

## Status: Partially Implemented

## Commands

| Command | Description |
|---------|-------------|
| kcm-doctor check <db> | Run full health check |
| kcm-doctor integrity <db> | Verify data integrity (Not Yet Implemented) |
| kcm-doctor wal <db> | Check WAL consistency (Not Yet Implemented) |
| kcm-doctor repair <db> | Attempt automatic repair (Not Yet Implemented) |

## Usage

```bash
# Run health check
kcm-doctor check my_knowledge.db

# Repair if needed
kcm-doctor repair my_knowledge.db
```
