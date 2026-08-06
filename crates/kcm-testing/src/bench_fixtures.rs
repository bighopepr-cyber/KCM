//! Shared benchmark fixtures for KCM.
//!
//! This module provides canonical, deterministic, and reproducible dataset
//! generators that are shared across all benchmark suites. Every benchmark
//! must use these fixtures instead of generating its own data.
//!
//! # Invariants
//!
//! All generated data satisfies KCM type system constraints:
//! - Confidence values: `[0.0, 1.0)` (exclusive of 1.0 to avoid edge cases)
//! - SubjectID: `u32` in `[0, subject_range)`
//! - PredicateID: `u8` in `[0, predicate_range)`
//! - ObjectID: `u32` in `[0, object_range)`
//! - EvidenceID: always `EvidenceID::UNKNOWN`
//! - ContextID: always `ContextID::NULL`
//! - Version: always `1`
//! - Priority: always `0`
//! - Owner: always `0`
//!
//! # Determinism
//!
//! All fixtures use deterministic algorithms (modular arithmetic).
//! No randomness, no timestamps, no environmental dependencies.
//! The same fixture parameters always produce identical datasets.

use kcm_core::bitmap::Bitmap;
use kcm_core::dictionary::Dictionary;
use kcm_core::types::*;
use kcm_core::vec::DenseVec;
use kcm_runtime::database::KnowledgeDatabase;
use kcm_storage::column::Schema;
use std::sync::Arc;

/// Configuration for generating deterministic benchmark datasets.
/// All fields are documented and validated against KCM type constraints.
#[derive(Debug, Clone)]
pub struct DatasetConfig {
    /// Number of facts to generate. Must be > 0.
    pub fact_count: usize,
    /// Number of distinct subjects. Must be in [1, u32::MAX].
    pub subject_range: u32,
    /// Number of distinct predicates. Must be in [1, 256].
    pub predicate_range: u8,
    /// Number of distinct objects. Must be in [1, u32::MAX].
    pub object_range: u32,
    /// Base confidence value. Result is clamped to [0.0, 1.0).
    pub base_confidence: f64,
    /// Confidence increment per fact (result is clamped).
    pub confidence_step: f64,
}

impl DatasetConfig {
    /// Create a config for a given fact count with safe defaults.
    ///
    /// Defaults:
    /// - subject_range: 100
    /// - predicate_range: 10
    /// - object_range: 200
    /// - base_confidence: 0.5
    /// - confidence_step: 0.0001
    pub fn for_count(fact_count: usize) -> Self {
        DatasetConfig {
            fact_count,
            subject_range: 100,
            predicate_range: 10,
            object_range: 200,
            base_confidence: 0.5,
            confidence_step: 0.0001,
        }
    }

    /// Validate that all parameters are within KCM type constraints.
    pub fn validate(&self) -> Result<(), String> {
        if self.fact_count == 0 {
            return Err("fact_count must be > 0".into());
        }
        if self.subject_range == 0 {
            return Err("subject_range must be > 0".into());
        }
        if self.predicate_range == 0 {
            return Err("predicate_range must be > 0".into());
        }
        if self.object_range == 0 {
            return Err("object_range must be > 0".into());
        }
        if self.base_confidence < 0.0 || self.base_confidence >= 1.0 {
            return Err(format!(
                "base_confidence must be in [0.0, 1.0), got {}",
                self.base_confidence
            ));
        }
        if self.confidence_step < 0.0 {
            return Err("confidence_step must be >= 0.0".into());
        }
        Ok(())
    }

    /// Compute confidence for a given fact index, guaranteed in [0.0, 1.0).
    fn confidence_for_index(&self, index: usize) -> f64 {
        let raw = self.base_confidence + (index as f64 * self.confidence_step);
        raw.fract()
    }
}

/// Pre-computed dense vector of u32 values.
/// Deterministic: same config always produces same data.
/// Used for column sequential scan, random access, and push benchmarks.
pub struct ColumnFixture {
    pub data: DenseVec<u32>,
}

impl ColumnFixture {
    pub fn new(size: usize) -> Self {
        let mut data = DenseVec::new(size).unwrap();
        for i in 0..size {
            data.push(i as u32).unwrap();
        }
        ColumnFixture { data }
    }
}

/// Pre-computed dense vector of u8 values for SIMD filter benchmarks.
pub struct U8ColumnFixture {
    pub data: DenseVec<u8>,
}

impl U8ColumnFixture {
    /// Creates a column where `data[i] = i % 256`.
    /// Filter target value should be < 256.
    pub fn new(size: usize) -> Self {
        let mut data = DenseVec::new(size).unwrap();
        for i in 0..size {
            data.push((i % 256) as u8).unwrap();
        }
        U8ColumnFixture { data }
    }
}

