# kcm-schema

Schema generation tool for KCM.

## Status: Planned

## Commands

| Command | Description |
|---------|-------------|
| kcm-schema show <db> | Show current schema |
| kcm-schema generate <spec> | Generate schema from spec |
| kcm-schema validate <db> | Validate schema |
| kcm-schema diff <db1> <db2> | Compare schemas |

## Usage

```bash
# Show schema
kcm-schema show my_knowledge.db

# Validate schema
kcm-schema validate my_knowledge.db
```
