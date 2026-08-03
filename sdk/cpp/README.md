# KCM C++ SDK

C++ RAII wrapper over the C FFI.

## Status: Planned

## Architecture

- RAII wrapper over `kcm-interface` C API
- Modern C++17 interface
- Header-only or compiled library

## API Design

```cpp
#include <kcm/database.h>

kcm::Database db("my_knowledge.db");
db.insert({"planet", "orbits", "sun", 0.99});
auto results = db.query("SELECT * FROM facts WHERE subject = 'planet'");
// RAII: db closed automatically
```

## Features

- Automatic resource management (RAII)
- Exception-safe error handling
- Move semantics
- Range-based iteration over query results
