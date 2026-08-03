# kcm-export

Data export tool for KCM.

## Status: Planned

## Commands

| Command | Description |
|---------|-------------|
| kcm-export csv <db> <output> | Export to CSV |
| kcm-export json <db> <output> | Export to JSON |
| kcm-export parquet <db> <output> | Export to Parquet |
| kcm-export query <db> <kql> <out> | Export query results |

## Usage

```bash
# Export to CSV
kcm-export csv my_knowledge.db output.csv

# Export query results
kcm-export query my_knowledge.db "SELECT * FROM facts" results.csv
```
