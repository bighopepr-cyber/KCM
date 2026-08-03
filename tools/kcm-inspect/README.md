# kcm-inspect

Data inspection tool for KCM.

## Status: Planned

## Commands

| Command | Description |
|---------|-------------|
| kcm-inspect schema <db> | Show database schema |
| kcm-inspect columns <db> | Show column metadata |
| kcm-inspect dictionary <db> | Show dictionary contents |
| kcm-inspect stats <db> | Show database statistics |
| kcm-inspect row <db> <id> | Show specific row |

## Usage

```bash
# Show schema
kcm-inspect schema my_knowledge.db

# Show statistics
kcm-inspect stats my_knowledge.db
```
