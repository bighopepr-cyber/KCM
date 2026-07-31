use criterion::{black_box, criterion_group, criterion_main, Criterion};
use kcm_core::bitmap::Bitmap;
use kcm_core::types::*;
use kcm_core::vec::DenseVec;
use kcm_runtime::database::KnowledgeDatabase;

fn bench_dense_vec_allocation(c: &mut Criterion) {
    c.bench_function("dense_vec_1m_allocation", |b| {
        b.iter(|| DenseVec::<u64>::new(1_000_000).unwrap());
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
                )
                .unwrap();
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
            )
            .unwrap();
            kb.insert(&fact).unwrap();
        }

        b.iter(|| kb.query().execute().unwrap());
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
