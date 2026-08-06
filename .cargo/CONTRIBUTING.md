# Contributing to .cargo/

> For core engine contribution rules, refer to the repository [CONTRIBUTING.md](../CONTRIBUTING.md).

## Overview

This guide covers how to contribute to the KCM Cargo build configuration. Changes to `.cargo/config.toml` affect all developers and CI pipelines, so they require careful review.

## Before Contributing

1. Verify the change is necessary and cannot be achieved via environment variables
2. Check if the setting applies workspace-wide or only to specific crates
3. Open an issue for discussion before modifying build configuration
4. Consider the impact on all supported platforms (Linux, macOS)

## Coding Standards

### TOML Style

- Use comments to explain non-obvious settings
- Group related settings under their respective sections
- Follow existing indentation and formatting
- Keep configuration minimal — only include settings that differ from Cargo defaults

### Configuration Conventions

- Use `[build]` for workspace-wide settings
- Use `[Target.*]` for platform-specific settings
- Use `[registries]` for registry configuration
- Use `[net]` for network settings
- Use `[term]` for terminal output settings

## Module Architecture Rules

Cargo configuration is workspace-level and applies to all crates:

| Setting | Scope | Impact |
|---------|-------|--------|
| `[build]` | All crates | Linker and build flags |
| `[Target.*]` | Platform-specific | CPU targeting, platform optimizations |
| `[registries]` | Workspace-wide | Dependency source |
| `[net]` | Workspace-wide | Network behavior |
| `[term]` | Workspace-wide | Output verbosity |

## Documentation Rules

| Rule | Description |
|------|-------------|
| Comment all settings | Explain why each non-default setting exists |
| Reference platforms | Note which platforms a setting affects |
| Document overrides | Explain how to override via environment variables |

## Testing Requirements

| Requirement | Validation |
|-------------|-----------|
| Build succeeds | `cargo build --workspace` |
| Tests pass | `cargo test --workspace` |
| Clippy clean | `cargo clippy --workspace -- -D warnings` |
| Format clean | `cargo fmt --all -- --check` |
| Cross-platform | Verify on Linux and macOS |

## Performance Rules

- Minimize the number of rustflags — each adds compilation overhead
- Prefer `target-cpu=native` over specific instruction set flags
- Test build time impact of configuration changes

## Review Checklist

Before submitting a `.cargo/` configuration PR:

- [ ] Setting is necessary and not redundant
- [ ] Comment explains the purpose
- [ ] Does not introduce security risks
- [ ] Tested on at least one platform
- [ ] No hardcoded paths or credentials
- [ ] Compatible with CI environment

## Pull Request Requirements

| Requirement | Description |
|-------------|-------------|
| Descriptive title | Clearly state what configuration changed and why |
| Summary | Explain motivation and impact on build behavior |
| Platform notes | Specify which platforms are affected |
| CI verification | All CI checks must pass |

## References

- [CONTRIBUTING.md](../CONTRIBUTING.md) — Repository root contribution rules
- [Cargo Configuration Reference](https://doc.rust-lang.org/cargo/reference/config.html)
- `AGENTS.md` — Engineering constitution
- `.cargo/config.toml` — Current configuration
