# KCM Versioning Specification

**Document ID:** KCM-VER-001
**Version:** 1.0.0
**Status:** Active
**Owner:** Specification Lock (P4)
**Authority:** SSOT.md §6

---

## 1. Purpose

Defines KCM's versioning strategy: semantic versioning, compatibility guarantees, deprecation policy, and LTS policy.

## 2. Semantic Versioning

KCM follows [Semantic Versioning 2.0.0](https://semver.org/).

### 2.1 Version Format

```
MAJOR.MINOR.PATCH
```

### 2.2 Bumping Rules

| Change Type | Version Bump | Example |
|-------------|-------------|---------|
| Bug fix | Patch (0.0.x) | WAL replay fix |
| New feature | Minor (0.x.0) | New codec, new index |
| Breaking API change | Major (x.0.0) | Remove FFI function |
| Format change | Major (x.0.0) | Header layout change |
| Dependency change | Patch or Minor | Depends on impact |

### 2.3 Breaking Changes

A change is breaking if:
- Public API signature changes
- Public API removed
- Behavioral contract changes
- File format changes (incompatible)
- FFI function removed/changed
- REST endpoint removed/changed
- gRPC RPC removed/changed

## 3. Compatibility Guarantees

### 3.1 Stability Levels

| Level | Definition | Guarantee |
|-------|-----------|-----------|
| Experimental | May change in any release | No compatibility guarantee |
| Beta | Stable API, may have minor changes | Best-effort compatibility |
| Stable | API frozen, semantic versioning | Full backward compatibility |

### 3.2 Crate Stability

| Crate | Stability |
|-------|-----------|
| kcm-core | Stable |
| kcm-storage | Stable |
| kcm-compute | Stable |
| kcm-reasoning | Stable |
| kcm-optimizer | Beta |
| kcm-runtime | Stable |
| kcm-interface | Stable |
| kcm-distributed | Beta |
| kcm-ml | Experimental |
| kcm-security | Stable |
| kcm-compliance | Beta |
| kcm-testing | Internal |
| kcm-server | Stable |

### 3.3 Backward Compatibility

| Change Type | Compatibility Requirement |
|-------------|-------------------------|
| New public method | Additive, no breaking change |
| New crate | Additive, no breaking change |
| New dependency | Must justify existence per Dependency Policy |
| API signature change | Breaking — requires version bump |
| Remove public API | Breaking — requires version bump + migration |
| Format change | Breaking — requires version bump + migration |
| FFI change | Breaking — requires SDK version bump |

## 4. Deprecation Policy

### 4.1 Steps

| Step | Timeline | Action |
|------|----------|--------|
| 1. Announce | Release N | Mark as deprecated in docs |
| 2. Warn | Release N+1 | Add runtime warnings |
| 3. Remove | Release N+2 | Remove from codebase |

### 4.2 Deprecation Annotations

```rust
#[deprecated(since = "0.2.0", note = "Use new_function() instead")]
pub fn old_function() { ... }
```

## 5. LTS (Long-Term Support) Policy

### 5.1 LTS Versions

| Version | Support Level | End of Life |
|---------|--------------|-------------|
| 1.0.x | Full support | 2030-06 |
| 1.1.x | Full support | 2030-09 |
| 2.0.x | Full support | 2031-06 |

### 5.2 Support Guarantees

- Security patches for 3 years after EOL
- Bug fixes for 2 years after EOL
- No feature additions after EOL
- Migration guides provided for major versions

## 6. File Format Versioning

### 6.1 Current Format

- **Magic:** `"KCMDB"` (5 bytes)
- **Version:** `2` (1 byte)
- **Total header:** 31 bytes

### 6.2 Version Evolution

| Version | Changes |
|---------|---------|
| 1 | Initial format |
| 2 | Added tombstone bitmap, BLAKE3 checksum |

### 6.3 Forward Compatibility

- Unknown fields skipped during load
- Version mismatch rejected with `KcmError::Corrupted`
- Future versions may add fields without breaking read

## 7. SDK Versioning

| Language | Package | Version Strategy |
|----------|---------|-----------------|
| Rust | kcm-core (crate) | Semantic versioning |
| C | FFI via kcm-interface | Tied to Rust crate |
| Python | kcm (PyPI) | Semantic versioning |
| JavaScript | @kcm/js (npm) | Semantic versioning |
| TypeScript | @kcm/ts (npm) | Semantic versioning |
| Go | github.com/kcm/go-sdk | Semantic versioning |
| Java | io.kcm:sdk (Maven) | Semantic versioning |
| .NET | Kcm.Sdk (NuGet) | Semantic versioning |
| C++ | libkcm (system lib) | Semantic versioning |

## 8. References

- **Implements:** SSOT.md §6 (Version Bumping Rules)
- **Depends on:** None
- **Related:** KCM_API_SPEC, KCM_COLUMNAR_FORMAT_SPEC
