# KCM Indexing Specification

**Document ID:** KCM-INDEX-001
**Version:** 1.0.0
**Status:** Active
**Owner:** Specification Lock (P4)
**Authority:** P3 (PRD2.md §6)

---

## 1. Purpose

Defines KCM's index structures: BitmapIndex, ZoneMap, BloomFilter, CompositeIndex, and index selection rules.

## 2. Index Types

### 2.1 BitmapIndex

**Purpose:** Equality lookup on low-cardinality u8 columns.

```rust
pub struct BitmapIndex {
    values: Vec<u8>,
    bitmaps: Vec<Bitmap>,
}
```

**Construction:** `BitmapIndex::new(column: &[u8], row_count: usize)`
- Creates one Bitmap per distinct value
- Each bitmap has bits set at positions where that value occurs

**Operations:**
| Method | Complexity | Description |
|--------|-----------|-------------|
| `lookup(value)` | O(log k) | Binary search for value, return bitmap |
| `range_query(low, high)` | O(k × n/64) | Union of bitmaps for values in range |

**Parameters:**
- k = number of distinct values
- n = row count

**Use Cases:**
- Predicate column (≤256 distinct values)
- Context column
- Evidence column

### 2.2 ZoneMap

**Purpose:** Range filtering per block.

```rust
pub struct ZoneMap {
    min_values: Vec<i64>,
    max_values: Vec<i64>,
    row_ranges: Vec<(usize, usize)>,
}
```

**Construction:** `ZoneMap::new(column: &[i64], block_size: usize)`
- Divides column into blocks of `block_size` rows
- Records min/max per block

**Operations:**
| Method | Complexity | Description |
|--------|-----------|-------------|
| `range_query(low, high)` | O(b) | Return blocks where max >= low AND min <= high |

**Parameters:**
- b = number of blocks = ceil(n / block_size)
- Default block_size: 1000 rows

**Use Cases:**
- Timestamp column (range queries on time)

### 2.3 BloomFilter

**Purpose:** Probabilistic membership test.

```rust
pub struct BloomFilter {
    words: Vec<u64>,
    num_bits: usize,
    num_hashes: usize,
}
```

**Construction:** `BloomFilter::new(capacity: usize)`
- `num_bits = max(capacity × 10, 1024)`
- `num_hashes = 7`
- Hash function: `((value << 32) | seed) × 0x9e3779b97f4a7c15` (fibonacci hashing)

**Operations:**
| Method | Complexity | Description |
|--------|-----------|-------------|
| `insert(value)` | O(k) | Set k bits |
| `contains(value)` | O(k) | Check k bits |
| `estimated_memory_bytes()` | O(1) | Memory usage |

**Parameters:**
- False positive rate: ~0.8% (with 10 bits per element, 7 hashes)
- Memory: ~1.25 bytes per element

**Use Cases:**
- Subject column (exclusion filter before exact lookup)
- Object column (exclusion filter before exact lookup)

### 2.4 CompositeIndex

**Purpose:** (subject, predicate) pair lookup.

```rust
pub struct CompositeIndex {
    entries: HashMap<(u32, u8), Vec<usize>>,
}
```

**Construction:** `CompositeIndex::build(subjects, predicates, row_count)`

**Operations:**
| Method | Complexity | Description |
|--------|-----------|-------------|
| `lookup(subject, predicate)` | O(1) amortized | Return row IDs for pair |
| `entry_count()` | O(1) | Number of distinct pairs |
| `total_rows()` | O(k) | Total indexed rows |

**Use Cases:**
- Subject-predicate pair queries
- Triple pattern matching

## 3. Index Selection Rules

| Column | Recommended Index | Rationale |
|--------|------------------|-----------|
| Predicate | BitmapIndex | Low cardinality (≤256) |
| Context | BitmapIndex | Low cardinality (≤256) |
| Evidence | BitmapIndex | Low cardinality (≤256) |
| Timestamp | ZoneMap | Range queries |
| Subject | BloomFilter + CompositeIndex | Exclusion + pair queries |
| Object | BloomFilter + CompositeIndex | Exclusion + pair queries |
| Confidence | None (scan) | Continuous range, low benefit |
| Version | None (scan) | Rarely queried directly |
| Priority | None (scan) | Rarely queried directly |
| Owner | BitmapIndex (optional) | Medium cardinality |

## 4. Index Building

### 4.1 When to Build

- On database load (if index metadata present)
- On explicit `rebuild_index()` call
- After bulk insert (batch optimization)

### 4.2 Memory Budget

| Index | Memory Formula | Typical (1M rows) |
|-------|---------------|-------------------|
| BitmapIndex | k × (n/8 + 8) bytes | ~32 KB (256 values) |
| ZoneMap | b × 24 bytes | ~24 KB (1000 blocks) |
| BloomFilter | num_bits / 8 bytes | ~125 KB |
| CompositeIndex | entries × (12 + vec) bytes | ~12 MB |

## 5. Invariants

| Invariant | Enforcement |
|-----------|-------------|
| BitmapIndex bitmaps have same length | Construction verifies |
| ZoneMap blocks cover all rows | Construction partitioning |
| BloomFilter no false negatives | Mathematical guarantee |
| CompositeIndex keys are unique | HashMap semantics |

## 6. References

- **Implements:** PRD2.md §6 (Indexing)
- **Depends on:** KCM_DATA_MODEL_SPEC, KCM_COLUMNAR_FORMAT_SPEC
- **Related:** KCM_QUERY_EXECUTION_SPEC, KCM_COMPRESSION_SPEC
