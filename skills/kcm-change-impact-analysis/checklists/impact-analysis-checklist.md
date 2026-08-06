# Impact Analysis Checklist

## Change Identification

- [ ] Change scope has been clearly defined
- [ ] Change type has been categorized (bug fix/feature/API/format/security)
- [ ] Change urgency has been assessed
- [ ] Change stakeholders have been identified

## Module Impact

- [ ] All directly affected crates have been identified
- [ ] All transitively affected crates have been identified
- [ ] Dependency direction has been validated (no cycles introduced)
- [ ] Interface changes have been documented

## Specification Impact

- [ ] SSOT document impact has been assessed
- [ ] Specification document impact has been assessed
- [ ] Frozen contract impact has been assessed
- [ ] Backward compatibility impact has been documented

## Compatibility Analysis

- [ ] API compatibility has been validated
- [ ] FFI compatibility has been validated (if applicable)
- [ ] Storage format compatibility has been validated (if applicable)
- [ ] Protocol compatibility has been validated (if applicable)
- [ ] SDK compatibility has been validated (if applicable)

## Testing Impact

- [ ] Required test changes have been identified
- [ ] New test requirements have been identified
- [ ] Regression test requirements have been identified
- [ ] Security test requirements have been identified (if applicable)

## Documentation Impact

- [ ] Documentation changes have been identified
- [ ] API documentation changes have been identified
- [ ] Specification document changes have been identified
- [ ] Changelog entry requirements have been identified

## Risk Assessment

- [ ] Risk level has been classified (low/medium/high/critical)
- [ ] Risk factors have been documented
- [ ] Mitigation strategies have been proposed
- [ ] Rollback strategy has been identified
