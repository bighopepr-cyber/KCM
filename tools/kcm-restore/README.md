# kcm-restore

Database restore tool for KCM.

## Status: Planned

## Commands

| Command | Description |
|---------|-------------|
| kcm-restore from <backup> <db> | Restore from backup |
| kcm-restore list <db> | List restore points |
| kcm-restore verify <backup> | Verify restore integrity |

## Usage

```bash
# Restore from backup
kcm-restore from backups/my_knowledge_2026-08-03.kcm my_knowledge.db
```
