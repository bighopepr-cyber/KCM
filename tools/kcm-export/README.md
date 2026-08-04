# kcm-export

Data export tool for KCM.

## Status: Partially Implemented

## Commands

| Command | Description |
|---------|-------------|
| kcm-export json <db> <output> | Export to JSON |
| kcm-export csv <db> <output> | Export to CSV |
| kcm-export query <db> <kql> <out> (Not Yet Implemented) | Export query results |

## Usage

```bash
# Export to CSV
kcm-export csv my_knowledge.db output.csv

# Export query results
kcm-export query my_knowledge.db "SELECT * FROM facts" results.csv
```
