# KCM C++ SDK

Official SDK for the KCM Knowledge Columnar Model.

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![Version](https://img.shields.io/badge/version-1.0.0-blue)]()
[![License](https://img.shields.io/badge/license-MIT-green)]()
[![KCM Engine Compatible](https://img.shields.io/badge/KCM%20Engine-1.0.0-orange)]()

## Installation

```bash
cd sdk/cpp && mkdir build && cd build && cmake .. && make
```

Requires C++17 compiler (GCC 7+, Clang 5+, MSVC 2017+) and `libkcm` built from the `kcm-interface` crate.

## Quickstart

```cpp
#include <kcm.hpp>
#include <iostream>

int main() {
    kcm::Database db;

    db.insert({1, 0, 2, 0.95, 0, 0, 0, 0, 0, 0});
    db.insert({2, 1, 3, 0.90, 0, 0, 0, 0, 0, 0});

    auto results = db.queryAll();
    for (const auto& fact : results) {
        std::cout << "Subject: " << fact.subject << ", Object: " << fact.object << std::endl;
    }

    std::cout << "Fact count: " << db.fact_count() << std::endl;
    return 0;
}
```

## API Reference

### `kcm::Database`

| Operation | Signature | Description |
|-----------|-----------|-------------|
| `Database()` | Constructor | Create in-memory database |
| `insert(fact)` | `void insert(const Fact& fact)` | Insert a fact |
| `update(row_id, fact)` | `void update(uint64_t row_id, const Fact& fact)` | Update by row ID |
| `remove(row_id)` | `void remove(uint64_t row_id)` | Delete by row ID |
| `fact_count()` | `uint64_t fact_count() const` | Total count |
| `active_count()` | `uint64_t active_count() const` | Active count |
| `query(kql)` | `Query query(const std::string& kql)` | Execute KQL query |
| `queryAll()` | `std::vector<Fact> queryAll()` | Query all facts |
| `begin_transaction()` | `Transaction begin_transaction()` | Begin transaction |
| `save(path)` | `void save(const std::string& path)` | Save to file |
| `load(path)` | `void load(const std::string& path)` | Load from file |
| `verify(path)` | `static void verify(const std::string& path)` | Verify integrity |

### `kcm::Query`

| Operation | Signature | Description |
|-----------|-----------|-------------|
| `next()` | `std::optional<Fact> next()` | Get next result |
| `collect()` | `std::vector<Fact> collect()` | Collect all results |

### `kcm::Transaction`

| Operation | Signature | Description |
|-----------|-----------|-------------|
| `commit()` | `void commit()` | Commit transaction |
| `rollback()` | `void rollback()` | Rollback transaction |

## Error Handling

C++ exceptions are thrown on errors. All methods that can fail throw `kcm::KcmError`:

| Error Code | Description |
|------------|-------------|
| `NotFound` | Resource not found |
| `OutOfMemory` | Insufficient memory |
| `InvalidArgument` | Invalid argument |
| `Io` | I/O error |
| `Corrupted` | Data corruption |
| `Conflict` | Concurrent conflict |
| `TransactionAborted` | Transaction aborted |

## Use Cases

### Basic Query

```cpp
#include <kcm.hpp>

int main() {
    kcm::Database db;

    db.insert({1, 0, 2, 0.95, 0, 0, 0, 0, 0, 0});
    db.insert({2, 1, 3, 0.90, 0, 0, 0, 0, 0, 0});

    auto results = db.queryAll();
    for (const auto& fact : results) {
        std::cout << "S=" << fact.subject << " O=" << fact.object << std::endl;
    }
    return 0;
}
```

### API Integration

```cpp
#include <kcm.hpp>
#include <vector>

std::vector<kcm::Fact> fetch_all(kcm::Database& db) {
    return db.queryAll();
}
```

### Transaction

```cpp
#include <kcm.hpp>

int main() {
    kcm::Database db;
    auto txn = db.begin_transaction();

    db.insert({10, 0, 20, 0.85, 0, 0, 0, 0, 0, 0});

    if (db.active_count() > 0) {
        txn.commit();
    } else {
        txn.rollback();
    }
    return 0;
}
```

## Build

```bash
mkdir build && cd build
cmake ..
make
ctest
```

## Full Documentation

See [docs/sdk/cpp.md](../../docs/sdk/cpp.md) for the complete reference.

## Examples

See [examples/](examples/) for more examples including:
- `basic/basic.cpp` — Getting started
- `basic/01_basic_crud.cpp` — CRUD operations
- `basic/02_transactions.cpp` — Transaction management
- `basic/03_persistence.cpp` — Save/load databases
- `basic/04_query_patterns.cpp` — KQL query patterns
- `basic/05_error_handling.cpp` — Error handling patterns

## License

MIT

Made by bighopepr-cyber
