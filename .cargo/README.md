# .cargo/ Configuration

## Overview

The `.cargo/` directory contains Cargo build configuration for the KCM workspace. It defines workspace-level settings that control compilation behavior, linker configuration, and dependency fetching.

## Purpose

Provides centralized Cargo build settings applied to all crates in the KCM workspace, ensuring consistent compilation across development and CI environments.

## Responsibilities

| Responsibility | Description |
|----------------|-------------|
| Build optimization | Platform-specific CPU targeting via `target-cpu=native` |
| Linker configuration | Optional mold linker support for faster builds |
| Registry configuration | Sparse protocol for crates.io index fetching |
| Network settings | Git fetch behavior and retry policies |
| Terminal output | Verbose compilation output |

## Folder Structure

```
.cargo/
├── config.toml      # Cargo build configuration
├── README.md        # This file
├── SECURITY.md      # Security policy
├── CONTRIBUTING.md  # Contribution guidelines
└── CODE_OF_CONDUCT.md # Community guidelines
```

## Public API

This directory does not expose a public API. Configuration is consumed automatically by Cargo when building the workspace.

## Internal Components

### config.toml

Workspace-level Cargo configuration with the following sections:

| Section | Purpose |
|---------|---------|
| `[build]` | Build-time settings (linker flags) |
| `[Target.*]` | Platform-specific rustflags |
| `[registries.crates-io]` | Registry protocol configuration |
| `[net]` | Network behavior settings |
| `[term]` | Terminal output settings |

## Dependencies

No external dependencies. Configuration is consumed by Cargo itself.

## Integration

The `.cargo/config.toml` applies automatically to all `cargo build`, `cargo test`, and `cargo bench` commands executed within the workspace root. No explicit configuration is needed.

## Build

```bash
cargo build --workspace
```

Settings from `.cargo/config.toml` are applied automatically.

## Run

```bash
cargo run --bin kcm-server
```

## Test

```bash
cargo test --workspace
```

## Examples

Override configuration locally via environment variables:

```bash
# Override target CPU
RUSTFLAGS="-C target-cpu=x86-64-v3" cargo build --release

# Disable verbose output
CARGO_TERM_VERBOSE=false cargo build
```

## References

- [Cargo Configuration Documentation](https://doc.rust-lang.org/cargo/reference/config.html)
- [Cargo Build Profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)
- `AGENTS.md` — Engineering constitution
- `docs/specs/KCM_SPECIFICATION.md` — Technical specification