/// Pre-computed bitmap with deterministic density.
/// Density = number_of_set_bits / total_bits.
pub struct BitmapFixture {
    pub bitmap: Bitmap,
    pub density_bits_per: usize,
}

impl BitmapFixture {
    /// Create a bitmap where every `step`-th bit is set.
    pub fn new(size: usize, step: usize) -> Self {
        let mut bitmap = Bitmap::new(size);
        let mut count = 0;
        for i in (0..size).step_by(step) {
            bitmap.set(i);
            count += 1;
        }
        BitmapFixture {
            bitmap,
            density_bits_per: count,
        }
    }
}

/// Pre-computed DenseVec<u64> for memory allocation benchmarks.
pub struct DenseVecU64Fixture {
    pub data: DenseVec<u64>,
}

impl DenseVecU64Fixture {
    pub fn new(size: usize) -> Self {
        let mut data = DenseVec::new(size).unwrap();
        for i in 0..size as u64 {
            data.push(i).unwrap();
        }
        DenseVecU64Fixture { data }
    }
}

/// Pre-computed dictionary with deterministic entries.
pub struct DictionaryFixture {
    pub dict: Dictionary,
}

impl DictionaryFixture {
    pub fn new(size: usize) -> Self {
        let mut dict = Dictionary::new();
        for i in 0..size {
            dict.insert(&format!("key_{}", i)).unwrap();
        }
        DictionaryFixture { dict }
    }
}

/// A pre-populated Schema with deterministic facts.
///
/// Capacity is allocated as `fact_count` — exactly enough for the initial
/// facts. Benchmarks that call mutating operations (like `infer_forward_chaining`
/// which appends derived facts) must rebuild the schema inside each `b.iter()`
/// from a pre-stored `Vec<Fact>` to avoid schema mutation accumulating across
/// Criterion iterations.
pub struct SchemaFixture {
    pub schema: Schema,
}

impl SchemaFixture {
    /// Create a schema fixture for benchmarking.
    ///
    /// Allocates capacity exactly equal to `fact_count`. If a benchmark needs
    /// to call mutating operations that append rows, it must rebuild the schema
    /// per iteration from a stored `Vec<Fact>` rather than reusing this fixture.
    pub fn new(config: &DatasetConfig) -> Self {
        config.validate().expect("Invalid dataset config");
        let capacity = config.fact_count.max(1);
        let mut schema = Schema::new(capacity).expect("Schema allocation failed");
        for i in 0..config.fact_count {
            let fact = deterministic_fact(i, config);
            schema
                .append_fact(&fact)
                .expect("Failed to append initial fact");
        }
        SchemaFixture { schema }
    }
}

/// A pre-populated KnowledgeDatabase with deterministic facts.
pub struct DatabaseFixture {
    pub kb: Arc<KnowledgeDatabase>,
    pub config: DatasetConfig,
}

impl DatabaseFixture {
    pub fn new(config: &DatasetConfig) -> Self {
        config.validate().expect("Invalid dataset config");
        let kb = KnowledgeDatabase::new().unwrap();
        for i in 0..config.fact_count {
            let fact = deterministic_fact(i, config);
            kb.insert(&fact).unwrap();
        }
        DatabaseFixture {
            kb: Arc::new(kb),
            config: config.clone(),
        }
    }
}

/// Pre-computed compressed data fixture.
pub struct CompressionFixture {
    pub zstd_compressed: Vec<u8>,
    pub lz4_compressed: Vec<u8>,
    pub original_size: usize,
}

impl CompressionFixture {
    pub fn new(size: usize) -> Self {
        use kcm_storage::compress::{Compressor, Lz4Compressor, ZstdCompressor};
        let data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
        let zstd_compressed = ZstdCompressor::default_level().compress(&data).unwrap();
        let lz4_compressed = Lz4Compressor::default_level().compress(&data).unwrap();
        CompressionFixture {
            zstd_compressed,
            lz4_compressed,
            original_size: size,
        }
    }
}

/// Self-contained WAL benchmark fixture.
///
/// Owns the complete lifecycle of benchmark resources:
/// - Temporary directory (auto-cleaned via `TempDir`)
/// - WAL creation, population, flushing, and file-handle closure
/// - Deterministic dataset generation
/// - Pre-replay validation (file exists, size > 0, entry count)
///
/// The fixture is designed so that every WAL-related benchmark can use it
/// without implementing its own setup logic. The `TempDir` is stored in
/// the struct to keep the directory (and WAL file) alive for the lifetime
/// of the benchmark.
///
/// # Lifecycle
///
/// ```text
/// WalBenchmarkFixture::new(config)
///   ├── create TempDir
///   ├── create WAL file
///   ├── populate with deterministic facts
///   ├── flush + sync + close file handles
///   ├── validate: file exists, size > 0, replayable
///   └── return fixture (TempDir stays alive)
///
/// Drop(fixture)
///   └── TempDir::drop → deletes temp directory + WAL file
/// ```
pub struct WalBenchmarkFixture {
    _dir: tempfile::TempDir,
    pub wal_path: std::path::PathBuf,
    pub entry_count: usize,
}

