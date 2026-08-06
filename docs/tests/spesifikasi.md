# Tests Technical Specification

## Overview

This document specifies the technical design and implementation of the KCM test infrastructure, including integration tests, SDK cross-language consistency tests, mock server, and validation tools. The test infrastructure ensures all KCM SDKs implement identical behavior against the KCM API surface.

## Scope

The test infrastructure covers:

- Integration testing of the full KCM workspace
- SDK cross-language consistency testing
- API surface validation against SSOT specifications
- Mock server for isolated SDK testing
- Test orchestration and reporting

## Responsibilities

### Integration Testing

- Validates cross-crate interactions within the workspace
- Exercises build system and workspace configuration
- Verifies end-to-end functionality of the KCM engine

### SDK Cross-Language Testing

- Validates that all SDKs produce identical results for identical operations
- Ensures API surface compliance against SSOT specifications
- Detects behavioral drift between SDK implementations
- Provides regression detection for API changes

## Technical Specification

### integration_test.sh

The integration test script orchestrates workspace-level tests:

- Executes `cargo build --workspace` and validates build success
- Runs `cargo test --workspace` and validates all tests pass
- Runs `cargo clippy --workspace -- -D warnings` and validates zero warnings
- Runs `cargo fmt --all -- --check` and validates formatting
- Produces machine-readable results for CI integration

**Execution Flow:**

1. Validate prerequisites (Rust toolchain, cargo)
2. Build workspace in release mode
3. Execute unit and integration tests
4. Run clippy lints
5. Verify code formatting
6. Report results

### SDK Testing

The SDK test infrastructure consists of four components:

#### Mock Server (`mock_server.py`)

A Flask-based REST API mock server implementing the KCM API surface from `KCM_API_SPEC.md` §3 with in-memory storage. Used for SDK integration testing without requiring the real KCM engine.

**Endpoints:**

| Method | Path | Description |
|--------|------|-------------|
| GET | `/health` | Health check |
| POST | `/facts` | Insert fact |
| GET | `/facts` | List/query facts |
| GET | `/facts/{id}` | Get fact by ID |
| PUT | `/facts/{id}` | Update fact |
| DELETE | `/facts/{id}` | Delete fact |
| GET | `/stats` | Metrics |
| GET | `/metrics` | Prometheus format |

**Configuration:**

- Default port: 8080
- Bind address: 127.0.0.1 (localhost only)
- Storage: In-memory (non-persistent)

#### Cross-Language Consistency Tests (`cross_language_test.py`)

Runs identical test sequences across all SDK implementations and compares results for behavioral consistency.

**Test Sequence:**

1. Insert facts with known values
2. Query and verify returned data
3. Update facts and verify changes
4. Delete facts and verify removal
5. Transaction commit/rollback semantics
6. Save/load database persistence

**SDK Selection:**

Set `KCM_SDK` environment variable to target specific SDK:
- `python` — Python SDK
- `javascript` — JavaScript SDK
- All SDKs tested if unset

#### Consistency Matrix (`consistency_matrix.json`)

JSON-based test case registry tracking test definitions and per-SDK pass/fail status.

**Structure:**

```json
{
  "version": "1.0",
  "test_cases": [
    {
      "id": "TC-001",
      "name": "Insert and retrieve fact",
      "category": "crud",
      "sdk_results": {
        "python": "pass",
        "javascript": "pass"
      }
    }
  ]
}
```

**Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `version` | string | Matrix schema version |
| `test_cases` | array | List of test case definitions |
| `test_cases[].id` | string | Unique test identifier |
| `test_cases[].name` | string | Human-readable test description |
| `test_cases[].category` | string | Test category (crud, transaction, persistence) |
| `test_cases[].sdk_results` | object | Per-SDK pass/fail/fail reason |

#### API Validation (`validate_sdk_api.py`)

Reads each SDK's source code and validates compliance against SSOT specifications.

**Validations:**

- All 18 FFI functions are exposed (§2.2)
- All 10 Fact fields are present (§2.1)
- Error codes match the SSOT enum (§2.1)
- Required classes and methods exist per SDK

### Test Orchestration (`run_all_tests.sh`)

Shell script that sequences all test components:

1. Start mock server (background)
2. Wait for mock server readiness
3. Execute cross-language consistency tests
4. Execute API validation
5. Stop mock server
6. Aggregate results

## Architecture

The test infrastructure follows a layered architecture:

