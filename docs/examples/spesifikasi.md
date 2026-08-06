# Examples Technical Specification

## Overview

This document specifies the technical architecture, components, and design of the KCM `examples/` directory. Examples are educational, self-contained code demonstrations that show how to use KCM across multiple programming languages.

## Scope

- Rust examples (implemented): `basic_usage`, `reasoning`, `transactions`
- Python examples (planned)
- JavaScript examples (planned)
- Go examples (planned)
- Java examples (planned)
- Cross-language getting-started guides

## Responsibilities

| Responsibility | Owner |
|----------------|-------|
| Rust example implementation | Core team |
| Language-specific examples | Language contributors |
| Getting started guides | Documentation team |
| Example CI integration | DevOps |
| Security review of examples | Security team |

## Technical Specification

### Rust Examples

#### basic_usage

| Property | Value |
|----------|-------|
| File | `examples/rust/examples/basic_usage.rs` |
| Purpose | Demonstrate core KCM operations: database creation, fact insertion, query execution |
| API Surface | `KnowledgeDatabase::new()`, `insert()`, `query()`, `close()` |
| Data Model | Synthetic knowledge graph facts (subjects, predicates, objects, confidence) |
| Error Handling | All operations use `Result<T, KcmError>` |
| Output | Printed query results to stdout |

#### reasoning

| Property | Value |
|----------|-------|
| File | `examples/rust/examples/reasoning.rs` |
| Purpose | Demonstrate forward-chaining inference engine |
| API Surface | `KnowledgeDatabase::new()`, `insert_rule()`, `infer()`, `query()` |
| Data Model | Inference rules and derived facts |
| Error Handling | All operations use `Result<T, KcmError>` |
| Output | Printed inferred facts to stdout |

#### transactions

| Property | Value |
|----------|-------|
| File | `examples/rust/examples/transactions.rs` |
| Purpose | Demonstrate transactional operations: begin, commit, rollback |
| API Surface | `KnowledgeDatabase::new()`, `begin_transaction()`, `commit()`, `rollback()`, `query()` |
| Data Model | Transactional fact modifications with isolation |
| Error Handling | All operations use `Result<T, KcmError>` |
| Output | Printed transaction state changes to stdout |

### Planned Examples

| Language | Examples | Status |
|----------|----------|--------|
| Python | basic_usage, reasoning, transactions | Planned |
| JavaScript | basic_usage, reasoning, transactions | Planned |
| Go | basic_usage, reasoning, transactions | Planned |
| Java | basic_usage, reasoning, transactions | Planned |

## Architecture

### Directory Structure

```
examples/
├── README.md                    # Overview and status
├── SECURITY.md                  # Security policy for examples
├── CONTRIBUTING.md              # Contribution guidelines
├── CODE_OF_CONDUCT.md           # Community guidelines
├── rust/
│   ├── Cargo.toml               # Rust package manifest
│   ├── Cargo.lock               # Dependency lock file
│   ├── README.md                # Rust-specific instructions
│   ├── src/
│   │   ├── main.rs              # Module declarations
│   │   ├── basic_usage.rs       # Source module
│   │   ├── reasoning.rs         # Source module
│   │   └── transactions.rs      # Source module
│   └── examples/
│       ├── basic_usage.rs       # Runnable example
│       ├── reasoning.rs         # Runnable example
│       └── transactions.rs      # Runnable example
├── python/                      # Planned
├── javascript/                  # Planned
├── go/                          # Planned
└── java/                        # Planned
```

### Design Principles

| Principle | Description |
|-----------|-------------|
| Self-contained | Each example is a standalone program |
| No external state | Examples use in-memory databases only |
| Deterministic | Same input produces same output |
| Educational | Code prioritizes clarity over performance |
| Error-demonstrating | All error paths are handled explicitly |

## Internal Components

### Rust Example Components

| Component | File | Purpose |
|-----------|------|---------|
| BasicUsage | `examples/basic_usage.rs` | Database lifecycle, CRUD operations |
| Reasoning | `examples/reasoning.rs` | Rule definition, inference execution |
| Transactions | `examples/transactions.rs` | ACID transaction operations |
| ModuleRoot | `src/main.rs` | Module declarations and re-exports |

### Example Lifecycle

```
1. Initialize KnowledgeDatabase (in-memory)
2. Define schema / insert facts / define rules
3. Execute operations (query / infer / transact)
4. Print results to stdout
5. Close database
```

