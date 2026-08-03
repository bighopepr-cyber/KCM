# Versioning Policy

| Field | Value |
|-------|-------|
| **Document ID** | KCM-REPO-008 |
| **Title** | Versioning Policy |
| **Version** | 1.0.0 |
| **Date** | 2026-08-03 |
| **Status** | Authoritative |
| **Authority** | Engineering Orchestrator (P1) |

---

## 1. Semantic Versioning

KCM follows [Semantic Versioning 2.0.0](https://semver.org/):

```
MAJOR.MINOR.PATCH
```

- **MAJOR**: Breaking API changes
- **MINOR**: New features, backward-compatible
- **PATCH**: Bug fixes, backward-compatible

## 2. Crate Versioning

| Crate | Current | Strategy |
|-------|---------|----------|
| kcm-core | 0.1.0 | Independent |
| kcm-storage | 0.1.0 | Independent |
| kcm-compute | 0.1.0 | Independent |
| kcm-reasoning | 0.1.0 | Independent |
| kcm-optimizer | 0.1.0 | Independent |
| kcm-runtime | 0.1.0 | Independent |
| kcm-interface | 0.1.0 | Independent |
| kcm-distributed | 0.1.0 | Independent |
| kcm-ml | 0.1.0 | Independent |
| kcm-security | 0.1.0 | Independent |
| kcm-compliance | 0.1.0 | Independent |
| kcm-testing | 0.1.0 | Independent |
| kcm-server | 0.1.0 | Independent |

## 3. API Stability Guarantees

- **Stable**: Will not change in minor/patch releases
- **Unstable**: May change in any release (prefixed with `unstable_`)
- **Deprecated**: Will be removed in next MAJOR release

## 4. Breaking Change Policy

Breaking changes require:
1. Deprecation notice in MINOR release
2. Migration guide published
3. 6-month deprecation period (or next MAJOR)

## 5. Rust Toolchain Versioning

Pinned via `rust-toolchain.toml`:
```toml
[toolchain]
channel = "stable"
```
