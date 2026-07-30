# KNOWLEDGE COLUMNAR MODEL (KCM) – TECHNICAL PRD
## Full Rust Architecture Implementation

---

## PART 1: EXECUTIVE SUMMARY & VISION

### 1.1 Project Definition

**Knowledge Columnar Model (KCM)** adalah paradigma representasi pengetahuan berbasis kolom yang menghilangkan pointer-based graph tradisional dan mengganti dengan **columnar relation space** yang dapat diproses dengan SIMD, dikompresi secara independen, dan dioptimalkan untuk reasoning engine modern. Diimplementasikan 100% dalam Rust untuk maximum performance, memory safety, dan zero runtime overhead.

**Core Thesis**: *Knowledge is not an object graph. Knowledge is a columnar relation space.*

### 1.2 Fundamental Architectural Principles

1. **Pointer-free Architecture**: Zero pointer chasing via columnar layout
2. **Columnar Native Storage**: Setiap aspek knowledge adalah kolom independen linear
3. **Vectorization-Ready**: SIMD intrinsics (SSE4.2, AVX2, AVX-512, NEON)
4. **Dictionary-Encoded Everything**: Semua string/reference → integer dictionary
5. **Deterministic Execution**: Identical input → identical output, selalu
6. **Explainable by Design**: Setiap hasil reasoning membawa evidence dan confidence
7. **Compression-Native**: Delta, Gorilla, Dictionary, RLE per-column
8. **Cache-Friendly**: Linear arrays dengan spatial locality tinggi
9. **Parallel-Safe**: Lock-free readers, write-locked modifications
10. **Production-Grade**: Full ACID, crash recovery, validation

### 1.3 Rust Architecture Decision

**Why Rust?**
- **Memory Safety**: No segfaults, no garbage collection pauses
- **Zero-Cost Abstractions**: Compile-time optimizations, runtime performance
- **SIMD Ready**: Direct access ke CPU intrinsics via packed_simd
- **Concurrency**: Ownership model prevents data races at compile-time
- **Determinism**: No runtime randomness (except thread scheduling, controlled)
- **Deployment**: Single binary, no runtime dependencies
- **Performance**: Comparable to C++, better than Java/Python

---

## PART 2: RUST PROJECT STRUCTURE

### 2.1 Workspace Organization

```toml
# Cargo.toml (root workspace)

[workspace]
members = [
    "crates/kcm-core",
    "crates/kcm-storage",
    "crates/kcm-compute",
    "crates/kcm-reasoning",
    "crates/kcm-optimizer",
    "crates/kcm-runtime",
    "crates/kcm-interface",
]

resolver = "2"
```

### 2.2 Core Crate Dependencies

```toml
# crates/kcm-core/Cargo.toml

[package]
name = "kcm-core"
version = "0.1.0"
edition = "2021"

[dependencies]
# Zero dependencies - pure Rust
parking_lot = "0.12"       # Fast synchronization primitives
siphasher = "0.3"          # SipHash for dictionary
num-traits = "0.2"         # Numeric traits

[dev-dependencies]
criterion = "0.5"          # Benchmarking
proptest = "1.0"           # Property-based testing
quickcheck = "1.0"
```

### 2.3 Storage Crate Dependencies

```toml
# crates/kcm-storage/Cargo.toml

[dependencies]
kcm-core = { path = "../kcm-core" }
parking_lot = "0.12"
zstd = "0.13"              # Zstandard compression
lz4 = "1.24"               # LZ4 compression
blake3 = "1.5"             # Blake3 hashing
thiserror = "1.0"          # Error handling macro
```

### 2.4 Compute Crate Dependencies

```toml
# crates/kcm-compute/Cargo.toml

[dependencies]
kcm-core = { path = "../kcm-core" }
kcm-storage = { path = "../kcm-storage" }
packed_simd_2 = "0.3"      # SIMD operations
```

### 2.5 Runtime Crate Dependencies

```toml
# crates/kcm-runtime/Cargo.toml

[dependencies]
kcm-core = { path = "../kcm-core" }
kcm-storage = { path = "../kcm-storage" }
kcm-compute = { path = "../kcm-compute" }
kcm-reasoning = { path = "../kcm-reasoning" }
kcm-optimizer = { path = "../kcm-optimizer" }
parking_lot = "0.12"
rayon = "1.7"              # Data parallelism
crossbeam = "0.8"          # Multi-threading utilities
tokio = { version = "1.35", features = ["full"] }  # Async (optional)
```

### 2.6 Interface Crate Dependencies

```toml
# crates/kcm-interface/Cargo.toml

[dependencies]
kcm-runtime = { path = "../kcm-runtime" }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

---

## PART 3: RUST TYPE SYSTEM & CORE TYPES

### 3.1 Foundational Type Definitions

```rust
// crates/kcm-core/src/types.rs

use std::fmt;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

/// Unique sequential row identifier (0-indexed)
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RowID(pub u64);

impl RowID {
    pub fn new(id: u64) -> Self {
        RowID(id)
    }
    
    pub fn next(self) -> RowID {
        RowID(self.0 + 1)
    }
    
    pub fn as_usize(self) -> usize {
        self.0 as usize
    }
}

/// Subject entity reference (0-indexed into dictionary)
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SubjectID(pub u32);

impl SubjectID {
    pub fn new(id: u32) -> Self {
        SubjectID(id)
    }
    
    pub fn as_usize(self) -> usize {
        self.0 as usize
    }
}

/// Predicate/relationship type (0-indexed)
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PredicateID(pub u8);

impl PredicateID {
    pub fn new(id: u8) -> Self {
        PredicateID(id)
    }
    
    pub fn as_usize(self) -> usize {
        self.0 as usize
    }
}

/// Object entity reference (0-indexed into dictionary)
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ObjectID(pub u32);

impl ObjectID {
    pub fn new(id: u32) -> Self {
        ObjectID(id)
    }
    
    pub fn as_usize(self) -> usize {
        self.0 as usize
    }
}

/// Context/domain scope
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ContextID(pub u8);

impl ContextID {
    pub const NULL: Self = ContextID(0);
    
    pub fn new(id: u8) -> Self {
        ContextID(id)
    }
}

/// Evidence type
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EvidenceID(pub u8);

impl EvidenceID {
    pub const UNKNOWN: Self = EvidenceID(0);
    
    pub fn new(id: u8) -> Self {
        EvidenceID(id)
    }
}

/// Confidence score (probabilistic)
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Confidence(pub f64);

impl Confidence {
    pub fn new(value: f64) -> Result<Self, String> {
        if value.is_nan() || value.is_infinite() {
            return Err("Confidence must be finite".to_string());
        }
        if !(0.0..=1.0).contains(&value) {
            return Err("Confidence must be in [0.0, 1.0]".to_string());
        }
        Ok(Confidence(value))
    }
    
    pub fn multiply(&self, other: Confidence) -> Confidence {
        let product = (self.0 * other.0).min(1.0).max(0.0);
        Confidence(product)
    }
    
    pub fn combine_or(&self, other: Confidence) -> Confidence {
        let combined = self.0 + other.0 - (self.0 * other.0);
        Confidence(combined.min(1.0).max(0.0))
    }
}

/// Core knowledge fact
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Fact {
    pub subject: SubjectID,
    pub predicate: PredicateID,
    pub object: ObjectID,
    pub confidence: f64,           // Raw f64, validated at insert
    pub evidence: EvidenceID,
    pub timestamp: i64,            // Nanoseconds since epoch
    pub context: ContextID,
    pub version: i32,
    pub priority: i8,
    pub owner: u16,
}

impl Fact {
    pub fn new(
        subject: SubjectID,
        predicate: PredicateID,
        object: ObjectID,
        confidence: f64,
    ) -> Result<Self, String> {
        Confidence::new(confidence)?;
        
        Ok(Fact {
            subject,
            predicate,
            object,
            confidence,
            evidence: EvidenceID::UNKNOWN,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as i64,
            context: ContextID::NULL,
            version: 1,
            priority: 0,
            owner: 0,
        })
    }
}

/// Column identifier for query operations
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ColumnID {
    RowID = 0,
    Subject = 1,
    Predicate = 2,
    Object = 3,
    Confidence = 4,
    Evidence = 5,
    Timestamp = 6,
    Context = 7,
    Version = 8,
    Priority = 9,
    Owner = 10,
}

impl ColumnID {
    pub fn as_usize(self) -> usize {
        self as usize
    }
    
    pub fn all() -> &'static [ColumnID] {
        &[
            ColumnID::RowID,
            ColumnID::Subject,
            ColumnID::Predicate,
            ColumnID::Object,
            ColumnID::Confidence,
            ColumnID::Evidence,
            ColumnID::Timestamp,
            ColumnID::Context,
            ColumnID::Version,
            ColumnID::Priority,
            ColumnID::Owner,
        ]
    }
}

/// Error types
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum KcmError {
    NotFound(String),
    OutOfMemory,
    InvalidArgument(String),
    Io(String),
    Corrupted(String),
    Conflict(String),
    TransactionAborted,
}

impl fmt::Display for KcmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KcmError::NotFound(msg) => write!(f, "NotFound: {}", msg),
            KcmError::OutOfMemory => write!(f, "OutOfMemory"),
            KcmError::InvalidArgument(msg) => write!(f, "InvalidArgument: {}", msg),
            KcmError::Io(msg) => write!(f, "Io: {}", msg),
            KcmError::Corrupted(msg) => write!(f, "Corrupted: {}", msg),
            KcmError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            KcmError::TransactionAborted => write!(f, "TransactionAborted"),
        }
    }
}

impl std::error::Error for KcmError {}

pub type KcmResult<T> = Result<T, KcmError>;
```

### 3.2 Dense Vector Implementation

```rust
// crates/kcm-core/src/vec.rs

use std::alloc::{alloc, dealloc, Layout};
use std::ptr::NonNull;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

/// Dense, aligned vector for SIMD operations
pub struct DenseVec<T: Copy> {
    ptr: NonNull<T>,
    capacity: usize,
    len: usize,
    alignment: usize,
    _phantom: PhantomData<T>,
}

