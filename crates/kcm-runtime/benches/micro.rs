#![allow(clippy::unwrap_used, clippy::panic)]
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use kcm_core::bitmap::Bitmap;
use kcm_core::dictionary::Dictionary;
use kcm_core::types::*;
use kcm_core::vec::DenseVec;
use kcm_runtime::database::KnowledgeDatabase;
use kcm_storage::compress::{Compressor, Lz4Compressor, RleCompressor, ZstdCompressor};
use kcm_testing::bench_fixtures::*;
use std::time::Duration;

// ============================================================================
// STANDARDIZED BENCHMARK CONFIGURATION
//
// All benchmarks use these defaults for reproducibility across machines.
// Individual benchmarks may override for specific measurement needs.
// ============================================================================

const BENCH_MEASUREMENT_TIME: Duration = Duration::from_secs(5);
const BENCH_WARM_UP_TIME: Duration = Duration::from_secs(3);
const BENCH_SAMPLE_SIZE: usize = 100;

/// Configure a benchmark group with standardized settings.
fn configure_standard(group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>) {
    group.measurement_time(BENCH_MEASUREMENT_TIME);
    group.warm_up_time(BENCH_WARM_UP_TIME);
    group.sample_size(BENCH_SAMPLE_SIZE);
}

/// Configure a benchmark group with extended settings for slow benchmarks.
fn configure_extended(group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>) {
    group.measurement_time(Duration::from_secs(10));
    group.warm_up_time(BENCH_WARM_UP_TIME);
    group.sample_size(BENCH_SAMPLE_SIZE);
}

// ============================================================================
// COLUMN OPERATIONS
// ============================================================================

fn bench_column_sequential_scan(c: &mut Criterion) {
    let mut group = c.benchmark_group("column_sequential_scan");
    configure_standard(&mut group);
    for &size in COLUMN_SIZES {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let fixture = ColumnFixture::new(size);
            b.iter(|| black_box(fixture.data.iter().sum::<u32>()));
        });
    }
    group.finish();
}

fn bench_column_random_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("column_random_access");
    configure_standard(&mut group);
    for &size in COLUMN_SIZES {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let fixture = ColumnFixture::new(size);
            b.iter(|| {
                let mut sum = 0u32;
                for i in (0..size).step_by(17) {
                    sum = sum.wrapping_add(fixture.data[i]);
                }
                black_box(sum)
            });
        });
    }
    group.finish();
}

fn bench_column_simd_filter(c: &mut Criterion) {
    use kcm_compute::simd::SimdOps;
    let mut group = c.benchmark_group("column_simd_filter");
    configure_standard(&mut group);
    for &size in BITMAP_SIZES {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let fixture = U8ColumnFixture::new(size);
            b.iter(|| {
                black_box(
                    fixture
                        .data
                        .as_slice()
                        .simd_filter_eq(128u8)
                        .iter()
                        .filter(|&&v| v)
                        .count(),
                )
            });
        });
    }
    group.finish();
}

fn bench_column_push(c: &mut Criterion) {
    let mut group = c.benchmark_group("column_push");
    configure_standard(&mut group);
    for &size in &[10_000, 100_000, 1_000_000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter(|| {
                let mut vec: DenseVec<u32> =
                    DenseVec::new(size).expect("Failed to allocate DenseVec with capacity");
                for i in 0..size {
                    vec.push(i as u32)
                        .expect("Failed to push element into DenseVec");
                }
                black_box(&vec);
            });
        });
    }
    group.finish();
}

// ============================================================================
// BITMAP OPERATIONS
// ============================================================================

fn bench_bitmap_set(c: &mut Criterion) {
    let mut group = c.benchmark_group("bitmap_set");
    configure_standard(&mut group);
    for &size in BITMAP_SIZES {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let mut bitmap = Bitmap::new(size);
            b.iter(|| {
                for i in (0..size).step_by(10) {
                    bitmap.set(i);
                }
            });
        });
    }
    group.finish();
}

