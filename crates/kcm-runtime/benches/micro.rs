use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use kcm_core::bitmap::Bitmap;
use kcm_core::dictionary::Dictionary;
use kcm_core::types::*;
use kcm_core::vec::DenseVec;
use kcm_runtime::database::KnowledgeDatabase;
use std::time::Duration;

// ============================================================================
// COLUMN OPERATIONS
// ============================================================================

fn bench_column_sequential_scan(c: &mut Criterion) {
    let mut group = c.benchmark_group("column_sequential_scan");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(100);
    group.warm_up_time(Duration::from_secs(3));
    for size in &[1_000, 10_000, 100_000, 1_000_000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut vec: DenseVec<u32> = DenseVec::new(size).unwrap();
            for i in 0..size {
                vec.push(i as u32).unwrap();
            }
            b.iter(|| black_box(vec.iter().sum::<u32>()));
        });
    }
    group.finish();
}

fn bench_column_random_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("column_random_access");
    for size in &[1_000, 10_000, 100_000, 1_000_000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut vec: DenseVec<u32> = DenseVec::new(size).unwrap();
            for i in 0..size {
                vec.push(i as u32).unwrap();
            }
            b.iter(|| {
                let mut sum = 0u32;
                for i in (0..size).step_by(17) {
                    sum = sum.wrapping_add(vec[i]);
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
    for size in &[10_000, 100_000, 1_000_000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut vec: DenseVec<u8> = DenseVec::new(size).unwrap();
            for i in 0..size {
                vec.push((i % 256) as u8).unwrap();
            }
            b.iter(|| {
                black_box(
                    vec.as_slice()
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
    for size in &[10_000, 100_000, 1_000_000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut vec: DenseVec<u32> = DenseVec::new(size).unwrap();
                for i in 0..size {
                    vec.push(i as u32).unwrap();
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
    for size in &[10_000, 100_000, 1_000_000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
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
    for size in &[10_000, 100_000, 1_000_000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut bitmap = Bitmap::new(size);
            for i in (0..size).step_by(10) {
                bitmap.set(i);
            }
            b.iter(|| {
                for i in (0..size).step_by(17) {
                    black_box(bitmap.get(i));
                }
            });
        });
    }
    group.finish();
}

fn bench_bitmap_count(c: &mut Criterion) {
    let mut group = c.benchmark_group("bitmap_count");
    for size in &[10_000, 100_000, 1_000_000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut bitmap = Bitmap::new(size);
            for i in (0..size).step_by(10) {
                bitmap.set(i);
            }
            b.iter(|| black_box(bitmap.count_ones()));
        });
    }
    group.finish();
}

fn bench_bitmap_bitwise(c: &mut Criterion) {
    let mut group = c.benchmark_group("bitmap_bitwise");
    for size in &[10_000, 100_000, 1_000_000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut b1 = Bitmap::new(size);
            let mut b2 = Bitmap::new(size);
            for i in (0..size).step_by(3) {
                b1.set(i);
            }
            for i in (0..size).step_by(5) {
                b2.set(i);
            }
            b.iter(|| {
                let mut r = b1.clone();
                r.and_inplace(&b2);
                black_box(r);
            });
        });
    }
    group.finish();
}

fn bench_bitmap_or(c: &mut Criterion) {
    let mut group = c.benchmark_group("bitmap_or");
    for size in &[10_000, 100_000, 1_000_000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut b1 = Bitmap::new(size);
            let mut b2 = Bitmap::new(size);
            for i in (0..size).step_by(3) {
                b1.set(i);
            }
            for i in (0..size).step_by(5) {
                b2.set(i);
            }
            b.iter(|| {
                let mut r = b1.clone();
                r.or_inplace(&b2);
                black_box(r);
            });
        });
    }
    group.finish();
}

fn bench_bitmap_iter_set_bits(c: &mut Criterion) {
    let mut group = c.benchmark_group("bitmap_iter_set_bits");
    for size in &[10_000, 100_000, 1_000_000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut bitmap = Bitmap::new(size);
            for i in (0..size).step_by(7) {
                bitmap.set(i);
            }
            b.iter(|| black_box(bitmap.iter_set_bits().count()));
        });
    }
    group.finish();
}

// ============================================================================
// DICTIONARY OPERATIONS
// ============================================================================

fn bench_dictionary_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("dictionary_insert");
    for size in &[1_000, 10_000, 100_000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut dict = Dictionary::new();
                for i in 0..size {
                    dict.insert(&format!("key_{}", i));
                }
                black_box(dict)
            });
        });
    }
    group.finish();
}

