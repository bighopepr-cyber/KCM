# kcm-bench

Benchmarking tool for KCM.

## Status: Implemented

## Commands

| Command | Description |
|---------|-------------|
| kcm-bench run <db> | Run full benchmark suite |
| kcm-bench query <db> | Benchmark query performance |
| kcm-bench insert <db> | Benchmark insert performance |
| kcm-bench batch <db> | Run batch operations |

## Usage

```bash
# Run benchmarks
kcm-bench run my_knowledge.db

# Run batch operations
kcm-bench batch my_knowledge.db
```
