# Folder Convention

| Field | Value |
|-------|-------|
| **Document ID** | KCM-REPO-004 |
| **Title** | Folder Convention |
| **Version** | 1.0.0 |
| **Date** | 2026-08-03 |
| **Status** | Authoritative |
| **Authority** | Engineering Orchestrator (P1) |

---

## 1. Target Repository Layout

```
KCM/
+-- crates/              Core engine crates (13 crates)
+-- sdk/                 SDK implementations (9 languages)
+-- tools/               CLI tools (17 tools)
+-- integrations/        Third-party integrations (15)
+-- deployment/          Docker, K8s, Helm, Terraform
+-- website/             Documentation website
+-- docs/                Specifications and documentation
+-- examples/            Usage examples (per-language)
+-- benchmark-results/   Benchmark artifacts
+-- scripts/             Build and automation scripts
+-- .github/             CI/CD and GitHub config
+-- assets/              Static assets
+-- tests/               Cross-crate integration tests
+-- third_party/         Vendored dependencies
```

## 2. Folder Requirements

Every folder MUST have:

1. A README.md explaining its purpose
2. A clear owner (team or individual)
3. A defined scope
4. No undocumented content

## 3. Folder Registry

| Folder | Purpose | Owner | Scope |
|--------|---------|-------|-------|
| crates/ | Core engine implementation | Engine Team | 13 Rust crates |
| sdk/ | Language SDKs | SDK Team | 9 language implementations |
| tools/ | CLI tools | Platform Team | 17 command-line tools |
| integrations/ | Third-party integrations | Integration Team | 15 integration adapters |
| deployment/ | Deployment configs | DevOps Team | Docker, K8s, Helm, Terraform |
| website/ | Documentation site | Documentation Team | Static HTML site |
| docs/ | Specifications | Architecture Team | All documentation |
| examples/ | Usage examples | Developer Experience | Per-language examples |
| benchmark-results/ | Benchmark data | Performance Team | Automated artifacts |
| scripts/ | Build scripts | DevOps Team | Automation scripts |
| .github/ | GitHub config | DevOps Team | CI/CD, templates |
| assets/ | Static assets | Design Team | Images, logos |
| tests/ | Integration tests | QA Team | Cross-crate tests |
| third_party/ | Vendored deps | Security Team | Audited dependencies |
