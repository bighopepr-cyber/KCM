# Cargo Configuration Technical Specification

## Overview

This specification defines the Cargo build configuration for the KCM workspace, including platform-specific optimizations, registry settings, and network behavior.

## Scope

This specification covers `.cargo/config.toml` and related build configuration files. It does not cover individual crate `Cargo.toml` settings.

## Responsibilities

| Responsibility | Description |
|----------------|-------------|
| Build configuration | Workspace-level Cargo settings applied to all crates |
| Optimization | Platform-specific CPU targeting and compiler flags |
| Toolchain management | Rust version pinning and registry configuration |

## Technical Specification

### config.toml Settings

```toml
# [build] section
[build]
# Optional: rustflags for linker configuration

# [Target] sections - platform-specific rustflags
[Target.'cfg(target_os = "linux")']
rustflags = ["-C", "target-cpu=native"]

[Target.'cfg(target_os = "macos")']
rustflags = ["-C", "target-cpu=native"]

# [registries] section
[registries.crates-io]
protocol = "sparse"

# [net] section
[net]
git-fetch-with-cli = true
retry = 3

# [term] section
[term]
verbose = true
```

### rust-toolchain.toml

```toml
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
```

### Profile Settings

```toml
[profile.release]
opt-level = 3
lto = "thin"
codegen-units = 1
panic = "abort"

[profile.dev]
opt-level = 0
debug = true
incremental = true

[profile.dev.package."*"]
opt-level = 2
```

## Architecture

```
.cargo/
├── config.toml           # Workspace Cargo configuration
├── rust-toolchain.toml   # Pinned Rust toolchain (optional)
└── README.md             # Documentation
```

## Internal Components

### config.toml Sections

| Section | Purpose | Impact |
|---------|---------|--------|
| `[build]` | Workspace-wide build flags | Affects all crates |
| `[Target.*]` | Platform-specific flags | Linux/macOS only |
| `[registries]` | Dependency source | crates.io access |
| `[net]` | Network behavior | Git fetch, retries |
| `[term]` | Output verbosity | Developer experience |

### rustflags Breakdown

| Flag | Purpose | Platform |
|------|---------|----------|
| `-C target-cpu=native` | Optimize for local CPU | Linux, macOS |
| `-C link-arg=-fuse-ld=mold` | Use mold linker | Linux (optional) |

## Data Model

### Configuration Schema

```
CargoConfig:
  build: BuildConfig
  targets: Map<TargetSpec, TargetConfig>
  registries: Map<RegistryName, RegistryConfig>
  net: NetConfig
  term: TermConfig
```

### Target Spec Format

```
cfg(target_os = "linux")
cfg(target_os = "macos")
cfg(target_arch = "x86_64")
```

## Execution Flow

### Build Configuration Resolution

```
1. Cargo reads .cargo/config.toml
2. Merge with user-level config (~/.cargo/config.toml)
3. Apply environment variable overrides
4. Resolve target-specific settings
5. Apply to compilation
```

### Target Resolution

```
1. Detect host platform
2. Match target spec against [Target.*] sections
3. Apply matching rustflags
4. Fall back to [build] section for unmatched targets
```

## Public API

This configuration does not expose a public API. Settings are consumed by Cargo internally.

## Configuration

### Complete Configuration Reference

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `build.rustflags` | Array | `[]` | Extra flags for rustc |
| `build.jobs` | Integer | CPU count | Parallel compilation jobs |
| `target.*.rustflags` | Array | `[]` | Platform-specific flags |
| `registries.*.protocol` | String | `"git"` | Index protocol |
| `net.git-fetch-with-cli` | Boolean | `false` | Use git CLI for fetches |
| `net.retry` | Integer | `2` | Network retry count |
| `term.verbose` | Boolean | `false` | Verbose output |

### Environment Variable Overrides

| Variable | Overrides |
|----------|-----------|
| `RUSTFLAGS` | `build.rustflags` and `target.*.rustflags` |
| `CARGO_BUILD_JOBS` | `build.jobs` |
| `CARGO_TERM_VERBOSE` | `term.verbose` |
| `CARGO_NET_RETRY` | `net.retry` |

## Dependencies

| Dependency | Type | Justification |
|-----------|------|---------------|
| Cargo | Build tool | Workspace build system |
| Rust toolchain | Compiler | Code compilation |

## Error Handling

| Error | Cause | Resolution |
|-------|-------|------------|
| Invalid TOML | Syntax error in config.toml | Fix TOML syntax |
| Unknown setting | Typo in setting name | Check Cargo documentation |
| Conflicting flags | Incompatible rustflags | Remove conflicting flags |
| Platform not found | Target spec mismatch | Add appropriate `[Target.*]` section |

## Performance Characteristics

| Setting | Impact | Measurement |
|---------|--------|-------------|
| `target-cpu=native` | 10-30% faster runtime | Criterion benchmarks |
| `lto = "thin"` | 5-15% faster runtime, slower compile | Build time comparison |
| `codegen-units = 1` | 5-10% faster runtime | Criterion benchmarks |
| `incremental = true` | 50% faster rebuilds | Build time measurement |

## Security Considerations

- No custom registries allowed without security review
- No TLS/certificate verification overrides
- No credential-bearing URLs in configuration
- All rustflags reviewed for security implications

## Integration

The Cargo configuration integrates with:

```
Cargo.toml (workspace) ← .cargo/config.toml
rust-toolchain.toml    ← .cargo/config.toml
CI pipeline            ← .cargo/config.toml
Developer environment  ← .cargo/config.toml
```

## Sequence Diagram

### Build Configuration Loading

```
Developer/Cargo → Read .cargo/config.toml
  → Parse TOML
  → Merge with user config
  → Apply env overrides
  → Resolve target-specific settings
  → Apply to rustc invocation
```

## Architecture Diagram

```
┌─────────────────────────────────────┐
│         .cargo/config.toml          │
├─────────┬──────────┬────────┬───────┤
│ build   │ target   │ net    │ term  │
│ flags   │ platform │ config │ output│
├─────────┴──────────┴────────┴───────┤
│         Cargo Build System          │
├─────────────────────────────────────┤
│         Rust Compiler (rustc)       │
└─────────────────────────────────────┘
```

## References

- [Cargo Configuration Reference](https://doc.rust-lang.org/cargo/reference/config.html)
- [Cargo Build Profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- `AGENTS.md` — Engineering constitution
- `docs/specs/KCM_SPECIFICATION.md` — Technical specification

## SSOT Alignment

| SSOT Requirement | Specification | Implementation | Test |
|-----------------|---------------|----------------|------|
| R-BUILD-001 | Platform-specific CPU targeting | `config.toml [Target.*]` | CI build verification |
| R-BUILD-002 | Sparse registry protocol | `config.toml [registries]` | Dependency resolution |
| R-BUILD-003 | Git fetch via CLI | `config.toml [net]` | CI git operations |
| R-BUILD-004 | Verbose output | `config.toml [term]` | Developer experience |
