# KCM Compression Specification

**Document ID:** KCM-COMP-001
**Version:** 1.0.0
**Status:** Active
**Owner:** Specification Lock (P4)
**Authority:** P4 (PRD2.md §2.2)

---

## 1. Purpose

Defines KCM's encoding and compression strategies for columnar data: encoding types, compression codecs, and per-column assignments.

## 2. Compressor Trait

```rust
pub trait Compressor {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, KcmError>;
    fn decompress(&self, data: &[u8], expected_size: usize) -> Result<Vec<u8>, KcmError>;
}
```

All codecs implement this trait. `expected_size` is used by LZ4 for output buffer allocation.

## 3. Encoding Types

Encoding transforms typed values into bytes before compression.

### 3.1 Identity

- **Description:** Raw byte representation, no transformation
- **Implementation:** `std::slice::from_raw_parts` cast
- **Used by:** Priority (`i8`)

### 3.2 Dictionary

- **Description:** Maps string/entity references to u32 IDs via `DictionaryCodec`
- **Implementation:** Values stored as raw u32/u8 IDs; dictionary stored separately
- **Used by:** Subject, Predicate, Object, Evidence, Context, Owner
- **Note:** Dictionary is stored in the database header, not in column blocks

### 3.3 Delta

- **Description:** Stores difference between consecutive values
- **Implementation:**
  - First value stored as raw bytes
  - Subsequent values stored as `value[i] - value[i-1]` (wrapping)
  - i64 variant: 8-byte deltas
  - i32 variant: 4-byte deltas
- **Used by:** Timestamp (`i64`), Version (`i32`)
- **Benefit:** Reduces storage for monotonically increasing sequences

### 3.4 Gorilla

- **Description:** XOR-based floating-point encoding
- **Implementation:**
  - First value stored as raw 8 bytes
  - Subsequent values:
    - If XOR == 0: store single `0x00` flag (value same as previous)
    - If XOR != 0: store `0x01` flag + leading zeros (1 byte) + shifted XOR (8 bytes) + trailing zeros (1 byte)
- **Used by:** Confidence (`f64`)
- **Benefit:** Exploits temporal correlation in confidence values

### 3.5 RLE (Encoding Layer)

- **Description:** Run-length encoding at the encoding layer (distinct from RLE compression)
- **Implementation:** For u8/i8 columns, values are stored as raw bytes (identity encoding)
- **Used by:** Predicate, Evidence, Context, Priority
- **Note:** Actual RLE compression happens at the compression layer

## 4. Compression Codecs

Compression operates on the byte output of encoding.

### 4.1 Zstd (Zstandard)

- **Level:** 3 (default)
- **Algorithm:** Zstandard general-purpose compression
- **Strengths:** High compression ratio, fast decompression
- **Use case:** General-purpose, high-ratio columns
- **Used by:** Subject, Object, Confidence, Timestamp, Owner

### 4.2 LZ4

- **Mode:** FAST(1)
- **Algorithm:** LZ4 block compression
- **Strengths:** Fastest compression/decompression
- **Use case:** Speed-critical columns
- **Used by:** Version
- **Note:** Requires `expected_size` for decompression buffer allocation

### 4.3 RLE (Compression Layer)

- **Algorithm:** Run-Length Encoding
- **Format:** `[value: u8][count: u32 LE]` pairs
- **Strengths:** Excellent for low-cardinality data
- **Use case:** Columns with few distinct values
- **Used by:** Predicate, Evidence, Context, Priority

### 4.4 Noop

- **Algorithm:** Identity (no compression)
- **Use case:** Already compressed data or small columns
- **Used by:** None in default configuration

## 5. Per-Column Assignment

| Column | Type | Encoding | Compression | Rationale |
|--------|------|----------|-------------|-----------|
| Subject | u32 | Dictionary | Zstd | High cardinality, string references |
| Predicate | u8 | Dictionary | RLE | Low cardinality (≤256 distinct) |
| Object | u32 | Dictionary | Zstd | High cardinality, string references |
| Confidence | f64 | Gorilla | Zstd | Temporal correlation in floats |
| Evidence | u8 | Dictionary | RLE | Low cardinality |
| Timestamp | i64 | Delta | Zstd | Monotonically increasing |
| Context | u8 | Dictionary | RLE | Low cardinality |
| Version | i32 | Delta | LZ4 | Monotonically increasing, speed-critical |
| Priority | i8 | Identity | RLE | Low cardinality, small range |
| Owner | u16 | Dictionary | Zstd | Medium cardinality |

## 6. Hashing

### 6.1 BLAKE3

- **Used for:** File integrity checksums, key derivation
- **Output:** 32 bytes (256 bits)
- **API:**
  - `hash_blake3(data: &[u8]) -> [u8; 32]`
  - `hash_blake3_hex(data: &[u8]) -> String` (64-char hex string)
- **Justification:** Fastest cryptographic hash, parallelizable, better than SHA-256 in every metric

### 6.2 CRC32

- **Used for:** WAL entry integrity checks
- **Polynomial:** 0xEDB88320 (reflected)
- **Output:** 4 bytes (32 bits)
- **Justification:** Fast, sufficient for WAL entry-level corruption detection

## 7. Compression Pipeline

```
Typed Values → Encoding → Raw Bytes → Compression → Compressed Bytes
                                              ↓
                                    [Stored in Column Block]
```

### 7.1 Decompression Pipeline

```
Compressed Bytes → Decompression → Raw Bytes → Decoding → Typed Values
```

### 7.2 Round-Trip Guarantee

For all valid inputs: `decode(encode(values)) == values`

## 8. Performance Characteristics

| Codec | Compression Speed | Decompression Speed | Ratio (typical) |
|-------|------------------|--------------------|----------------| 
| Zstd (3) | ~500 MB/s | ~1500 MB/s | 3-5x |
| LZ4 (FAST) | ~800 MB/s | ~4000 MB/s | 2-3x |
| RLE | ~200 MB/s | ~300 MB/s | 5-10x (low cardinality) |
| Noop | ∞ | ∞ | 1x |

## 9. References

- **Implements:** PRD2.md §2.2 (Compression Codecs)
- **Depends on:** KCM_DATA_MODEL_SPEC (type definitions)
- **Related:** KCM_COLUMNAR_FORMAT_SPEC, KCM_INDEXING_SPEC
