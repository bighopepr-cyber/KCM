# KCM Indexing Specification

**Document ID:** KCM-INDEX-001  
**Version:** 1.0.0  
**Status:** Derived  
**Depends on:** KCM-DATA-001

---

## 1. Purpose

Defines index structures for accelerating query execution.

---

## 2. Index Types

### 2.1 Bitmap Index

| Property | Value |
|----------|-------|
| **Structure** | One bitmap per unique value |
| **Best for** | Low-cardinality columns (≤256 values) |
| **Used on** | Predicate column, context column |
| **Update strategy** | Rebuild from column on schema change |

**Construction:**
```
For each unique value v in column:
  Create bitmap of size row_count
  Set bit i for each row where column[i] == v
```

**Lookup:** `lookup(value) -> Option<&Bitmap>` — Binary search on sorted value array, return corresponding bitmap.

**Range query:** `range_query(low, high) -> Result<Bitmap>` — OR all bitmaps in range [low, high].

### 2.2 Zone Map

| Property | Value |
|----------|-------|
| **Structure** | Min/Max per block |
| **Best for** | Range queries on sorted data |
| **Used on** | Timestamp column, integer columns |
| **Block size** | Configurable (default: 1000 rows) |

**Construction:**
```
For each block of block_size rows:
  Compute min and max values
  Store (min, max, row_range)
```

**Range query:** Returns row ranges where block_max >= low AND block_min <= high.

### 2.3 Bloom Filter

| Property | Value |
|----------|-------|
| **Structure** | Bit array with k hash functions |
| **Best for** | Membership testing (probabilistic) |
| **Used on** | Subject/Object membership pre-filter |
| **False positive rate** | ~1% with 10 bits/element, 7 hashes |

**Operations:**
- `insert(value)` — Set k bits
- `contains(value) -> bool` — Check k bits (may false-positive)

### 2.4 Composite Index

#### 2.4.1 Structure

```rust
struct CompositeIndex {
    entries: HashMap<(u32, u8), Vec<usize>>,
}
```

Key = (SubjectID, PredicateID), Value = Vec<row_index>

#### 2.4.2 Operations

| Operation | Algorithm | Complexity |
|-----------|-----------|------------|
| build(subjects, predicates) | Single-pass hash table construction | O(n) |
| lookup(subject, predicate) | HashMap get | O(1) amortized |

#### 2.4.3 Usage

Accelerates queries that filter on both subject and predicate simultaneously:
```rust
let index = CompositeIndex::build(&subjects, &predicates, row_count);
if let Some(rows) = index.lookup(SubjectID(1).0, PredicateID(0).0) {
    // rows contains matching row indices
}
```

### 2.5 Dictionary Codec

| Property | Value |
|----------|-------|
| **Structure** | HashMap<String, u32> + Vec<String> |
| **Best for** | String-to-integer encoding |
| **Used on** | All string-valued columns via SharedDictionary |
| **Thread safety** | Arc<RwLock<Dictionary>> |

---

## 3. Index Selection

The IndexSelectionOptimizer chooses indexes based on predicate type:

| Predicate | Preferred Index | Rationale |
|-----------|----------------|-----------|
| EqualPredicate(v) | BitmapIndex | Exact match on low-cardinality |
| EqualContext(v) | BitmapIndex | Exact match on context |
| EqualSubject(v) | Composite (Subject, Predicate) | Subject-predicate pair lookup |
| EqualObject(v) | BloomFilter | Pre-filter before full scan |
| RangeTimestamp(lo, hi) | ZoneMap | Block-level range skip |

---

## 4. Integration

Indexes are constructed on-demand from column data:

```rust
// Construct bitmap index from predicate column
let index = BitmapIndex::new(schema.predicate_col.as_slice(), schema.len())?;

// Lookup
let bitmap = index.lookup(predicate_id);
```

---

## 5. Constraints

| Constraint | Rationale |
|------------|-----------|
| Bitmap index requires ≤256 unique values | Memory scales with cardinality |
| ZoneMap is approximate | Block-level filtering only |
| BloomFilter has false positives | Must verify with full column scan |

---

## 6. References

- **Depends on:** KCM_DATA_MODEL_SPEC (KCM_DATA_MODEL_SPEC)
- **Parent specs:** KCM_SPECIFICATION (KCM_SPECIFICATION)
- **Related:** KCM_QUERY_EXECUTION_SPEC (KCM_QUERY_EXECUTION_SPEC), KCM_COMPRESSION_SPEC (KCM_COMPRESSION_SPEC)
