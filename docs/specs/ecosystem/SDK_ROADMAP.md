# SDK Roadmap

| Field | Value |
|-------|-------|
| **Document ID** | KCM-ECO-003 |
| **Title** | SDK Roadmap |
| **Version** | 1.0.0 |
| **Date** | 2026-08-03 |
| **Status** | Authoritative |
| **Authority** | Engineering Orchestrator (P1) |

---

## 1. SDK Registry

| # | Language | Architecture | API Style | Packaging | Priority | Timeline |
|---|----------|-------------|-----------|-----------|----------|----------|
| 1 | Rust | Native crate | Direct API | crates.io | P0 | Current |
| 2 | Python | PyO3 bindings | kcm.Database() | PyPI | P1 | Q4 2026 |
| 3 | JavaScript | N-API bindings | kcm.Database() | npm | P1 | Q4 2026 |
| 4 | TypeScript | Typed JS wrapper | kcm.Database() | npm | P1 | Q1 2027 |
| 5 | Go | CGo FFI | kcm.NewDatabase() | go.dev | P2 | Q1 2027 |
| 6 | Java | JNI bindings | new KcmDatabase() | Maven | P2 | Q2 2027 |
| 7 | .NET | P/Invoke | new KcmDatabase() | NuGet | P2 | Q2 2027 |
| 8 | C | Raw FFI | KCM_DatabaseNew() | system lib | P0 | Current |
| 9 | C++ | RAII wrapper | kcm::Database() | system lib | P2 | Q2 2027 |

## 2. API Design Pattern

All SDKs follow a consistent API pattern:

```rust
// Rust (native)
let db = KnowledgeDatabase::new()?;
db.insert(&fact)?;

// Python
db = kcm.Database()
db.insert(fact)

// JavaScript
const db = new kcm.Database()
db.insert(fact)
```

## 3. Packaging Strategy

| Language | Package Manager | Distribution |
|----------|----------------|--------------|
| Rust | crates.io | Public registry |
| Python | PyPI | Public registry |
| JavaScript | npm | Public registry |
| Go | go.dev | Module proxy |
| Java | Maven Central | Public repository |
| .NET | NuGet | Public feed |