fn bench_bitmap_get(c: &mut Criterion) {
    let mut group = c.benchmark_group("bitmap_get");
    configure_standard(&mut group);
    for &size in BITMAP_SIZES {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let fixture = BitmapFixture::new(size, 10);
            b.iter(|| {
                for i in (0..size).step_by(17) {
                    black_box(fixture.bitmap.get(i));
                }
            });
        });
    }
    group.finish();
}

fn bench_bitmap_count(c: &mut Criterion) {
    let mut group = c.benchmark_group("bitmap_count");
    configure_standard(&mut group);
    for &size in BITMAP_SIZES {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let fixture = BitmapFixture::new(size, 10);
            b.iter(|| black_box(fixture.bitmap.count_ones()));
        });
    }
    group.finish();
}

fn bench_bitmap_bitwise(c: &mut Criterion) {
    let mut group = c.benchmark_group("bitmap_bitwise");
    configure_standard(&mut group);
    for &size in BITMAP_SIZES {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let b1 = BitmapFixture::new(size, 3);
            let b2 = BitmapFixture::new(size, 5);
            b.iter(|| {
                let mut r = b1.bitmap.clone();
                r.and_inplace(&b2.bitmap);
                black_box(r);
            });
        });
    }
    group.finish();
}

fn bench_bitmap_or(c: &mut Criterion) {
    let mut group = c.benchmark_group("bitmap_or");
    configure_standard(&mut group);
    for &size in BITMAP_SIZES {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let b1 = BitmapFixture::new(size, 3);
            let b2 = BitmapFixture::new(size, 5);
            b.iter(|| {
                let mut r = b1.bitmap.clone();
                r.or_inplace(&b2.bitmap);
                black_box(r);
            });
        });
    }
    group.finish();
}

fn bench_bitmap_iter_set_bits(c: &mut Criterion) {
    let mut group = c.benchmark_group("bitmap_iter_set_bits");
    configure_standard(&mut group);
    for &size in BITMAP_SIZES {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let fixture = BitmapFixture::new(size, 7);
            b.iter(|| black_box(fixture.bitmap.iter_set_bits().count()));
        });
    }
    group.finish();
}

// ============================================================================
// DICTIONARY OPERATIONS
// ============================================================================

fn bench_dictionary_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("dictionary_insert");
    configure_standard(&mut group);
    for &size in DICTIONARY_SIZES {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter(|| {
                let mut dict = Dictionary::new();
                for i in 0..size {
                    dict.insert(&format!("key_{}", i))
                        .expect("Failed to insert key into Dictionary");
                }
                black_box(dict)
            });
        });
    }
    group.finish();
}

fn bench_dictionary_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("dictionary_lookup");
    configure_standard(&mut group);
    for &size in DICTIONARY_SIZES {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let fixture = DictionaryFixture::new(size);
            let keys: Vec<String> = (0..size).map(|i| format!("key_{}", i)).collect();
            b.iter(|| {
                for key in &keys {
                    black_box(fixture.dict.lookup(key));
                }
            });
        });
    }
    group.finish();
}

fn bench_dictionary_insert_existing(c: &mut Criterion) {
    let mut group = c.benchmark_group("dictionary_insert_existing");
    configure_standard(&mut group);
    for &size in DICTIONARY_SIZES {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let mut dict = Dictionary::new();
            for i in 0..size {
                dict.insert(&format!("key_{}", i))
                    .expect("Failed to insert key into Dictionary during setup");
            }
            b.iter(|| {
                for i in 0..size {
                    black_box(
                        dict.insert(&format!("key_{}", i))
                            .expect("Failed to insert existing key into Dictionary"),
                    );
                }
            });
        });
    }
    group.finish();
}

// ============================================================================
// DATABASE OPERATIONS
// ============================================================================

