# kcm-backup

Database backup tool for KCM.

## Status: Planned

## Commands

| Command | Description |
|---------|-------------|
| kcm-backup create <db> | Create full backup |
| kcm-backup list | List available backups |
| kcm-backup verify <backup> | Verify backup integrity |
| kcm-backup delete <backup> | Delete a backup |

## Usage

```bash
# Create backup
kcm-backup create my_knowledge.db

# List backups
kcm-backup list

# Verify backup
kcm-backup verify backups/my_knowledge_2026-08-03.kcm
```
