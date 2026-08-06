# Skill Router

> Document ID: KCM-ROUTE-001 | Version: 1.0.0

## Overview

The Skill Router selects which skills activate based on change type, not fixed order.

## Routing Rules

### By Change Type

| Change Type | Primary Skill | Supporting Skills |
|-------------|--------------|-------------------|
| Storage Change | P6 DB Specialist | P5, P4, P7, P10, P9, P8, P11 |
| API Change | P4 Spec Lock | P5, P11, P9, P12 |
| FFI Change | P4 Spec Lock | P7, P5, P10, P9, P11 |
| Security Change | P7 Security | P4, P5, P10, P9, P11 |
| Performance Change | P8 Performance | P6, P10, P9, P11 |
| Bug Fix | P14 Debugging | P10, P9, P12 |
| New Feature | P2 Planning | P3, P4, P5, Domain, P10, P9, P8, P11, P13, P12 |
| Documentation | P11 Documentation | P4, P12 |
| Refactoring | P5 Architecture | P10, P9, P11, P12 |
| Release | P12 Release | P1 |

### By Component

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

### By Risk Level

| Risk | Required Skills | Approval |
|------|----------------|----------|
| Low | P10 + P9 | 1 reviewer |
| Medium | P4 + P5 + P9 + P11 | 2 reviewers |
| High | P4 + P5 + P7 + P9 + P11 + P12 | All guardians |
| Critical | All skills | P1 |