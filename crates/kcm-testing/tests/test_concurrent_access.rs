#![allow(clippy::unwrap_used, clippy::panic)]
use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

#[test]
fn test_concurrent_inserts_10_threads() {
    let kb = Arc::new(KnowledgeDatabase::new().unwrap());
    let mut handles = Vec::new();

    for t in 0..10 {
        let kb = kb.clone();
        handles.push(thread::spawn(move || {
            for i in 0..200 {
                let fact =
                    Fact::new(SubjectID(t * 1000 + i), PredicateID(0), ObjectID(i), 0.9).unwrap();
                kb.insert(&fact).unwrap();
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
    assert_eq!(kb.fact_count(), 2000);
}

#[test]
fn test_concurrent_queries() {
    let kb = Arc::new(KnowledgeDatabase::new().unwrap());
    for i in 0..500 {
        let fact = Fact::new(SubjectID(i % 10), PredicateID(0), ObjectID(i), 0.8).unwrap();
        kb.insert(&fact).unwrap();
    }

    let mut handles = Vec::new();
    for _ in 0..8 {
        let kb = kb.clone();
        handles.push(thread::spawn(move || {
            for _ in 0..100 {
                let results = kb.query().with_predicate(PredicateID(0)).execute().unwrap();
                assert_eq!(results.len(), 500);
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
}

#[test]
fn test_concurrent_mixed_read_write() {
    let kb = Arc::new(KnowledgeDatabase::new().unwrap());
    let running = Arc::new(AtomicBool::new(true));

    let mut handles = Vec::new();
    for t in 0..4 {
        let kb = kb.clone();
        let running = running.clone();
        handles.push(thread::spawn(move || {
            let mut count = 0u64;
            while running.load(Ordering::Relaxed) {
                let fact = Fact::new(
                    SubjectID(t * 1000 + count as u32 % 1000),
                    PredicateID(0),
                    ObjectID(count as u32),
                    0.9,
                )
                .unwrap();
                if kb.insert(&fact).is_ok() {
                    count += 1;
                }
            }
            count
        }));
    }

    for _ in 0..4 {
        let kb = kb.clone();
        let running = running.clone();
        handles.push(thread::spawn(move || {
            let mut count = 0u64;
            while running.load(Ordering::Relaxed) {
                let _ = kb.query().execute().unwrap();
                count += 1;
            }
            count
        }));
    }

    thread::sleep(std::time::Duration::from_millis(500));
    running.store(false, Ordering::Relaxed);

    let mut total_writes = 0u64;
    let mut total_reads = 0u64;
    for h in handles {
        let val = h.join().unwrap();
        if total_writes + total_reads < 2000 {
            total_writes += val;
        } else {
            total_reads += val;
        }
    }

    assert!(total_writes > 0, "At least some writes should occur");
    assert!(total_reads > 0, "At least some reads should occur");
}

#[test]
fn test_concurrent_insert_delete() {
    let kb = Arc::new(KnowledgeDatabase::new().unwrap());
    for i in 0..100 {
        let fact = Fact::new(SubjectID(i), PredicateID(0), ObjectID(i), 0.9).unwrap();
        kb.insert(&fact).unwrap();
    }

    let mut handles = Vec::new();
    for t in 0..4 {
        let kb = kb.clone();
        handles.push(thread::spawn(move || {
            for i in 0..50 {
                let fact = Fact::new(
                    SubjectID(1000 + t * 100 + i),
                    PredicateID(1),
                    ObjectID(i),
                    0.8,
                )
                .unwrap();
                kb.insert(&fact).unwrap();
            }
        }));
    }
    for i in (0..100u64).step_by(2) {
        kb.delete(RowID(i)).unwrap();
    }

    for h in handles {
        h.join().unwrap();
    }
    assert!(
        kb.active_fact_count() >= 50,
        "Some original facts should remain active"
    );
    assert!(
        kb.active_fact_count() <= 250,
        "Deleted facts should not be active"
    );
    assert!(kb.fact_count() >= 100);
}

#[test]
fn test_concurrent_dictionary_access() {
    use kcm_core::dictionary::SharedDictionary;
    let dict = Arc::new(SharedDictionary::new());
    let mut handles = Vec::new();

    for t in 0..4 {
        let dict = dict.clone();
        handles.push(thread::spawn(move || {
            for i in 0..500 {
                let id = dict.insert(&format!("t{}_v{}", t, i)).unwrap();
                let val = dict.get(id).unwrap();
                assert!(val.starts_with(&format!("t{}_v{}", t, i)));
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    assert!(dict.len() > 2000);
}

#[test]
fn test_concurrent_metrics() {
    use kcm_runtime::metrics::Metrics;
    let metrics = Arc::new(Metrics::new());
    let mut handles = Vec::new();

    for _ in 0..8 {
        let metrics = metrics.clone();
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                metrics.record_query(10, true);
                metrics.record_insert(true);
                metrics.record_cache_hit();
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }

    let snap = metrics.snapshot();
    assert_eq!(snap.queries_total, 8000);
    assert_eq!(snap.inserts_total, 8000);
    assert!((snap.cache_hit_ratio - 1.0).abs() < f64::EPSILON);
}
