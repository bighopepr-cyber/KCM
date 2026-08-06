# kcm-profile

Performance profiling tool for KCM.

## Status: Implemented

## Commands

| Command | Description |
|---------|-------------|
| kcm-profile query <db> <kql> | Profile a query |
| kcm-profile insert <db> | Profile insert operations |
| kcm-profile memory <db> | Profile memory usage |

## Usage

```bash
# Profile a query
kcm-profile query my_knowledge.db "SELECT * FROM facts"
```
