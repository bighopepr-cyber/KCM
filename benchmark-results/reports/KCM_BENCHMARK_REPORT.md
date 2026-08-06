# KCM Performance Benchmark Report

**Generated**: 2026-08-01T03:25:21Z

## Environment

- **os**: placeholder - populated at benchmark execution time
- **cpu**: placeholder
- **cores**: 0
- **ram_mb**: 0
- **rust_version**: placeholder
- **llvm_version**: placeholder

## Performance Results

| Benchmark | Median | Lower | Upper | Throughput |
|-----------|--------|-------|-------|------------|
| bitmap_get/10000 | 830 ns | 725 ns | 944 ns | 1204921 ops/s |
| bitmap_get/100000 | 7.61 µs | 6.90 µs | 8.43 µs | 131397 ops/s |
| bitmap_get/1000000 | 68.70 µs | 58.90 µs | 81.24 µs | 14556 ops/s |
| column_sequential_scan/1000 | 90 ns | 79 ns | 100 ns | 11156979 ops/s |
| column_sequential_scan/10000 | 567 ns | 519 ns | 621 ns | 1764540 ops/s |
| column_sequential_scan/100000 | 7.35 µs | 6.34 µs | 8.60 µs | 136078 ops/s |
| column_sequential_scan/1000000 | 94.59 µs | 88.68 µs | 101.33 µs | 10572 ops/s |
| dictionary_lookup/1000 | 43.66 µs | 38.38 µs | 49.18 µs | 22903 ops/s |
| dictionary_lookup/10000 | 541.84 µs | 460.50 µs | 631.51 µs | 1846 ops/s |
| dictionary_lookup/100000 | 6.87 ms | 6.24 ms | 7.58 ms | 146 ops/s |