impl<T: Copy> DenseVec<T> {
    const MIN_ALIGNMENT: usize = 64;  // Cache line
    
    pub fn new(capacity: usize) -> Result<Self, String> {
        Self::with_alignment(capacity, Self::MIN_ALIGNMENT)
    }
    
    pub fn with_alignment(capacity: usize, alignment: usize) 
        -> Result<Self, String> 
    {
        if capacity == 0 {
            return Ok(DenseVec {
                ptr: NonNull::dangling(),
                capacity: 0,
                len: 0,
                alignment,
                _phantom: PhantomData,
            });
        }
        
        let layout = Layout::from_size_align(
            capacity * std::mem::size_of::<T>(),
            alignment.max(std::mem::align_of::<T>()),
        ).map_err(|e| format!("Layout error: {}", e))?;
        
        let ptr = unsafe { alloc(layout) } as *mut T;
        let ptr = NonNull::new(ptr)
            .ok_or_else(|| "Allocation failed".to_string())?;
        
        Ok(DenseVec {
            ptr,
            capacity,
            len: 0,
            alignment,
            _phantom: PhantomData,
        })
    }
    
    pub fn push(&mut self, value: T) -> Result<(), String> {
        if self.len >= self.capacity {
            return Err("Vector full".to_string());
        }
        
        unsafe {
            *self.ptr.as_ptr().add(self.len) = value;
        }
        self.len += 1;
        Ok(())
    }
    
    pub fn len(&self) -> usize {
        self.len
    }
    
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    
    pub fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
    
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.as_slice().iter()
    }
}

impl<T: Copy> Index<usize> for DenseVec<T> {
    type Output = T;
    
    fn index(&self, idx: usize) -> &Self::Output {
        &self.as_slice()[idx]
    }
}

impl<T: Copy> IndexMut<usize> for DenseVec<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.as_mut_slice()[idx]
    }
}

