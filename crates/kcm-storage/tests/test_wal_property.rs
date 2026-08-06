#![allow(clippy::unwrap_used, clippy::panic)]
use kcm_core::types::*;
use kcm_storage::wal::WriteAheadLog;

#[test]
fn test_wal_concurrent_appends_deterministic() {
    use std::sync::Arc;
    let dir = tempfile::tempdir().unwrap();
    let wal_path = dir.path().join("concurrent.wal");
    let wal = Arc::new(WriteAheadLog::new(&wal_path).unwrap());

    let mut handles = Vec::new();
    for t in 0..4 {
        let wal = wal.clone();
        handles.push(std::thread::spawn(move || {
            for i in 0..100u32 {
                let fact =
                    Fact::new(SubjectID(t * 100 + i), PredicateID(0), ObjectID(i), 0.9).unwrap();
                wal.append_fact(&fact).unwrap();
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    wal.flush_buffer().unwrap();

    let mut count = 0u64;
    WriteAheadLog::new(&wal_path)
        .unwrap()
        .replay(|_| {
            count += 1;
            Ok(())
        })
        .unwrap();
    assert_eq!(count, 400);
}

#[test]
fn test_wal_empty_file_handling() {
    let dir = tempfile::tempdir().unwrap();
    let wal_path = dir.path().join("empty.wal");
    WriteAheadLog::new(&wal_path).unwrap();

    let mut count = 0u64;
    WriteAheadLog::new(&wal_path)
        .unwrap()
        .replay(|_| {
            count += 1;
            Ok(())
        })
        .unwrap();
    assert_eq!(count, 0);
}

#[test]
fn test_wal_delete_roundtrip() {
    let dir = tempfile::tempdir().unwrap();
    let wal_path = dir.path().join("del.wal");
    let wal = WriteAheadLog::new(&wal_path).unwrap();

    let fact = Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap();
    wal.append_fact(&fact).unwrap();
    wal.append_delete(0).unwrap();
    wal.flush_buffer().unwrap();

    let mut ops = Vec::new();
    WriteAheadLog::new(&wal_path)
        .unwrap()
        .replay(|entry| {
            match entry {
                kcm_storage::wal::WALEntry::Insert { .. } => ops.push("insert"),
                kcm_storage::wal::WALEntry::Delete { .. } => ops.push("delete"),
            }
            Ok(())
        })
        .unwrap();

    assert_eq!(ops, vec!["insert", "delete"]);
}

#[test]
fn test_wal_checksum_verify() {
    let dir = tempfile::tempdir().unwrap();
    let wal_path = dir.path().join("check.wal");
    let wal = WriteAheadLog::new(&wal_path).unwrap();

    for i in 0..50u32 {
        let fact = Fact::new(SubjectID(i), PredicateID(0), ObjectID(i), 0.9).unwrap();
        wal.append_fact(&fact).unwrap();
    }
    wal.flush_buffer().unwrap();

    let wal2 = WriteAheadLog::new(&wal_path).unwrap();
    assert!(wal2.verify_integrity().is_ok());
}
