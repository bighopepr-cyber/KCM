# Code Review Example

## Change: Add new query operator

### Review Findings

| # | Severity | Category | Finding | Recommendation |
|---|----------|----------|---------|----------------|
| 1 | High | Hidden Bug | Missing bounds check | Add bounds validation |
| 2 | Medium | Maintainability | Function too long | Split into helpers |
| 3 | Low | Quality | Missing doc comment | Add doc comment |

### Decision
- **CONDITIONAL APPROVAL** — Fix High issue before merge