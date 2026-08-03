# kcm-snapshot

Point-in-time snapshot tool for KCM.

## Status: Planned

## Commands

| Command | Description |
|---------|-------------|
| kcm-snapshot create <db> | Create a snapshot |
| kcm-snapshot list <db> | List available snapshots |
| kcm-snapshot restore <db> <snap> | Restore from snapshot |
| kcm-snapshot delete <db> <snap> | Delete a snapshot |

## Usage

```bash
# Create snapshot
kcm-snapshot create my_knowledge.db

# List snapshots
kcm-snapshot list my_knowledge.db
```