fn bench_database_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("database_insert");
    configure_extended(&mut group);
    for &batch in &[100, 1_000, 10_000] {
        group.bench_with_input(BenchmarkId::from_parameter(batch), &batch, |b, &batch| {
            let config = DatasetConfig::for_count(batch);
            b.iter_batched(
                || {
                    KnowledgeDatabase::new()
                        .expect("Failed to create KnowledgeDatabase for insert benchmark")
                },
                |kb| {
                    for i in 0..batch {
                        let fact = deterministic_fact(i, &config);
                        kb.insert(&fact)
                            .expect("Failed to insert fact into KnowledgeDatabase");
                    }
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_database_query(c: &mut Criterion) {
    let mut group = c.benchmark_group("database_query");
    configure_standard(&mut group);
    for &size in DATABASE_SIZES {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let config = DatasetConfig::for_count(size);
            let fixture = DatabaseFixture::new(&config);
            b.iter(|| {
                black_box(
                    fixture
                        .kb
                        .query()
                        .with_predicate(PredicateID(5))
                        .execute()
                        .expect("Failed to execute predicate query in database_query benchmark"),
                )
            });
        });
    }
    group.finish();
}

fn bench_database_query_filtered(c: &mut Criterion) {
    let mut group = c.benchmark_group("database_query_filtered");
    configure_standard(&mut group);
    for &size in &[10_000, 100_000, 1_000_000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let config = DatasetConfig::for_count(size);
            let fixture = DatabaseFixture::new(&config);
            b.iter(|| {
                black_box(
                    fixture
                        .kb
                        .query()
                        .with_subject(SubjectID(50))
                        .with_confidence(0.5)
                        .execute()
                        .expect(
                            "Failed to execute filtered query in database_query_filtered benchmark",
                        ),
                )
            });
        });
    }
    group.finish();
}

fn bench_database_join(c: &mut Criterion) {
    let mut group = c.benchmark_group("database_join");
    configure_standard(&mut group);
    for &size in BITMAP_SIZES {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let config = DatasetConfig {
                fact_count: size,
                subject_range: 1000,
                predicate_range: 1,
                object_range: 1000,
                base_confidence: 0.5,
                confidence_step: 0.0001,
            };
            let fixture = DatabaseFixture::new(&config);
            let left = fixture
                .kb
                .query()
                .with_subject(SubjectID(500))
                .execute()
                .expect("Failed to execute join setup query in database_join benchmark");
            let right: Vec<usize> = (0..size).collect();
            b.iter(|| {
                black_box(
                    left.iter()
                        .flat_map(|f| {
                            right
                                .iter()
                                .filter(move |&&r| r % 1000 == f.subject.0 as usize % 1000)
                        })
                        .count(),
                )
            });
        });
    }
    group.finish();
}

// ============================================================================
// INFERENCE / REASONING
// ============================================================================

fn bench_inference_pattern_matching(c: &mut Criterion) {
    use kcm_compute::simd::SimdOps;
    let mut group = c.benchmark_group("inference_pattern_matching");
    configure_standard(&mut group);
    for &size in INFERENCE_SIZES {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let config = DatasetConfig::for_count(size);
            let fixture = SchemaFixture::new(&config);
            b.iter(|| {
                let predicates = fixture.schema.predicate_col.as_slice();
                black_box(
                    predicates
                        .simd_filter_eq(5u8)
                        .iter()
                        .filter(|&&v| v)
                        .count(),
                )
            });
        });
    }
    group.finish();
}

