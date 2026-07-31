# KCM Versioning Specification

**Document ID:** KCM-VER-001  
**Version:** 1.0.0

---

## 1. Purpose

Defines versioning strategy, schema migration, and compatibility guarantees.

---

## 2. Version Strategy

### 2.1 Semantic Versioning

KCM follows [Semantic Versioning 2.0](https://semver.org/):

```
MAJOR.MINOR.PATCH

MAJOR: Breaking API or storage format changes
MINOR: Backward-compatible feature additions
PATCH: Backward-compatible bug fixes
```

### 2.2 Version Scope

| Component | Version | Current |
|-----------|---------|---------|
| kcm-core | Independent | 0.1.0 |
| kcm-storage | Independent | 0.1.0 |
| kcm-compute | Independent | 0.1.0 |
| kcm-reasoning | Independent | 0.1.0 |
| kcm-optimizer | Independent | 0.1.0 |
| kcm-runtime | Independent | 0.1.0 |
| kcm-interface | Independent | 0.1.0 |
| kcm-distributed | Independent | 0.1.0 |
| kcm-ml | Independent | 0.1.0 |
| kcm-security | Independent | 0.1.0 |
| kcm-compliance | Independent | 0.1.0 |
| kcm-testing | Independent | 0.1.0 |

---

## 3. Storage Format Versioning

### 3.1 File Format Version

```
File Header Byte 5: Format version (u8)
```

Current: `2`

### 3.2 Migration Rules

| Change Type | Version Action | Migration Required |
|-------------|---------------|-------------------|
| Add column | MAJOR | Yes — new file format |
| Remove column | MAJOR | Yes — new file format |
| Change column type | MAJOR | Yes — new file format |
| Add index type | MINOR | No — backward compatible |
| Change compression | MINOR | No — transparent decompression |
| Change block size | PATCH | No — transparent |

### 3.3 Backward Compatibility

| Scenario | Support |
|----------|---------|
| New code reads old format | Yes (within MAJOR version) |
| Old code reads new format | No (within MINOR version) |
| Format migration tool | Provided for MAJOR version changes |

---

## 4. API Versioning

### 4.1 C FFI Stability

| Guarantee | Scope |
|-----------|-------|
| Function signatures stable within MAJOR | Yes |
| Error codes stable within MAJOR | Yes |
| Struct layouts stable within MAJOR | Yes |
| New functions added in MINOR | Yes |
| Deprecated functions marked in MINOR | Yes |
| Deprecated functions removed in MAJOR | Yes |

### 4.2 Rust API Stability

| Guarantee | Scope |
|-----------|-------|
| Public trait implementations | Stable within MAJOR |
| Generic bounds | Stable within MAJOR |
| Error type variants | Additive in MINOR |

---

## 5. WAL Compatibility

| Property | Rule |
|----------|------|
| Entry format | Fixed per MAJOR version |
| Replay | WAL entries from same MAJOR version can be replayed |
| Cross-version | WAL from different MAJOR version is ignored (not replayed) |

---

## 6. Constraints

| Constraint | Rationale |
|------------|-----------|
| No implicit migration | User must explicitly upgrade |
| WAL forward-compatible within MAJOR | Recovery must work across patches |
| Breaking changes require MAJOR bump | Prevents silent data corruption |

---

## 7. References

- **Depends on:** KCM_SPECIFICATION (KCM_SPECIFICATION)
- **Parent specs:** KCM_SPECIFICATION (KCM_SPECIFICATION)
- **Related:** KCM_COLUMNAR_FORMAT_SPEC (KCM_COLUMNAR_FORMAT_SPEC), KCM_API_SPEC (KCM_API_SPEC)
