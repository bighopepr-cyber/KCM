use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use kcm_core::bitmap::Bitmap;
use kcm_core::dictionary::Dictionary;
use kcm_core::types::*;
use kcm_core::vec::DenseVec;
use kcm_runtime::database::KnowledgeDatabase;
use kcm_storage::compress::{Compressor, Lz4Compressor, RleCompressor, ZstdCompressor};
use kcm_testing::bench_fixtures::*;
use std::time::Duration;

// ============================================================================
// COLUMN OPERATIONS
// ============================================================================

fn bench_column_sequential_scan(c: &mut Criterion) {
    let mut group = c.benchmark_group("column_sequential_scan");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(100);
    group.warm_up_time(Duration::from_secs(3));
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
    for &size in &[10_000, 100_000, 1_000_000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter(|| {
                let mut vec: DenseVec<u32> = DenseVec::new(size).expect("Failed to allocate DenseVec with capacity");
                for i in 0..size {
                    vec.push(i as u32).expect("Failed to push element into DenseVec");
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
    for &size in DICTIONARY_SIZES {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter(|| {
                let mut dict = Dictionary::new();
                for i in 0..size {
                    dict.insert(&format!("key_{}", i)).expect("Failed to insert key into Dictionary");
                }
                black_box(dict)
            });
        });
    }
    group.finish();
}

fn bench_dictionary_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("dictionary_lookup");
    for &size in DICTIONARY_SIZES {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let fixture = DictionaryFixture::new(size);
            b.iter(|| {
                for i in 0..size {
                    black_box(fixture.dict.lookup(&format!("key_{}", i)));
                }
            });
        });
    }
    group.finish();
}