fn bench_inference_full_engine(c: &mut Criterion) {
    use kcm_reasoning::inference::InferenceEngine;
    use kcm_reasoning::rule::{Rule, RulePattern};
    use kcm_storage::column::Schema;
    let mut group = c.benchmark_group("inference_full_engine");
    configure_extended(&mut group);
    for &size in INFERENCE_SIZES {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let mut engine = InferenceEngine::new().with_max_iterations(1);
            let rule = Rule::new(
                1,
                "test_rule".to_string(),
                RulePattern::subject_predicate_object(None, PredicateID(0), None),
                PredicateID(1),
                Box::new(|confs| confs.first().copied().unwrap_or(0.0) * 0.9),
            );
            engine.register_rule(rule).expect("Failed to register rule");
            let config = DatasetConfig {
                fact_count: size,
                subject_range: 100,
                predicate_range: 5,
                object_range: 100,
                base_confidence: 0.5,
                confidence_step: 0.0001,
            };
            config
                .validate()
                .expect("Invalid dataset config for inference benchmark");
            let facts: Vec<Fact> = (0..size).map(|i| deterministic_fact(i, &config)).collect();
            let derived_budget = (size / config.predicate_range as usize).max(1);
            let schema_capacity = size + derived_budget;
            b.iter(|| {
                let mut schema = Schema::new(schema_capacity)
                    .expect("Failed to allocate schema for inference benchmark");
                for fact in &facts {
                    schema
                        .append_fact(fact)
                        .expect("Failed to insert fact into benchmark schema");
                }
                black_box(
                    engine
                        .infer_forward_chaining(&mut schema)
                        .expect("Inference failed during benchmark"),
                )
            });
        });
    }
    group.finish();
}

fn bench_rule_registry(c: &mut Criterion) {
    use kcm_reasoning::rule::{Rule, RulePattern, RuleRegistry};
    let mut group = c.benchmark_group("rule_registry");
    configure_standard(&mut group);
    for &count in &[10, 50, 100] {
        group.bench_with_input(BenchmarkId::from_parameter(count), &count, |b, &count| {
            b.iter(|| {
                let mut registry = RuleRegistry::new();
                for i in 0..count {
                    registry
                        .register(Rule::new(
                            i,
                            format!("rule_{}", i),
                            RulePattern::subject_predicate_object(None, PredicateID(0), None),
                            PredicateID(1),
                            Box::new(|c| c.first().copied().unwrap_or(0.0)),
                        ))
                        .expect("Failed to register rule in rule_registry benchmark");
                }
                black_box(registry.all_enabled());
            });
        });
    }
    group.finish();
}

// ============================================================================
// STORAGE I/O
// ============================================================================

fn bench_wal_append(c: &mut Criterion) {
    use kcm_storage::wal::WriteAheadLog;
    let mut group = c.benchmark_group("wal_append");
    configure_standard(&mut group);
    for &batch in &[100, 1_000, 10_000] {
        group.bench_with_input(BenchmarkId::from_parameter(batch), &batch, |b, &batch| {
            let dir = tempfile::tempdir()
                .unwrap_or_else(|e| panic!("Failed to create temp dir for wal_append: {}", e));
            let wal = WriteAheadLog::new(dir.path().join("bench.wal"))
                .unwrap_or_else(|e| panic!("Failed to create WAL for wal_append: {}", e));
            let config = DatasetConfig::for_count(batch);
            b.iter(|| {
                for i in 0..batch {
                    let fact = deterministic_fact(i, &config);
                    wal.append_fact(&fact).unwrap_or_else(|e| {
                        panic!("Failed to append fact {} to WAL in wal_append: {}", i, e)
                    });
                }
                wal.flush_buffer()
                    .unwrap_or_else(|e| panic!("Failed to flush WAL buffer in wal_append: {}", e));
            });
        });
    }
    group.finish();
}

