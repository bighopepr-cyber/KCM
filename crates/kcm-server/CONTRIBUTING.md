# Contributing to kcm-server

> For project-wide contribution guidelines, see the root `CONTRIBUTING.md`.

## Overview

kcm-server is the HTTP and gRPC server binary for the KCM knowledge engine. Contributions to this crate must maintain the reliability, security, and performance of the network-facing services. This document defines the coding standards, architecture rules, testing requirements, and review process specific to kcm-server.

## Before Contributing

1. Read `AGENTS.md` for the engineering constitution and non-negotiable rules
2. Read the root `CONTRIBUTING.md` for project-wide standards
3. Read `docs/PRD2.md §19` for the interface layer specification
4. Read `docs/PRD3.md §28` for the server specification
5. Check existing issues and PRs to avoid duplicate work
6. Open an issue for non-trivial changes before implementing

## Coding Standards

- All Rust code follows `rustfmt` defaults (enforced via `cargo fmt`)
- No `unwrap()` in production code paths
- No `panic!()` in production code
- No `TODO`, `FIXME`, or `HACK` markers in production code
- All public functions return `Result<T, KcmError>`
- Use `?` operator for error propagation; never ignore errors
- Prefer `log` macros over `println!` or `eprintln!`
- Follow existing module organization and naming conventions

## Module Architecture Rules

kcm-server depends on a strict subset of KCM crates:

| Dependency | Purpose |
|-----------|---------|
| kcm-core | Core types (Fact, KcmError, RowID) |
| kcm-runtime | KnowledgeDatabase, transactions, metrics |
| kcm-interface | REST handlers, gRPC handlers, middleware, FFI bridge |
| kcm-security | RBAC, encryption, audit log |

Rules:

- kcm-server must not import or use any crate not listed above
- kcm-server must not implement business logic directly; delegate to kcm-interface
- kcm-server owns only server configuration, startup, shutdown, and TLS setup
- All handler logic lives in `kcm-interface`; kcm-server wires handlers into the server framework
- Security middleware is configured in kcm-server but implemented in kcm-interface/kcm-security

## Documentation Rules

- Every public module must have a module-level doc comment
- Every public struct and function must have doc comments
- Doc comments must describe behavior, not implementation details
- Examples in doc comments must compile (`cargo test --doc`)
- Security-relevant behavior must be documented in `SECURITY.md`

## Testing Requirements

### Endpoint Tests

Every REST endpoint must have at least one integration test that:

- Sends a valid request and asserts the expected response
- Tests authentication (valid token, invalid token, missing token)
- Tests error cases (malformed input, missing parameters)
- Runs against a real actix-web test server (not mocked)

### gRPC Tests

Every gRPC RPC must have at least one integration test that:

- Sends a valid request via tonic client and asserts the expected response
- Tests authentication and authorization
- Tests error cases
- Runs against a real tonic test server

### Integration Tests

- Full request lifecycle tests (HTTP → interface → runtime → storage)
- Graceful shutdown tests
- TLS configuration tests
- Rate limiting tests

All tests must pass before submitting a PR:

```bash
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check
```

## Performance Rules

- P99 latency for REST endpoints must remain below 100ms
- P99 latency for gRPC RPCs must remain below 50ms
- No allocations in hot paths (request handling, response serialization)
- Connection pooling must be configured appropriately
- Benchmark regression detection must be run before merging performance-sensitive changes

## Review Checklist

Before requesting review, verify:

- [ ] All tests pass
- [ ] No clippy warnings
- [ ] Code passes fmt check
- [ ] No unwrap in production code
- [ ] No TODO/FIXME/HACK markers
- [ ] Doc comments present on public APIs
- [ ] Error handling uses `Result<T, KcmError>`
- [ ] No new external dependencies added (or justified in PR description)
- [ ] Security implications assessed
- [ ] Performance impact assessed

## Pull Request Requirements

- PR title follows conventional format: `fix:`, `feat:`, `refactor:`, `docs:`, `test:`
- PR description includes: what changed, why it changed, how to test it
- PR links to relevant issue(s)
- PR includes test coverage for new functionality
- PR does not introduce placeholder or stub implementations
- PR passes all CI checks before merge

## References

- Root `CONTRIBUTING.md` — project-wide contribution guidelines
- `AGENTS.md` — engineering constitution and non-negotiable rules
- `docs/PRD2.md §19` — interface layer specification
- `docs/PRD3.md §28` — server specification
- `docs/PRD-TESTING& BRACHMARCK.md` — testing strategy and benchmarks
