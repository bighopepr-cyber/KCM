# Performance Checklist

## Benchmark
- [ ] Every performance claim has benchmark
- [ ] Benchmarks use criterion
- [ ] Statistical analysis present
- [ ] Baseline stored in benchmark-results/

## SIMD
- [ ] AVX2 with runtime detection
- [ ] Scalar fallback present
- [ ] No undefined behavior
- [ ] Correct alignment

## Memory
- [ ] < 34 bytes per fact
- [ ] 64-byte alignment for hot data
- [ ] No unnecessary allocations
- [ ] Cache-friendly access patterns

## Regression
- [ ] < 5% regression: acceptable
- [ ] 5-10% regression: warning with justification
- [ ] > 10% regression: failure, blocks merge
