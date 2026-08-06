#![allow(clippy::unwrap_used, clippy::panic)]
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;

fn bench_backup_cycle(c: &mut Criterion) {
    let mut group = c.benchmark_group("backup_cycle");

    for &size in &[1000, 10000] {
        group.bench_with_input(
            BenchmarkId::new("insert_compact_save", size),
            &size,
            |b, &size| {
                b.iter_batched(
                    || {
                        let db = KnowledgeDatabase::new().unwrap();
                        for i in 0..size {
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
                        db
                    },
                    |db| {
                        let _ = db.compact();
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }
    group.finish();
}

fn bench_delete_reinsert(c: &mut Criterion) {
    let mut group = c.benchmark_group("delete_reinsert");

    for &size in &[1000, 5000] {
        group.bench_with_input(BenchmarkId::new("delete_all", size), &size, |b, &size| {
            b.iter_batched(
                || {
                    let db = KnowledgeDatabase::new().unwrap();
                    for i in 0..size {
                        db.insert(
                            &Fact::new(
                                SubjectID((i % 1000) as u32),
                                PredicateID(0),
                                ObjectID(i as u32),
                                0.95,
                            )
                            .unwrap(),
                        )
                        .unwrap();
                    }
                    db
                },
                |db| {
                    for i in 0..size as u64 {
                        let _ = db.delete(RowID(i));
                    }
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group!(benches, bench_backup_cycle, bench_delete_reinsert);
criterion_main!(benches);
