# kcm-import

Data import tool for KCM.

## Status: Planned

## Commands

| Command | Description |
|---------|-------------|
| kcm-import csv <db> <file> | Import from CSV |
| kcm-import json <db> <file> | Import from JSON |
| kcm-import parquet <db> <file> | Import from Parquet |
| kcm-import schema <file> | Infer schema from file |

## Usage

```bash
# Import CSV
kcm-import csv my_knowledge.db data.csv

# Import JSON
kcm-import json my_knowledge.db data.json
```