fn bench_wal_replay(c: &mut Criterion) {
    use kcm_storage::wal::WriteAheadLog;
    let mut group = c.benchmark_group("wal_replay");
    configure_standard(&mut group);
    for &count in WAL_SIZES {
        group.bench_with_input(BenchmarkId::from_parameter(count), &count, |b, &count| {
            let config = DatasetConfig::for_count(count);
            let fixture = WalBenchmarkFixture::new(&config);

            // Pre-measurement validation: confirm fixture integrity.
            let wal_path = fixture.path();
            let metadata = std::fs::metadata(wal_path).unwrap_or_else(|e| {
                panic!(
                    "wal_replay setup: WAL file missing at {:?}: {}",
                    wal_path, e
                )
            });
            assert!(
                metadata.len() > 0,
                "wal_replay setup: WAL file is empty at {:?}",
                wal_path
            );

            // Measurement: open WAL and replay. No filesystem preparation.
            b.iter(|| {
                let wal_r = WriteAheadLog::new(fixture.path()).unwrap_or_else(|e| {
                    panic!(
                        "wal_replay: failed to open WAL at {:?}: {}",
                        fixture.path(),
                        e
                    )
                });
                let mut replayed = 0u64;
                wal_r
                    .replay(|_| {
                        replayed += 1;
                        Ok(())
                    })
                    .unwrap_or_else(|e| {
                        panic!("wal_replay: replay failed at {:?}: {}", fixture.path(), e)
                    });
                black_box(replayed)
            });
        });
    }
    group.finish();
}

fn bench_file_format_save_load(c: &mut Criterion) {
    use kcm_storage::file_format::DatabaseFile;
    let mut group = c.benchmark_group("file_format_save_load");
    configure_standard(&mut group);
    for &size in FILE_FORMAT_SIZES {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let config = DatasetConfig::for_count(size);
            let fixture = FileFormatFixture::new(&config);
            b.iter(|| {
                black_box(
                    DatabaseFile::load(&fixture.path)
                        .expect("Failed to load database file in file_format benchmark"),
                )
            });
        });
    }
    group.finish();
}

// ============================================================================
// COMPRESSION
// ============================================================================

fn bench_compression_encode(c: &mut Criterion) {
    let mut group = c.benchmark_group("compression_encode");
    configure_standard(&mut group);
    let fixture = CompressionFixture::new(100_000);
    let data: Vec<u8> = (0..100_000).map(|i| (i % 256) as u8).collect();
    group.bench_function("zstd", |b| {
        let zstd = ZstdCompressor::default_level();
        b.iter(|| {
            black_box(
                zstd.compress(&data)
                    .expect("Failed to compress data with zstd"),
            )
        });
    });
    group.bench_function("lz4", |b| {
        let lz4 = Lz4Compressor::default_level();
        b.iter(|| {
            black_box(
                lz4.compress(&data)
                    .expect("Failed to compress data with lz4"),
            )
        });
    });
    let _ = fixture;
    group.finish();
}

fn bench_compression_decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("compression_decode");
    configure_standard(&mut group);
    let fixture = CompressionFixture::new(100_000);
    let zstd = ZstdCompressor::default_level();
    let lz4 = Lz4Compressor::default_level();
    let data: Vec<u8> = (0..100_000).map(|i| (i % 256) as u8).collect();
    group.bench_function("zstd", |b| {
        b.iter(|| {
            black_box(
                zstd.decompress(&fixture.zstd_compressed, fixture.original_size)
                    .expect("Failed to decompress zstd data in compression_decode benchmark"),
            )
        });
    });
    group.bench_function("lz4", |b| {
        b.iter(|| {
            black_box(
                lz4.decompress(&fixture.lz4_compressed, fixture.original_size)
                    .expect("Failed to decompress lz4 data in compression_decode benchmark"),
            )
        });
    });
    let _ = data;
    group.finish();
}

fn bench_rle_encode(c: &mut Criterion) {
    let mut group = c.benchmark_group("rle_encode");
    configure_standard(&mut group);
    for &size in &[1_000, 10_000, 100_000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let rle = RleCompressor;
            let data: Vec<u8> = (0..size).map(|i| (i % 10) as u8).collect();
            b.iter(|| {
                black_box(
                    rle.compress(&data)
                        .expect("Failed to RLE compress data in rle_encode benchmark"),
                )
            });
        });
    }
    group.finish();
}

