#![allow(clippy::unwrap_used, clippy::panic)]
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;

fn bench_batch_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_insert");
    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter_batched(
                || {
                    let facts: Vec<Fact> = (0..size)
                        .map(|i| {
                            Fact::new(
                                SubjectID((i % 1000) as u32),
                                PredicateID((i % 10) as u8),
                                ObjectID((i % 500) as u32),
                                (i as f64 % 1000.0) / 1000.0,
                            )
                            .unwrap()
                        })
                        .collect();
                    (KnowledgeDatabase::new().unwrap(), facts)
                },
                |(db, facts)| {
                    for fact in &facts {
                        db.insert(fact).unwrap();
                    }
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_filtered_query(c: &mut Criterion) {
    let mut group = c.benchmark_group("filtered_query");
    let db = KnowledgeDatabase::new().unwrap();
    for i in 0..10_000 {
        db.insert(
            &Fact::new(
                SubjectID((i % 1000) as u32),
                PredicateID((i % 10) as u8),
                ObjectID((i % 500) as u32),
                (i as f64 % 1000.0) / 1000.0,
            )
            .unwrap(),
        )
        .unwrap();
    }

    group.bench_function("subject_filter", |b| {
        b.iter(|| db.query().with_subject(SubjectID(1)).execute().unwrap());
    });

    group.bench_function("predicate_filter", |b| {
        b.iter(|| db.query().with_predicate(PredicateID(5)).execute().unwrap());
    });

    group.bench_function("combined_filter", |b| {
        b.iter(|| {
            db.query()
                .with_subject(SubjectID(1))
                .with_predicate(PredicateID(5))
                .execute()
                .unwrap()
        });
    });

    group.finish();
}

fn bench_concurrent_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_ops");
    group.bench_function("single_thread_insert_1000", |b| {
        b.iter_batched(
            || KnowledgeDatabase::new().unwrap(),
            |db| {
                for i in 0..1000 {
                    db.insert(
                        &Fact::new(
                            SubjectID((i % 100) as u32),
                            PredicateID(0),
                            ObjectID(i as u32),
                            0.95,
                        )
                        .unwrap(),
                    )
                    .unwrap();
                }
            },
            criterion::BatchSize::SmallInput,
        );
    });

    group.bench_function("single_thread_query_1000", |b| {
        let db = KnowledgeDatabase::new().unwrap();
        for i in 0..1000 {
            db.insert(
                &Fact::new(
                    SubjectID((i % 100) as u32),
                    PredicateID(0),
                    ObjectID(i as u32),
                    0.95,
                )
                .unwrap(),
            )
            .unwrap();
        }
        b.iter(|| {
            for _ in 0..1000 {
                db.query().execute().unwrap();
            }
        });
    });

    group.finish();
}

fn bench_dictionary(c: &mut Criterion) {
    let mut group = c.benchmark_group("dictionary");
    group.bench_function("insert_1000_subjects", |b| {
        b.iter_batched(
            || KnowledgeDatabase::new().unwrap(),
            |db| {
                for i in 0..1000 {
                    let _ = db.dict_insert_subject(&format!("subject_{}", i));
                }
            },
            criterion::BatchSize::SmallInput,
        );
    });

    group.bench_function("lookup_1000_subjects", |b| {
        let db = KnowledgeDatabase::new().unwrap();
        for i in 0..1000 {
            let _ = db.dict_insert_subject(&format!("subject_{}", i));
        }
        b.iter(|| {
            for i in 0..1000 {
                let _ = db.dict_lookup_subject(&format!("subject_{}", i));
            }
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_batch_insert,
    bench_filtered_query,
    bench_concurrent_ops,
    bench_dictionary,
);
criterion_main!(benches);
