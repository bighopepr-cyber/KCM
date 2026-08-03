# Naming Convention

| Field | Value |
|-------|-------|
| **Document ID** | KCM-REPO-005 |
| **Title** | Naming Convention |
| **Version** | 1.0.0 |
| **Date** | 2026-08-03 |
| **Status** | Authoritative |
| **Authority** | Engineering Orchestrator (P1) |

---

## 1. Crate Naming

Format: `kcm-{domain}` (lowercase, hyphen-separated)

Examples: kcm-core, kcm-storage, kcm-compute

## 2. Module/File Naming

Format: `snake_case`

Examples: kcm_storage::column, file_format.rs, wal_state.rs

## 3. Type Naming

Format: `PascalCase`

Examples: KnowledgeDatabase, WriteAheadLog, KcmError

## 4. Function Naming

Format: `snake_case`

Examples: insert_fact(), flush_buffer(), replay_entries()

## 5. Constant Naming

Format: `SCREAMING_SNAKE_CASE`

Examples: MAX_FACT_COUNT, WAL_BUFFER_SIZE, COLUMN_SIZES

## 6. Test Naming

Format: `test_{subject}_{scenario}`

Examples: test_wal_append_basic, test_bitmap_set_get_roundtrip

## 7. Benchmark Naming

Format: `bench_{subject}_{operation}`

Examples: bench_column_sequential_scan, bench_bitmap_set_1m

## 8. Documentation Naming

Format: `KCM_{DOMAIN}_{TYPE}.md`

Examples: KCM_STORAGE_SPEC.md, KCM_BENCHMARK_REPORT.md

## 9. Document ID Format

Format: `KCM-{DOMAIN}-{NNN}`

Examples: KCM-REPO-001, KCM-ECO-001, KCM-SDK-001
