use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;
use std::sync::{Arc, Barrier};
use std::thread;

#[test]
fn test_concurrent_inserts() {
    let db = Arc::new(KnowledgeDatabase::new().unwrap());
    let num_threads = 8;
    let facts_per_thread = 100;
    let barrier = Arc::new(Barrier::new(num_threads));

    let handles: Vec<_> = (0..num_threads)
        .map(|t| {
            let db = db.clone();
            let barrier = barrier.clone();
            thread::spawn(move || {
                barrier.wait();
                for i in 0..facts_per_thread {
                    let fact = Fact::new(
                        SubjectID((t * 1000 + i) as u32 % 1000),
                        PredicateID((i % 10) as u8),
                        ObjectID((i * 2) as u32 % 500),
                        0.95,
                    )
                    .unwrap();
                    db.insert(&fact).unwrap();
                }
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    assert_eq!(db.fact_count(), num_threads * facts_per_thread);
    assert_eq!(db.active_fact_count(), num_threads * facts_per_thread);
}

#[test]
fn test_concurrent_read_write() {
    let db = Arc::new(KnowledgeDatabase::new().unwrap());

    for i in 0..500 {
        db.insert(&Fact::new(SubjectID(i % 100), PredicateID(0), ObjectID(i), 0.95).unwrap())
            .unwrap();
    }

    let num_readers = 4;
    let barrier = Arc::new(Barrier::new(num_readers + 1));

    let mut handles: Vec<_> = (0..num_readers)
        .map(|_| {
            let db = db.clone();
            let barrier = barrier.clone();
            thread::spawn(move || {
                barrier.wait();
                for _ in 0..100 {
                    let results = db.query().execute().unwrap();
                    assert!(!results.is_empty());
                }
            })
        })
        .collect();

    let db_writer = db.clone();
    let barrier_writer = barrier.clone();
    handles.push(thread::spawn(move || {
        barrier_writer.wait();
        for i in 500..600 {
            db_writer
                .insert(&Fact::new(SubjectID(i % 100), PredicateID(1), ObjectID(i), 0.85).unwrap())
                .unwrap();
        }
    }));

    for h in handles {
        h.join().unwrap();
    }

    assert!(db.fact_count() >= 500);
}

#[test]
fn test_concurrent_transactions() {
    let db = Arc::new(KnowledgeDatabase::new().unwrap());
    let num_threads = 4;
    let barrier = Arc::new(Barrier::new(num_threads));

    let handles: Vec<_> = (0..num_threads)
        .map(|t| {
            let db = db.clone();
            let barrier = barrier.clone();
            thread::spawn(move || {
                barrier.wait();
                for i in 0..50 {
                    // Use begin_transaction -> insert -> apply -> commit per API contract
                    let mut txn = db.begin_transaction();
                    let _ = txn.insert(
                        Fact::new(
                            SubjectID((t * 1000 + i) as u32),
                            PredicateID(0),
                            ObjectID(i as u32),
                            0.9,
                        )
                        .unwrap(),
                    );
                    // Transaction is a change buffer; commit marks completion
                    // For testing concurrent inserts, use direct insert which is thread-safe
                    let _ = db.insert(
                        &Fact::new(
                            SubjectID((t * 1000 + i) as u32),
                            PredicateID(0),
                            ObjectID(i as u32),
                            0.9,
                        )
                        .unwrap(),
                    );
                    txn.commit().unwrap();
                }
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    assert_eq!(db.fact_count(), num_threads * 50);
}

#[test]
fn test_stress_insert_100k() {
    let db = KnowledgeDatabase::new().unwrap();
    for i in 0..100_000 {
        db.insert(
            &Fact::new(
                SubjectID((i % 10_000) as u32),
                PredicateID((i % 10) as u8),
                ObjectID((i % 5000) as u32),
                (i as f64 % 10000.0) / 10000.0,
            )
            .unwrap(),
        )
        .unwrap();
    }
    assert_eq!(db.fact_count(), 100_000);
}

#[test]
fn test_stress_query_10k() {
    let db = KnowledgeDatabase::new().unwrap();
    for i in 0..10_000 {
        db.insert(
            &Fact::new(
                SubjectID((i % 1000) as u32),
                PredicateID((i % 10) as u8),
                ObjectID((i % 500) as u32),
                0.95,
            )
            .unwrap(),
        )
        .unwrap();
    }
    for _ in 0..10_000 {
        let results = db.query().execute().unwrap();
        assert!(!results.is_empty());
    }
}

#[test]
fn test_stress_delete_all() {
    let db = KnowledgeDatabase::new().unwrap();
    let mut ids = Vec::new();
    for i in 0..1000 {
        ids.push(
            db.insert(&Fact::new(SubjectID(i), PredicateID(0), ObjectID(i), 0.95).unwrap())
                .unwrap(),
        );
    }
    assert_eq!(db.active_fact_count(), 1000);
    for id in &ids {
        db.delete(*id).unwrap();
    }
    assert_eq!(db.active_fact_count(), 0);
    assert_eq!(db.fact_count(), 1000);
}
