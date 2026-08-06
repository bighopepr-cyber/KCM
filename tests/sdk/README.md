# KCM SDK Testing

Cross-language consistency tests, mock server, and API validation for ensuring all KCM SDKs implement identical behavior against the KCM API surface.

## Purpose

This directory ensures that all SDK implementations (Python, JavaScript, and future languages) produce identical results for identical operations. It validates API surface compliance, behavioral consistency, and cross-language parity against the authoritative SSOT specifications.

The SDK tests are the primary mechanism for detecting behavioral drift between SDK implementations. Any change to the KCM API surface must be validated across all SDKs before release.

## Cross-Language Consistency Testing

The cross-language consistency test suite (`cross_language_test.py`) runs identical test sequences against each SDK implementation and compares results for behavioral parity.

### Test Sequence

Each SDK executes the same operations in the same order:

1. Insert facts with known values
2. Query and verify returned data
3. Update facts and verify changes
4. Delete facts and verify removal
5. Transaction commit/rollback semantics
6. Save/load database persistence

Results are recorded in `consistency_matrix.json` and compared across all SDKs. Any discrepancy indicates a behavioral drift that must be resolved before release.

### SDK Selection

Set the `KCM_SDK` environment variable to target a specific SDK:

```bash
KCM_SDK=python python tests/sdk/cross_language_test.py
KCM_SDK=javascript python tests/sdk/cross_language_test.py
```

If unset, all available SDKs are tested.

## API Validation

`validate_sdk_api.py` reads each SDK's source code and validates compliance against SSOT specifications:

- All 18 FFI functions are exposed (`KCM_API_SPEC.md` §2.2)
- All 10 Fact fields are present (`KCM_API_SPEC.md` §2.1)
- Error codes match the SSOT enum (`KCM_API_SPEC.md` §2.1)
- Required classes and methods exist per SDK

API validation runs independently of the mock server and does not require a running KCM instance.

## Mock Server

The mock server (`mock_server.py`) implements the KCM REST API from `KCM_API_SPEC.md` §3 with in-memory storage. It provides a lightweight, isolated environment for SDK integration testing without requiring the real KCM engine.

### Endpoints

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

### Usage

```bash
# Start mock server (default port 8080, localhost only)
python tests/sdk/mock_server.py

# Custom port
python tests/sdk/mock_server.py --port 9090
```

The mock server binds to `127.0.0.1` only. It does not implement authentication.

## Test Workflow

```
1. Start mock server (background, localhost:8080)
2. Wait for /health endpoint to respond
3. Execute cross-language consistency tests for each SDK
4. Execute API validation for each SDK
5. Update consistency_matrix.json with results
6. Stop mock server
7. Report aggregated results
```

## Running Tests

### Prerequisites

```bash
pip install flask requests
```

### Run all SDK tests

```bash
bash tests/sdk/run_all_tests.sh
```

### Run individual test suites

```bash
# Mock server only
python tests/sdk/mock_server.py &

# Cross-language consistency tests
python tests/sdk/cross_language_test.py

# API compliance validation
python tests/sdk/validate_sdk_api.py
```

## Adding New Tests

1. Define the test case in `consistency_matrix.json` with a unique ID and category
2. Implement the test logic in `cross_language_test.py` (or a new test script)
3. Ensure the test runs identically across all SDK implementations
4. Update `README.md` if adding new infrastructure or changing workflow
5. Validate that the test passes in CI before submitting PR

### Test Case Categories

| Category | Description |
|----------|-------------|
| `crud` | Basic CRUD operations |
| `transaction` | Transaction commit/rollback semantics |
| `persistence` | Save/load database persistence |
| `error` | Error handling and edge cases |
| `security` | Authentication and authorization |

## Relationship with KCM Components

This directory validates the API surface defined by several KCM components:

| Component | Relationship |
|-----------|-------------|
| `kcm-interface` | Provides the C FFI (18 functions) and REST API (8 endpoints) that SDKs wrap |
| `kcm-interface` (C FFI) | Defines the contract that all SDK bindings must implement |
| `kcm-interface` (REST) | Defines the HTTP API that the mock server implements |
| `kcm-interface` (PyO3) | Defines the Python bindings that the Python SDK wraps |
| `kcm-core` | Provides core types (Fact, RowID, SubjectID, Confidence) that SDKs expose |
| `kcm-runtime` | Provides the engine (KnowledgeDatabase) that SDKs interact with |

### SDK-to-Engine Flow

```
SDK Implementation
    ↓ (wraps)
kcm-interface (FFI / REST / PyO3)
    ↓ (calls)
kcm-runtime (KnowledgeDatabase)
    ↓ (uses)
kcm-core (Types, Storage, Compute)
```

The SDK tests validate that the top layer (SDK) produces identical behavior regardless of implementation language, by testing against the mock server which implements the `kcm-interface` API surface.

## Quality Gates

| Gate | Threshold | Enforcement |
|------|-----------|-------------|
| Test pass rate | 100% | CI blocks merge |
| Cross-language consistency | All SDKs identical | CI blocks merge |
| API surface coverage | 100% of SSOT | CI blocks merge |
| Mock server availability | Starts cleanly | Pre-test check |

## Statement of Behavioral Parity

This folder ensures all SDKs have identical behavior against the KCM API. Any discrepancy between SDK implementations is treated as a defect and must be resolved before release. The consistency matrix provides an auditable record of behavioral parity across all SDK versions.

## SSOT References

- `docs/specs/KCM_API_SPEC.md` — API contracts (FFI, REST, Python bindings)
- `sdk/README.md` — SDK API reference
- `AGENTS.md` — Engineering constitution
- `docs/PRD-TESTING& BRACHMARCK.md` — Testing strategy and quality gates