impl WalBenchmarkFixture {
    /// Create a fully populated and validated WAL fixture.
    ///
    /// All I/O happens here — inside the constructor, outside `b.iter()`.
    /// Returns a descriptive panic on any failure.
    pub fn new(config: &DatasetConfig) -> Self {
        config
            .validate()
            .unwrap_or_else(|e| panic!("WalBenchmarkFixture: invalid config: {}", e));

        let dir = tempfile::tempdir()
            .unwrap_or_else(|e| panic!("WalBenchmarkFixture: failed to create temp dir: {}", e));
        let wal_path = dir.path().join("bench.wal");

        // --- Phase 1: Create and populate the WAL ---
        {
            let wal = kcm_storage::wal::WriteAheadLog::new(&wal_path).unwrap_or_else(|e| {
                panic!(
                    "WalBenchmarkFixture: failed to create WAL at {:?}: {}",
                    wal_path, e
                )
            });
            for i in 0..config.fact_count {
                let fact = deterministic_fact(i, config);
                wal.append_fact(&fact).unwrap_or_else(|e| {
                    panic!(
                        "WalBenchmarkFixture: failed to append fact {} to WAL: {}",
                        i, e
                    )
                });
            }
            wal.flush_buffer().unwrap_or_else(|e| {
                panic!("WalBenchmarkFixture: failed to flush WAL buffer: {}", e)
            });
        }
        // WAL file handle is now closed — all data is on disk.

        // --- Phase 2: Validate the WAL file ---
        let metadata = std::fs::metadata(&wal_path).unwrap_or_else(|e| {
            panic!(
                "WalBenchmarkFixture: WAL file does not exist after flush at {:?}: {}",
                wal_path, e
            )
        });
        let file_size = metadata.len();
        assert!(
            file_size > 0,
            "WalBenchmarkFixture: WAL file at {:?} is empty (0 bytes) after populating {} entries",
            wal_path,
            config.fact_count,
        );

        // --- Phase 3: Dry-run replay to verify entry count ---
        let verify_wal = kcm_storage::wal::WriteAheadLog::new(&wal_path).unwrap_or_else(|e| {
            panic!(
                "WalBenchmarkFixture: failed to reopen WAL for verification at {:?}: {}",
                wal_path, e
            )
        });
        let mut replay_count: usize = 0;
        verify_wal
            .replay(|_| {
                replay_count += 1;
                Ok(())
            })
            .unwrap_or_else(|e| {
                panic!(
                    "WalBenchmarkFixture: WAL integrity check failed during replay at {:?}: {}",
                    wal_path, e
                )
            });
        assert_eq!(
            replay_count, config.fact_count,
            "WalBenchmarkFixture: WAL entry count mismatch — expected {}, got {} (file: {:?}, size: {} bytes)",
            config.fact_count, replay_count, wal_path, file_size,
        );

        WalBenchmarkFixture {
            _dir: dir,
            wal_path,
            entry_count: config.fact_count,
        }
    }

    /// Return the WAL file path for use by benchmarks.
    pub fn path(&self) -> &std::path::Path {
        &self.wal_path
    }

    /// Return the expected entry count for assertions in benchmarks.
    pub fn expected_count(&self) -> usize {
        self.entry_count
    }
}

/// Pre-computed file format save/load fixture.
pub struct FileFormatFixture {
    _dir: tempfile::TempDir,
    pub schema: Schema,
    pub path: std::path::PathBuf,
}

impl FileFormatFixture {
    pub fn new(config: &DatasetConfig) -> Self {
        config.validate().expect("Invalid dataset config");
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("bench.kcm");
        let mut schema = Schema::new(config.fact_count).unwrap();
        for i in 0..config.fact_count {
            let fact = deterministic_fact(i, config);
            schema.append_fact(&fact).unwrap();
        }
        kcm_storage::file_format::DatabaseFile::save(&schema, &path).unwrap();
        FileFormatFixture {
            _dir: dir,
            schema,
            path,
        }
    }
}

/// Generate a deterministic fact for a given index and config.
///
/// All values satisfy KCM type constraints:
/// - confidence is in [0.0, 1.0) via `.fract()`
/// - subject is in [0, subject_range)
/// - predicate is in [0, predicate_range)
/// - object is in [0, object_range)
pub fn deterministic_fact(index: usize, config: &DatasetConfig) -> Fact {
    let confidence = config.confidence_for_index(index);
    Fact::new(
        SubjectID((index as u32) % config.subject_range),
        PredicateID((index as u32 % config.predicate_range as u32) as u8),
        ObjectID((index as u32) % config.object_range),
        confidence,
    )
    .unwrap_or_else(|e| panic!("Invalid benchmark data at index {}: {}", index, e))
}

