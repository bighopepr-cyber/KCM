# KCM SDK Testing

Cross-language consistency tests and mock server for validating that all KCM SDKs implement the same API surface correctly.

## Test Pyramid

| Layer | Count | Speed | Purpose |
|-------|-------|-------|---------|
| Unit | 14 | < 1s | Single SDK function correctness |
| Integration | 8 | 1-5s | Mock server ↔ SDK interaction |
| Cross-Language | 6 | 5-30s | Identical behavior across SDKs |
| API Compliance | 2 | < 1s | API surface and FFI contract validation |

## Directory Structure

```
tests/sdk/
├── README.md                    # This file
├── mock_server.py               # REST API mock server (Flask)
├── cross_language_test.py       # Runs identical test suite across all SDKs
├── consistency_matrix.json      # Test case definitions and SDK pass/fail tracking
├── validate_sdk_api.py          # Validates SDK API surface against SSOT
├── run_all_tests.sh             # Orchestrator script
└── reports/                     # Generated test reports (gitignored)
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

### Run against a specific SDK

```bash
KCM_SDK=python python tests/sdk/cross_language_test.py
KCM_SDK=javascript python tests/sdk/cross_language_test.py
```

## Mock Server

The mock server (`mock_server.py`) implements the REST API from `KCM_API_SPEC.md` §3 with in-memory storage. It is used for SDK integration testing without requiring the real KCM engine.

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
# Start mock server (default port 8080)
python tests/sdk/mock_server.py

# Custom port
python tests/sdk/mock_server.py --port 9090
```

## Cross-Language Consistency Tests

Validates that all SDKs produce identical results for the same operations:

1. Insert facts with known values
2. Query and verify returned data
3. Update facts and verify changes
4. Delete facts and verify removal
5. Transaction commit/rollback semantics
6. Save/load database persistence

Each SDK runs the same test sequence against the mock server and results are compared for consistency.

## API Compliance Validation

`validate_sdk_api.py` reads each SDK's source code and validates:

- All 18 FFI functions are exposed (§2.2)
- All 10 Fact fields are present (§2.1)
- Error codes match the SSOT enum (§2.1)
- Required classes/methods exist per SDK

## Quality Gates

| Gate | Threshold | Enforcement |
|------|-----------|-------------|
| Test pass rate | 100% | CI blocks merge |
| Cross-language consistency | All SDKs identical | CI blocks merge |
| API surface coverage | 100% of SSOT | CI blocks merge |
| Mock server availability | Starts cleanly | Pre-test check |

## SSOT References

- `docs/specs/KCM_API_SPEC.md` — API contracts
- `sdk/README.md` — SDK API reference
- `AGENTS.md` — Engineering constitution
