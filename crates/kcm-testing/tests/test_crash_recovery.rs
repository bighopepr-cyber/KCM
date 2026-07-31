use kcm_core::types::*;
use kcm_storage::column::Schema;
use kcm_storage::file_format::DatabaseFile;
use kcm_storage::recovery::RecoveryManager;
use kcm_storage::wal::WriteAheadLog;

#[test]
fn test_crash_recovery_partial_wal() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.kcm");
    let wal_path = dir.path().join("test.wal");

    let mut schema = Schema::new(1000).unwrap();
    for i in 0..5 {
        schema
            .append_fact(&Fact::new(SubjectID(i), PredicateID(0), ObjectID(i), 0.9).unwrap())
            .unwrap();
    }
    DatabaseFile::save(&schema, &db_path).unwrap();

    {
        let wal = WriteAheadLog::new(&wal_path).unwrap();
        for i in 5..15u32 {
            wal.append_fact(&Fact::new(SubjectID(i), PredicateID(0), ObjectID(i), 0.9).unwrap())
                .unwrap();
        }
        wal.flush_buffer().unwrap();
    }

    let recovered = RecoveryManager::recover(&db_path, &wal_path).unwrap();
    assert_eq!(recovered.len(), 15);
}

#[test]
fn test_crash_recovery_wal_only_no_db() {
    let dir = tempfile::tempdir().unwrap();
    let wal_path = dir.path().join("test.wal");

    {
        let wal = WriteAheadLog::new(&wal_path).unwrap();
        for i in 0..10u32 {
            wal.append_fact(&Fact::new(SubjectID(i), PredicateID(0), ObjectID(i), 0.9).unwrap())
                .unwrap();
        }
        wal.flush_buffer().unwrap();
    }

    let db_path = dir.path().join("nonexistent.kcm");
    let recovered = RecoveryManager::recover(&db_path, &wal_path).unwrap();
    assert_eq!(recovered.len(), 10);
}

#[test]
fn test_crash_recovery_fresh() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("new.kcm");
    let wal_path = dir.path().join("new.wal");
    let schema = RecoveryManager::recover(&db_path, &wal_path).unwrap();
    assert_eq!(schema.len(), 0);
}

#[test]
fn test_crash_recovery_corrupted_db_fallback() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.kcm");
    let wal_path = dir.path().join("test.wal");
    let backup_path = dir.path().join("test.kcm.backup");

    let mut schema = Schema::new(100).unwrap();
    for i in 0..5 {
        schema
            .append_fact(&Fact::new(SubjectID(i), PredicateID(0), ObjectID(i), 0.9).unwrap())
            .unwrap();
    }
    DatabaseFile::save(&schema, &backup_path).unwrap();
    DatabaseFile::save(&schema, &db_path).unwrap();

    let mut data = std::fs::read(&db_path).unwrap();
    data[0] = 0xFF;
    std::fs::write(&db_path, &data).unwrap();

    {
        let wal = WriteAheadLog::new(&wal_path).unwrap();
        for i in 5..10u32 {
            wal.append_fact(&Fact::new(SubjectID(i), PredicateID(0), ObjectID(i), 0.9).unwrap())
                .unwrap();
        }
        wal.flush_buffer().unwrap();
    }

    let recovered = RecoveryManager::recover(&db_path, &wal_path).unwrap();
    assert!(recovered.len() >= 5);
}

#[test]
fn test_crash_recovery_wal_truncation_after_replay() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.kcm");
    let wal_path = dir.path().join("test.wal");

    let mut schema = Schema::new(100).unwrap();
    schema
        .append_fact(&Fact::new(SubjectID(0), PredicateID(0), ObjectID(0), 0.9).unwrap())
        .unwrap();
    DatabaseFile::save(&schema, &db_path).unwrap();

    {
        let wal = WriteAheadLog::new(&wal_path).unwrap();
        wal.append_fact(&Fact::new(SubjectID(1), PredicateID(0), ObjectID(1), 0.9).unwrap())
            .unwrap();
        wal.flush_buffer().unwrap();
    }

    RecoveryManager::recover(&db_path, &wal_path).unwrap();

    let wal_after = std::fs::read(&wal_path).unwrap();
    assert!(
        wal_after.is_empty(),
        "WAL should be truncated after successful recovery"
    );
}

#[test]
fn test_crash_recovery_deterministic() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.kcm");
    let wal_path = dir.path().join("test.wal");

    let mut schema = Schema::new(100).unwrap();
    for i in 0..10u32 {
        let conf = 0.5 + (i as f64 * 0.025);
        schema
            .append_fact(&Fact::new(SubjectID(i), PredicateID(0), ObjectID(i), conf).unwrap())
            .unwrap();
    }
    DatabaseFile::save(&schema, &db_path).unwrap();

    {
        let wal = WriteAheadLog::new(&wal_path).unwrap();
        for i in 10..20u32 {
            let conf = 0.5 + (i as f64 * 0.025);
            wal.append_fact(&Fact::new(SubjectID(i), PredicateID(0), ObjectID(i), conf).unwrap())
                .unwrap();
        }
        wal.flush_buffer().unwrap();
    }

    let recovered1 = RecoveryManager::recover(&db_path, &wal_path).unwrap();
    DatabaseFile::save(&recovered1, &db_path).unwrap();
    let wal_path2 = dir.path().join("test2.wal");
    let recovered2 = RecoveryManager::recover(&db_path, &wal_path2).unwrap();

    assert_eq!(recovered1.len(), recovered2.len());
    for i in 0..recovered1.len() {
        let f1 = recovered1.get_fact(i).unwrap();
        let f2 = recovered2.get_fact(i).unwrap();
        assert_eq!(f1.subject, f2.subject);
        assert_eq!(f1.predicate, f2.predicate);
        assert_eq!(f1.object, f2.object);
    }
}