fn bench_dictionary_insert_existing(c: &mut Criterion) {
    let mut group = c.benchmark_group("dictionary_insert_existing");
    for &size in DICTIONARY_SIZES {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let mut dict = Dictionary::new();
            for i in 0..size {
                dict.insert(&format!("key_{}", i)).expect("Failed to insert key into Dictionary during setup");
            }
            b.iter(|| {
                for i in 0..size {
                    black_box(dict.insert(&format!("key_{}", i)).expect("Failed to insert existing key into Dictionary"));
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
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(100);
    for &batch in &[100, 1_000, 10_000] {
        group.bench_with_input(BenchmarkId::from_parameter(batch), &batch, |b, &batch| {
            let config = DatasetConfig::for_count(batch);
            b.iter_batched(
                || KnowledgeDatabase::new().expect("Failed to create KnowledgeDatabase for insert benchmark"),
                |kb| {
                    for i in 0..batch {
                        let fact = deterministic_fact(i, &config);
                        kb.insert(&fact).expect("Failed to insert fact into KnowledgeDatabase");
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
                        .expect("Failed to execute filtered query in database_query_filtered benchmark"),
                )
            });
        });
    }
    group.finish();
}

fn bench_database_join(c: &mut Criterion) {
    let mut group = c.benchmark_group("database_join");
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
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(100);
    group.warm_up_time(Duration::from_secs(3));
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
    for &batch in &[100, 1_000, 10_000] {
        group.bench_with_input(BenchmarkId::from_parameter(batch), &batch, |b, &batch| {
            let dir = tempfile::tempdir().expect("Failed to create temp directory for WAL benchmark");
            let wal = WriteAheadLog::new(dir.path().join("bench.wal")).expect("Failed to create WAL for benchmark");
            let config = DatasetConfig::for_count(batch);
            b.iter(|| {
                for i in 0..batch {
                    let fact = deterministic_fact(i, &config);
                    wal.append_fact(&fact).expect("Failed to append fact to WAL in wal_append benchmark");
                }
                wal.flush_buffer().expect("Failed to flush WAL buffer in wal_append benchmark");
            });
        });
    }
    group.finish();
}

fn bench_wal_replay(c: &mut Criterion) {
    use kcm_storage::wal::WriteAheadLog;
    let mut group = c.benchmark_group("wal_replay");
    for &count in WAL_SIZES {
        group.bench_with_input(BenchmarkId::from_parameter(count), &count, |b, &count| {
            let config = DatasetConfig::for_count(count);
            let fixture = WALFixture::new(&config);
            b.iter(|| {
                let wal_r = WriteAheadLog::new(&fixture.wal_path).unwrap();
                let mut count = 0u64;
                wal_r
                    .replay(|_| {
                        count += 1;
                        Ok(())
                    })
                    .unwrap();
                black_box(count)
            });
        });
    }
    group.finish();
}

fn bench_file_format_save_load(c: &mut Criterion) {
    use kcm_storage::file_format::DatabaseFile;
    let mut group = c.benchmark_group("file_format_save_load");
    for &size in FILE_FORMAT_SIZES {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let config = DatasetConfig::for_count(size);
            let fixture = FileFormatFixture::new(&config);
            b.iter(|| black_box(DatabaseFile::load(&fixture.path).unwrap()));
        });
    }
    group.finish();
}

// ============================================================================
// COMPRESSION
// ============================================================================

fn bench_compression_encode(c: &mut Criterion) {
    let mut group = c.benchmark_group("compression_encode");
    let fixture = CompressionFixture::new(100_000);
    let data: Vec<u8> = (0..100_000).map(|i| (i % 256) as u8).collect();
    group.bench_function("zstd", |b| {
        let zstd = ZstdCompressor::default_level();
        b.iter(|| black_box(zstd.compress(&data).unwrap()));
    });
    group.bench_function("lz4", |b| {
        let lz4 = Lz4Compressor::default_level();
        b.iter(|| black_box(lz4.compress(&data).unwrap()));
    });
    let _ = fixture;
    group.finish();
}

fn bench_compression_decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("compression_decode");
    let fixture = CompressionFixture::new(100_000);
    let zstd = ZstdCompressor::default_level();
    let lz4 = Lz4Compressor::default_level();
    let data: Vec<u8> = (0..100_000).map(|i| (i % 256) as u8).collect();
    group.bench_function("zstd", |b| {
        b.iter(|| {
            black_box(
                zstd.decompress(&fixture.zstd_compressed, fixture.original_size)
                    .unwrap(),
            )
        });
    });
    group.bench_function("lz4", |b| {
        b.iter(|| {
            black_box(
                lz4.decompress(&fixture.lz4_compressed, fixture.original_size)
                    .unwrap(),
            )
        });
    });
    let _ = data;
    group.finish();
}

fn bench_rle_encode(c: &mut Criterion) {
    let mut group = c.benchmark_group("rle_encode");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(100);
    for &size in &[1_000, 10_000, 100_000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let rle = RleCompressor;
            let data: Vec<u8> = (0..size).map(|i| (i % 10) as u8).collect();
            b.iter(|| black_box(rle.compress(&data).unwrap()));
        });
    }
    group.finish();
}

fn bench_rle_decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("rle_decode");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(100);
    for &size in &[1_000, 10_000, 100_000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let rle = RleCompressor;
            let data: Vec<u8> = (0..size).map(|i| (i % 10) as u8).collect();
            let compressed = rle.compress(&data).unwrap();
            b.iter(|| black_box(rle.decompress(&compressed, size).unwrap()));
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
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(100);
    for &batch_size in &[100, 1_000, 10_000] {
        group.bench_with_input(
            BenchmarkId::from_parameter(batch_size),
            &batch_size,
            |b, &batch_size| {
                let config = DatasetConfig::for_count(batch_size);
                b.iter_batched(
                    || {
                        let kb = KnowledgeDatabase::new().unwrap();
                        let txn = kb.begin_transaction();
                        (kb, txn)
                    },
                    |(kb, mut txn)| {
                        for i in 0..batch_size {
                            let fact = deterministic_fact(i, &config);
                            txn.insert(fact).unwrap();
                        }
                        txn.apply_to_schema(&mut kb.get_schema_mut()).unwrap();
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
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(100);
    group.bench_function("commit", |b| {
        b.iter_batched(
            || {
                let kb = KnowledgeDatabase::new().unwrap();
                let mut txn = kb.begin_transaction();
                for i in 0..100 {
                    let fact = deterministic_fact(i, &DatasetConfig::for_count(100));
                    txn.insert(fact).unwrap();
                }
                txn.apply_to_schema(&mut kb.get_schema_mut()).unwrap();
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
);

criterion_main!(benches);
