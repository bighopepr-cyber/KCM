# kcm-backup

Database backup tool for KCM.

## Status: Implemented

## Commands

| Command | Description |
|---------|-------------|
| kcm-backup create | Create full backup |
| kcm-backup verify -p <path> | Verify backup integrity |
| kcm-backup list | List available backups |

## Usage

```bash
# Create backup
kcm-backup create my_knowledge.db

# List backups
kcm-backup list

# Verify backup
kcm-backup verify backups/my_knowledge_2026-08-03.kcm
```
