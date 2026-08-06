# Skill Router

> Document ID: KCM-ROUTE-001 | Version: 2.0.0 | Status: Active

## Overview

The Skill Router selects which skills activate based on change type, component, and risk level. It does NOT use a fixed order — routing is deterministic based on input classification.

## Routing Algorithm

```
1. Classify task type (Feature/BugFix/Security/Performance/Docs/Refactor/Release/Emergency)
2. Identify affected components (crates, APIs, SDKs, docs)
3. Determine risk level (Low/Medium/High/Critical)
4. Select primary skill based on change type
5. Select supporting skills based on component and risk
6. Validate routing against Authority System
7. Generate execution pipeline
```

## Routing by Change Type

| Change Type | Primary Skill | Supporting Skills | Pipeline |
|-------------|--------------|-------------------|----------|
| Storage Change | P6 DB Specialist | P5, P4, P7, P10, P9, P8, P11 | standard |
| API Change | P4 Spec Lock | P5, P11, P9, P12 | standard |
| FFI Change | P4 Spec Lock | P7, P5, P10, P9, P11 | standard |
| Security Change | P7 Security | P4, P5, P10, P9, P11 | standard |
| Performance Change | P8 Performance | P6, P10, P9, P11 | optimization |
| Bug Fix | P14 Debugging | P10, P9, P12 | bugfix |
| New Feature | P2 Planning | P3, P4, P5, Domain, P10, P9, P8, P11, P13, P12 | feature |
| Documentation | P11 Documentation | P4, P12 | documentation |
| Refactoring | P5 Architecture | P10, P9, P11, P12 | refactor |
| Release | P12 Release | P1 | release |
| Emergency | P14 Debugging | P10, P9, P12 | emergency |

## Routing by Component

| Component | Primary Skill | Authority |
|-----------|--------------|-----------|
| kcm-core | P5 Arch Guardian | Block |
| kcm-storage | P6 DB Specialist | Block |
| kcm-compute | P6 DB Specialist | Block |
| kcm-reasoning | P6 DB Specialist | Block |
| kcm-optimizer | P6 DB Specialist | Block |
| kcm-runtime | P6 DB Specialist | Block |
| kcm-interface | P4 Spec Lock | Veto |
| kcm-distributed | P5 Arch Guardian | Block |
| kcm-ml | P5 Arch Guardian | Block |
| kcm-security | P7 Security | Block |
| kcm-compliance | P7 Security | Block |
| kcm-testing | P9 Testing | Block |
| kcm-server | P5 Arch Guardian | Block |

## Routing by Risk Level

| Risk | Criteria | Required Skills | Approval Required |
|------|---------|----------------|-------------------|
| Low | Internal, no API change, no security | P10 + P9 | 1 reviewer |
| Medium | API change, non-breaking, no security | P4 + P5 + P9 + P11 | 2 reviewers |
| High | Breaking change, security impact, format change | P4 + P5 + P7 + P9 + P11 + P12 | All guardians |
| Critical | Production impact, data loss, vulnerability | All skills | P1 |

## Risk Assessment Matrix

| Factor | Low | Medium | High | Critical |
|--------|-----|--------|------|----------|
| API Surface | No change | Additive | Breaking | Removed |
| Security | No impact | Indirect | Direct | Vulnerability |
| Performance | No change | < 5% | 5-15% | > 15% |
| Data Format | No change | Compatible | Incompatible | Data loss |
| Dependencies | No change | Additive | Remove | Core change |
| Documentation | Minor | Moderate | Major | Missing |

## Multi-Component Routing

When a task affects multiple components, the router:

1. Identifies ALL affected components
2. Selects the PRIMARY skill for the most critical component
3. Adds SUPPORTING skills for all other affected components
4. Validates no circular dependencies in skill chain
5. Orders skills by authority hierarchy

## Routing Validation

| Check | Method | Pass Criteria |
|-------|--------|--------------|
| Primary skill exists | Lookup table | Skill found |
| Supporting skills exist | Lookup table | All skills found |
| No circular dependencies | Graph analysis | Acyclic |
| Authority hierarchy respected | Authority matrix | No override violations |
| Pipeline matches type | Pipeline lookup | Correct pipeline |
