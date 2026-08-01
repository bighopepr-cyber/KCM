# KCM Engineering Task Plan: Inference Benchmark Memory Engineering

## Requirement
Redesign the inference benchmark subsystem to scale deterministically with input size, eliminating `KcmError::OutOfMemory` caused by fixed-capacity limitations. Audit and optimize memory usage in inference and benchmark components.

## Affected Crates
- `kcm-core` (Dynamic resizing for `DenseVec`, `Dictionary`)
- `kcm-testing` (Benchmark fixture updates)
- `kcm-runtime` (Benchmark update)

## Implementation Strategy
1. **Dynamic `DenseVec`**: Refactor `DenseVec` to support resizing. Implement `reserve` or automatic doubling when `len == capacity`. Remove hard `OutOfMemory` check on `push`.
2. **Dynamic `Dictionary`**: Remove `u32::MAX` entry limitation. Implement dynamic resizing for the `entries` `Vec`.
3. **Benchmark Fixture Refactoring**: Update `DatasetConfig` and fixture generators in `kcm-testing/src/bench_fixtures.rs` to not depend on hard-coded capacities. Use dynamic structures for data generation.
4. **Benchmark Cleanup**: Update `kcm-runtime/benches/micro.rs` to ensure separation of setup and measurement, proper diagnostic reporting instead of `unwrap()`/`expect()`.
5. **Memory Engineering**: Replace transient heap objects and unnecessary clones in `kcm-reasoning/src/inference.rs` with reusable buffers.
6. **Documentation Update**: Update `docs/KCM_ARCHITECTURE.md` and related specs to reflect dynamic memory scaling invariants.

## Testing Strategy
- Unit tests for dynamic `DenseVec` and `Dictionary`.
- Integration tests in `kcm-testing` to verify benchmark scaling without `OutOfMemory`.
- Performance verification: Run benchmarks and confirm stability and deterministic scaling.

## Risks
- **Performance**: Dynamic resizing might introduce overhead during benchmark runs if not managed correctly. Mitigation: Pre-allocate initial capacity in benchmark setup (not measured).
- **Correctness**: Incorrect resizing logic in `DenseVec` or `Dictionary` could lead to data corruption. Mitigation: Comprehensive unit tests and property testing.

## Specification Impact
- `PRD.md` (Core types, storage)
- `docs/KCM_ARCHITECTURE.md` (System architecture)
- `docs/KCM_RUNTIME_SPEC.md` (Concurrency, memory)

## Engineering Gates Verification
- Gate 1-6 apply.
