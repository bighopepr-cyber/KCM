use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use kcm_core::bitmap::Bitmap;
use kcm_core::dictionary::Dictionary;
use kcm_core::types::*;
use kcm_core::vec::DenseVec;
use kcm_runtime::database::KnowledgeDatabase;

fn bench_column_sequential_scan(c: &mut Criterion) {
    let mut group = c.benchmark_group("column_sequential_scan");
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
    let mut group = c.benchmark_group("column_simd_filter");
    for size in &[10_000, 100_000, 1_000_000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut vec: DenseVec<u8> = DenseVec::new(size).unwrap();
            for i in 0..size {
                vec.push((i % 256) as u8).unwrap();
            }
            b.iter(|| black_box(vec.iter().filter(|&&v| v > 128).count()));
        });
    }
    group.finish();
}

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
    for size in &[100_000, 1_000_000] {
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

fn bench_database_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("database_insert");
    for batch in &[100, 1_000, 10_000] {
        group.bench_with_input(BenchmarkId::from_parameter(batch), batch, |b, &batch| {
            b.iter_batched(
                || KnowledgeDatabase::new().unwrap(),
                |kb| {
                    for i in 0..*batch {
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
    for dataset in &[1_000, 10_000, 100_000] {
        group.bench_with_input(
            BenchmarkId::from_parameter(dataset),
            dataset,
            |b, &dataset| {
                let kb = KnowledgeDatabase::new().unwrap();
                for i in 0..*dataset {
                    Fact::new(
                        SubjectID((i % 100) as u32),
                        PredicateID((i % 10) as u8),
                        ObjectID((i % 200) as u32),
                        0.75,
                    )
                    .and_then(|f| kb.insert(&f))
                    .unwrap();
                }
                b.iter(|| black_box(kb.query().with_predicate(PredicateID(5)).execute().unwrap()));
            },
        );
    }
    group.finish();
}

fn bench_inference_pattern_matching(c: &mut Criterion) {
    let mut group = c.benchmark_group("inference_pattern_matching");
    for dataset in &[1_000, 10_000, 100_000] {
        group.bench_with_input(
            BenchmarkId::from_parameter(dataset),
            dataset,
            |b, &schema_size| {
                let mut schema = kcm_storage::Schema::new(*schema_size).unwrap();
                for i in 0..*schema_size {
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
                    let mut matches = 0usize;
                    for i in 0..schema.len() {
                        if schema.predicate_col.get(i) == Some(5) {
                            matches += 1;
                        }
                    }
                    black_box(matches)
                });
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_column_sequential_scan,
    bench_column_random_access,
    bench_column_simd_filter,
    bench_bitmap_set,
    bench_bitmap_count,
    bench_bitmap_bitwise,
    bench_dictionary_insert,
    bench_dictionary_lookup,
    bench_database_insert,
    bench_database_query,
    bench_inference_pattern_matching,
);

criterion_main!(benches);
