# kcm-perf

Performance analyzer for KCM.

## Status: Planned

## Commands

| Command | Description |
|---------|-------------|
| kcm-perf benchmark <db> | Run benchmarks |
| kcm-perf compare <base> <target> | Compare performance |
| kcm-perf regression <db> | Check for regressions |
| kcm-perf report <results> | Generate report |

## Usage

```bash
# Run performance analysis
kcm-perf benchmark my_knowledge.db

# Compare against baseline
kcm-perf compare baseline.json current.json
```
