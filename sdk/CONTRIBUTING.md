# Contributing to KCM SDKs

This guide covers contributing to KCM language-specific SDKs. For core engine contributions, see the root [CONTRIBUTING.md](../CONTRIBUTING.md).

## SDK Structure Standards

Every SDK must follow this directory layout:

```
sdk/<language>/
├── src/                # Source code
│   └── <entry-point>   # Main module/namespace
├── tests/              # Test suite
│   ├── unit/           # Unit tests
│   └── integration/    # Integration tests (optional)
├── examples/           # Runnable examples
│   └── basic.rs|py|js  # At minimum: basic usage example
├── README.md           # SDK documentation
└── <build-file>        # Language-specific build configuration
```

### Required Files Per SDK

| File | Rust | Python | JavaScript | TypeScript | Go | Java | .NET | C | C++ |
|------|------|--------|------------|------------|-----|------|------|---|-----|
| Build config | Cargo.toml | pyproject.toml | package.json | package.json | go.mod | pom.xml | *.csproj | Makefile | CMakeLists.txt |
| Entry point | src/lib.rs | src/__init__.py | src/index.js | src/index.ts | kcm.go | src/main/java/... | src/*.cs | src/kcm.c | src/kcm.cpp |
| Tests | tests/ | tests/ | tests/ | tests/ | *_test.go | src/test/... | tests/ | tests/ | tests/ |
| Examples | examples/ | examples/ | examples/ | examples/ | examples/ | examples/ | examples/ | examples/ | examples/ |
| README | README.md | README.md | README.md | README.md | README.md | README.md | README.md | README.md | README.md |

## API Surface Rules

### Core API (All SDKs)

Every SDK must implement these operations with identical semantics:

```python
# Database lifecycle
db = Database(path)          # Open or create
db.close()                   # Close database

# CRUD operations
row_id = db.insert(fact)     # Insert a fact
results = db.query(kql)      # Query facts (KQL)
db.update(fact)              # Update a fact
db.delete(row_id)            # Delete by row ID

# Statistics
count = db.fact_count()      # Total facts
count = db.active_count()    # Active facts

# Transactions
txn = db.begin_transaction() # Begin transaction
db.commit(txn)               # Commit
db.rollback(txn)             # Rollback

# Persistence
db.save(path)                # Save to file
db = Database.load(path)     # Load from file

# Integrity
db.verify()                  # Verify database integrity
```

### Naming Conventions

| Language | Convention | Example |
|----------|-----------|---------|
| Rust | snake_case | `db.insert_fact(...)` |
| Python | snake_case | `db.insert_fact(...)` |
| JavaScript | camelCase | `db.insertFact(...)` |
| TypeScript | camelCase | `db.insertFact(...)` |
| Go | PascalCase | `db.InsertFact(...)` |
| Java | camelCase | `db.insertFact(...)` |
| .NET | PascalCase | `db.InsertFact(...)` |
| C | KCM_PascalCase | `KCM_DatabaseInsertFact(...)` |
| C++ | PascalCase | `db.InsertFact(...)` |

### Error Handling

| Language | Error Model |
|----------|------------|
| Rust | `Result<T, KcmError>` |
| Python | `raise KcmError(...)` |
| JavaScript | `throw new KcmError(...)` |
| TypeScript | `throw new KcmError(...)` |
| Go | `error` return value |
| Java | `throws KcmException` |
| .NET | `throws KcmException` |
| C | `KCM_Result` enum return + out param |
| C++ | `throw KcmError(...)` or `std::expected` |

## Testing Requirements

### Test Coverage

- Minimum 80% line coverage for all SDKs
- 100% coverage for core API operations (insert, query, delete, update)
- All error paths must be tested
- All edge cases must be tested (empty database, null inputs, etc.)

### Test Categories

| Category | Purpose | Required |
|----------|---------|----------|
| Unit | Test individual functions | Yes |
| Integration | Test cross-module interactions | Yes |
| FFI | Test core engine communication | Yes (non-Rust) |
| Performance | Benchmark critical paths | Recommended |

### Test Naming

| Language | Convention | Example |
|----------|-----------|---------|
| Rust | `test_<function>_<scenario>` | `test_insert_fact_valid_input` |
| Python | `test_<function>_<scenario>` | `test_insert_fact_valid_input` |
| JavaScript | `test('<description>')` | `test('inserts a fact with valid input')` |
| Go | `Test_<Function>_<Scenario>` | `Test_InsertFact_ValidInput` |
| Java | `test<Method><Scenario>()` | `testInsertFactValidInput()` |
| .NET | `<Method>_<Scenario>()` | `InsertFact_ValidInput()` |
| C | `test_<function>_<scenario>` | `test_insert_fact_valid_input` |
| C++ | `TEST(Suite, <description>)` | `TEST(Database, InsertFactValidInput)` |

### Running Tests

```bash
# Rust
cargo test --workspace

# Python
pytest tests/ -v

# JavaScript
npm test

# TypeScript
npm test

# Go
go test -v ./...

# Java
mvn test

# .NET
dotnet test

# C
make test

# C++
cmake --build build && ctest --test-dir build
```

## Example Requirements

### Minimum Examples

Every SDK must include at least these examples:

| Example | Description | Required |
|---------|-------------|----------|
| basic | Create DB, insert, query, close | Yes |
| transactions | Begin, insert, commit, rollback | Yes |
| query | KQL query examples | Yes |
| persistence | Save and load database | Yes |

### Example Standards

1. **Self-contained**: Each example must run independently
2. **Well-documented**: Each example has comments explaining each step
3. **Error-free**: Each example must execute without errors
4. **Realistic**: Use realistic data, not placeholder values

### Example Template

```python
"""
KCM SDK - Basic Usage Example

This example demonstrates core database operations:
- Creating/opening a database
- Inserting facts
- Querying facts
- Closing the database
"""
import kcm

def main():
    # Create or open a database
    db = kcm.Database("example.db")

    # Insert a fact
    row_id = db.insert({
        "subject": "planet",
        "predicate": "orbits",
        "object": "sun",
        "confidence": 0.99,
    })
    print(f"Inserted fact with row_id: {row_id}")

    # Query facts
    results = db.query("SELECT * FROM facts WHERE subject = 'planet'")
    for fact in results:
        print(f"Found: {fact}")

    # Get statistics
    print(f"Total facts: {db.fact_count()}")
    print(f"Active facts: {db.active_count()}")

    # Close the database
    db.close()

if __name__ == "__main__":
    main()
```

## Documentation Requirements

### README.md

Every SDK README must include:

1. **Installation**: How to install the SDK
2. **Quick Start**: Minimal working example
3. **API Reference**: Link to or inline API documentation
4. **Examples**: List of available examples
5. **Configuration**: SDK-specific configuration options
6. **Error Handling**: How errors are reported
7. **Contributing**: Link to this document
8. **License**: MIT license reference

### API Documentation

| Language | Documentation Style |
|----------|-------------------|
| Rust | `///` doc comments + `#[doc]` attributes |
| Python | Google-style docstrings |
| JavaScript | JSDoc comments |
| TypeScript | TSDoc comments |
| Go | Go doc comments |
| Java | Javadoc comments |
| .NET | XML doc comments |
| C | `///` doc comments (Doxygen-compatible) |
| C++ | `///` doc comments (Doxygen-compatible) |

### Docstring Template (Python)

```python
def insert_fact(self, fact: dict) -> int:
    """Insert a knowledge fact into the database.

    Args:
        fact: A dictionary containing fact fields:
            - subject (str): The subject of the fact
            - predicate (str): The predicate/relationship
            - object (str): The object of the fact
            - confidence (float): Confidence score (0.0-1.0)

    Returns:
        The row ID of the inserted fact.

    Raises:
        KcmError: If the fact is invalid or the database is closed.

    Example:
        >>> db = kcm.Database("test.db")
        >>> row_id = db.insert_fact({
        ...     "subject": "planet",
        ...     "predicate": "orbits",
        ...     "object": "sun",
        ...     "confidence": 0.99,
        ... })
        >>> print(row_id)
        1
    """
```

## SSOT Compliance

### Requirement Traceability

Every SDK change must trace to an SSOT requirement:

```
SSOT Requirement → SDK Implementation → SDK Tests → SDK Examples
```

### SSOT Documents

| Document | SDK Relevance |
|----------|--------------|
| PRD.md | Core types, API semantics |
| PRD2.md | Storage format, persistence |
| PRD3.md | Distributed, security |
| sdk/README.md | Cross-SDK API surface |

### Validation

Run the SDK API validation script to check compliance:

```bash
bash scripts/validate-sdk-api.sh
```

This checks:
- All SDKs have required files
- All SDKs have required API surface
- All SDKs have tests
- All SDKs have examples
- All SDKs have documentation

## Pull Request Checklist

Before submitting an SDK PR:

- [ ] Code follows language-specific style guide
- [ ] All tests pass
- [ ] New APIs have tests
- [ ] New APIs have examples
- [ ] New APIs have documentation
- [ ] API matches cross-SDK surface
- [ ] README is updated
- [ ] No placeholder implementations
- [ ] Error handling is complete
- [ ] SSOT traceability is documented

## References

- [CONTRIBUTING.md](../CONTRIBUTING.md) — Repository-wide contribution guidelines
- [SECURITY.md](SECURITY.md) — SDK security policy
- [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) — SDK community guidelines
- [docs/sdk/compatibility.md](../docs/sdk/compatibility.md) — Compatibility matrix
- [SSOT.md](../SSOT.md) — Single Source of Truth
- [AGENTS.md](../AGENTS.md) — Engineering constitution