```
┌─────────────────────────────────────┐
│         CI Pipeline                 │
│   (GitHub Actions / Local)         │
├─────────────────────────────────────┤
│      Test Orchestration            │
│   run_all_tests.sh                 │
├──────────┬──────────────────────────┤
│          │                          │
│  ┌───────▼────────┐  ┌─────────────▼──────┐
│  │ Integration    │  │   SDK Testing      │
│  │ Tests          │  │                    │
│  │ (shell)        │  │ ┌────────────────┐ │
│  │                │  │ │ Mock Server    │ │
│  │ cargo build    │  │ │ (Flask)        │ │
│  │ cargo test     │  │ └────────────────┘ │
│  │ cargo clippy   │  │ ┌────────────────┐ │
│  │ cargo fmt      │  │ │ Cross-Language │ │
│  │                │  │ │ Tests          │ │
│  │                │  │ └────────────────┘ │
│  │                │  │ ┌────────────────┐ │
│  │                │  │ │ API Validation │ │
│  │                │  │ └────────────────┘ │
│  └────────────────┘  └────────────────────┘
├─────────────────────────────────────┤
│      KCM Engine                     │
│   (workspace under test)            │
└─────────────────────────────────────┘
```

## Internal Components

| Component | Language | Purpose |
|-----------|----------|---------|
| `integration_test.sh` | Bash | Workspace build and test orchestration |
| `mock_server.py` | Python (Flask) | REST API mock for SDK testing |
| `cross_language_test.py` | Python | Cross-SDK behavioral consistency |
| `consistency_matrix.json` | JSON | Test case registry and results |
| `validate_sdk_api.py` | Python | API surface compliance validation |
| `run_all_tests.sh` | Bash | Test suite orchestration |

## Data Model

### consistency_matrix.json

The consistency matrix is the authoritative record of test case definitions and results.

**Schema:**

- Top-level `version` field for schema evolution
- `test_cases` array with ordered test definitions
- Each test case has a unique `id`, descriptive `name`, and `category`
- `sdk_results` object tracks per-SDK outcomes

**Test Categories:**

| Category | Description |
|----------|-------------|
| `crud` | Basic CRUD operations (insert, query, update, delete) |
| `transaction` | Transaction commit and rollback semantics |
| `persistence` | Save and load database persistence |
| `error` | Error handling and edge cases |
| `security` | Authentication and authorization |

## Execution Flow

### Integration Test Flow

```
1. Validate prerequisites
2. cargo build --workspace --release
3. cargo test --workspace
4. cargo clippy --workspace -- -D warnings
5. cargo fmt --all -- --check
6. Report results
```

### SDK Test Flow

```
1. Start mock server (background, port 8080)
2. Wait for /health endpoint to respond
3. For each SDK in test matrix:
   a. Set KCM_SDK environment variable
   b. Execute cross_language_test.py
   c. Record results in consistency_matrix.json
4. Execute validate_sdk_api.py for each SDK
5. Aggregate results
6. Stop mock server
7. Report results
```

## Public API

### Mock Server Endpoints

The mock server exposes the standard KCM REST API (§3 of `KCM_API_SPEC.md`). All endpoints accept and return JSON. The mock server does not implement authentication.

### Test Script Interface

| Script | Input | Output |
|--------|-------|--------|
| `integration_test.sh` | None (reads workspace) | Exit code + summary |
| `cross_language_test.py` | `KCM_SDK` env var | Test results + matrix update |
| `validate_sdk_api.py` | SDK source paths | Validation report |
| `run_all_tests.sh` | None | Aggregated results |

## Configuration

| Parameter | Default | Description |
|-----------|---------|-------------|
| `KCM_SDK` | (all) | Target SDK for cross-language tests |
| `KCM_MOCK_PORT` | 8080 | Mock server port |
| `KCM_TEST_TIMEOUT` | 300 | Test timeout in seconds |
| `KCM_TEST_VERBOSE` | false | Enable verbose test output |

## Dependencies

| Dependency | Version | Purpose |
|------------|---------|---------|
| Python 3 | ≥ 3.8 | Test script runtime |
| Flask | ≥ 2.0 | Mock server framework |
| requests | ≥ 2.25 | HTTP client for test assertions |
| Bash | ≥ 4.0 | Orchestration scripts |
| Rust/Cargo | (workspace) | Build and test execution |

## Error Handling

| Error | Handling | Exit Code |
|-------|----------|-----------|
| Mock server fails to start | Abort test suite | 1 |
| SDK test fails | Record in matrix, continue | 0 (aggregated) |
| API validation fails | Record failure, continue | 0 (aggregated) |
| Integration test fails | Abort, report failure | 1 |
| Timeout exceeded | Kill process, report timeout | 1 |

## Performance Characteristics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Mock server startup | < 2s | Time to `/health` 200 |
| Cross-language test suite | < 60s | Total execution time |
| API validation | < 10s | Per-SDK validation time |
| Integration tests | < 120s | Full workspace test suite |

## Security Considerations