## Data Model

### Fact Structure (used across examples)

```
Fact {
    subject: String,      // Entity name (e.g., "Alice")
    predicate: String,    // Relationship (e.g., "knows")
    object: String,       // Target entity (e.g., "Bob")
    confidence: f64,      // Confidence score (0.0 - 1.0)
}
```

### Rule Structure (reasoning example)

```
Rule {
    name: String,                    // Rule identifier
    conditions: Vec<FactPattern>,    // IF conditions
    conclusion: FactPattern,         // THEN conclusion
    confidence: f64,                 // Inference confidence
}
```

### Transaction State

```
Transaction {
    id: TransactionId,
    status: Active | Committed | RolledBack,
    modifications: Vec<FactModification>,
}
```

## Execution Flow

### Example Execution Sequence

```
User runs: cargo run --example basic_usage
        │
        ▼
┌─────────────────┐
│  main()          │
├─────────────────┤
│ 1. Create DB     │ ← KnowledgeDatabase::new(":memory:")
│ 2. Insert facts  │ ← db.insert(fact)
│ 3. Query facts   │ ← db.query(pattern)
│ 4. Print results │ ← println!("{:?}", results)
│ 5. Close DB      │ ← db.close()
└─────────────────┘
        │
        ▼
    Process exits (0)
```

## Public API

### Example Entry Points

| Example | Entry Point | Signature |
|---------|-------------|-----------|
| basic_usage | `main()` | `fn main() -> Result<(), KcmError>` |
| reasoning | `main()` | `fn main() -> Result<(), KcmError>` |
| transactions | `main()` | `fn main() -> Result<(), KcmError>` |

### Key KCM API Functions Used

| Function | Crate | Purpose |
|----------|-------|---------|
| `KnowledgeDatabase::new()` | kcm-runtime | Create in-memory database |
| `db.insert()` | kcm-runtime | Insert a fact |
| `db.query()` | kcm-runtime | Execute a query |
| `db.insert_rule()` | kcm-reasoning | Define an inference rule |
| `db.infer()` | kcm-reasoning | Run inference engine |
| `db.begin_transaction()` | kcm-runtime | Start a transaction |
| `db.commit()` | kcm-runtime | Commit transaction |
| `db.rollback()` | kcm-runtime | Rollback transaction |
| `db.close()` | kcm-runtime | Close database |

## Configuration

Examples use no external configuration. All settings are hardcoded for educational clarity.

| Setting | Value | Rationale |
|---------|-------|-----------|
| Database backend | In-memory | No disk I/O, no persistence |
| Thread pool | Default | No custom threading |
| Compression | None | In-memory only, no compression needed |
| WAL | Disabled | In-memory, no durability needed |
| Log level | INFO | Demonstrates logging |

## Dependencies

### Rust

| Dependency | Version | Purpose |
|------------|---------|---------|
| kcm-core | path dependency | Core types |
| kcm-storage | path dependency | Storage engine |
| kcm-compute | path dependency | Query engine |
| kcm-reasoning | path dependency | Inference engine |
| kcm-runtime | path dependency | Database runtime |

### Python (planned)

| Dependency | Purpose |
|------------|---------|
| kcm-python | Python bindings via PyO3 |

### JavaScript (planned)

| Dependency | Purpose |
|------------|---------|
| kcm-js | Node.js bindings |

### Go (planned)

| Dependency | Purpose |
|------------|---------|
| kcm-go | Go bindings via C FFI |

### Java (planned)

| Dependency | Purpose |
|------------|---------|
| kcm-java | Java bindings via JNI |

## Error Handling

All examples follow the KCM error model:

```rust
fn main() -> Result<(), KcmError> {
    // All operations return Result<T, KcmError>
    let db = KnowledgeDatabase::new(":memory:")
        .map_err(|e| eprintln!("Failed to create database: {}", e))?;
    
    // No unwrap() — all errors are handled
    db.insert(fact)
        .map_err(|e| eprintln!("Failed to insert: {}", e))?;
    
    Ok(())
}
```

### Error Categories in Examples

| Error Type | Example Usage |
|------------|---------------|
| `NotFound` | Query returns no results |
| `InvalidArgument` | Malformed fact or rule |
| `Io` | Database creation failure (unlikely in-memory) |
| `Conflict` | Transaction conflict |

## Performance Characteristics

Examples are not performance benchmarks. Expected characteristics:

