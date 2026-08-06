# {{CRATE_NAME}}

{{ONE_LINE_DESCRIPTION}}

## Overview

{{DETAILED_OVERVIEW}}

## Purpose

{{PURPOSE}}

## Responsibilities

| Responsibility | Description |
|---------------|-------------|
| {{RESP_1}} | {{RESP_1_DESC}} |
| {{RESP_2}} | {{RESP_2_DESC}} |

## Folder Structure

```
{{CRATE_DIR}}/
├── src/
│   ├── lib.rs
│   └── {{MODULES}}
├── tests/
└── Cargo.toml
```

## Public API

{{PUBLIC_API_DESCRIPTION}}

| Type/Function | Description |
|--------------|-------------|
| {{API_1}} | {{API_1_DESC}} |

## Internal Components

{{INTERNAL_COMPONENTS}}

## Dependencies

| Dependency | Justification |
|-----------|---------------|
| {{DEP_1}} | {{DEP_1_JUSTIFICATION}} |

## Integration

{{INTEGRATION_DESCRIPTION}}

## Build

```bash
cargo build -p {{CRATE_NAME}}
```

## Test

```bash
cargo test -p {{CRATE_NAME}}
```

## Examples

```rust
{{CODE_EXAMPLE}}
```

## References

- [SSOT.md](../../SSOT.md)
- [AGENTS.md](../../AGENTS.md)
- [CONTRIBUTING.md](../../CONTRIBUTING.md)
