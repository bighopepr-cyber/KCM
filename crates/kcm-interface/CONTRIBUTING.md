# Contributing to kcm-interface

> For general contribution guidelines, see the root `CONTRIBUTING.md`.

## Overview

kcm-interface is the external-facing boundary of the KCM system. Changes here affect all consumers — C FFI callers, Python users, REST API clients, and KQL users. All contributions must be reviewed with extra scrutiny.

## Before Contributing

1. Read the root `CONTRIBUTING.md` and `AGENTS.md`.
2. Understand the crate's dependency chain: kcm-core → kcm-storage → kcm-runtime → kcm-security → **kcm-interface**.
3. Check the SSOT documents (`docs/PRD2.md §19`, `docs/PRD.md §7`) for the specification governing your change.
4. Open an issue or discussion before implementing non-trivial changes.

## Coding Standards

### FFI Safety Rules

- Every FFI function must have a `# Safety` doc comment.
- Every FFI function must validate all pointer arguments for null.
- Every FFI function must return `KCM_Error` on failure — never panic.
- `#[repr(C)]` types (`KCM_Fact`, `KCM_Error`) must not use Rust-specific layouts.
- No Rust `String` or `Vec` crosses the FFI boundary; use C-compatible types only.
- All `unsafe` blocks must have a comment explaining why `unsafe` is necessary.

### REST API Conventions

- All handlers return `Result<HttpResponse, KcmError>` or equivalent.
- Request bodies are validated against schema before processing.
- Authentication middleware protects all non-public endpoints.
- Error responses use a consistent JSON structure.
- Endpoints are versioned via path prefix (`/api/v1/...`).

## Module Architecture Rules

| Dependency | Allowed? | Purpose |
|------------|----------|---------|
| kcm-core | Yes | Core types (Fact, RowID, KcmError) |
| kcm-storage | Yes | Storage engine access |
| kcm-runtime | Yes | KnowledgeDatabase, transactions |
| kcm-security | Yes | RBAC, encryption, audit |
| External crates | Per `AGENTS.md` | Must justify existence |

kcm-interface must **never** depend on kcm-compute, kcm-reasoning, kcm-optimizer, kcm-distributed, or kcm-ml directly.

## Documentation Rules

- Every public FFI function has `# Safety`, `# Arguments`, and `# Returns` sections.
- Every REST endpoint has OpenAPI annotations via `openapi.rs`.
- Every middleware module has a module-level doc comment explaining its purpose.
- KQL grammar changes are documented in `kql_parser.rs` with examples.

## Testing Requirements

| Test Type | Scope | Location |
|-----------|-------|----------|
| FFI tests | Each of the 18 FFI functions | `tests/ffi/` |
| REST endpoint tests | Each REST handler | `tests/rest/` |
| KQL parser tests | Grammar rules, edge cases | `tests/kql/` |
| Middleware tests | Auth, CORS, rate limit, logging, request ID | `tests/middleware/` |
| Integration tests | Full lifecycle (DB create → insert → query → save) | `tests/integration/` |

- All tests must pass: `cargo test --workspace`
- FFI tests must cover null-pointer, out-of-memory, and concurrent access scenarios.
- KQL parser tests must include malformed input rejection.

## Performance Rules

- FFI call overhead must remain below 1μs for simple operations.
- REST response latency must remain within the benchmarks defined in `docs/PRD-TESTING& BRACHMARCK.md`.
- No allocations in hot FFI paths where avoidable.
- Middleware must not add more than 100μs of latency per request.

## Review Checklist

Before submitting a PR, verify:

- [ ] `cargo build --workspace` passes
- [ ] `cargo test --workspace` passes
- [ ] `cargo clippy --workspace -- -D warnings` clean
- [ ] `cargo fmt --all -- --check` clean
- [ ] All FFI functions have `# Safety` docs
- [ ] All new REST endpoints have OpenAPI specs
- [ ] All new code has tests
- [ ] No `unwrap()` in production code
- [ ] No `todo!()`, `unimplemented!()`, `FIXME`, or `TODO` markers
- [ ] SSOT documents updated if behavior changed
- [ ] No dependency additions without justification

## Pull Request Requirements

1. PR title describes the change concisely.
2. PR description references the SSOT requirement addressed.
3. All CI checks pass.
4. At least one reviewer with FFI or security expertise approves.
5. SSOT validation script passes (`bash scripts/validate-ssot.sh`).
6. Breaking changes to FFI or REST API require a version bump and migration notes.

## References

- Root `CONTRIBUTING.md`
- `AGENTS.md` — Non-negotiable rules and engineering gates
- `docs/PRD2.md §19` — Interface specification (SSOT)
- `docs/PRD.md §7` — FFI function definitions (SSOT)
- `docs/SSOT.md` — Single source of truth
