# Contributing to scripts/

> For core engine contribution rules, refer to the repository [CONTRIBUTING.md](../CONTRIBUTING.md).

## Overview

This guide describes how to contribute to the `scripts/` directory, which contains build automation, validation utilities, and CLI tools. Contributions must follow the coding standards, architecture rules, and review processes defined here.

## Before Contributing

1. Read the repository [CONTRIBUTING.md](../CONTRIBUTING.md) for general contribution rules.
2. Read [scripts/SECURITY.md](SECURITY.md) for security requirements.
3. Identify whether your change is a **script modification** or a **new CLI tool**.
4. Ensure your change addresses an existing issue or has been discussed with maintainers.
5. Review existing code in `scripts/` to understand conventions.

## Coding Standards

### Shell Scripts

- All bash scripts must pass `shellcheck` with zero warnings.
- Use `set -euo pipefail` at the top of every bash script.
- Double-quote all variable expansions.
- Use `[[ ]]` instead of `[ ]` for conditional tests.
- Prefer `readonly` for variables that should not be reassigned.
- Use `local` for function-scoped variables.

### Python Scripts

- All Python scripts must pass `pylint` with no errors.
- Follow PEP 8 style conventions.
- Use type hints for function signatures.
- Handle exceptions explicitly; do not use bare `except:`.

### Consistent Error Handling

- All scripts must exit with meaningful exit codes (`0` for success, non-zero for failure).
- Error messages must be written to `stderr`.
- Scripts must clean up temporary resources on failure (trap handlers in bash, context managers in Python).

## Module Architecture Rules

| Category | Rules |
|---|---|
| Shell/Python scripts | Standalone — must not depend on other scripts in the directory. Each script must be independently executable. |
| `kcm-cli/` tools | Workspace members — each CLI tool is a Rust crate under `kcm-cli/`. Must follow Rust workspace conventions and dependency policies defined in `AGENTS.md`. |
| Shared utilities | If multiple scripts need the same logic, extract it into a shared module within `kcm-cli/` rather than duplicating shell functions. |

## Documentation Rules

- Every script must include a usage section at the top (shell: comment block; Python: docstring).
- Every script must accept `--help` or `-h` to display usage information.
- CLI tools must provide `--help` output for every subcommand.
- Complex algorithms or non-obvious logic must include inline comments explaining the rationale.

## Testing Requirements

- Shell scripts must include test cases or be testable via the validation framework.
- Python scripts must include unit tests where applicable.
- CLI tools must have corresponding test cases in `crates/kcm-testing/` or local `tests/` directories.
- All changes must be validated with the existing test suite before submission.

## Performance Rules

- Scripts must complete in reasonable time for CI pipeline use (under 60 seconds for validation scripts).
- CLI tools must not introduce performance regressions — verify with `cargo bench` if the change affects hot paths.
- Avoid unnecessary file I/O or expensive operations in validation scripts.

## Review Checklist

Before submitting a pull request, verify:

- [ ] Code passes `shellcheck` (bash) or `pylint` (Python).
- [ ] Code passes `cargo clippy --workspace -- -D warnings` (Rust CLI tools).
- [ ] Code passes `cargo fmt --all -- --check` (Rust CLI tools).
- [ ] All tests pass.
- [ ] Scripts have usage documentation.
- [ ] No hardcoded paths, secrets, or credentials.
- [ ] Error handling is complete and meaningful.
- [ ] Changes do not break existing script invocations.

## Pull Request Requirements

- PR title must clearly describe the change (e.g., `scripts: add path validation to validate-ssot.sh`).
- PR description must explain what the change does, why it is needed, and how it was tested.
- PR must reference the related issue or specification.
- PR must not introduce new dependencies without justification per `AGENTS.md` Dependency Policy.
- PR must pass all CI checks before merge.

## References

- [Repository CONTRIBUTING.md](../CONTRIBUTING.md)
- [scripts/SECURITY.md](SECURITY.md)
- [AGENTS.md — Dependency Policy](../AGENTS.md)
- [scripts/README.md](README.md)
