#![allow(clippy::unwrap_used, clippy::panic)]
use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

#[test]
fn test_massive_insert_100k_sequential() {
    let kb = KnowledgeDatabase::new().unwrap();
    let start = Instant::now();
    for i in 0..100_000u32 {
        let fact = Fact::new(
            SubjectID(i % 1000),
            PredicateID(0),
            ObjectID(i),
            0.5 + (i as f64 * 0.000005).min(0.99),
        )
        .unwrap();
        kb.insert(&fact).unwrap();
    }
    let elapsed = start.elapsed();
    assert_eq!(kb.fact_count(), 100_000);
    println!("100K sequential insert: {:?}", elapsed);
    assert!(elapsed.as_millis() < 60_000, "Took too long: {:?}", elapsed);
}

#[test]
fn test_massive_concurrent_insert_100k() {
    let kb = Arc::new(KnowledgeDatabase::new().unwrap());
    let count = Arc::new(AtomicU64::new(0));
    let mut handles = Vec::new();
    let start = Instant::now();
    for t in 0..10u32 {
        let kb = kb.clone();
        let count = count.clone();
        handles.push(std::thread::spawn(move || {
            for i in 0..10_000u32 {
                let fact = Fact::new(
                    SubjectID(t * 10_000 + i % 1000),
                    PredicateID(0),
                    ObjectID(i),
                    0.5,
                )
                .unwrap();
                if kb.insert(&fact).is_ok() {
                    count.fetch_add(1, Ordering::Relaxed);
                }
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    let elapsed = start.elapsed();
    let total = count.load(Ordering::Relaxed);
    assert!(total >= 100_000, "Expected at least 100K, got {}", total);
    println!("100K concurrent insert (10 threads): {:?}", elapsed);
}

#[test]
fn test_mixed_workload_10k() {
    let kb = Arc::new(KnowledgeDatabase::new().unwrap());
    for i in 0..10_000u32 {
        let fact = Fact::new(SubjectID(i % 100), PredicateID(0), ObjectID(i), 0.8).unwrap();
        kb.insert(&fact).unwrap();
    }

    let mut handles = Vec::new();
    for _ in 0..5 {
        let kb = kb.clone();
        handles.push(std::thread::spawn(move || {
            for _ in 0..1000 {
                let _ = kb.query().with_predicate(PredicateID(0)).execute();
            }
        }));
    }
    for t in 0..5u32 {
        let kb = kb.clone();
        handles.push(std::thread::spawn(move || {
            for i in 0..1000u32 {
                let fact = Fact::new(
                    SubjectID(10_000 + t * 1000 + i),
                    PredicateID(1),
                    ObjectID(i),
                    0.9,
                )
                .unwrap();
                kb.insert(&fact).unwrap();
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
    assert!(kb.fact_count() >= 10_000);
}

#[test]
fn test_delete_reclaim_cycle() {
    let kb = KnowledgeDatabase::new().unwrap();
    for i in 0..50_000u32 {
        let fact = Fact::new(SubjectID(i % 1000), PredicateID(0), ObjectID(i), 0.8).unwrap();
        kb.insert(&fact).unwrap();
    }
    assert_eq!(kb.active_fact_count(), 50_000);

    for i in 0..25_000u64 {
        kb.delete(RowID(i)).unwrap();
    }
    assert_eq!(kb.active_fact_count(), 25_000);
    assert_eq!(kb.fact_count(), 50_000);

    let compacted = kb.compact().unwrap();
    assert_eq!(compacted.fact_count(), 25_000);
    assert_eq!(compacted.active_fact_count(), 25_000);
}

#[test]
fn test_persistence_crash_recovery() {
    use kcm_storage::column::Schema;
    use kcm_storage::file_format::DatabaseFile;
    use kcm_storage::recovery::RecoveryManager;
    use kcm_storage::wal::WriteAheadLog;

    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.kcm");
    let wal_path = dir.path().join("test.wal");

    let mut schema = Schema::new(1000).unwrap();
    for i in 0..100u32 {
        schema
            .append_fact(&Fact::new(SubjectID(i), PredicateID(0), ObjectID(i), 0.9).unwrap())
            .unwrap();
    }
    DatabaseFile::save(&schema, &db_path).unwrap();

    {
        let wal = WriteAheadLog::new(&wal_path).unwrap();
        for i in 100..200u32 {
            let fact = Fact::new(SubjectID(i), PredicateID(1), ObjectID(i), 0.8).unwrap();
            wal.append_fact(&fact).unwrap();
        }
        wal.flush_buffer().unwrap();
    }

    let recovered = RecoveryManager::recover(&db_path, &wal_path).unwrap();
    assert_eq!(recovered.len(), 200);

    assert!(std::fs::read(&wal_path).unwrap().is_empty());
}

#[test]
fn test_wal_checksum_detection() {
    use kcm_storage::wal::WriteAheadLog;

    let dir = tempfile::tempdir().unwrap();
    let wal_path = dir.path().join("test.wal");

    {
        let wal = WriteAheadLog::new(&wal_path).unwrap();
        for i in 0..10u32 {
            let fact = Fact::new(SubjectID(i), PredicateID(0), ObjectID(i), 0.9).unwrap();
            wal.append_fact(&fact).unwrap();
        }
        wal.flush_buffer().unwrap();
    }

    let mut data = std::fs::read(&wal_path).unwrap();
    if data.len() > 40 {
        data[40] ^= 0xFF;
        std::fs::write(&wal_path, &data).unwrap();
    }

    let wal = WriteAheadLog::new(&wal_path).unwrap();
    let result = wal.verify_integrity();
    assert!(result.is_err());
}
