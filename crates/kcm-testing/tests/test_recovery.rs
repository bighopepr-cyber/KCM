#![allow(clippy::unwrap_used, clippy::panic)]
use kcm_core::types::*;
use kcm_storage::column::Schema;
use kcm_storage::file_format::DatabaseFile;
use kcm_storage::recovery::RecoveryManager;
use kcm_storage::wal::WriteAheadLog;

fn create_test_schema(n: usize) -> Schema {
    let mut schema = Schema::new(1000).unwrap();
    for i in 0..n {
        let fact = Fact::new(
            SubjectID(i as u32),
            PredicateID((i % 10) as u8),
            ObjectID((i * 10) as u32),
            0.5 + (i as f64 * 0.01),
        )
        .unwrap();
        schema.append_fact(&fact).unwrap();
    }
    schema
}

#[test]
fn test_recovery_db_plus_wal() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.kcm");
    let wal_path = dir.path().join("test.wal");

    let schema = create_test_schema(10);
    DatabaseFile::save(&schema, &db_path).unwrap();

    {
        let wal = WriteAheadLog::new(&wal_path).unwrap();
        for i in 10..15u32 {
            let fact = Fact::new(SubjectID(i), PredicateID(0), ObjectID(i), 0.9).unwrap();
            wal.append_fact(&fact).unwrap();
        }
        wal.flush_buffer().unwrap();
    }

    let recovered = RecoveryManager::recover(&db_path, &wal_path).unwrap();
    assert_eq!(recovered.len(), 15);
}

#[test]
fn test_recovery_wal_only() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("nonexistent.kcm");
    let wal_path = dir.path().join("test.wal");

    {
        let wal = WriteAheadLog::new(&wal_path).unwrap();
        for i in 0..5u32 {
            let fact = Fact::new(SubjectID(i), PredicateID(0), ObjectID(i), 0.9).unwrap();
            wal.append_fact(&fact).unwrap();
        }
        wal.flush_buffer().unwrap();
    }

    let recovered = RecoveryManager::recover(&db_path, &wal_path).unwrap();
    assert_eq!(recovered.len(), 5);
}

#[test]
fn test_recovery_fresh() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("new.kcm");
    let wal_path = dir.path().join("new.wal");
    let schema = RecoveryManager::recover(&db_path, &wal_path).unwrap();
    assert_eq!(schema.len(), 0);
}

#[test]
fn test_file_format_save_load_verify() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.kcm");
    let schema = create_test_schema(20);
    DatabaseFile::save(&schema, &path).unwrap();
    assert!(DatabaseFile::verify(&path).unwrap());
    let loaded = DatabaseFile::load(&path).unwrap();
    assert_eq!(loaded.len(), 20);
    for i in 0..20 {
        let orig = schema.get_fact(i).unwrap();
        let restored = loaded.get_fact(i).unwrap();
        assert_eq!(orig.subject, restored.subject);
        assert_eq!(orig.predicate, restored.predicate);
        assert_eq!(orig.object, restored.object);
    }
}

#[test]
fn test_backup_and_restore() {
    let dir = tempfile::tempdir().unwrap();
    let backup_dir = dir.path().join("backups");
    let mgr = kcm_storage::backup::BackupManager::new(&backup_dir).unwrap();
    let schema = create_test_schema(10);
    let path = mgr.create_full_backup(&schema).unwrap();
    let restored = kcm_storage::backup::RestoreManager::restore(&path).unwrap();
    assert_eq!(restored.len(), 10);
}

#[test]
fn test_wal_fact_fields_preserved() {
    let dir = tempfile::tempdir().unwrap();
    let wal_path = dir.path().join("test.wal");

    {
        let wal = WriteAheadLog::new(&wal_path).unwrap();
        let mut fact = Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap();
        fact.version = 42;
        fact.priority = 7;
        fact.owner = 99;
        wal.append_fact(&fact).unwrap();
        wal.flush_buffer().unwrap();
    }

    let wal = WriteAheadLog::new(&wal_path).unwrap();
    let mut facts = Vec::new();
    wal.replay(|entry| {
        if let Some(fact) = entry.to_fact() {
            facts.push(fact);
        }
        Ok(())
    })
    .unwrap();
    assert_eq!(facts.len(), 1);
    assert_eq!(facts[0].version, 42);
    assert_eq!(facts[0].priority, 7);
    assert_eq!(facts[0].owner, 99);
}
