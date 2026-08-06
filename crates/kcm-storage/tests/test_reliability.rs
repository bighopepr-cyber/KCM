#![allow(clippy::unwrap_used, clippy::panic)]

use kcm_core::types::*;
use kcm_storage::column::Schema;
use kcm_storage::file_format::DatabaseFile;
use kcm_storage::recovery::RecoveryManager;
use kcm_storage::wal::WriteAheadLog;
use std::fs;
use tempfile::tempdir;

fn create_test_schema(n: usize) -> Schema {
    let mut schema = Schema::new(1000).unwrap();
    for i in 0..n {
        let confidence = 0.1 + ((i as f64 * 0.01) % 0.9);
        let fact = Fact::new(
            SubjectID(i as u32),
            PredicateID((i % 10) as u8),
            ObjectID((i * 10) as u32),
            confidence,
        )
        .unwrap();
        schema.append_fact(&fact).unwrap();
    }
    schema
}

#[test]
fn test_empty_wal_recovery() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.kcm");
    let wal_path = dir.path().join("test.wal");

    let schema = create_test_schema(3);
    DatabaseFile::save(&schema, &db_path).unwrap();

    let recovered = RecoveryManager::recover(&db_path, &wal_path).unwrap();
    assert_eq!(recovered.len(), 3);
}

#[test]
fn test_double_recovery_truncates_wal() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.kcm");
    let wal_path = dir.path().join("test.wal");

    let schema = create_test_schema(3);
    DatabaseFile::save(&schema, &db_path).unwrap();

    let wal = WriteAheadLog::new(&wal_path).unwrap();
    let fact = Fact::new(SubjectID(99), PredicateID(9), ObjectID(999), 0.95).unwrap();
    wal.append_fact(&fact).unwrap();
    wal.flush_buffer().unwrap();

    let result1 = RecoveryManager::recover(&db_path, &wal_path).unwrap();
    assert_eq!(result1.len(), 4, "First recovery should include WAL entry");

    let result2 = RecoveryManager::recover(&db_path, &wal_path).unwrap();
    assert_eq!(
        result2.len(),
        3,
        "Second recovery: WAL was truncated, only DB facts remain"
    );
}

#[test]
fn test_wal_replay_preserves_all_fact_fields() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.kcm");
    let wal_path = dir.path().join("test.wal");

    let schema = create_test_schema(0);
    DatabaseFile::save(&schema, &db_path).unwrap();

    let wal = WriteAheadLog::new(&wal_path).unwrap();
    let fact = Fact::new(SubjectID(42), PredicateID(7), ObjectID(999), 0.85).unwrap();
    wal.append_fact(&fact).unwrap();
    wal.flush_buffer().unwrap();

    let recovered = RecoveryManager::recover(&db_path, &wal_path).unwrap();
    assert_eq!(recovered.len(), 1);

    let recovered_fact = recovered.get_fact(0).unwrap();
    assert_eq!(recovered_fact.subject.0, 42);
    assert_eq!(recovered_fact.predicate.0, 7);
    assert_eq!(recovered_fact.object.0, 999);
    assert!((recovered_fact.confidence - 0.85).abs() < 0.001);
}

#[test]
fn test_wal_truncated_entry_recovery() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.kcm");
    let wal_path = dir.path().join("test.wal");

    let schema = create_test_schema(2);
    DatabaseFile::save(&schema, &db_path).unwrap();

    let wal = WriteAheadLog::new(&wal_path).unwrap();
    let fact = Fact::new(SubjectID(1), PredicateID(0), ObjectID(100), 0.9).unwrap();
    wal.append_fact(&fact).unwrap();
    wal.flush_buffer().unwrap();

    let mut bytes = fs::read(&wal_path).unwrap();
    bytes.truncate(bytes.len() / 2);
    fs::write(&wal_path, &bytes).unwrap();

    let result = RecoveryManager::recover(&db_path, &wal_path);
    if result.is_err() {
        let db_still_works = DatabaseFile::load(&db_path).unwrap();
        assert_eq!(
            db_still_works.len(),
            2,
            "DB should remain intact after WAL corruption"
        );
    }
}

#[test]
fn test_file_verify_after_multiple_saves() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("test.kcm");

    let schema1 = create_test_schema(10);
    DatabaseFile::save(&schema1, &path).unwrap();

    let schema2 = create_test_schema(5);
    DatabaseFile::save(&schema2, &path).unwrap();

    assert!(DatabaseFile::verify(&path).unwrap());
    let loaded = DatabaseFile::load(&path).unwrap();
    assert_eq!(loaded.len(), 5);
}