fn bench_rle_decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("rle_decode");
    configure_standard(&mut group);
    for &size in &[1_000, 10_000, 100_000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let rle = RleCompressor;
            let data: Vec<u8> = (0..size).map(|i| (i % 10) as u8).collect();
            let compressed = rle
                .compress(&data)
                .expect("Failed to RLE compress data in rle_decode setup");
            b.iter(|| {
                black_box(
                    rle.decompress(&compressed, size)
                        .expect("Failed to RLE decompress data in rle_decode benchmark"),
                )
            });
        });
    }
    group.finish();
}

// ============================================================================
// DISTRIBUTED
// ============================================================================

fn bench_sharding(c: &mut Criterion) {
    use kcm_distributed::sharding::{
        ConsistentHashSharding, HashSharding, RangeSharding, ShardingStrategy,
    };
    let mut group = c.benchmark_group("sharding");
    configure_standard(&mut group);
    let hash = HashSharding;
    let range = RangeSharding::new(vec![250, 500, 750]);
    let ch = ConsistentHashSharding::new(16, 150);
    group.bench_function("hash_routing", |b| {
        b.iter(|| {
            for i in 0..10_000u32 {
                black_box(hash.get_shard_id(i, 16));
            }
        });
    });
    group.bench_function("range_routing", |b| {
        b.iter(|| {
            for i in 0..10_000u32 {
                black_box(range.get_shard_id(i, 4));
            }
        });
    });
    group.bench_function("consistent_hash_routing", |b| {
        b.iter(|| {
            for i in 0..10_000u32 {
                black_box(ch.get_shard_for_key(i));
            }
        });
    });
    group.finish();
}

// ============================================================================
// MEMORY METRICS
// ============================================================================

fn bench_memory_metrics(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_metrics");
    configure_standard(&mut group);
    group.bench_function("per_fact_memory_100k", |b| {
        let config = DatasetConfig::for_count(100_000);
        b.iter(|| {
            let fixture = DatabaseFixture::new(&config);
            black_box(fixture.kb.fact_count())
        });
    });
    group.bench_function("bitmap_memory_1m", |b| {
        b.iter(|| {
            let fixture = BitmapFixture::new(1_000_000, 7);
            black_box(fixture.bitmap.count_ones())
        });
    });
    group.bench_function("dictionary_memory_100k", |b| {
        b.iter(|| {
            let fixture = DictionaryFixture::new(100_000);
            black_box(fixture.dict.len())
        });
    });
    group.bench_function("dense_vec_memory_1m", |b| {
        b.iter(|| {
            let fixture = DenseVecU64Fixture::new(1_000_000);
            black_box(fixture.data.len())
        });
    });
    group.finish();
}

// ============================================================================
// TRANSACTION OPERATIONS
// ============================================================================

