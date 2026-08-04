# kcm-snapshot

Point-in-time snapshot tool for KCM.

## Status: Implemented

## Commands

| Command | Description |
|---------|-------------|
| kcm-snapshot create | Create a snapshot (default: 1000 facts) |
| kcm-snapshot list | List available snapshots |
| kcm-snapshot restore -i <id> | Restore from snapshot by ID |
| kcm-snapshot delete -i <id> | Delete a snapshot by ID |

## Usage

```bash
# Create snapshot
kcm-snapshot create

# List snapshots
kcm-snapshot list

# Restore snapshot
kcm-snapshot restore -i snap_1691234567

# Delete snapshot
kcm-snapshot delete -i snap_1691234567
```