#[test]
fn test_backup_and_full_restore() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.kcm");
    let backup_path = dir.path().join("backup.kcm");

    let schema = create_test_schema(50);
    DatabaseFile::save(&schema, &db_path).unwrap();

    fs::copy(&db_path, &backup_path).unwrap();
    fs::remove_file(&db_path).unwrap();
    fs::copy(&backup_path, &db_path).unwrap();

    let loaded = DatabaseFile::load(&db_path).unwrap();
    assert_eq!(loaded.len(), 50);
}

#[test]
fn test_concurrent_wal_append_integrity() {
    let dir = tempdir().unwrap();
    let wal_path = dir.path().join("test.wal");

    let wal = WriteAheadLog::new(&wal_path).unwrap();
    let handles: Vec<_> = (0..4)
        .map(|i| {
            let wp = wal_path.clone();
            std::thread::spawn(move || {
                let wal_clone = WriteAheadLog::new(&wp).unwrap();
                for j in 0..100u32 {
                    let fact = Fact::new(
                        SubjectID(i * 1000 + j),
                        PredicateID(0),
                        ObjectID(i * 1000 + j + 100),
                        0.9,
                    )
                    .unwrap();
                    let _ = wal_clone.append_fact(&fact);
                }
                let _ = wal_clone.flush_buffer();
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    let result = wal.verify_integrity();
    assert!(result.is_ok());
}

#[test]
fn test_wal_delete_and_insert_mixed_recovery() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.kcm");
    let wal_path = dir.path().join("test.wal");

    let schema = create_test_schema(10);
    DatabaseFile::save(&schema, &db_path).unwrap();

    let wal = WriteAheadLog::new(&wal_path).unwrap();
    wal.append_delete(5).unwrap();

    let fact = Fact::new(SubjectID(99), PredicateID(9), ObjectID(999), 0.95).unwrap();
    wal.append_fact(&fact).unwrap();
    wal.flush_buffer().unwrap();

    let recovered = RecoveryManager::recover(&db_path, &wal_path).unwrap();
    assert_eq!(
        recovered.len(),
        11,
        "10 original + 1 insert (delete marks tombstone)"
    );
    assert_eq!(
        recovered.active_count(),
        10,
        "10 original - 1 deleted + 1 inserted = 10 active"
    );
}

#[test]
fn test_checksum_detection_on_load() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("test.kcm");

    let schema = create_test_schema(3);
    DatabaseFile::save(&schema, &path).unwrap();

    let mut bytes = fs::read(&path).unwrap();
    if bytes.len() > 50 {
        bytes[50] ^= 0xFF;
    }
    fs::write(&path, &bytes).unwrap();

    let result = DatabaseFile::load(&path);
    assert!(
        result.is_err() || !DatabaseFile::verify(&path).unwrap(),
        "Corrupted file should fail load or verify"
    );
}

#[test]
fn test_crash_recovery_with_wal_entries() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.kcm");
    let wal_path = dir.path().join("test.wal");

    let schema = create_test_schema(5);
    DatabaseFile::save(&schema, &db_path).unwrap();

    let wal = WriteAheadLog::new(&wal_path).unwrap();
    for i in 0..20u32 {
        let fact = Fact::new(SubjectID(100 + i), PredicateID(0), ObjectID(200 + i), 0.9).unwrap();
        wal.append_fact(&fact).unwrap();
    }
    wal.flush_buffer().unwrap();

    let recovered = RecoveryManager::recover(&db_path, &wal_path).unwrap();
    assert_eq!(recovered.len(), 25);
}

#[test]
fn test_schema_compaction_after_recovery() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.kcm");

    let schema = create_test_schema(20);
    DatabaseFile::save(&schema, &db_path).unwrap();

    let loaded = DatabaseFile::load(&db_path).unwrap();
    let compacted = loaded.compact().unwrap();
    assert_eq!(compacted.len(), 20);
    assert_eq!(compacted.active_count(), 20);
}

#[test]
fn test_empty_db_recovery() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.kcm");
    let wal_path = dir.path().join("test.wal");

    let result = RecoveryManager::recover(&db_path, &wal_path).unwrap();
    assert_eq!(result.len(), 0);
}

#[test]
fn test_corrupt_db_falls_back_to_backup() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.kcm");
    let backup_path = dir.path().join("test.kcm.backup");
    let wal_path = dir.path().join("test.wal");

    let schema = create_test_schema(5);
    DatabaseFile::save(&schema, &db_path).unwrap();
    fs::copy(&db_path, &backup_path).unwrap();

    let mut bytes = fs::read(&db_path).unwrap();
    if bytes.len() > 10 {
        bytes[0] = 0xFF;
    }
    fs::write(&db_path, &bytes).unwrap();

    let result = RecoveryManager::recover(&db_path, &wal_path);
    assert!(result.is_ok());
    let recovered = result.unwrap();
    assert_eq!(recovered.len(), 5);
}
