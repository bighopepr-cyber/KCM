# KCM Compression Specification

**Document ID:** KCM-COMP-001  
**Version:** 1.0.0  
**Status:** Derived  
**Depends on:** KCM-FORMAT-001

---

## 1. Purpose

Defines compression and encoding strategies for columnar storage.

---

## 2. Compression Architecture

```
Column<T>
    │
    ├── In-Memory: DenseVec<T> (uncompressed, O(1) access)
    │
    └── On-Disk: Compressed bytes
            │
            ├── Encoder (logical encoding)
            └── Compressor (physical compression)
```

### 2.1 Compression Pipeline

```
Write Path:
  Append values to DenseVec
  → flush_data()
  → Encode (Delta/Gorilla/RLE)
  → Compress (Zstd/LZ4)
  → Write to disk

Read Path:
  Read compressed bytes from disk
  → Decompress (Zstd/LZ4)
  → Decode (Delta/Gorilla/RLE)
  → Populate DenseVec
```

---

## 3. Encoding Algorithms

### 3.1 Dictionary Encoding

| Property | Value |
|----------|-------|
| **Algorithm** | Maps values to integer IDs via dictionary |
| **Best for** | Low-cardinality string/ID columns |
| **Used on** | subject (u32), predicate (u8), object (u32), evidence (u8), context (u8), owner (u16) |
| **Ratio** | O(n × log(k)) where k = unique values |

### 3.2 Delta Encoding

| Property | Value |
|----------|-------|
| **Algorithm** | Store differences between consecutive values |
| **Best for** | Monotonically increasing sequences |
| **Used on** | timestamp (i64), version (i32) |
| **Format** | First value raw + (n-1) delta values, each as LE bytes |

### 3.3 Gorilla Encoding

| Property | Value |
|----------|-------|
| **Algorithm** | XOR-based floating-point encoding |
| **Best for** | Slowly changing float sequences |
| **Used on** | confidence (f64) |
| **Format** | First value raw + XOR of consecutive values |

### 3.4 Run-Length Encoding (RLE)

| Property | Value |
|----------|-------|
| **Algorithm** | Encode consecutive identical values as (value, count) pairs |
| **Best for** | Low-cardinality columns with long runs |
| **Used on** | predicate, evidence, context, priority |
| **Format** | [u8 value, u32 LE count] repeated |

### 3.5 Identity Encoding

| Property | Value |
|----------|-------|
| **Algorithm** | No transformation |
| **Best for** | Already compact values |
| **Used on** | priority (i8) with RLE compressor |

---

## 4. Physical Compression

### 4.1 Zstd

| Property | Value |
|----------|-------|
| **Level** | 3 (default) |
| **Best for** | General-purpose, high ratio |
| **Used on** | subject, object, confidence, timestamp, owner |
| **API** | `zstd::encode_all` / `zstd::decode_all` |

### 4.2 LZ4

| Property | Value |
|----------|-------|
| **Level** | 1 (default) |
| **Best for** | Fast decompression |
| **Used on** | version column |
| **API** | `lz4::block::compress` / `lz4::block::decompress` |

### 4.3 RLE Compressor

| Property | Value |
|----------|-------|
| **Algorithm** | Custom RLE for byte arrays |
| **Used on** | predicate, evidence, context, priority (as compression) |
| **Format** | [u8 value, u32 LE count] pairs |

### RLE Binary Format

Each run: `[u8 value][u32 LE count]` = 5 bytes per run.

Example: [0x01, 0x03, 0x00, 0x00, 0x00] = value 0x01 repeated 3 times.

### 4.4 Noop (Passthrough)

| Property | Value |
|----------|-------|
| **Algorithm** | No compression |
| **Used on** | Test scenarios, small columns |

---

## 5. Codec Registry

**See KCM_DATA_MODEL_SPEC (Section 5.2) for the authoritative codec-per-column registry.** This section documents the encoding and compression implementations available in the storage engine.

---

## 6. Compression API

```rust
trait Compressor {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, KcmError>;
    fn decompress(&self, data: &[u8], expected_size: usize) -> Result<Vec<u8>, KcmError>;
}
```

### Implementations

| Struct | Compressor |
|--------|------------|
| ZstdCompressor | Zstd at configurable level |
| Lz4Compressor | LZ4 block compression |
| RleCompressor | Custom RLE |
| NoopCompressor | Passthrough |

---

## 7. Validation

| Check | Method | Frequency |
|-------|--------|-----------|
| Compress/decompress roundtrip | Unit tests | Every CI |
| Compression ratio > 1.0 | Benchmark | Weekly |
| Decompressed data matches original | Integration test | Every commit |

---

## 8. Constraints

| Constraint | Rationale |
|------------|-----------|
| No lossy compression | Knowledge integrity must be preserved |
| Decompression must be deterministic | Identical compressed data produces identical output |
| Column encoding chosen at schema creation | Prevents runtime encoding mismatch |

---

## 9. References

- **Depends on:** KCM_COLUMNAR_FORMAT_SPEC (KCM_COLUMNAR_FORMAT_SPEC)
- **Parent specs:** KCM_SPECIFICATION (KCM_SPECIFICATION)
- **Related:** KCM_DATA_MODEL_SPEC (KCM_DATA_MODEL_SPEC), KCM_COLUMNAR_FORMAT_SPEC (KCM_COLUMNAR_FORMAT_SPEC)
