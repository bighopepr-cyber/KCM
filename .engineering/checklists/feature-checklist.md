# Feature Checklist

> Document ID: KCM-CHK-FEAT-001 | Version: 1.0.0

## Pre-Implementation

- [ ] Task classified as Feature
- [ ] Pipeline selected: feature.md
- [ ] Risk level assessed
- [ ] SSOT requirement identified
- [ ] Specification exists or will be created

## Planning Phase

- [ ] P16 Intelligence completed
- [ ] P2 Planning completed
- [ ] P3 Impact Analysis completed
- [ ] Execution plan created
- [ ] All affected files identified
- [ ] All affected specs identified
- [ ] All required skills identified
- [ ] Dependencies mapped
- [ ] Risks assessed

## Validation Phase

- [ ] P4 Specification Lock validated
- [ ] P5 Architecture Guardian validated
- [ ] No frozen contract violations
- [ ] No architecture violations
- [ ] SSOT alignment confirmed

## Implementation Phase

- [ ] Code follows coding standards
- [ ] Error handling complete (Result<T, KcmError>)
- [ ] No unwrap/panic/TODO in production code
- [ ] SAFETY comments on unsafe blocks
- [ ] Unit tests written
- [ ] Integration tests written

## Quality Phase

- [ ] P10 Code Quality validated
- [ ] cargo fmt --check passes
- [ ] cargo clippy -- -D warnings passes
- [ ] cargo build --workspace passes
- [ ] cargo test --lib passes
- [ ] cargo test --test passes

## Testing Phase

- [ ] P9 Testing Verification completed
- [ ] All unit tests pass
- [ ] All integration tests pass
- [ ] All property tests pass
- [ ] Test coverage 100%

## Benchmark Phase (if performance-related)

- [ ] P8 Performance Engineer completed
- [ ] Baseline benchmark recorded
- [ ] Comparison benchmark recorded
- [ ] Regression < 5%

## Documentation Phase

- [ ] P11 Documentation Guardian completed
- [ ] README updated (if needed)
- [ ] spesifikasi.md updated
- [ ] CHANGELOG updated
- [ ] Cross-references validated

## Review Phase

- [ ] P13 Code Review Auditor completed
- [ ] No critical findings
- [ ] All major findings addressed

## Release Phase

- [ ] P12 Release Readiness validated
- [ ] All CI jobs pass
- [ ] No regressions
- [ ] Version bumped
- [ ] Changelog updated

## Final Approval

- [ ] P1 Engineering Orchestrator approved
- [ ] All deliverables complete
- [ ] All exit criteria met