/// Canonical dataset sizes for benchmarking.
/// These sizes provide scaling from 1K to 10M facts.
pub const COLUMN_SIZES: &[usize] = &[1_000, 10_000, 100_000, 1_000_000, 10_000_000];
pub const BITMAP_SIZES: &[usize] = &[10_000, 100_000, 1_000_000, 10_000_000];
pub const DICTIONARY_SIZES: &[usize] = &[1_000, 10_000, 100_000, 1_000_000];
pub const DATABASE_SIZES: &[usize] = &[100, 1_000, 10_000, 100_000, 1_000_000];
pub const INFERENCE_SIZES: &[usize] = &[1_000, 10_000, 100_000, 1_000_000];
pub const WAL_SIZES: &[usize] = &[1_000, 10_000, 100_000, 1_000_000];
pub const FILE_FORMAT_SIZES: &[usize] = &[1_000, 10_000, 100_000, 1_000_000];

/// Extended sizes for enterprise-scale scalability testing.
pub const SCALE_SIZES: &[usize] = &[10_000_000, 100_000_000, 1_000_000_000];

/// Compression-specific sizes (byte counts).
pub const COMPRESSION_SIZES: &[usize] = &[1_000, 10_000, 100_000, 1_000_000, 10_000_000];

/// Sharding route counts.
pub const SHARDING_SIZES: &[usize] = &[1_000, 10_000, 100_000, 1_000_000];

/// Transaction batch sizes.
pub const TRANSACTION_SIZES: &[usize] = &[100, 1_000, 10_000, 100_000, 1_000_000];

/// Index sizes for lookup benchmarks.
pub const INDEX_SIZES: &[usize] = &[1_000, 10_000, 100_000, 1_000_000];

/// Optimizer query plan sizes.
pub const OPTIMIZER_SIZES: &[usize] = &[10, 50, 100, 500, 1_000];

/// Pre-computed rule registry for reasoning benchmarks.
pub struct RuleFixture {
    pub rules: Vec<(
        u32,
        String,
        kcm_reasoning::rule::RulePattern,
        PredicateID,
        f64,
    )>,
}

impl RuleFixture {
    pub fn new(count: u32) -> Self {
        let rules: Vec<(
            u32,
            String,
            kcm_reasoning::rule::RulePattern,
            PredicateID,
            f64,
        )> = (0..count)
            .map(|i| {
                (
                    i,
                    format!("rule_{}", i),
                    kcm_reasoning::rule::RulePattern::subject_predicate_object(
                        None,
                        PredicateID((i % 10) as u8),
                        None,
                    ),
                    PredicateID(((i + 10) % 20) as u8),
                    0.9,
                )
            })
            .collect();
        RuleFixture { rules }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dataset_config_validation() {
        let valid = DatasetConfig::for_count(100);
        assert!(valid.validate().is_ok());

        let invalid_zero = DatasetConfig {
            fact_count: 0,
            ..DatasetConfig::for_count(100)
        };
        assert!(invalid_zero.validate().is_err());

        let invalid_conf = DatasetConfig {
            base_confidence: 1.0,
            ..DatasetConfig::for_count(100)
        };
        assert!(invalid_conf.validate().is_err());
    }

    #[test]
    fn test_deterministic_fact_range() {
        let config = DatasetConfig::for_count(1000);
        for i in 0..1000 {
            let fact = deterministic_fact(i, &config);
            assert!(fact.confidence >= 0.0 && fact.confidence < 1.0);
            assert!(fact.subject.0 < config.subject_range);
            assert!((fact.predicate.0 as u32) < config.predicate_range as u32);
            assert!(fact.object.0 < config.object_range);
        }
    }

    #[test]
    fn test_deterministic_reproducibility() {
        let config = DatasetConfig::for_count(100);
        let f1 = deterministic_fact(42, &config);
        let f2 = deterministic_fact(42, &config);
        assert_eq!(f1.subject, f2.subject);
        assert_eq!(f1.predicate, f2.predicate);
        assert_eq!(f1.object, f2.object);
        assert!((f1.confidence - f2.confidence).abs() < 1e-15);
    }

    #[test]
    fn test_schema_fixture() {
        let config = DatasetConfig::for_count(10);
        let fixture = SchemaFixture::new(&config);
        assert_eq!(fixture.schema.len(), 10);
    }

    #[test]
    fn test_database_fixture() {
        let config = DatasetConfig::for_count(10);
        let fixture = DatabaseFixture::new(&config);
        assert_eq!(fixture.kb.fact_count(), 10);
    }
}
