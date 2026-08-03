# kcm-compact

Storage compaction tool for KCM.

## Status: Planned

## Commands

| Command | Description |
|---------|-------------|
| kcm-compact run <db> | Run full compaction |
| kcm-compact analyze <db> | Analyze fragmentation |
| kcm-compact columns <db> | Compact specific columns |
| kcm-compact stats <db> | Show compaction statistics |

## Usage

```bash
# Analyze before compaction
kcm-compact analyze my_knowledge.db

# Run compaction
kcm-compact run my_knowledge.db
```