fn bench_transaction_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("transaction_insert");
    configure_standard(&mut group);
    for &batch_size in &[100, 1_000, 10_000] {
        group.bench_with_input(
            BenchmarkId::from_parameter(batch_size),
            &batch_size,
            |b, &batch_size| {
                let config = DatasetConfig::for_count(batch_size);
                b.iter_batched(
                    || {
                        let kb = KnowledgeDatabase::new()
                            .expect("Failed to create KnowledgeDatabase for transaction benchmark");
                        let txn = kb.begin_transaction();
                        (kb, txn)
                    },
                    |(kb, mut txn)| {
                        for i in 0..batch_size {
                            let fact = deterministic_fact(i, &config);
                            txn.insert(fact)
                                .expect("Failed to insert fact into transaction");
                        }
                        txn.apply_to_schema(&mut kb.get_schema_mut())
                            .expect("Failed to apply transaction to schema");
                        black_box(txn);
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }
    group.finish();
}

fn bench_transaction_commit_rollback(c: &mut Criterion) {
    let mut group = c.benchmark_group("transaction_commit_rollback");
    configure_standard(&mut group);
    group.bench_function("commit", |b| {
        b.iter_batched(
            || {
                let kb = KnowledgeDatabase::new()
                    .expect("Failed to create KnowledgeDatabase for commit/rollback benchmark");
                let mut txn = kb.begin_transaction();
                for i in 0..100 {
                    let fact = deterministic_fact(i, &DatasetConfig::for_count(100));
                    txn.insert(fact)
                        .expect("Failed to insert fact into transaction during commit setup");
                }
                txn.apply_to_schema(&mut kb.get_schema_mut())
                    .expect("Failed to apply transaction to schema during commit setup");
                txn
            },
            |txn| {
                let _ = txn.commit();
            },
            criterion::BatchSize::SmallInput,
        );
    });
    group.finish();
}

// ============================================================================
// SCALABILITY BENCHMARKS — million-row and beyond
// ============================================================================

fn bench_scalability_column_scan(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalability_column_scan");
    configure_standard(&mut group);
    for &size in &[1_000_000, 10_000_000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let fixture = ColumnFixture::new(size);
            b.iter(|| black_box(fixture.data.iter().sum::<u32>()));
        });
    }
    group.finish();
}

fn bench_scalability_bitmap(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalability_bitmap");
    configure_standard(&mut group);
    for &size in &[1_000_000, 10_000_000] {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("set_{}", size)),
            &size,
            |b, &size| {
                let mut bitmap = Bitmap::new(size);
                b.iter(|| {
                    for i in (0..size).step_by(10) {
                        bitmap.set(i);
                    }
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("count_ones_{}", size)),
            &size,
            |b, &size| {
                let fixture = BitmapFixture::new(size, 7);
                b.iter(|| black_box(fixture.bitmap.count_ones()));
            },
        );
    }
    group.finish();
}

fn bench_scalability_database_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalability_database_insert");
    configure_standard(&mut group);
    for &size in &[100_000, 1_000_000] {
        group.throughput(criterion::Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let config = DatasetConfig::for_count(size);
            b.iter_batched(
                || {
                    KnowledgeDatabase::new()
                        .expect("Failed to create KnowledgeDatabase for scalability benchmark")
                },
                |kb| {
                    for i in 0..size {
                        let fact = deterministic_fact(i, &config);
                        kb.insert(&fact)
                            .expect("Failed to insert fact in scalability benchmark");
                    }
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_scalability_wal_replay(c: &mut Criterion) {
    use kcm_storage::wal::WriteAheadLog;
    let mut group = c.benchmark_group("scalability_wal_replay");
    configure_standard(&mut group);
    for &count in [100_000usize, 1_000_000].iter() {
        group.throughput(criterion::Throughput::Elements(count as u64));
        group.bench_with_input(BenchmarkId::from_parameter(count), &count, |b, &count| {
            let config = DatasetConfig::for_count(count);
            let fixture = WalBenchmarkFixture::new(&config);
            b.iter(|| {
                let wal_r = WriteAheadLog::new(fixture.path())
                    .unwrap_or_else(|e| panic!("scalability_wal_replay: open failed: {}", e));
                let mut replayed = 0u64;
                wal_r
                    .replay(|_| {
                        replayed += 1;
                        Ok(())
                    })
                    .unwrap_or_else(|e| panic!("scalability_wal_replay: replay failed: {}", e));
                black_box(replayed)
            });
        });
    }
    group.finish();
}

fn bench_scalability_compression(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalability_compression");
    configure_standard(&mut group);
    let zstd = ZstdCompressor::default_level();
    let lz4 = Lz4Compressor::default_level();
    for &size in &[1_000_000, 10_000_000] {
        let data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
        group.throughput(criterion::Throughput::Bytes(size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("zstd_{}", size)),
            &size,
            |b, _| {
                b.iter(|| black_box(zstd.compress(&data).expect("Zstd compress failed")));
            },
        );
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("lz4_{}", size)),
            &size,
            |b, _| {
                b.iter(|| black_box(lz4.compress(&data).expect("Lz4 compress failed")));
            },
        );
    }
    group.finish();
}

fn bench_scalability_inference(c: &mut Criterion) {
    use kcm_reasoning::inference::InferenceEngine;
    use kcm_reasoning::rule::{Rule, RulePattern};
    use kcm_storage::column::Schema;
    let mut group = c.benchmark_group("scalability_inference");
    configure_extended(&mut group);
    for &size in &[100_000, 1_000_000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let mut engine = InferenceEngine::new().with_max_iterations(1);
            let rule = Rule::new(
                1,
                "scale_rule".to_string(),
                RulePattern::subject_predicate_object(None, PredicateID(0), None),
                PredicateID(1),
                Box::new(|confs| confs.first().copied().unwrap_or(0.0) * 0.9),
            );
            engine
                .register_rule(rule)
                .expect("Failed to register rule for scalability benchmark");
            let config = DatasetConfig {
                fact_count: size,
                subject_range: 100,
                predicate_range: 5,
                object_range: 100,
                base_confidence: 0.5,
                confidence_step: 0.0001,
            };
            config
                .validate()
                .expect("Invalid config for scalability inference benchmark");
            let facts: Vec<Fact> = (0..size).map(|i| deterministic_fact(i, &config)).collect();
            let derived_budget = (size / config.predicate_range as usize).max(1);
            let schema_capacity = size + derived_budget;
            b.iter(|| {
                let mut schema = Schema::new(schema_capacity)
                    .expect("Failed to allocate schema for scalability inference benchmark");
                for fact in &facts {
                    schema
                        .append_fact(fact)
                        .expect("Failed to insert fact for scalability inference benchmark");
                }
                black_box(
                    engine
                        .infer_forward_chaining(&mut schema)
                        .expect("Inference failed in scalability benchmark"),
                )
            });
        });
    }
    group.finish();
}

fn bench_scalability_transaction(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalability_transaction");
    configure_standard(&mut group);
    for &batch_size in &[10_000, 100_000] {
        group.bench_with_input(
            BenchmarkId::from_parameter(batch_size),
            &batch_size,
            |b, &batch_size| {
                let config = DatasetConfig::for_count(batch_size);
                b.iter_batched(
                    || {
                        let kb = KnowledgeDatabase::new()
                            .expect("Failed to create KB for scalability transaction benchmark");
                        let txn = kb.begin_transaction();
                        (kb, txn)
                    },
                    |(kb, mut txn)| {
                        for i in 0..batch_size {
                            let fact = deterministic_fact(i, &config);
                            txn.insert(fact).expect("Transaction insert failed");
                        }
                        txn.apply_to_schema(&mut kb.get_schema_mut())
                            .expect("Transaction apply failed");
                        black_box(txn);
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }
    group.finish();
}

// ============================================================================
// INFERENCE FULL ENGINE
// ============================================================================

criterion_group!(
    benches,
    bench_column_sequential_scan,
    bench_column_random_access,
    bench_column_simd_filter,
    bench_column_push,
    bench_bitmap_set,
    bench_bitmap_get,
    bench_bitmap_count,
    bench_bitmap_bitwise,
    bench_bitmap_or,
    bench_bitmap_iter_set_bits,
    bench_dictionary_insert,
    bench_dictionary_lookup,
    bench_dictionary_insert_existing,
    bench_database_insert,
    bench_database_query,
    bench_database_query_filtered,
    bench_database_join,
    bench_inference_pattern_matching,
    bench_inference_full_engine,
    bench_rule_registry,
    bench_wal_append,
    bench_wal_replay,
    bench_file_format_save_load,
    bench_compression_encode,
    bench_compression_decode,
    bench_rle_encode,
    bench_rle_decode,
    bench_sharding,
    bench_memory_metrics,
    bench_transaction_insert,
    bench_transaction_commit_rollback,
    bench_scalability_column_scan,
    bench_scalability_bitmap,
    bench_scalability_database_insert,
    bench_scalability_wal_replay,
    bench_scalability_compression,
    bench_scalability_inference,
    bench_scalability_transaction,
);

criterion_main!(benches);