| Metric | Target |
|--------|--------|
| Startup time | < 1 second |
| Execution time | < 30 seconds |
| Memory usage | < 100 MB |
| Disk usage | 0 bytes (in-memory only) |

## Security Considerations

- Examples use in-memory databases only — no disk persistence.
- No secrets, keys, or credentials in example code.
- No network connections in examples.
- No user input processing in examples.
- See [SECURITY.md](../../examples/SECURITY.md) for full security policy.

## Integration

### CI Integration

| Component | Status |
|-----------|--------|
| Rust examples build | Integrated (`cargo build --examples`) |
| Rust examples run | Integrated (CI validation) |
| Python examples | Planned |
| JavaScript examples | Planned |
| Go examples | Planned |
| Java examples | Planned |

### Build Commands

```bash
# Build all Rust examples
cargo build --examples --manifest-path examples/rust/Cargo.toml

# Run specific example
cargo run --example basic_usage --manifest-path examples/rust/Cargo.toml
```

## Sequence Diagram

### basic_usage Execution

```
User          main()         KnowledgeDatabase    Storage
  │              │                  │                │
  │──run──►      │                  │                │
  │              │──new(":memory:")──►              │
  │              │                  │──init──►       │
  │              │◄──db─────────────│                │
  │              │──insert(fact)───►│                │
  │              │                  │──write──►      │
  │              │◄──Ok────────────│                │
  │              │──query(pattern)─►│                │
  │              │                  │──scan──►       │
  │              │◄──results────────│                │
  │              │──println!()      │                │
  │              │──close()────────►│                │
  │              │                  │──flush──►      │
  │              │◄──Ok────────────│                │
  │◄──exit(0)───│                  │                │
```

### transactions Execution

```
User          main()         Transaction    KnowledgeDatabase
  │              │                │                │
  │──run──►      │                │                │
  │              │──new()────────►│──begin────────►│
  │              │                │                │
  │              │──insert(f1)───►│──stage────────►│
  │              │                │                │
  │              │──commit()─────►│──flush────────►│
  │              │                │                │
  │              │──begin()──────►│──begin────────►│
  │              │                │                │
  │              │──insert(f2)───►│──stage────────►│
  │              │                │                │
  │              │──rollback()───►│──discard──────►│
  │              │                │                │
  │◄──exit(0)───│                │                │
```

## Architecture Diagram

```
┌─────────────────────────────────────────────────┐
│                  examples/                       │
├──────────┬──────────┬──────────┬────────┬───────┤
│  rust/   │ python/  │  go/     │  java/ │  js/  │
│ (active) │ (planned)│(planned) │(planned│(planned│
├──────────┴──────────┴──────────┴────────┴───────┤
│              KCM Core Libraries                  │
├──────────┬──────────┬──────────┬────────────────┤
│ kcm-core │kcm-store │kcm-compute│kcm-reasoning  │
├──────────┴──────────┴──────────┴────────────────┤
│              kcm-runtime                         │
└─────────────────────────────────────────────────┘
```

## References

- [Examples README](../../examples/README.md)
- [Examples SECURITY.md](../../examples/SECURITY.md)
- [Examples CONTRIBUTING.md](../../examples/CONTRIBUTING.md)
- [PRD.md — Core Types](../../docs/PRD.md)
- [PRD2.md — Runtime](../../docs/PRD2.md)
- [PRD3.md — Reasoning](../../docs/PRD3.md)
- [AGENTS.md](../../AGENTS.md)

## SSOT Alignment

| SSOT Document | Requirement | Example Coverage |
|---------------|-------------|-----------------|
| PRD.md §3 | Core types (Fact, RowID, SubjectID, Confidence) | basic_usage demonstrates Fact creation |
| PRD.md §5 | Query engine (ScanOp, FilterOp) | basic_usage demonstrates query execution |
| PRD2.md §18 | Runtime (KnowledgeDatabase, Transaction) | transactions demonstrates ACID operations |
| PRD3.md §26 | Reasoning engine (forward-chaining) | reasoning demonstrates rule inference |
| PRD-TESTING §1 | Test pyramid | Examples are supplementary educational material |
| AGENTS.md | Error model (KcmError) | All examples use Result<T, KcmError> |
| AGENTS.md | No unwrap() | All examples follow no-unwrap policy |
| AGENTS.md | In-memory operations | All examples use in-memory databases only |
