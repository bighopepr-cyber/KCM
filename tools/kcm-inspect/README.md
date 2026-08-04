# kcm-inspect

Data inspection tool for KCM.

## Status: Implemented

## Commands

| Command | Description |
|---------|-------------|
| kcm-inspect schema <db> | Show database schema |
| kcm-inspect columns <db> | Show column metadata |
| kcm-inspect stats <db> | Show database statistics |
| kcm-inspect dictionary <db> | Show dictionary contents |

## Usage

```bash
# Show schema
kcm-inspect schema my_knowledge.db

# Show statistics
kcm-inspect stats my_knowledge.db
```
