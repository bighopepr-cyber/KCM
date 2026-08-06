# KCM C++ SDK

Official SDK for the KCM Knowledge Columnar Model

<!-- badges:start -->
[![Build Status](https://img.shields.io/badge/build-pending-lightgrey)]()
[![Version](https://img.shields.io/badge/version-1.0.0-blue)]()
[![License](https://img.shields.io/badge/license-MIT-green)]()
<!-- badges:end -->

The C++ SDK provides a modern C++17 RAII wrapper over the C FFI. All resources are automatically managed, and errors are reported via exceptions.

## Installation

### vcpkg

```bash
vcpkg install kcm
```

### Conan

```ini
# conanfile.txt
[requires]
kcm/1.0.0
```

```bash
conan install .
```

### Manual

```bash
cargo build --release -p kcm-interface
# Headers: sdk/c/kcm.h, sdk/cpp/kcm.hpp
# Library: target/release/libkcm.so (or kcm.lib on Windows)
```

Add `sdk/c/` and `sdk/cpp/` to your include path and link against `libkcm`.

## Quickstart

```cpp
#include "kcm.hpp"
#include <iostream>

int main() {
    try {
        kcm::Database db;

        kcm::Fact fact;
        fact.subject = 1;
        fact.predicate = 2;
        fact.object = 3;
        fact.confidence = 0.95;

        db.insert(fact);

        std::cout << "Total facts: " << db.fact_count() << std::endl;
        std::cout << "Active facts: " << db.active_count() << std::endl;
    } catch (const kcm::Error& e) {
        std::cerr << "KCM error: " << e.what() << std::endl;
        return 1;
    }
    return 0;
}
```

Compile:

```bash
g++ -std=c++17 -o kcm_example kcm_example.cpp -L/path/to/lib -lkcm
```

## API Reference

### `kcm::Error`

Exception type thrown by all KCM C++ operations.

```cpp
class Error : public std::runtime_error {
public:
    explicit Error(KCM_Error code, const std::string& msg = "");
    KCM_Error code() const;
};
```

| Method | Description |
|--------|-------------|
| `code()` | Returns the underlying `KCM_Error` code |
| `what()` | Returns human-readable error message (inherited from `std::runtime_error`) |

### `kcm::Fact`

Knowledge fact with 10 attributes.

```cpp
struct Fact {
    uint32_t subject = 0;
    uint8_t  predicate = 0;
    uint32_t object = 0;
    double   confidence = 0.0;
    uint8_t  evidence = 0;
    int64_t  timestamp = 0;
    uint8_t  context = 0;
    int32_t  version = 0;
    int8_t   priority = 0;
    uint16_t owner = 0;
};
```

All fields have sensible defaults. Set only the fields you need.

### `kcm::Database`

Main entry point for KCM operations. Manages database lifecycle via RAII.

```cpp
class Database {
public:
    Database();
    ~Database();

    Database(const Database&) = delete;
    Database& operator=(const Database&) = delete;
    Database(Database&& o) noexcept;

    void insert(const Fact& fact);
    void update(uint64_t row_id, const Fact& fact);
    void remove(uint64_t row_id);

    uint64_t fact_count() const;
    uint64_t active_count() const;

    Query query(const std::string& kql);
    Transaction begin_transaction();

    void save(const std::string& path);
    void load(const std::string& path);

    static void verify(const std::string& path);

    KCM_Database* raw();
};
```

#### Constructor

```cpp
kcm::Database db;
```

Creates a new in-memory database. Throws `kcm::Error` on failure.

#### `insert`

```cpp
void insert(const Fact& fact);
```

Insert a fact. Throws `kcm::Error` on failure.

#### `update`

```cpp
void update(uint64_t row_id, const Fact& fact);
```

Update a fact by row ID. Throws `kcm::Error` if not found.

#### `remove`

```cpp
void remove(uint64_t row_id);
```

Delete a fact by row ID. Throws `kcm::Error` if not found.

#### `fact_count`

```cpp
uint64_t fact_count() const;
```

Returns total fact count (including deleted).

#### `active_count`

```cpp
uint64_t active_count() const;
```

Returns active (non-deleted) fact count.

#### `query`

```cpp
Query query(const std::string& kql);
```

Execute a KQL query. Returns a `Query` object for iteration.

#### `begin_transaction`

```cpp
Transaction begin_transaction();
```

Begin a new transaction. Returns a `Transaction` object.

#### `save`

```cpp
void save(const std::string& path);
```

Save the database to a file. Throws `kcm::Error` on I/O failure.

#### `load`

```cpp
void load(const std::string& path);
```

Load a database from a file. Throws `kcm::Error` on I/O or corruption.

#### `verify`

```cpp
static void verify(const std::string& path);
```

Verify database file integrity. Throws `kcm::Error` if corrupted.

#### `raw`

```cpp
KCM_Database* raw();
```

Returns the underlying C FFI handle. Use for interoperability with C code.

### `kcm::Query`

RAII wrapper for query result iteration.

```cpp
class Query {
public:
    explicit Query(KCM_Query* q);
    ~Query();

    Query(const Query&) = delete;
    Query& operator=(const Query&) = delete;
    Query(Query&& o) noexcept;

    std::optional<Fact> next();
    std::vector<Fact> collect();
};
```

#### `next`

```cpp
std::optional<Fact> next();
```

Returns the next fact, or `std::nullopt` if no more results.

#### `collect`

```cpp
std::vector<Fact> collect();
```

Collect all remaining results into a vector.

### `kcm::Transaction`

RAII wrapper for transaction management.

```cpp
class Transaction {
public:
    explicit Transaction(KCM_Transaction* t);
    ~Transaction();

    Transaction(const Transaction&) = delete;
    Transaction& operator=(const Transaction&) = delete;
    Transaction(Transaction&& o) noexcept;

    void commit();
    void rollback();
};
```

#### `commit`

```cpp
void commit();
```

Commit the transaction. Throws `kcm::Error` on failure.

#### `rollback`

```cpp
void rollback();
```

Rollback the transaction. Safe to call multiple times.

## Error Handling

All errors are reported via `kcm::Error` exceptions. Catch by reference:

```cpp
try {
    kcm::Database db;
    db.insert(fact);
} catch (const kcm::Error& e) {
    std::cerr << "Error [" << e.code() << "]: " << e.what() << std::endl;
}
```

| `KCM_Error` Code | Exception Message |
|-------------------|-------------------|
| `KCM_ERR_NOT_FOUND` | "not_found" |
| `KCM_ERR_OUT_OF_MEMORY` | "out_of_memory" |
| `KCM_ERR_INVALID_ARGUMENT` | "invalid_argument" |
| `KCM_ERR_IO` | "io" |
| `KCM_ERR_CORRUPTED` | "corrupted" |
| `KCM_ERR_CONFLICT` | "conflict" |
| `KCM_ERR_TRANSACTION_ABORTED` | "transaction_aborted" |

## Example Code

### Transactions

```cpp
kcm::Database db;

auto txn = db.begin_transaction();
try {
    kcm::Fact fact;
    fact.subject = 1;
    fact.predicate = 2;
    fact.object = 3;
    fact.confidence = 0.9;

    db.insert(fact);
    txn.commit();
} catch (...) {
    txn.rollback();
    throw;
}
```

### Query and Collect

```cpp
kcm::Database db;
auto results = db.query("SELECT * FROM facts");
auto facts = results.collect();

for (const auto& f : facts) {
    std::cout << "Subject=" << f.subject
              << " Object=" << f.object
              << " Confidence=" << f.confidence << std::endl;
}
```

### Iterative Query

```cpp
kcm::Database db;
auto q = db.query("SELECT * FROM facts");
while (auto fact = q.next()) {
    std::cout << "Subject=" << fact->subject << std::endl;
}
```

### Save, Load, and Verify

```cpp
kcm::Database db;
db.save("knowledge.kcm");
db.load("knowledge.kcm");
kcm::Database::verify("knowledge.kcm");
```

## Benchmark

Build and run the benchmark suite:

```bash
cargo bench --workspace
```

| Metric | Target |
|--------|--------|
| Insert (1M facts) | < 2s |
| Query (100K results) | < 50ms |
| Save/Load (1M facts) | < 5s |
| Memory (1M facts) | < 512MB |

Results are published with each release. See `docs/PRD-TESTING& BRACHMARCK.md` for methodology.
