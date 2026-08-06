# Orchestration Example

## Scenario: Adding a new storage codec

### Step 1: Repository Intelligence (P16)
- Analyzed kcm-storage crate structure
- Identified existing codec implementations
- Mapped dependency impacts

### Step 2: Task Planner (P2)
- Created implementation plan
- Identified affected files: compress.rs, lib.rs, tests/
- Estimated effort: 2 days

### Step 3: Change Impact Analysis (P3)
- Direct impact: kcm-storage
- Indirect impact: kcm-compute, kcm-runtime
- Specification impact: KCM_COMPRESSION_SPEC.md

### Step 4: Specification Lock (P4)
- Validated no frozen contract violations
- Required spec update for new codec
- Approved spec change

### Step 5: Architecture Guardian (P5)
- Validated codec follows single-responsibility
- Confirmed no circular dependencies
- Approved architecture

### Step 6: Implementation
- Implemented codec in compress.rs
- Added unit tests
- Added property tests

### Step 7: Quality Gates
- Code Quality (P10): Passed
- Testing (P9): Passed
- Documentation (P11): Updated
- Release Readiness (P12): Approved