fn bench_dictionary_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("dictionary_lookup");
    for size in &[1_000, 10_000, 100_000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut dict = Dictionary::new();
            let keys: Vec<String> = (0..size).map(|i| format!("key_{}", i)).collect();
            for key in &keys {
                dict.insert(key);
            }
            b.iter(|| {
                for key in &keys {
                    black_box(dict.lookup(key));
                }
            });
        });
    }
    group.finish();
}

fn bench_dictionary_insert_existing(c: &mut Criterion) {
    let mut group = c.benchmark_group("dictionary_insert_existing");
    for size in &[1_000, 10_000, 100_000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut dict = Dictionary::new();
            for i in 0..size {
                dict.insert(&format!("key_{}", i));
            }
            b.iter(|| {
                for i in 0..size {
                    black_box(dict.insert(&format!("key_{}", i)));
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
    for batch in &[100, 1_000, 10_000, 100_000] {
        group.bench_with_input(BenchmarkId::from_parameter(batch), batch, |b, &batch| {
            b.iter_batched(
                || KnowledgeDatabase::new().unwrap(),
                |kb| {
                    for i in 0..batch {
                        let fact = Fact::new(
                            SubjectID((i % 100) as u32),
                            PredicateID((i % 10) as u8),
                            ObjectID((i % 200) as u32),
                            0.5 + (i as f64 % 0.5),
                        )
                        .unwrap();
                        kb.insert(&fact).unwrap();
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
    for dataset in &[1_000, 10_000, 100_000, 1_000_000] {
        group.bench_with_input(
            BenchmarkId::from_parameter(dataset),
            dataset,
            |b, &dataset| {
                let kb = KnowledgeDatabase::new().unwrap();
                for i in 0..dataset {
                    let fact = Fact::new(
                        SubjectID((i % 100) as u32),
                        PredicateID((i % 10) as u8),
                        ObjectID((i % 200) as u32),
                        0.75,
                    )
                    .unwrap();
                    kb.insert(&fact).unwrap();
                }
                b.iter(|| black_box(kb.query().with_predicate(PredicateID(5)).execute().unwrap()));
            },
        );
    }
    group.finish();
}

fn bench_database_query_filtered(c: &mut Criterion) {
    let mut group = c.benchmark_group("database_query_filtered");
    for dataset in &[10_000, 100_000, 1_000_000] {
        group.bench_with_input(
            BenchmarkId::from_parameter(dataset),
            dataset,
            |b, &dataset| {
                let kb = KnowledgeDatabase::new().unwrap();
                for i in 0..dataset {
                    let fact = Fact::new(
                        SubjectID((i % 100) as u32),
                        PredicateID((i % 10) as u8),
                        ObjectID((i % 200) as u32),
                        0.1 + (i as f64 * 0.001),
                    )
                    .unwrap();
                    kb.insert(&fact).unwrap();
                }
                b.iter(|| {
                    black_box(
                        kb.query()
                            .with_subject(SubjectID(50))
                            .with_confidence(0.5)
                            .execute()
                            .unwrap(),
                    )
                });
            },
        );
    }
    group.finish();
}

fn bench_database_join(c: &mut Criterion) {
    let mut group = c.benchmark_group("database_join");
    for size in &[10_000, 100_000, 1_000_000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let kb = KnowledgeDatabase::new().unwrap();
            for i in 0..size {
                let fact = Fact::new(
                    SubjectID(i % 1000),
                    PredicateID(0),
                    ObjectID((i % 1000) + 100_000),
                    0.8,
                )
                .unwrap();
                kb.insert(&fact).unwrap();
            }
            let left = kb.query().with_subject(SubjectID(500)).execute().unwrap();
            let right: Vec<usize> = (0..size as usize).collect();
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
    for dataset in &[1_000, 10_000, 100_000, 1_000_000] {
        group.bench_with_input(
            BenchmarkId::from_parameter(dataset),
            dataset,
            |b, &schema_size| {
                let mut schema = kcm_storage::Schema::new(schema_size).unwrap();
                for i in 0..schema_size {
                    schema
                        .append_fact(
                            &Fact::new(
                                SubjectID((i % 100) as u32),
                                PredicateID((i % 10) as u8),
                                ObjectID((i % 100) as u32),
                                0.8,
                            )
                            .unwrap(),
                        )
                        .unwrap();
                }
                b.iter(|| {
                    let predicates = schema.predicate_col.as_slice();
                    black_box(
                        predicates
                            .simd_filter_eq(5u8)
                            .iter()
                            .filter(|&&v| v)
                            .count(),
                    )
                });
            },
        );
    }
    group.finish();
}

fn bench_confidence_calculation(c: &mut Criterion) {
    use kcm_reasoning::confidence::ConfidenceCalculator;
    let mut group = c.benchmark_group("confidence_calculation");
    let values: Vec<f64> = (0..1000).map(|i| i as f64 / 1000.0).collect();
    group.bench_function("conjunction_1000", |b| {
        b.iter(|| {
            let mut result = 1.0f64;
            for &v in &values {
                result = ConfidenceCalculator::conjunction(result, v);
            }
            black_box(result)
        });
    });
    group.bench_function("disjunction_1000", |b| {
        b.iter(|| {
            let mut result = 0.0f64;
            for &v in &values {
                result = ConfidenceCalculator::disjunction(result, v);
            }
            black_box(result)
        });
    });
    group.bench_function("chain_1000", |b| {
        b.iter(|| black_box(ConfidenceCalculator::chain(&values)));
    });
    group.bench_function("weighted_1000", |b| {
        let weights: Vec<f64> = (0..1000).map(|i| (i as f64 + 1.0) / 1000.0).collect();
        b.iter(|| black_box(ConfidenceCalculator::weighted(&values, &weights)));
    });
    group.finish();
}

fn bench_inference_full_engine(c: &mut Criterion) {
    use kcm_reasoning::inference::InferenceEngine;
    use kcm_reasoning::rule::{Rule, RulePattern};
    let mut group = c.benchmark_group("inference_full_engine");
    for dataset in &[1_000, 10_000, 100_000] {
        group.bench_with_input(
            BenchmarkId::from_parameter(dataset),
            dataset,
            |b, &schema_size| {
                let mut engine = InferenceEngine::new().with_max_iterations(1);
                let rule = Rule::new(
                    1,
                    "test_rule".to_string(),
                    RulePattern::subject_predicate_object(None, PredicateID(0), None),
                    PredicateID(1),
                    Box::new(|confs| confs.first().copied().unwrap_or(0.0) * 0.9),
                );
                engine.register_rule(rule).unwrap();
                let mut schema = kcm_storage::Schema::new(schema_size).unwrap();
                for i in 0..schema_size {
                    schema
                        .append_fact(
                            &Fact::new(
                                SubjectID((i % 100) as u32),
                                PredicateID((i % 5) as u8),
                                ObjectID((i % 100) as u32),
                                0.8,
                            )
                            .unwrap(),
                        )
                        .unwrap();
                }
                b.iter(|| black_box(engine.infer_forward_chaining(&mut schema).unwrap()));
            },
        );
    }
    group.finish();
}

fn bench_rule_registry(c: &mut Criterion) {
    use kcm_reasoning::rule::{Rule, RulePattern, RuleRegistry};
    let mut group = c.benchmark_group("rule_registry");
    for count in &[10, 50, 100] {
        group.bench_with_input(BenchmarkId::from_parameter(count), count, |b, &count| {
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
                        .unwrap();
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
    for batch in &[100, 1_000, 10_000] {
        group.bench_with_input(BenchmarkId::from_parameter(batch), batch, |b, &batch| {
            let dir = tempfile::tempdir().unwrap();
            let wal = WriteAheadLog::new(dir.path().join("bench.wal")).unwrap();
            b.iter(|| {
                for i in 0..batch {
                    let fact =
                        Fact::new(SubjectID(i as u32), PredicateID(0), ObjectID(i as u32), 0.9)
                            .unwrap();
                    wal.append_fact(&fact).unwrap();
                }
                wal.flush_buffer().unwrap();
            });
        });
    }
    group.finish();
}

fn bench_wal_replay(c: &mut Criterion) {
    use kcm_storage::wal::WriteAheadLog;
    let mut group = c.benchmark_group("wal_replay");
    for count in &[1_000, 10_000, 100_000] {
        group.bench_with_input(BenchmarkId::from_parameter(count), count, |b, &count| {
            let dir = tempfile::tempdir().unwrap();
            let wal_path = dir.path().join("bench.wal");
            {
                let wal = WriteAheadLog::new(&wal_path).unwrap();
                for i in 0..count {
                    let fact =
                        Fact::new(SubjectID(i as u32), PredicateID(0), ObjectID(i as u32), 0.9)
                            .unwrap();
                    wal.append_fact(&fact).unwrap();
                }
                wal.flush_buffer().unwrap();
            }
            b.iter(|| {
                let wal_r = WriteAheadLog::new(&wal_path).unwrap();
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
    for size in &[1_000, 10_000, 100_000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let dir = tempfile::tempdir().unwrap();
            let path = dir.path().join("bench.kcm");
            let mut schema = kcm_storage::Schema::new(size).unwrap();
            for i in 0..size {
                schema
                    .append_fact(
                        &Fact::new(
                            SubjectID((i % 100) as u32),
                            PredicateID((i % 10) as u8),
                            ObjectID((i % 200) as u32),
                            0.5 + (i as f64 * 0.001),
                        )
                        .unwrap(),
                    )
                    .unwrap();
            }
            DatabaseFile::save(&schema, &path).unwrap();
            b.iter(|| black_box(DatabaseFile::load(&path).unwrap()));
        });
    }
    group.finish();
}

fn bench_compression_encode(c: &mut Criterion) {
    use kcm_storage::compress::{Compressor, Lz4Compressor, ZstdCompressor};
    let mut group = c.benchmark_group("compression_encode");
    let data: Vec<u8> = (0..100_000).map(|i| (i % 256) as u8).collect();
    group.bench_function("zstd", |b| {
        let zstd = ZstdCompressor::default_level();
        b.iter(|| black_box(zstd.compress(&data).unwrap()));
    });
    group.bench_function("lz4", |b| {
        let lz4 = Lz4Compressor::default_level();
        b.iter(|| black_box(lz4.compress(&data).unwrap()));
    });
    group.finish();
}

fn bench_compression_decode(c: &mut Criterion) {
    use kcm_storage::compress::{Compressor, Lz4Compressor, ZstdCompressor};
    let mut group = c.benchmark_group("compression_decode");
    let data: Vec<u8> = (0..100_000).map(|i| (i % 256) as u8).collect();
    let zstd = ZstdCompressor::default_level();
    let lz4 = Lz4Compressor::default_level();
    let zstd_compressed = zstd.compress(&data).unwrap();
    let lz4_compressed = lz4.compress(&data).unwrap();
    group.bench_function("zstd", |b| {
        b.iter(|| black_box(zstd.decompress(&zstd_compressed, data.len()).unwrap()));
    });
    group.bench_function("lz4", |b| {
        b.iter(|| black_box(lz4.decompress(&lz4_compressed, data.len()).unwrap()));
    });
    group.finish();
}

fn bench_codec_encode(c: &mut Criterion) {
    use kcm_storage::codec::{Codec, DeltaCodec, GorillaCodec, RleCodec};
    let mut group = c.benchmark_group("codec_encode");
    let delta_data: Vec<i64> = (0..100_000).map(|i| (i * 7 % 500) as i64).collect();
    let gorilla_data: Vec<f64> = (0..100_000).map(|i| 1.0 + i as f64 * 0.001).collect();
    let rle_data: Vec<u8> = (0..100_000).map(|i| (i % 10) as u8).collect();
    group.bench_function("delta", |b| {
        let codec = DeltaCodec;
        b.iter(|| black_box(codec.encode(&delta_data).unwrap()));
    });
    group.bench_function("gorilla", |b| {
        let codec = GorillaCodec;
        b.iter(|| black_box(codec.encode(&gorilla_data).unwrap()));
    });
    group.bench_function("rle", |b| {
        let codec = RleCodec;
        b.iter(|| black_box(codec.encode(&rle_data).unwrap()));
    });
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
            for i in 0..10_000 {
                black_box(hash.get_shard_id(i, 16));
            }
        });
    });
    group.bench_function("range_routing", |b| {
        b.iter(|| {
            for i in 0..10_000 {
                black_box(range.get_shard_id(i, 4));
            }
        });
    });
    group.bench_function("consistent_hash_routing", |b| {
        b.iter(|| {
            for i in 0..10_000 {
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
        b.iter(|| {
            let kb = KnowledgeDatabase::new().unwrap();
            for i in 0..100_000 {
                let fact = Fact::new(
                    SubjectID((i % 1000) as u32),
                    PredicateID((i % 10) as u8),
                    ObjectID((i % 2000) as u32),
                    0.75,
                )
                .unwrap();
                kb.insert(&fact).unwrap();
            }
            let snap = kb.fact_count();
            black_box(snap)
        });
    });
    group.bench_function("bitmap_memory_1m", |b| {
        b.iter(|| {
            let mut bitmap = Bitmap::new(1_000_000);
            for i in (0..1_000_000).step_by(7) {
                bitmap.set(i);
            }
            black_box(bitmap.count_ones())
        });
    });
    group.bench_function("dictionary_memory_100k", |b| {
        b.iter(|| {
            let mut dict = Dictionary::new();
            for i in 0..100_000 {
                dict.insert(&format!("key_{}", i));
            }
            black_box(dict.len())
        });
    });
    group.bench_function("dense_vec_memory_1m", |b| {
        b.iter(|| {
            let mut vec: DenseVec<u64> = DenseVec::new(1_000_000).unwrap();
            for i in 0..1_000_000u64 {
                vec.push(i).unwrap();
            }
            black_box(vec.len())
        });
    });
    group.finish();
}

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
    bench_confidence_calculation,
    bench_inference_full_engine,
    bench_rule_registry,
    bench_wal_append,
    bench_wal_replay,
    bench_file_format_save_load,
    bench_compression_encode,
    bench_compression_decode,
    bench_codec_encode,
    bench_sharding,
    bench_memory_metrics,
);

criterion_main!(benches);