- Mock server binds to localhost only (127.0.0.1)
- No authentication implemented (test infrastructure only)
- No production data in test fixtures
- Temporary files cleaned up after test execution
- No secrets hardcoded in test scripts
- See [tests/SECURITY.md](../../tests/SECURITY.md) for full security policy

## Integration

### CI Pipeline

The test infrastructure integrates with CI through:

- `run_all_tests.sh` as the primary entry point
- Exit codes for pass/fail signaling
- Machine-readable output for CI reporting
- `consistency_matrix.json` for historical tracking

### kcm-interface Integration

SDK tests validate against the API surface defined in `kcm-interface`:

- C FFI (18 functions per `KCM_API_SPEC.md` §2.2)
- REST endpoints (8 endpoints per `KCM_API_SPEC.md` §3)
- Python bindings (PyO3-based, per `KCM_API_SPEC.md` §4)

### Cross-SDK Consistency

All SDKs (Python, JavaScript, and future implementations) must:

- Expose identical API surfaces
- Produce identical results for identical operations
- Handle errors with consistent error codes
- Maintain behavioral parity across versions

## Sequence Diagram

```
┌──────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐
│ CI   │  │ run_all  │  │ mock_    │  │ cross_   │
│      │──│ _tests.sh│──│ server.py│  │ language │
└──────┘  └──────────┘  └──────────┘  └──────────┘
              │                │              │
              │  1. start      │              │
              │───────────────>│              │
              │                │              │
              │  2. /health    │              │
              │<───────────────│              │
              │                │              │
              │  3. run tests  │              │
              │──────────────────────────────>│
              │                │              │
              │                │  4. API calls│
              │                │<─────────────│
              │                │              │
              │  5. results    │              │
              │<──────────────────────────────│
              │                │              │
              │  6. validate   │              │
              │──────────────────────────────>│
              │                │              │
              │  7. stop       │              │
              │───────────────>│              │
              │                │              │
              │  8. report     │              │
              │<───────────────│              │
```

## Architecture Diagram

```
┌─────────────────────────────────────────────────────┐
│                    CI Pipeline                       │
│                                                     │
│  ┌───────────────────────────────────────────────┐  │
│  │           run_all_tests.sh                     │  │
│  │                                               │  │
│  │  ┌─────────────┐  ┌────────────────────────┐  │  │
│  │  │ integration │  │    SDK Testing          │  │  │
│  │  │ _test.sh    │  │                        │  │  │
│  │  │             │  │  ┌──────────────────┐  │  │  │
│  │  │ cargo build │  │  │  mock_server.py  │  │  │  │
│  │  │ cargo test  │  │  │  (Flask, :8080)  │  │  │  │
│  │  │ cargo fmt   │  │  └──────────────────┘  │  │  │
│  │  │ cargo clippy│  │  ┌──────────────────┐  │  │  │
│  │  │             │  │  │ cross_language   │  │  │  │
│  │  └─────────────┘  │  │ _test.py        │  │  │  │
│  │                    │  └──────────────────┘  │  │  │
│  │                    │  ┌──────────────────┐  │  │  │
│  │                    │  │ validate_sdk_api │  │  │  │
│  │                    │  │ .py              │  │  │  │
│  │                    │  └──────────────────┘  │  │  │
│  │                    │  ┌──────────────────┐  │  │  │
│  │                    │  │ consistency_     │  │  │  │
│  │                    │  │ matrix.json      │  │  │  │
│  │                    │  └──────────────────┘  │  │  │
│  │                    └────────────────────────┘  │  │
│  └───────────────────────────────────────────────┘  │
│                                                     │
│  ┌───────────────────────────────────────────────┐  │
│  │              KCM Workspace                    │  │
│  │  kcm-core │ kcm-storage │ kcm-compute │ ...  │  │
│  └───────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────┘
```

## References

- [PRD-TESTING & BENCHMARK](../PRD-TESTING&%20BRACHMARCK.md) — Testing strategy and quality gates
- [KCM API Specification](../specs/KCM_API_SPEC.md) — API contracts
- [SDK README](../../tests/sdk/README.md) — SDK test directory documentation
- [Engineering Constitution](../../AGENTS.md) — Project-wide engineering rules

## SSOT Alignment

This specification aligns with the following SSOT documents:

| SSOT Document | Section | Alignment |
|---------------|---------|-----------|
| PRD-TESTING | §1-8 | Test pyramid, quality gates, benchmark suite |
| KCM_API_SPEC | §2 | FFI functions, Fact fields, error codes |
| KCM_API_SPEC | §3 | REST API endpoints |
| KCM_API_SPEC | §4 | Python bindings |
| AGENTS.md | — | Engineering constitution, non-negotiable rules |