impl<T: Copy> Drop for DenseVec<T> {
    fn drop(&mut self) {
        if self.capacity > 0 {
            let layout = Layout::from_size_align(
                self.capacity * std::mem::size_of::<T>(),
                self.alignment.max(std::mem::align_of::<T>()),
            ).unwrap();
            
            unsafe {
                dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

impl<T: Copy> Clone for DenseVec<T> {
    fn clone(&self) -> Self {
        let mut new_vec = Self::with_alignment(self.capacity, self.alignment)
            .expect("Clone allocation failed");
        new_vec.len = self.len;
        new_vec.as_mut_slice().copy_from_slice(self.as_slice());
        new_vec
    }
}
```

### 3.3 Bitmap Implementation

```rust
// crates/kcm-core/src/bitmap.rs

/// Dense bitmap for fast filtering
pub struct Bitmap {
    words: Vec<u64>,
    len: usize,
}

impl Bitmap {
    const WORD_SIZE: usize = 64;
    
    pub fn new(capacity: usize) -> Self {
        let num_words = (capacity + Self::WORD_SIZE - 1) / Self::WORD_SIZE;
        Bitmap {
            words: vec![0u64; num_words],
            len: capacity,
        }
    }
    
    pub fn set(&mut self, idx: usize) {
        assert!(idx < self.len);
        let word_idx = idx / Self::WORD_SIZE;
        let bit_idx = idx % Self::WORD_SIZE;
        self.words[word_idx] |= 1u64 << bit_idx;
    }
    
    pub fn clear(&mut self, idx: usize) {
        assert!(idx < self.len);
        let word_idx = idx / Self::WORD_SIZE;
        let bit_idx = idx % Self::WORD_SIZE;
        self.words[word_idx] &= !(1u64 << bit_idx);
    }
    
    pub fn get(&self, idx: usize) -> bool {
        if idx >= self.len {
            return false;
        }
        let word_idx = idx / Self::WORD_SIZE;
        let bit_idx = idx % Self::WORD_SIZE;
        (self.words[word_idx] & (1u64 << bit_idx)) != 0
    }
    
    pub fn set_all(&mut self) {
        self.words.fill(u64::MAX);
    }
    
    pub fn clear_all(&mut self) {
        self.words.fill(0);
    }
    
    pub fn count_ones(&self) -> usize {
        self.words.iter().map(|w| w.count_ones() as usize).sum()
    }
    
    pub fn and_inplace(&mut self, other: &Bitmap) {
        assert_eq!(self.words.len(), other.words.len());
        for (a, b) in self.words.iter_mut().zip(&other.words) {
            *a &= b;
        }
    }
    
    pub fn or_inplace(&mut self, other: &Bitmap) {
        assert_eq!(self.words.len(), other.words.len());
        for (a, b) in self.words.iter_mut().zip(&other.words) {
            *a |= b;
        }
    }
    
    pub fn not_inplace(&mut self) {
        for word in &mut self.words {
            *word = !*word;
        }
        // Clear high bits beyond len
        let last_word_idx = (self.len + Self::WORD_SIZE - 1) / Self::WORD_SIZE - 1;
        let bits_in_last = self.len % Self::WORD_SIZE;
        if bits_in_last > 0 {
            let mask = (1u64 << bits_in_last) - 1;
            self.words[last_word_idx] &= mask;
        }
    }
    
    pub fn iter_set_bits(&self) -> impl Iterator<Item = usize> + '_ {
        self.words.iter().enumerate().flat_map(|(word_idx, &word)| {
            (0..Self::WORD_SIZE).filter_map(move |bit_idx| {
                if (word & (1u64 << bit_idx)) != 0 {
                    let idx = word_idx * Self::WORD_SIZE + bit_idx;
                    if idx < self.len {
                        return Some(idx);
                    }
                }
                None
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bitmap_operations() {
        let mut bitmap = Bitmap::new(128);
        
        bitmap.set(0);
        bitmap.set(63);
        bitmap.set(64);
        bitmap.set(127);
        
        assert!(bitmap.get(0));
        assert!(bitmap.get(63));
        assert!(bitmap.get(64));
        assert!(bitmap.get(127));
        assert!(!bitmap.get(1));
        
        assert_eq!(bitmap.count_ones(), 4);
    }
}
```

### 3.4 Dictionary Implementation

```rust
// crates/kcm-core/src/dictionary.rs

use std::collections::HashMap;
use parking_lot::RwLock;
use std::sync::Arc;

pub type DictID = u32;

/// Thread-safe, lock-free read dictionary
pub struct Dictionary {
    entries: Vec<String>,
    reverse_map: HashMap<String, DictID>,
}

impl Dictionary {
    pub fn new() -> Self {
        Dictionary {
            entries: vec![String::new()],  // ID 0 = NULL
            reverse_map: {
                let mut map = HashMap::new();
                map.insert(String::new(), 0);
                map
            },
        }
    }
    
    pub fn insert(&mut self, value: &str) -> DictID {
        if let Some(&id) = self.reverse_map.get(value) {
            return id;
        }
        
        let id = self.entries.len() as DictID;
        self.entries.push(value.to_string());
        self.reverse_map.insert(value.to_string(), id);
        id
    }
    
    pub fn get(&self, id: DictID) -> Option<&str> {
        self.entries.get(id as usize).map(|s| s.as_str())
    }
    
    pub fn lookup(&self, value: &str) -> Option<DictID> {
        self.reverse_map.get(value).copied()
    }
    
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.entries.len() <= 1
    }
    
    pub fn entries(&self) -> &[String] {
        &self.entries
    }
}

impl Default for Dictionary {
    fn default() -> Self {
        Self::new()
    }
}

/// Shared, synchronized dictionary for concurrent access
pub struct SharedDictionary(Arc<RwLock<Dictionary>>);

impl SharedDictionary {
    pub fn new() -> Self {
        SharedDictionary(Arc::new(RwLock::new(Dictionary::new())))
    }
    
    pub fn insert(&self, value: &str) -> DictID {
        self.0.write().insert(value)
    }
    
    pub fn get(&self, id: DictID) -> Option<String> {
        self.0.read().get(id).map(|s| s.to_string())
    }
    
    pub fn lookup(&self, value: &str) -> Option<DictID> {
        self.0.read().lookup(value)
    }
    
    pub fn len(&self) -> usize {
        self.0.read().len()
    }
}

impl Default for SharedDictionary {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for SharedDictionary {
    fn clone(&self) -> Self {
        SharedDictionary(self.0.clone())
    }
}
```

---

## PART 4: COLUMNAR STORAGE IMPLEMENTATION

### 4.1 Column Data Structure

```rust
// crates/kcm-storage/src/column.rs

use kcm_core::types::*;
use kcm_core::vec::DenseVec;
use std::mem::size_of;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ColumnEncoding {
    Identity,
    Dictionary,
    Delta,
    FrameOfReference,
    RLE,
    Gorilla,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CompressionCodec {
    None,
    Zstd,
    Lz4,
}

/// Single column of typed data
pub struct Column<T: Copy> {
    data: DenseVec<T>,
    encoding: ColumnEncoding,
    compression: CompressionCodec,
    row_count: u64,
}

impl<T: Copy> Column<T> {
    pub fn new(capacity: usize, encoding: ColumnEncoding, compression: CompressionCodec) 
        -> Result<Self, KcmError> 
    {
        let data = DenseVec::new(capacity)
            .map_err(|e| KcmError::Io(e))?;
        
        Ok(Column {
            data,
            encoding,
            compression,
            row_count: 0,
        })
    }
    
    pub fn append(&mut self, value: T) -> Result<(), KcmError> {
        self.data.push(value)
            .map_err(|e| KcmError::Io(e))?;
        self.row_count += 1;
        Ok(())
    }
    
    pub fn get(&self, idx: usize) -> Option<T> {
        if idx >= self.row_count as usize {
            return None;
        }
        Some(self.data[idx])
    }
    
    pub fn len(&self) -> usize {
        self.row_count as usize
    }
    
    pub fn is_empty(&self) -> bool {
        self.row_count == 0
    }
    
    pub fn as_slice(&self) -> &[T] {
        self.data.as_slice()
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }
}

/// Specialized columns for each type
pub type SubjectColumn = Column<u32>;
pub type ObjectColumn = Column<u32>;
pub type PredicateColumn = Column<u8>;
pub type ContextColumn = Column<u8>;
pub type EvidenceColumn = Column<u8>;
pub type ConfidenceColumn = Column<f64>;
pub type TimestampColumn = Column<i64>;
pub type VersionColumn = Column<i32>;
pub type PriorityColumn = Column<i8>;
pub type OwnerColumn = Column<u16>;

/// Schema definition
pub struct Schema {
    pub subject_col: SubjectColumn,
    pub predicate_col: PredicateColumn,
    pub object_col: ObjectColumn,
    pub confidence_col: ConfidenceColumn,
    pub evidence_col: EvidenceColumn,
    pub timestamp_col: TimestampColumn,
    pub context_col: ContextColumn,
    pub version_col: VersionColumn,
    pub priority_col: PriorityColumn,
    pub owner_col: OwnerColumn,
}

impl Schema {
    pub fn new(capacity: usize) -> Result<Self, KcmError> {
        Ok(Schema {
            subject_col: SubjectColumn::new(capacity, ColumnEncoding::Dictionary, CompressionCodec::Zstd)?,
            predicate_col: PredicateColumn::new(capacity, ColumnEncoding::Dictionary, CompressionCodec::RLE)?,
            object_col: ObjectColumn::new(capacity, ColumnEncoding::Dictionary, CompressionCodec::Zstd)?,
            confidence_col: ConfidenceColumn::new(capacity, ColumnEncoding::Gorilla, CompressionCodec::Zstd)?,
            evidence_col: EvidenceColumn::new(capacity, ColumnEncoding::Dictionary, CompressionCodec::RLE)?,
            timestamp_col: TimestampColumn::new(capacity, ColumnEncoding::Delta, CompressionCodec::Zstd)?,
            context_col: ContextColumn::new(capacity, ColumnEncoding::Dictionary, CompressionCodec::RLE)?,
            version_col: VersionColumn::new(capacity, ColumnEncoding::Delta, CompressionCodec::Lz4)?,
            priority_col: PriorityColumn::new(capacity, ColumnEncoding::Identity, CompressionCodec::RLE)?,
            owner_col: OwnerColumn::new(capacity, ColumnEncoding::Dictionary, CompressionCodec::Zstd)?,
        })
    }
    
    pub fn append_fact(&mut self, fact: &Fact) -> Result<(), KcmError> {
        self.subject_col.append(fact.subject.0)?;
        self.predicate_col.append(fact.predicate.0)?;
        self.object_col.append(fact.object.0)?;
        self.confidence_col.append(fact.confidence)?;
        self.evidence_col.append(fact.evidence.0)?;
        self.timestamp_col.append(fact.timestamp)?;
        self.context_col.append(fact.context.0)?;
        self.version_col.append(fact.version)?;
        self.priority_col.append(fact.priority)?;
        self.owner_col.append(fact.owner)?;
        Ok(())
    }
    
    pub fn len(&self) -> usize {
        self.subject_col.len()
    }
    
    pub fn get_fact(&self, idx: usize) -> Option<Fact> {
        Some(Fact {
            subject: SubjectID(self.subject_col.get(idx)?),
            predicate: PredicateID(self.predicate_col.get(idx)?),
            object: ObjectID(self.object_col.get(idx)?),
            confidence: self.confidence_col.get(idx)?,
            evidence: EvidenceID(self.evidence_col.get(idx)?),
            timestamp: self.timestamp_col.get(idx)?,
            context: ContextID(self.context_col.get(idx)?),
            version: self.version_col.get(idx)?,
            priority: self.priority_col.get(idx)?,
            owner: self.owner_col.get(idx)?,
        })
    }
}
```

### 4.2 Compression Codecs

```rust
// crates/kcm-storage/src/codec.rs

use kcm_core::types::*;

pub trait Codec<T: Copy> {
    fn encode(&self, data: &[T]) -> Result<Vec<u8>, KcmError>;
    fn decode(&self, data: &[u8], count: usize) -> Result<Vec<T>, KcmError>;
}

/// Delta encoding for monotonic sequences
pub struct DeltaCodec;

impl Codec<i64> for DeltaCodec {
    fn encode(&self, data: &[i64]) -> Result<Vec<u8>, KcmError> {
        if data.is_empty() {
            return Ok(vec![]);
        }
        
        let mut deltas = Vec::with_capacity(data.len());
        deltas.push(data[0]);
        
        for i in 1..data.len() {
            deltas.push(data[i] - data[i - 1]);
        }
        
        Ok(bincode::serialize(&deltas)
            .map_err(|e| KcmError::Io(e.to_string()))?)
    }
    
    fn decode(&self, data: &[u8], count: usize) -> Result<Vec<i64>, KcmError> {
        let deltas: Vec<i64> = bincode::deserialize(data)
            .map_err(|e| KcmError::Corrupted(e.to_string()))?;
        
        let mut result = Vec::with_capacity(count);
        let mut current = 0i64;
        
        for delta in deltas {
            current += delta;
            result.push(current);
        }
        
        Ok(result)
    }
}

/// RLE (Run-Length Encoding)
pub struct RleCodec;

impl Codec<u8> for RleCodec {
    fn encode(&self, data: &[u8]) -> Result<Vec<u8>, KcmError> {
        let mut result = Vec::new();
        
        let mut i = 0;
        while i < data.len() {
            let value = data[i];
            let mut count = 1u32;
            
            while i + count as usize < data.len() && data[i + count as usize] == value && count < u32::MAX {
                count += 1;
            }
            
            result.push(value);
            result.extend_from_slice(&count.to_le_bytes());
            i += count as usize;
        }
        
        Ok(result)
    }
    
    fn decode(&self, data: &[u8], _count: usize) -> Result<Vec<u8>, KcmError> {
        let mut result = Vec::new();
        let mut i = 0;
        
        while i < data.len() {
            let value = data[i];
            i += 1;
            
            if i + 4 > data.len() {
                return Err(KcmError::Corrupted("Incomplete RLE entry".to_string()));
            }
            
            let count = u32::from_le_bytes([data[i], data[i+1], data[i+2], data[i+3]]);
            i += 4;
            
            for _ in 0..count {
                result.push(value);
            }
        }
        
        Ok(result)
    }
}

/// Gorilla compression for float64 (time-series)
pub struct GorillaCodec;

impl Codec<f64> for GorillaCodec {
    fn encode(&self, data: &[f64]) -> Result<Vec<u8>, KcmError> {
        if data.is_empty() {
            return Ok(vec![]);
        }
        
        // Simplified Gorilla: just store deltas of bits
        let mut result = Vec::new();
        let mut prev_bits = data[0].to_bits();
        result.extend_from_slice(&data[0].to_le_bytes());
        
        for &value in &data[1..] {
            let bits = value.to_bits();
            let xor = bits ^ prev_bits;
            
            result.extend_from_slice(&xor.to_le_bytes());
            prev_bits = bits;
        }
        
        Ok(result)
    }
    
    fn decode(&self, data: &[u8], count: usize) -> Result<Vec<f64>, KcmError> {
        if count == 0 {
            return Ok(vec![]);
        }
        
        let mut result = Vec::with_capacity(count);
        
        if data.len() < 8 {
            return Err(KcmError::Corrupted("Incomplete Gorilla data".to_string()));
        }
        
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&data[0..8]);
        let mut prev_bits = u64::from_le_bytes(bytes);
        result.push(f64::from_bits(prev_bits));
        
        let mut i = 8;
        while result.len() < count && i + 8 <= data.len() {
            bytes.copy_from_slice(&data[i..i+8]);
            let xor = u64::from_le_bytes(bytes);
            prev_bits ^= xor;
            result.push(f64::from_bits(prev_bits));
            i += 8;
        }
        
        Ok(result)
    }
}
```

### 4.3 Index Structures

```rust
// crates/kcm-storage/src/index.rs

use kcm_core::bitmap::Bitmap;
use kcm_core::types::*;
use std::collections::HashMap;

/// Bitmap index for low-cardinality columns
pub struct BitmapIndex {
    values: Vec<u8>,
    bitmaps: Vec<Bitmap>,
}

impl BitmapIndex {
    pub fn new(column: &[u8], row_count: usize) -> Result<Self, KcmError> {
        let mut value_to_bitmap: HashMap<u8, Bitmap> = HashMap::new();
        
        for (idx, &value) in column.iter().enumerate() {
            value_to_bitmap.entry(value)
                .or_insert_with(|| Bitmap::new(row_count))
                .set(idx);
        }
        
        let mut values: Vec<u8> = value_to_bitmap.keys().copied().collect();
        values.sort_unstable();
        
        let bitmaps = values.iter()
            .map(|v| value_to_bitmap.remove(v).unwrap())
            .collect();
        
        Ok(BitmapIndex { values, bitmaps })
    }
    
    pub fn lookup(&self, value: u8) -> Option<&Bitmap> {
        self.values.binary_search(&value)
            .ok()
            .and_then(|idx| self.bitmaps.get(idx))
    }
    
    pub fn range_query(&self, low: u8, high: u8) -> Result<Bitmap, KcmError> {
        let start_idx = self.values.binary_search(&low)
            .unwrap_or_else(|idx| idx);
        let end_idx = self.values.binary_search(&high)
            .map(|idx| idx + 1)
            .unwrap_or_else(|idx| idx);
        
        let mut result = Bitmap::new(self.bitmaps[0].len());
        result.clear_all();
        
        for bitmap in &self.bitmaps[start_idx..end_idx] {
            result.or_inplace(bitmap);
        }
        
        Ok(result)
    }
}

/// Zone map for range filtering
pub struct ZoneMap {
    block_size: usize,
    min_values: Vec<i64>,
    max_values: Vec<i64>,
    row_ranges: Vec<(usize, usize)>,
}

impl ZoneMap {
    pub fn new(column: &[i64], block_size: usize) -> Result<Self, KcmError> {
        let mut min_values = Vec::new();
        let mut max_values = Vec::new();
        let mut row_ranges = Vec::new();
        
        let mut i = 0;
        while i < column.len() {
            let end = (i + block_size).min(column.len());
            let block = &column[i..end];
            
            min_values.push(*block.iter().min().unwrap_or(&0));
            max_values.push(*block.iter().max().unwrap_or(&0));
            row_ranges.push((i, end));
            
            i = end;
        }
        
        Ok(ZoneMap {
            block_size,
            min_values,
            max_values,
            row_ranges,
        })
    }
    
    pub fn range_query(&self, low: i64, high: i64) -> Vec<(usize, usize)> {
        self.row_ranges.iter()
            .zip(self.min_values.iter().zip(self.max_values.iter()))
            .filter_map(|(range, (&min, &max))| {
                if max >= low && min <= high {
                    Some(*range)
                } else {
                    None
                }
            })
            .collect()
    }
}

/// Bloom filter for fast membership testing
pub struct BloomFilter {
    bits: Vec<bool>,
    num_hashes: usize,
}

impl BloomFilter {
    pub fn new(capacity: usize) -> Self {
        let bits_needed = (capacity * 10).max(1000);
        BloomFilter {
            bits: vec![false; bits_needed],
            num_hashes: 7,
        }
    }
    
    pub fn insert(&mut self, value: u32) {
        for i in 0..self.num_hashes {
            let hash = Self::hash(value, i);
            let idx = hash % self.bits.len();
            self.bits[idx] = true;
        }
    }
    
    pub fn contains(&self, value: u32) -> bool {
        for i in 0..self.num_hashes {
            let hash = Self::hash(value, i);
            let idx = hash % self.bits.len();
            if !self.bits[idx] {
                return false;
            }
        }
        true
    }
    
    fn hash(value: u32, seed: usize) -> usize {
        let combined = ((value as u64) << 32) | (seed as u64);
        let result = combined.wrapping_mul(0x9e3779b97f4a7c15);
        (result >> 32) as usize
    }
}
```

---

## PART 5: COLUMNAR REASONING ALGEBRA

### 5.1 Operator Traits & Implementations

```rust
// crates/kcm-compute/src/algebra.rs

use kcm_core::types::*;
use kcm_core::bitmap::Bitmap;
use kcm_storage::Schema;
use std::marker::PhantomData;

pub trait Operator: Send + Sync {
    fn execute(&self) -> Result<Vec<usize>, KcmError>;
    fn estimated_rows(&self) -> usize;
}

/// Scan operator - full table scan with optional filtering
pub struct ScanOp<'a> {
    schema: &'a Schema,
    context_filter: Option<u8>,
    confidence_filter: Option<f64>,
}

impl<'a> ScanOp<'a> {
    pub fn new(schema: &'a Schema) -> Self {
        ScanOp {
            schema,
            context_filter: None,
            confidence_filter: None,
        }
    }
    
    pub fn with_context(mut self, ctx: u8) -> Self {
        self.context_filter = Some(ctx);
        self
    }
    
    pub fn with_confidence(mut self, conf: f64) -> Self {
        self.confidence_filter = Some(conf);
        self
    }
}

impl<'a> Operator for ScanOp<'a> {
    fn execute(&self) -> Result<Vec<usize>, KcmError> {
        let mut result = Vec::new();
        
        for (idx, confidence) in self.schema.confidence_col.iter().enumerate() {
            if let Some(conf_filter) = self.confidence_filter {
                if *confidence < conf_filter {
                    continue;
                }
            }
            result.push(idx);
        }
        
        Ok(result)
    }
    
    fn estimated_rows(&self) -> usize {
        self.schema.len()
    }
}

/// Filter operator - apply predicates
pub enum FilterPredicate {
    EqualSubject(u32),
    EqualPredicate(u8),
    EqualObject(u32),
    EqualContext(u8),
    InSet(Vec<u32>),
    RangeTimestamp(i64, i64),
}

pub struct FilterOp<'a> {
    rowids: Vec<usize>,
    schema: &'a Schema,
    predicate: FilterPredicate,
}

impl<'a> FilterOp<'a> {
    pub fn new(rowids: Vec<usize>, schema: &'a Schema, predicate: FilterPredicate) -> Self {
        FilterOp {
            rowids,
            schema,
            predicate,
        }
    }
}

impl<'a> Operator for FilterOp<'a> {
    fn execute(&self) -> Result<Vec<usize>, KcmError> {
        let mut result = Vec::new();
        
        for &idx in &self.rowids {
            let matches = match &self.predicate {
                FilterPredicate::EqualSubject(val) => {
                    self.schema.subject_col.get(idx) == Some(*val)
                }
                FilterPredicate::EqualPredicate(val) => {
                    self.schema.predicate_col.get(idx) == Some(*val)
                }
                FilterPredicate::EqualObject(val) => {
                    self.schema.object_col.get(idx) == Some(*val)
                }
                FilterPredicate::EqualContext(val) => {
                    self.schema.context_col.get(idx) == Some(*val)
                }
                FilterPredicate::InSet(vals) => {
                    if let Some(obj) = self.schema.object_col.get(idx) {
                        vals.contains(&obj)
                    } else {
                        false
                    }
                }
                FilterPredicate::RangeTimestamp(low, high) => {
                    if let Some(ts) = self.schema.timestamp_col.get(idx) {
                        ts >= *low && ts <= *high
                    } else {
                        false
                    }
                }
            };
            
            if matches {
                result.push(idx);
            }
        }
        
        Ok(result)
    }
    
    fn estimated_rows(&self) -> usize {
        (self.rowids.len() as f64 * 0.1).ceil() as usize
    }
}

/// Project operator - select specific columns
pub struct ProjectOp<'a> {
    rowids: Vec<usize>,
    schema: &'a Schema,
    columns: Vec<ColumnID>,
}

impl<'a> ProjectOp<'a> {
    pub fn new(rowids: Vec<usize>, schema: &'a Schema, columns: Vec<ColumnID>) -> Self {
        ProjectOp {
            rowids,
            schema,
            columns,
        }
    }
}

impl<'a> Operator for ProjectOp<'a> {
    fn execute(&self) -> Result<Vec<usize>, KcmError> {
        // For now, just return rowids
        // In real implementation, extract specific columns
        Ok(self.rowids.clone())
    }
    
    fn estimated_rows(&self) -> usize {
        self.rowids.len()
    }
}

/// Join operator - hash join on column
pub struct JoinOp<'a> {
    left_rowids: Vec<usize>,
    right_rowids: Vec<usize>,
    schema: &'a Schema,
    join_column: ColumnID,
}

impl<'a> JoinOp<'a> {
    pub fn new(
        left_rowids: Vec<usize>,
        right_rowids: Vec<usize>,
        schema: &'a Schema,
        join_column: ColumnID,
    ) -> Self {
        JoinOp {
            left_rowids,
            right_rowids,
            schema,
            join_column,
        }
    }
}

impl<'a> Operator for JoinOp<'a> {
    fn execute(&self) -> Result<Vec<usize>, KcmError> {
        use std::collections::HashMap;
        
        // Build hash table from right
        let mut hash_table: HashMap<u32, Vec<usize>> = HashMap::new();
        
        for &idx in &self.right_rowids {
            if let Some(key) = self.schema.object_col.get(idx) {
                hash_table.entry(key).or_insert_with(Vec::new).push(idx);
            }
        }
        
        // Probe with left
        let mut result = Vec::new();
        
        for &idx in &self.left_rowids {
            if let Some(key) = self.schema.object_col.get(idx) {
                if let Some(matches) = hash_table.get(&key) {
                    for &right_idx in matches {
                        result.push(idx);
                        result.push(right_idx);
                    }
                }
            }
        }
        
        Ok(result)
    }
    
    fn estimated_rows(&self) -> usize {
        let selectivity = 0.1; // Assume 10% join selectivity
        (self.left_rowids.len() as f64 * self.right_rowids.len() as f64 * selectivity) as usize
    }
}

/// Aggregate operator
pub enum AggregateFunc {
    Count,
    Sum,
    Avg,
    Min,
    Max,
}

pub struct AggregateOp<'a> {
    rowids: Vec<usize>,
    schema: &'a Schema,
    group_by: Option<ColumnID>,
    agg_func: AggregateFunc,
}

impl<'a> AggregateOp<'a> {
    pub fn new(
        rowids: Vec<usize>,
        schema: &'a Schema,
        group_by: Option<ColumnID>,
        agg_func: AggregateFunc,
    ) -> Self {
        AggregateOp {
            rowids,
            schema,
            group_by,
            agg_func,
        }
    }
}

impl<'a> Operator for AggregateOp<'a> {
    fn execute(&self) -> Result<Vec<usize>, KcmError> {
        match self.agg_func {
            AggregateFunc::Count => {
                println!("Count: {}", self.rowids.len());
            }
            AggregateFunc::Sum => {
                let sum: f64 = self.rowids.iter()
                    .filter_map(|&idx| self.schema.confidence_col.get(idx))
                    .sum();
                println!("Sum: {}", sum);
            }
            _ => {
                // TODO: Implement other aggregates
            }
        }
        
        Ok(self.rowids.clone())
    }
    
    fn estimated_rows(&self) -> usize {
        if self.group_by.is_some() {
            256 // Max 256 groups for low-cardinality columns
        } else {
            1
        }
    }
}
```

### 5.2 SIMD Accelerated Operations

```rust
// crates/kcm-compute/src/simd.rs

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

pub trait SimdOps<T: Copy> {
    fn simd_filter_eq(&self, value: T) -> Vec<bool>;
    fn simd_filter_ge(&self, value: T) -> Vec<bool>;
    fn simd_count(&self) -> usize;
}

#[cfg(target_arch = "x86_64")]
impl SimdOps<u8> for [u8] {
    fn simd_filter_eq(&self, value: u8) -> Vec<bool> {
        let mut result = Vec::with_capacity(self.len());
        
        unsafe {
            let value_vec = _mm256_set1_epi8(value as i8);
            
            for chunk in self.chunks_exact(32) {
                let data = _mm256_loadu_si256(chunk.as_ptr() as *const __m256i);
                let cmp = _mm256_cmpeq_epi8(data, value_vec);
                
                for i in 0..32 {
                    result.push((_mm256_extract_epi8(&cmp, i as i32) as u8) != 0);
                }
            }
            
            // Process remainder
            for &v in &self[self.len() - (self.len() % 32)..] {
                result.push(v == value);
            }
        }
        
        result
    }
    
    fn simd_filter_ge(&self, value: u8) -> Vec<bool> {
        self.iter().map(|&v| v >= value).collect()
    }
    
    fn simd_count(&self) -> usize {
        self.len()
    }
}

// Scalar fallback
impl SimdOps<u32> for [u32] {
    fn simd_filter_eq(&self, value: u32) -> Vec<bool> {
        self.iter().map(|&v| v == value).collect()
    }
    
    fn simd_filter_ge(&self, value: u32) -> Vec<bool> {
        self.iter().map(|&v| v >= value).collect()
    }
    
    fn simd_count(&self) -> usize {
        self.len()
    }
}

impl SimdOps<f64> for [f64] {
    fn simd_filter_eq(&self, value: f64) -> Vec<bool> {
        self.iter().map(|&v| (v - value).abs() < f64::EPSILON).collect()
    }
    
    fn simd_filter_ge(&self, value: f64) -> Vec<bool> {
        self.iter().map(|&v| v >= value).collect()
    }
    
    fn simd_count(&self) -> usize {
        self.len()
    }
}
```

---

## PART 6: INFERENCE ENGINE

### 6.1 Rule Definition

```rust
// crates/kcm-reasoning/src/rule.rs

use kcm_core::types::*;
use std::fmt;

pub type RuleID = u32;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RulePattern {
    Triple(Option<SubjectID>, PredicateID, Option<ObjectID>),
    And(Box<RulePattern>, Box<RulePattern>),
    Or(Box<RulePattern>, Box<RulePattern>),
    Not(Box<RulePattern>),
}

impl RulePattern {
    pub fn subject_predicate_object(
        s: Option<SubjectID>,
        p: PredicateID,
        o: Option<ObjectID>,
    ) -> Self {
        RulePattern::Triple(s, p, o)
    }
    
    pub fn and(left: RulePattern, right: RulePattern) -> Self {
        RulePattern::And(Box::new(left), Box::new(right))
    }
    
    pub fn or(left: RulePattern, right: RulePattern) -> Self {
        RulePattern::Or(Box::new(left), Box::new(right))
    }
    
    pub fn not(pattern: RulePattern) -> Self {
        RulePattern::Not(Box::new(pattern))
    }
}

pub type ConfidenceFormula = Box<dyn Fn(&[f64]) -> f64 + Send + Sync>;

pub struct Rule {
    pub id: RuleID,
    pub name: String,
    pub description: String,
    pub pattern: RulePattern,
    pub consequent_predicate: PredicateID,
    pub confidence_formula: ConfidenceFormula,
    pub enabled: bool,
    pub priority: i32,
}

impl Rule {
    pub fn new(
        id: RuleID,
        name: String,
        pattern: RulePattern,
        consequent_predicate: PredicateID,
        confidence_formula: ConfidenceFormula,
    ) -> Self {
        Rule {
            id,
            name,
            description: String::new(),
            pattern,
            consequent_predicate,
            confidence_formula,
            enabled: true,
            priority: 0,
        }
    }
    
    pub fn with_description(mut self, desc: String) -> Self {
        self.description = desc;
        self
    }
}

/// Rule registry
pub struct RuleRegistry {
    rules: std::collections::HashMap<RuleID, Rule>,
}

impl RuleRegistry {
    pub fn new() -> Self {
        RuleRegistry {
            rules: std::collections::HashMap::new(),
        }
    }
    
    pub fn register(&mut self, rule: Rule) -> Result<(), KcmError> {
        if self.rules.contains_key(&rule.id) {
            return Err(KcmError::Conflict("Rule already exists".to_string()));
        }
        self.rules.insert(rule.id, rule);
        Ok(())
    }
    
    pub fn get(&self, id: RuleID) -> Option<&Rule> {
        self.rules.get(&id)
    }
    
    pub fn all_enabled(&self) -> Vec<&Rule> {
        self.rules.values()
            .filter(|r| r.enabled)
            .collect()
    }
}

impl Default for RuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}
```

### 6.2 Inference Engine

```rust
// crates/kcm-reasoning/src/inference.rs

use crate::rule::{Rule, RulePattern, RuleID, RuleRegistry};
use kcm_core::types::*;
use kcm_storage::Schema;
use std::collections::HashSet;

pub struct InferenceEngine {
    rule_registry: RuleRegistry,
    max_iterations: usize,
    confidence_threshold: f64,
}

impl InferenceEngine {
    pub fn new() -> Self {
        InferenceEngine {
            rule_registry: RuleRegistry::new(),
            max_iterations: 1000,
            confidence_threshold: 0.3,
        }
    }
    
    pub fn register_rule(&mut self, rule: Rule) -> Result<(), KcmError> {
        self.rule_registry.register(rule)
    }
    
    pub fn infer_forward_chaining(&self, schema: &Schema) -> Result<Vec<(Fact, RuleID)>, KcmError> {
        let mut derived_facts = Vec::new();
        let mut iteration = 0;
        
        loop {
            iteration += 1;
            if iteration > self.max_iterations {
                break;
            }
            
            let mut new_facts = Vec::new();
            
            for rule in self.rule_registry.all_enabled() {
                if !rule.enabled {
                    continue;
                }
                
                // Match patterns
                let matches = self.find_pattern_matches(&rule.pattern, schema)?;
                
                for (subject, object, confidences) in matches {
                    // Compute confidence
                    let confidence = (rule.confidence_formula)(&confidences);
                    
                    if confidence >= self.confidence_threshold {
                        let mut fact = Fact::new(subject, rule.consequent_predicate, object, confidence)?;
                        fact.priority = rule.priority as i8;
                        
                        new_facts.push((fact, rule.id));
                    }
                }
            }
            
            if new_facts.is_empty() {
                break;
            }
            
            derived_facts.extend(new_facts);
        }
        
        Ok(derived_facts)
    }
    
    fn find_pattern_matches(
        &self,
        pattern: &RulePattern,
        schema: &Schema,
    ) -> Result<Vec<(SubjectID, ObjectID, Vec<f64>)>, KcmError> {
        match pattern {
            RulePattern::Triple(subj, pred, obj) => {
                let mut matches = Vec::new();
                
                for idx in 0..schema.len() {
                    if let Some(s) = schema.subject_col.get(idx) {
                        if let Some(p) = schema.predicate_col.get(idx) {
                            if let Some(o) = schema.object_col.get(idx) {
                                if let Some(c) = schema.confidence_col.get(idx) {
                                    let s_id = SubjectID(s);
                                    let p_id = PredicateID(p);
                                    let o_id = ObjectID(o);
                                    
                                    // Check filters
                                    if let Some(subject_filter) = subj {
                                        if *subject_filter != s_id {
                                            continue;
                                        }
                                    }
                                    
                                    if *pred != p_id {
                                        continue;
                                    }
                                    
                                    if let Some(object_filter) = obj {
                                        if *object_filter != o_id {
                                            continue;
                                        }
                                    }
                                    
                                    matches.push((s_id, o_id, vec![c]));
                                }
                            }
                        }
                    }
                }
                
                Ok(matches)
            }
            
            RulePattern::And(left, right) => {
                let left_matches = self.find_pattern_matches(left, schema)?;
                let right_matches = self.find_pattern_matches(right, schema)?;
                
                let mut result = Vec::new();
                for (ls, lo, mut lc) in left_matches {
                    for (rs, ro, mut rc) in &right_matches {
                        if lo == *rs {
                            lc.append(&mut rc.clone());
                            result.push((ls, *ro, lc.clone()));
                        }
                    }
                }
                
                Ok(result)
            }
            
            RulePattern::Or(left, right) => {
                let mut left_matches = self.find_pattern_matches(left, schema)?;
                let right_matches = self.find_pattern_matches(right, schema)?;
                
                left_matches.extend(right_matches);
                Ok(left_matches)
            }
            
            RulePattern::Not(_) => {
                Err(KcmError::InvalidArgument("Negation not fully implemented".to_string()))
            }
        }
    }
}

impl Default for InferenceEngine {
    fn default() -> Self {
        Self::new()
    }
}
```

### 6.3 Confidence Calculus

```rust
// crates/kcm-reasoning/src/confidence.rs

use kcm_core::types::Confidence;

pub struct ConfidenceCalculator;

impl ConfidenceCalculator {
    /// Conjunction: A ∧ B → conf(A) × conf(B)
    pub fn conjunction(a: f64, b: f64) -> f64 {
        (a * b).min(1.0).max(0.0)
    }
    
    /// Disjunction: A ∨ B → conf(A) + conf(B) - conf(A) × conf(B)
    pub fn disjunction(a: f64, b: f64) -> f64 {
        (a + b - (a * b)).min(1.0).max(0.0)
    }
    
    /// Negation: ¬A → 1 - conf(A)
    pub fn negation(a: f64) -> f64 {
        (1.0 - a).min(1.0).max(0.0)
    }
    
    /// Combine multiple confidences (chain)
    pub fn chain(values: &[f64]) -> f64 {
        values.iter().copied().fold(1.0, |acc, v| Self::conjunction(acc, v))
    }
    
    /// Weighted combination
    pub fn weighted(values: &[f64], weights: &[f64]) -> f64 {
        assert_eq!(values.len(), weights.len());
        let numerator: f64 = values.iter().zip(weights.iter())
            .map(|(v, w)| v * w)
            .sum();
        let denominator: f64 = weights.iter().sum();
        
        if denominator == 0.0 {
            0.0
        } else {
            (numerator / denominator).min(1.0).max(0.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_confidence_operations() {
        assert!((ConfidenceCalculator::conjunction(0.5, 0.6) - 0.3).abs() < 0.0001);
        assert!((ConfidenceCalculator::disjunction(0.5, 0.6) - 0.8).abs() < 0.0001);
        assert!((ConfidenceCalculator::negation(0.7) - 0.3).abs() < 0.0001);
    }
}
```

---

## PART 7: RUNTIME & TRANSACTION LAYER

### 7.1 Transaction Management

```rust
// crates/kcm-runtime/src/transaction.rs

use kcm_core::types::*;
use kcm_storage::Schema;
use parking_lot::RwLock;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransactionState {
    Active,
    Committed,
    RolledBack,
    Aborted,
}

pub struct Transaction {
    state: TransactionState,
    changes: Vec<(usize, Fact)>,  // (row_idx, fact)
    timestamp: i64,
}

impl Transaction {
    pub fn new() -> Self {
        Transaction {
            state: TransactionState::Active,
            changes: Vec::new(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as i64,
        }
    }
    
    pub fn insert(&mut self, fact: Fact) -> Result<(), KcmError> {
        if self.state != TransactionState::Active {
            return Err(KcmError::TransactionAborted);
        }
        self.changes.push((usize::MAX, fact));  // Special value for insert
        Ok(())
    }
    
    pub fn update(&mut self, row_idx: usize, fact: Fact) -> Result<(), KcmError> {
        if self.state != TransactionState::Active {
            return Err(KcmError::TransactionAborted);
        }
        self.changes.push((row_idx, fact));
        Ok(())
    }
    
    pub fn commit(mut self) -> Result<(), KcmError> {
        self.state = TransactionState::Committed;
        Ok(())
    }
    
    pub fn rollback(mut self) -> Result<(), KcmError> {
        self.state = TransactionState::RolledBack;
        self.changes.clear();
        Ok(())
    }
    
    pub fn state(&self) -> TransactionState {
        self.state
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Self::new()
    }
}

/// MVCC version store
pub struct VersionStore {
    versions: Vec<Arc<Schema>>,
    current_version: Arc<RwLock<usize>>,
}

impl VersionStore {
    pub fn new() -> Result<Self, KcmError> {
        let initial_schema = Schema::new(1_000_000)?;
        Ok(VersionStore {
            versions: vec![Arc::new(initial_schema)],
            current_version: Arc::new(RwLock::new(0)),
        })
    }
    
    pub fn current(&self) -> Arc<Schema> {
        let idx = *self.current_version.read();
        self.versions[idx].clone()
    }
    
    pub fn create_new_version(&mut self, schema: Schema) -> Result<(), KcmError> {
        self.versions.push(Arc::new(schema));
        *self.current_version.write() = self.versions.len() - 1;
        Ok(())
    }
}

impl Default for VersionStore {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
```

### 7.2 Knowledge Database

```rust
// crates/kcm-runtime/src/database.rs

use kcm_core::types::*;
use kcm_core::dictionary::{Dictionary, SharedDictionary, DictID};
use kcm_storage::Schema;
use parking_lot::RwLock;
use std::sync::Arc;
use crate::transaction::{Transaction, VersionStore};

pub struct KnowledgeDatabase {
    schema: Arc<RwLock<Schema>>,
    dictionaries: Arc<Dictionaries>,
    version_store: Arc<RwLock<VersionStore>>,
}

pub struct Dictionaries {
    pub subjects: SharedDictionary,
    pub objects: SharedDictionary,
    pub predicates: SharedDictionary,
    pub evidence: SharedDictionary,
    pub context: SharedDictionary,
    pub owner: SharedDictionary,
}

impl KnowledgeDatabase {
    pub fn new() -> Result<Self, KcmError> {
        let schema = Arc::new(RwLock::new(Schema::new(1_000_000)?));
        let dictionaries = Arc::new(Dictionaries {
            subjects: SharedDictionary::new(),
            objects: SharedDictionary::new(),
            predicates: SharedDictionary::new(),
            evidence: SharedDictionary::new(),
            context: SharedDictionary::new(),
            owner: SharedDictionary::new(),
        });
        let version_store = Arc::new(RwLock::new(VersionStore::new()?));
        
        Ok(KnowledgeDatabase {
            schema,
            dictionaries,
            version_store,
        })
    }
    
    pub fn begin_transaction(&self) -> Transaction {
        Transaction::new()
    }
    
    pub fn insert(&self, fact: &Fact) -> Result<RowID, KcmError> {
        let mut schema = self.schema.write();
        
        schema.append_fact(fact)?;
        let row_id = RowID(schema.len() as u64 - 1);
        
        Ok(row_id)
    }
    
    pub fn insert_batch(&self, facts: &[Fact]) -> Result<Vec<RowID>, KcmError> {
        let mut schema = self.schema.write();
        let mut row_ids = Vec::new();
        
        for fact in facts {
            schema.append_fact(fact)?;
            row_ids.push(RowID(schema.len() as u64 - 1));
        }
        
        Ok(row_ids)
    }
    
    pub fn query(&self) -> QueryBuilder {
        QueryBuilder::new(self.schema.read().clone())
    }
    
    pub fn get_fact(&self, row_id: RowID) -> Result<Option<Fact>, KcmError> {
        let schema = self.schema.read();
        Ok(schema.get_fact(row_id.as_usize()))
    }
    
    pub fn dict_insert_subject(&self, name: &str) -> DictID {
        self.dictionaries.subjects.insert(name)
    }
    
    pub fn dict_get_subject(&self, id: DictID) -> Option<String> {
        self.dictionaries.subjects.get(id)
    }
    
    pub fn dict_lookup_subject(&self, name: &str) -> Option<DictID> {
        self.dictionaries.subjects.lookup(name)
    }
    
    pub fn fact_count(&self) -> usize {
        self.schema.read().len()
    }
}

impl Default for KnowledgeDatabase {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

pub struct QueryBuilder {
    schema: Schema,
    subject_filter: Option<SubjectID>,
    predicate_filter: Option<PredicateID>,
    object_filter: Option<ObjectID>,
    confidence_filter: Option<f64>,
}

impl QueryBuilder {
    pub fn new(schema: Schema) -> Self {
        QueryBuilder {
            schema,
            subject_filter: None,
            predicate_filter: None,
            object_filter: None,
            confidence_filter: None,
        }
    }
    
    pub fn with_subject(mut self, subject: SubjectID) -> Self {
        self.subject_filter = Some(subject);
        self
    }
    
    pub fn with_predicate(mut self, predicate: PredicateID) -> Self {
        self.predicate_filter = Some(predicate);
        self
    }
    
    pub fn with_object(mut self, object: ObjectID) -> Self {
        self.object_filter = Some(object);
        self
    }
    
    pub fn with_confidence(mut self, threshold: f64) -> Self {
        self.confidence_filter = Some(threshold);
        self
    }
    
    pub fn execute(self) -> Result<Vec<Fact>, KcmError> {
        let mut result = Vec::new();
        
        for idx in 0..self.schema.len() {
            if let Some(fact) = self.schema.get_fact(idx) {
                let mut matches = true;
                
                if let Some(subj) = self.subject_filter {
                    if fact.subject != subj {
                        matches = false;
                    }
                }
                
                if let Some(pred) = self.predicate_filter {
                    if fact.predicate != pred {
                        matches = false;
                    }
                }
                
                if let Some(obj) = self.object_filter {
                    if fact.object != obj {
                        matches = false;
                    }
                }
                
                if let Some(conf_threshold) = self.confidence_filter {
                    if fact.confidence < conf_threshold {
                        matches = false;
                    }
                }
                
                if matches {
                    result.push(fact);
                }
            }
        }
        
        Ok(result)
    }
}
```

### 7.3 Executor with Thread Pool

```rust
// crates/kcm-runtime/src/executor.rs

use rayon::ThreadPool;
use std::sync::Arc;
use kcm_core::types::*;

pub struct Executor {
    thread_pool: ThreadPool,
}

impl Executor {
    pub fn new(num_threads: usize) -> Result<Self, KcmError> {
        let thread_pool = rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build()
            .map_err(|e| KcmError::Io(format!("Failed to build thread pool: {}", e)))?;
        
        Ok(Executor { thread_pool })
    }
    
    pub fn with_num_cpus() -> Result<Self, KcmError> {
        let num_cpus = num_cpus::get();
        Self::new(num_cpus)
    }
    
    pub fn num_threads(&self) -> usize {
        self.thread_pool.current_num_threads()
    }
    
    pub fn parallel_map<T, F, R>(&self, items: Vec<T>, f: F) -> Vec<R>
    where
        T: Send,
        F: Fn(T) -> R + Send + Sync,
        R: Send,
    {
        self.thread_pool.install(|| {
            items.into_par_iter()
                .map(f)
                .collect()
        })
    }
    
    pub fn parallel_filter<T, F>(&self, items: Vec<T>, f: F) -> Vec<T>
    where
        T: Send,
        F: Fn(&T) -> bool + Send + Sync,
    {
        self.thread_pool.install(|| {
            items.into_par_iter()
                .filter(f)
                .collect()
        })
    }
}

use rayon::prelude::*;

impl Default for Executor {
    fn default() -> Self {
        Self::with_num_cpus().unwrap()
    }
}
```

---

## PART 8: C API & FOREIGN FUNCTION INTERFACE

### 8.1 Safe C Wrapper

```rust
// crates/kcm-interface/src/lib.rs

use kcm_runtime::database::KnowledgeDatabase;
use kcm_core::types::*;
use std::sync::Arc;
use parking_lot::Mutex;
use std::ptr;
use std::ffi::CStr;
use std::os::raw::c_char;

// Opaque types for C API
pub struct KCM_Database {
    inner: Arc<Mutex<KnowledgeDatabase>>,
}

pub struct KCM_Transaction {
    inner: Arc<Mutex<crate::transaction::Transaction>>,
}

pub struct KCM_Query {
    inner: Vec<Fact>,
    position: usize,
}

#[repr(C)]
pub struct KCM_Fact {
    pub subject: u32,
    pub predicate: u8,
    pub object: u32,
    pub confidence: f64,
    pub evidence: u8,
    pub timestamp: i64,
    pub context: u8,
}

impl From<&Fact> for KCM_Fact {
    fn from(fact: &Fact) -> Self {
        KCM_Fact {
            subject: fact.subject.0,
            predicate: fact.predicate.0,
            object: fact.object.0,
            confidence: fact.confidence,
            evidence: fact.evidence.0,
            timestamp: fact.timestamp,
            context: fact.context.0,
        }
    }
}

impl From<&KCM_Fact> for Fact {
    fn from(kcm_fact: &KCM_Fact) -> Self {
        Fact {
            subject: SubjectID(kcm_fact.subject),
            predicate: PredicateID(kcm_fact.predicate),
            object: ObjectID(kcm_fact.object),
            confidence: kcm_fact.confidence,
            evidence: EvidenceID(kcm_fact.evidence),
            timestamp: kcm_fact.timestamp,
            context: ContextID(kcm_fact.context),
            version: 1,
            priority: 0,
            owner: 0,
        }
    }
}

#[repr(C)]
pub enum KCM_Error {
    KCM_OK = 0,
    KCM_ERR_NOT_FOUND = 1,
    KCM_ERR_OUT_OF_MEMORY = 2,
    KCM_ERR_INVALID_ARGUMENT = 3,
    KCM_ERR_IO = 4,
    KCM_ERR_CORRUPTED = 5,
    KCM_ERR_CONFLICT = 6,
    KCM_ERR_TRANSACTION_ABORTED = 7,
}

impl From<kcm_core::types::KcmError> for KCM_Error {
    fn from(err: kcm_core::types::KcmError) -> Self {
        match err {
            kcm_core::types::KcmError::NotFound(_) => KCM_Error::KCM_ERR_NOT_FOUND,
            kcm_core::types::KcmError::OutOfMemory => KCM_Error::KCM_ERR_OUT_OF_MEMORY,
            kcm_core::types::KcmError::InvalidArgument(_) => KCM_Error::KCM_ERR_INVALID_ARGUMENT,
            kcm_core::types::KcmError::Io(_) => KCM_Error::KCM_ERR_IO,
            kcm_core::types::KcmError::Corrupted(_) => KCM_Error::KCM_ERR_CORRUPTED,
            kcm_core::types::KcmError::Conflict(_) => KCM_Error::KCM_ERR_CONFLICT,
            kcm_core::types::KcmError::TransactionAborted => KCM_Error::KCM_ERR_TRANSACTION_ABORTED,
        }
    }
}

#[no_mangle]
pub extern "C" fn KCM_DatabaseNew(db_out: *mut *mut KCM_Database) -> KCM_Error {
    if db_out.is_null() {
        return KCM_Error::KCM_ERR_INVALID_ARGUMENT;
    }
    
    match KnowledgeDatabase::new() {
        Ok(kb) => {
            unsafe {
                *db_out = Box::into_raw(Box::new(KCM_Database {
                    inner: Arc::new(Mutex::new(kb)),
                }));
            }
            KCM_Error::KCM_OK
        }
        Err(e) => e.into(),
    }
}

#[no_mangle]
pub extern "C" fn KCM_DatabaseFree(db: *mut KCM_Database) {
    if !db.is_null() {
        unsafe {
            Box::from_raw(db);
        }
    }
}

#[no_mangle]
pub extern "C" fn KCM_DatabaseInsert(
    db: *mut KCM_Database,
    fact: *const KCM_Fact,
) -> KCM_Error {
    if db.is_null() || fact.is_null() {
        return KCM_Error::KCM_ERR_INVALID_ARGUMENT;
    }
    
    unsafe {
        let db = &*db;
        let fact_ref = &*fact;
        let kcm_fact = Fact::from(fact_ref);
        
        match db.inner.lock().insert(&kcm_fact) {
            Ok(_) => KCM_Error::KCM_OK,
            Err(e) => e.into(),
        }
    }
}

#[no_mangle]
pub extern "C" fn KCM_DatabaseQuery(
    db: *mut KCM_Database,
    query_out: *mut *mut KCM_Query,
) -> KCM_Error {
    if db.is_null() || query_out.is_null() {
        return KCM_Error::KCM_ERR_INVALID_ARGUMENT;
    }
    
    unsafe {
        let db = &*db;
        let kb = db.inner.lock();
        
        match kb.query().execute() {
            Ok(facts) => {
                *query_out = Box::into_raw(Box::new(KCM_Query {
                    inner: facts,
                    position: 0,
                }));
                KCM_Error::KCM_OK
            }
            Err(e) => e.into(),
        }
    }
}

#[no_mangle]
pub extern "C" fn KCM_QueryNext(
    query: *mut KCM_Query,
    fact_out: *mut KCM_Fact,
    has_next: *mut bool,
) -> KCM_Error {
    if query.is_null() || fact_out.is_null() || has_next.is_null() {
        return KCM_Error::KCM_ERR_INVALID_ARGUMENT;
    }
    
    unsafe {
        let query_ref = &mut *query;
        
        if query_ref.position < query_ref.inner.len() {
            let fact = &query_ref.inner[query_ref.position];
            *fact_out = KCM_Fact::from(fact);
            query_ref.position += 1;
            *has_next = query_ref.position < query_ref.inner.len();
            KCM_Error::KCM_OK
        } else {
            *has_next = false;
            KCM_Error::KCM_OK
        }
    }
}

#[no_mangle]
pub extern "C" fn KCM_QueryFree(query: *mut KCM_Query) {
    if !query.is_null() {
        unsafe {
            Box::from_raw(query);
        }
    }
}

#[no_mangle]
pub extern "C" fn KCM_ErrorMessage(err: KCM_Error) -> *const c_char {
    let msg = match err {
        KCM_Error::KCM_OK => "OK",
        KCM_Error::KCM_ERR_NOT_FOUND => "Not found",
        KCM_Error::KCM_ERR_OUT_OF_MEMORY => "Out of memory",
        KCM_Error::KCM_ERR_INVALID_ARGUMENT => "Invalid argument",
        KCM_Error::KCM_ERR_IO => "I/O error",
        KCM_Error::KCM_ERR_CORRUPTED => "Data corrupted",
        KCM_Error::KCM_ERR_CONFLICT => "Conflict",
        KCM_Error::KCM_ERR_TRANSACTION_ABORTED => "Transaction aborted",
    };
    
    msg.as_ptr() as *const c_char
}
```

---

## PART 9: TESTING & BENCHMARKS

### 9.1 Unit Tests

```rust
// crates/kcm-core/tests/test_core.rs

#[cfg(test)]
mod tests {
    use kcm_core::types::*;
    use kcm_core::vec::DenseVec;
    use kcm_core::bitmap::Bitmap;
    use kcm_core::dictionary::Dictionary;
    
    #[test]
    fn test_types() {
        let subject = SubjectID::new(42);
        assert_eq!(subject.0, 42);
        
        let confidence = Confidence::new(0.75).unwrap();
        assert_eq!(confidence.0, 0.75);
        
        // Test confidence bounds
        assert!(Confidence::new(1.5).is_err());
        assert!(Confidence::new(f64::NAN).is_err());
    }
    
    #[test]
    fn test_dense_vec() {
        let mut vec = DenseVec::<u32>::new(100).unwrap();
        
        vec.push(42).unwrap();
        vec.push(43).unwrap();
        
        assert_eq!(vec.len(), 2);
        assert_eq!(vec[0], 42);
        assert_eq!(vec[1], 43);
    }
    
    #[test]
    fn test_bitmap() {
        let mut bitmap = Bitmap::new(256);
        
        bitmap.set(0);
        bitmap.set(100);
        bitmap.set(255);
        
        assert!(bitmap.get(0));
        assert!(bitmap.get(100));
        assert!(bitmap.get(255));
        assert!(!bitmap.get(1));
        
        assert_eq!(bitmap.count_ones(), 3);
    }
    
    #[test]
    fn test_dictionary() {
        let mut dict = Dictionary::new();
        
        let id1 = dict.insert("hello");
        let id2 = dict.insert("world");
        let id1_again = dict.insert("hello");
        
        assert_eq!(id1, id1_again);
        assert_ne!(id1, id2);
        
        assert_eq!(dict.get(id1), Some("hello"));
        assert_eq!(dict.get(id2), Some("world"));
    }
}
```

### 9.2 Integration Tests

```rust
// crates/kcm-runtime/tests/test_integration.rs

#[cfg(test)]
mod tests {
    use kcm_runtime::database::KnowledgeDatabase;
    use kcm_core::types::*;
    
    #[test]
    fn test_insert_query() {
        let kb = KnowledgeDatabase::new().unwrap();
        
        let fact = Fact::new(
            SubjectID(1),
            PredicateID(0),
            ObjectID(2),
            0.9,
        ).unwrap();
        
        let _row_id = kb.insert(&fact).unwrap();
        
        assert_eq!(kb.fact_count(), 1);
        
        let results = kb.query()
            .with_subject(SubjectID(1))
            .execute()
            .unwrap();
        
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].subject, SubjectID(1));
    }
    
    #[test]
    fn test_transaction() {
        let kb = KnowledgeDatabase::new().unwrap();
        
        let mut txn = kb.begin_transaction();
        
        let fact = Fact::new(
            SubjectID(10),
            PredicateID(5),
            ObjectID(20),
            0.8,
        ).unwrap();
        
        txn.insert(fact).unwrap();
        txn.commit().unwrap();
        
        assert_eq!(txn.state(), crate::transaction::TransactionState::Committed);
    }
    
    #[test]
    fn test_inference() {
        use kcm_reasoning::inference::InferenceEngine;
        use kcm_reasoning::rule::{Rule, RulePattern};
        use kcm_storage::Schema;
        
        let mut engine = InferenceEngine::new();
        
        let rule = Rule::new(
            1,
            "test_rule".to_string(),
            RulePattern::subject_predicate_object(None, PredicateID(0), None),
            PredicateID(1),
            Box::new(|confs| confs[0] * 0.9),
        );
        
        engine.register_rule(rule).unwrap();
        
        let mut schema = Schema::new(100).unwrap();
        let fact = Fact::new(
            SubjectID(1),
            PredicateID(0),
            ObjectID(2),
            0.9,
        ).unwrap();
        schema.append_fact(&fact).unwrap();
        
        let derived = engine.infer_forward_chaining(&schema).unwrap();
        assert!(!derived.is_empty());
    }
}
```

### 9.3 Benchmarks

```rust
// benches/micro.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use kcm_core::vec::DenseVec;
use kcm_core::bitmap::Bitmap;
use kcm_runtime::database::KnowledgeDatabase;
use kcm_core::types::*;

fn bench_dense_vec_allocation(c: &mut Criterion) {
    c.bench_function("dense_vec_1m_allocation", |b| {
        b.iter(|| {
            DenseVec::<u64>::new(1_000_000).unwrap()
        });
    });
}

fn bench_bitmap_operations(c: &mut Criterion) {
    c.bench_function("bitmap_set_1m", |b| {
        let mut bitmap = black_box(Bitmap::new(1_000_000));
        b.iter(|| {
            for i in 0..1_000_000 {
                bitmap.set(i);
            }
        });
    });
}

fn bench_insert_query(c: &mut Criterion) {
    c.bench_function("insert_1k_facts", |b| {
        b.iter(|| {
            let kb = KnowledgeDatabase::new().unwrap();
            for i in 0..1000 {
                let fact = Fact::new(
                    SubjectID(i % 100),
                    PredicateID((i % 10) as u8),
                    ObjectID((i % 200) as u32),
                    0.5 + (i as f64 % 0.5),
                ).unwrap();
                kb.insert(&fact).unwrap();
            }
        });
    });
}

fn bench_query(c: &mut Criterion) {
    c.bench_function("query_1k_facts", |b| {
        let kb = KnowledgeDatabase::new().unwrap();
        for i in 0..1000 {
            let fact = Fact::new(
                SubjectID(i % 100),
                PredicateID((i % 10) as u8),
                ObjectID((i % 200) as u32),
                0.75,
            ).unwrap();
            kb.insert(&fact).unwrap();
        }
        
        b.iter(|| {
            kb.query().execute().unwrap()
        });
    });
}

criterion_group!(
    benches,
    bench_dense_vec_allocation,
    bench_bitmap_operations,
    bench_insert_query,
    bench_query
);

criterion_main!(benches);
```

---

## PART 10: BUILD & DEPLOYMENT

### 10.1 Cargo Configuration

```toml
# Cargo.toml (root)

[workspace]
members = [
    "crates/kcm-core",
    "crates/kcm-storage",
    "crates/kcm-compute",
    "crates/kcm-reasoning",
    "crates/kcm-optimizer",
    "crates/kcm-runtime",
    "crates/kcm-interface",
]

resolver = "2"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true

[profile.bench]
inherits = "release"
```

### 10.2 Build Script

```bash
#!/bin/bash
# scripts/build.sh

set -e

echo "Building KCM..."

# Build all crates
cargo build --release --workspace

# Build with SIMD optimizations
RUSTFLAGS="-C target-cpu=native -C target-feature=+avx2" \
    cargo build --release --workspace

# Run tests
cargo test --release --all

# Run benchmarks
cargo bench --all

echo "Build complete!"
```

### 10.3 Cargo Test Script

```bash
#!/bin/bash
# scripts/test.sh

set -e

echo "Running tests..."

# Unit tests
cargo test --lib --all

# Integration tests
cargo test --test '*' --all

# Doc tests
cargo test --doc --all

# Check code coverage (requires tarpaulin)
cargo tarpaulin --out Html --output-dir coverage/

echo "Tests complete! Coverage report: coverage/index.html"
```

---

## PART 11: PERFORMANCE SPECIFICATION (RUST-SPECIFIC)

### 11.1 Memory Layout Optimization

```rust
// Example: Cache-line aligned structures

#[repr(C, align(64))]
struct CacheAlignedData {
    data: u64,
    padding: [u64; 7],
}

// Use in DenseVec with 64-byte alignment
impl DenseVec<CacheAlignedData> {
    pub fn new_cache_aligned(capacity: usize) -> Result<Self, String> {
        Self::with_alignment(capacity, 64)
    }
}
```

### 11.2 Zero-Copy Operations

```rust
// DenseVec provides zero-copy access via as_slice()
fn process_facts(vec: &DenseVec<u32>) {
    // No copying, direct slice access
    let slice: &[u32] = vec.as_slice();
    
    // Can be passed to SIMD functions
    // Can be memory-mapped from disk
}
```

### 11.3 Determinism Guarantees

```rust
// Rust guarantees:
// 1. No garbage collection → deterministic timing
// 2. Ownership model → no data races
// 3. LLVM optimizations → same assembly from same source
// 4. No undefined behavior (unsafe code isolated)

// Test for determinism:
#[test]
fn test_determinism() {
    let kb = KnowledgeDatabase::new().unwrap();
    
    // Insert same data
    for i in 0..100 {
        let fact = Fact::new(
            SubjectID(i),
            PredicateID(0),
            ObjectID(i * 2),
            0.5 + (i as f64 * 0.001),
        ).unwrap();
        kb.insert(&fact).unwrap();
    }
    
    // Run query 100 times
    let mut results_history = Vec::new();
    for _ in 0..100 {
        let results = kb.query().execute().unwrap();
        results_history.push(results);
    }
    
    // Verify all identical
    for i in 1..100 {
        assert_eq!(results_history[0], results_history[i]);
    }
}
```

---

## PART 12: ADVANCED FEATURES

### 12.1 Async/Await Support (Optional)

```rust
// crates/kcm-runtime/src/async_executor.rs

use tokio::task;
use kcm_core::types::*;

pub struct AsyncExecutor {
    runtime: tokio::runtime::Runtime,
}

impl AsyncExecutor {
    pub fn new() -> Result<Self, KcmError> {
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| KcmError::Io(e.to_string()))?;
        
        Ok(AsyncExecutor { runtime })
    }
    
    pub fn block_on<F>(&self, f: F) -> F::Output
    where
        F: std::future::Future,
    {
        self.runtime.block_on(f)
    }
}

// Example: Async query
pub async fn async_query(kb: &KnowledgeDatabase) -> Result<Vec<Fact>, KcmError> {
    task::spawn_blocking(|| {
        kb.query().execute()
    })
    .await
    .map_err(|e| KcmError::Io(e.to_string()))?
}
```

### 12.2 Serialization (serde)

```rust
// crates/kcm-core/Cargo.toml additions

[dependencies]
serde = { version = "1.0", features = ["derive"], optional = true }
serde_json = { version = "1.0", optional = true }

[features]
serialization = ["serde", "serde_json"]

# In types.rs

#[cfg_attr(feature = "serialization", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Fact {
    pub subject: SubjectID,
    pub predicate: PredicateID,
    pub object: ObjectID,
    pub confidence: f64,
    pub evidence: EvidenceID,
    pub timestamp: i64,
    pub context: ContextID,
    pub version: i32,
    pub priority: i8,
    pub owner: u16,
}
```

### 12.3 Python Bindings (PyO3)

```rust
// crates/kcm-interface/Cargo.toml additions

[dependencies]
pyo3 = { version = "0.20", features = ["extension-module"] }

# crates/kcm-interface/src/python.rs

use pyo3::prelude::*;
use kcm_runtime::database::KnowledgeDatabase;
use kcm_core::types::*;

#[pyclass]
pub struct PyKnowledgeBase {
    kb: KnowledgeDatabase,
}

#[pymethods]
impl PyKnowledgeBase {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(PyKnowledgeBase {
            kb: KnowledgeDatabase::new()
                .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?,
        })
    }
    
    fn insert(&self, subject: u32, predicate: u8, object: u32, confidence: f64) -> PyResult<()> {
        let fact = Fact::new(
            SubjectID(subject),
            PredicateID(predicate),
            ObjectID(object),
            confidence,
        ).map_err(|e| pyo3::exceptions::PyException::new_err(e))?;
        
        self.kb.insert(&fact)
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?;
        
        Ok(())
    }
    
    fn fact_count(&self) -> usize {
        self.kb.fact_count()
    }
}

#[pymodule]
fn kcm(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyKnowledgeBase>()?;
    Ok(())
}
```

---

## PART 13: DEPLOYMENT CHECKLIST

### Pre-Release

- [ ] All tests passing (cargo test --all)
- [ ] No warnings (cargo clippy --all)
- [ ] Code formatted (cargo fmt --all)
- [ ] Benchmarks stable (cargo bench)
- [ ] Documentation complete (cargo doc)
- [ ] Security audit (cargo audit)
- [ ] MIRI checks (cargo +nightly miri test)
- [ ] Fuzzing passed (24+ hours)

### Release

- [ ] Version bumped (Cargo.toml)
- [ ] CHANGELOG updated
- [ ] Git tag created (v0.1.0)
- [ ] Publish to crates.io (cargo publish)
- [ ] Documentation published (docs.rs)
- [ ] GitHub release created

### Post-Release

- [ ] Monitor crates.io downloads
- [ ] Track GitHub issues
- [ ] Respond to community feedback
- [ ] Plan next release

---

## PART 14: SUCCESS CRITERIA

### Functional

✓ Insert, Query, Update, Delete all work
✓ Transactions with ACID semantics
✓ Inference engine with rule execution
✓ Confidence calculus (multiply, combine)
✓ Deterministic execution
✓ Dictionary encoding
✓ Bitmap indices
✓ Compression

### Performance (Target per implementation phase)

Phase 1:
- [ ] 1M fact scan < 100ms
- [ ] Dictionary lookup < 1µs
- [ ] Bitmap filter < 10ms

Phase 2:
- [ ] Join 2×1M facts < 50ms
- [ ] Inference 10 rules < 100ms
- [ ] Memory < 100MB per 1M facts

Phase 3:
- [ ] 10M facts in < 1GB memory
- [ ] SIMD speedup > 4x
- [ ] Parallel scaling > 8x on 16 cores

### Quality

- [ ] 95%+ test coverage
- [ ] 0 unsafe code in public API
- [ ] Documentation complete
- [ ] CI/CD pipeline automated
- [ ] Fuzzing 24+ hours
- [ ] Load test 1 week continuous

---

**END OF RUST TECHNICAL PRD**

Rust-based Knowledge Columnar Model adalah sistem pengetahuan berbasis kolom yang memberikan performance tinggi, memory safety, dan explainability built-in. Dioptimalkan untuk production deployments dengan zero runtime overhead dan predictable behavior.
