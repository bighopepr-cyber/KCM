use criterion::{black_box, criterion_group, criterion_main, Criterion};
use kcm_core::bitmap::Bitmap;
use kcm_core::types::*;
use kcm_core::vec::DenseVec;
use kcm_runtime::database::KnowledgeDatabase;

fn bench_dense_vec_allocation(c: &mut Criterion) {
    c.bench_function("dense_vec_1m_allocation", |b| {
        b.iter(|| black_box(DenseVec::<u64>::new(1_000_000).unwrap()));
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

    c.bench_function("bitmap_get_1m", |b| {
        let mut bitmap = Bitmap::new(1_000_000);
        for i in 0..1_000_000 {
            bitmap.set(i);
        }
        b.iter(|| {
            for i in 0..1_000_000 {
                black_box(bitmap.get(i));
            }
        });
    });

    c.bench_function("bitmap_and_1m", |b| {
        let mut a = Bitmap::new(1_000_000);
        let mut b_map = Bitmap::new(1_000_000);
        for i in 0..1_000_000 {
            a.set(i);
            if i % 2 == 0 {
                b_map.set(i);
            }
        }
        b.iter(|| {
            a.and_inplace(&b_map);
        });
    });
}

fn bench_insert_1k(c: &mut Criterion) {
    c.bench_function("insert_1k_facts", |b| {
        b.iter(|| {
            let kb = KnowledgeDatabase::new().unwrap();
            for i in 0..1000 {
                let fact = Fact::new(
                    SubjectID(i % 100),
                    PredicateID((i % 10) as u8),
                    ObjectID((i % 200) as u32),
                    0.5 + (i as f64 % 0.5),
                )
                .unwrap();
                kb.insert(&fact).unwrap();
            }
        });
    });
}

fn bench_insert_10k(c: &mut Criterion) {
    c.bench_function("insert_10k_facts", |b| {
        b.iter(|| {
            let kb = KnowledgeDatabase::new().unwrap();
            for i in 0..10_000 {
                let fact = Fact::new(
                    SubjectID(i % 1000),
                    PredicateID((i % 10) as u8),
                    ObjectID((i % 2000) as u32),
                    0.5 + (i as f64 % 0.5),
                )
                .unwrap();
                kb.insert(&fact).unwrap();
            }
        });
    });
}

fn bench_query_1k(c: &mut Criterion) {
    let kb = KnowledgeDatabase::new().unwrap();
    for i in 0..1000 {
        let fact = Fact::new(
            SubjectID(i % 100),
            PredicateID((i % 10) as u8),
            ObjectID((i % 200) as u32),
            0.75,
        )
        .unwrap();
        kb.insert(&fact).unwrap();
    }

    c.bench_function("query_1k_facts", |b| {
        b.iter(|| black_box(kb.query().execute().unwrap()));
    });
}

fn bench_query_filtered(c: &mut Criterion) {
    let kb = KnowledgeDatabase::new().unwrap();
    for i in 0..1000 {
        let fact = Fact::new(
            SubjectID(i % 100),
            PredicateID((i % 10) as u8),
            ObjectID((i % 200) as u32),
            0.1 + (i as f64 * 0.001),
        )
        .unwrap();
        kb.insert(&fact).unwrap();
    }

    c.bench_function("query_filtered_1k", |b| {
        b.iter(|| {
            black_box(
                kb.query()
                    .with_subject(SubjectID(50))
                    .with_confidence(0.5)
                    .execute()
                    .unwrap(),
            )
        });
    });
}

fn bench_batch_insert(c: &mut Criterion) {
    c.bench_function("batch_insert_1k", |b| {
        b.iter(|| {
            let kb = KnowledgeDatabase::new().unwrap();
            let facts: Vec<Fact> = (0..1000)
                .map(|i| {
                    Fact::new(
                        SubjectID(i % 100),
                        PredicateID((i % 10) as u8),
                        ObjectID((i % 200) as u32),
                        0.75,
                    )
                    .unwrap()
                })
                .collect();
            kb.insert_batch(&facts).unwrap();
        });
    });
}

fn bench_update(c: &mut Criterion) {
    let kb = KnowledgeDatabase::new().unwrap();
    let mut row_ids = Vec::new();
    for i in 0..1000 {
        let fact = Fact::new(
            SubjectID(i % 100),
            PredicateID((i % 10) as u8),
            ObjectID((i % 200) as u32),
            0.75,
        )
        .unwrap();
        row_ids.push(kb.insert(&fact).unwrap());
    }

    c.bench_function("update_1k_facts", |b| {
        let mut idx = 0;
        b.iter(|| {
            let fact = Fact::new(SubjectID(999), PredicateID(9), ObjectID(9999), 0.99).unwrap();
            kb.update(row_ids[idx % 1000], &fact).unwrap();
            idx += 1;
        });
    });
}

fn bench_delete(c: &mut Criterion) {
    c.bench_function("delete_1k_facts", |b| {
        b.iter(|| {
            let kb = KnowledgeDatabase::new().unwrap();
            let mut row_ids = Vec::new();
            for i in 0..1000 {
                let fact = Fact::new(
                    SubjectID(i % 100),
                    PredicateID((i % 10) as u8),
                    ObjectID((i % 200) as u32),
                    0.75,
                )
                .unwrap();
                row_ids.push(kb.insert(&fact).unwrap());
            }
            for row_id in &row_ids {
                kb.delete(*row_id).unwrap();
            }
        });
    });
}

criterion_group!(
    benches,
    bench_dense_vec_allocation,
    bench_bitmap_operations,
    bench_insert_1k,
    bench_insert_10k,
    bench_query_1k,
    bench_query_filtered,
    bench_batch_insert,
    bench_update,
    bench_delete
);

criterion_main!(benches);
