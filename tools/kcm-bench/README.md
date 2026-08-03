# kcm-bench

Benchmarking tool for KCM.

## Status: Planned

## Commands

| Command | Description |
|---------|-------------|
| kcm-bench run <db> | Run full benchmark suite |
| kcm-bench query <db> | Benchmark query performance |
| kcm-bench insert <db> | Benchmark insert performance |
| kcm-bench compare <base> <current> | Compare results |

## Usage

```bash
# Run benchmarks
kcm-bench run my_knowledge.db

# Compare results
kcm-bench compare baseline.json current.json
```
