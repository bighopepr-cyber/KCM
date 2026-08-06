# kcm-restore

Database restore tool for KCM.

## Status: Implemented

## Commands

| Command | Description |
|---------|-------------|
| kcm-restore from -b <backup> | Restore from backup |
| kcm-restore list | List available backups |
| kcm-restore verify -p <path> | Verify restore point integrity |

## Usage

```bash
# Restore from backup
kcm-restore from -b backups/my_knowledge_2026-08-03.kcm

# List restore points
kcm-restore list

# Verify restore point
kcm-restore verify -p backups/my_knowledge_2026-08-03.kcm
```
