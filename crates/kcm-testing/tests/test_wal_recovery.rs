use kcm_core::types::*;
use kcm_storage::backup::{BackupManager, RestoreManager};
use kcm_storage::column::Schema;
use kcm_storage::file_format::DatabaseFile;
use kcm_storage::recovery::RecoveryManager;
use kcm_storage::wal::WriteAheadLog;

#[test]
fn test_wal_empty_file() {
    let dir = tempfile::tempdir().unwrap();
    let wal_path = dir.path().join("empty.wal");
    std::fs::write(&wal_path, b"").unwrap();
    let wal = WriteAheadLog::new(&wal_path).unwrap();
    let mut count = 0;
    wal.replay(|_| {
        count += 1;
        Ok(())
    })
    .unwrap();
    assert_eq!(count, 0);
}

#[test]
fn test_wal_truncated_entry() {
    let dir = tempfile::tempdir().unwrap();
    let wal_path = dir.path().join("truncated.wal");
    std::fs::write(&wal_path, [1u8, 0, 0, 0, 1]).unwrap();
    let wal = WriteAheadLog::new(&wal_path).unwrap();
    let result = wal.replay(|_| Ok(()));
    assert!(result.is_err());
}

#[test]
fn test_wal_corrupted_op_type() {
    let dir = tempfile::tempdir().unwrap();
    let wal_path = dir.path().join("corrupt.wal");
    std::fs::write(&wal_path, [99u8, 0, 0, 0, 0, 0, 0, 0]).unwrap();
    let wal = WriteAheadLog::new(&wal_path).unwrap();
    let result = wal.replay(|_| Ok(()));
    assert!(result.is_err());
}

#[test]
fn test_wal_integrity_check_valid() {
    let dir = tempfile::tempdir().unwrap();
    let wal_path = dir.path().join("valid.wal");
    let wal = WriteAheadLog::new(&wal_path).unwrap();
    for i in 0..10u32 {
        let fact = Fact::new(SubjectID(i), PredicateID(0), ObjectID(i), 0.9).unwrap();
        wal.append_fact(&fact).unwrap();
    }
    wal.flush_buffer().unwrap();
    wal.verify_integrity().unwrap();
}

#[test]
fn test_wal_integrity_check_corrupted() {
    let dir = tempfile::tempdir().unwrap();
    let wal_path = dir.path().join("corrupt.wal");
    let wal = WriteAheadLog::new(&wal_path).unwrap();
    for i in 0..10u32 {
        let fact = Fact::new(SubjectID(i), PredicateID(0), ObjectID(i), 0.9).unwrap();
        wal.append_fact(&fact).unwrap();
    }
    wal.flush_buffer().unwrap();

    let mut data = std::fs::read(&wal_path).unwrap();
    if data.len() > 40 {
        data[40] = data[40].wrapping_add(1);
        std::fs::write(&wal_path, &data).unwrap();
    }
    let wal2 = WriteAheadLog::new(&wal_path).unwrap();
    assert!(wal2.verify_integrity().is_err());
}

#[test]
fn test_wal_delete_entry_replay() {
    let dir = tempfile::tempdir().unwrap();
    let wal_path = dir.path().join("delete.wal");
    let wal = WriteAheadLog::new(&wal_path).unwrap();
    let fact = Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap();
    wal.append_fact(&fact).unwrap();
    wal.append_delete(0).unwrap();
    wal.flush_buffer().unwrap();

    let wal2 = WriteAheadLog::new(&wal_path).unwrap();
    let mut insert_count = 0;
    let mut delete_count = 0;
    let mut deleted_row_id = None;
    wal2.replay(|entry| {
        match entry {
            kcm_storage::wal::WALEntry::Insert { .. } => {
                insert_count += 1;
            }
            kcm_storage::wal::WALEntry::Delete { row_id } => {
                delete_count += 1;
                deleted_row_id = Some(row_id);
            }
        }
        Ok(())
    })
    .unwrap();
    assert_eq!(insert_count, 1);
    assert_eq!(delete_count, 1);
    assert_eq!(deleted_row_id, Some(0));
}

#[test]
fn test_file_format_corrupted_magic() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.kcm");
    let schema = Schema::new(5).unwrap();
    DatabaseFile::save(&schema, &path).unwrap();
    let mut data = std::fs::read(&path).unwrap();
    data[0] = 0xFF;
    std::fs::write(&path, &data).unwrap();
    assert!(DatabaseFile::load(&path).is_err());
}

#[test]
fn test_file_format_corrupted_version() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.kcm");
    let schema = Schema::new(5).unwrap();
    DatabaseFile::save(&schema, &path).unwrap();
    let mut data = std::fs::read(&path).unwrap();
    data[5] = 0xFF;
    std::fs::write(&path, &data).unwrap();
    assert!(DatabaseFile::load(&path).is_err());
}

#[test]
fn test_file_format_truncated() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.kcm");
    let schema = Schema::new(10).unwrap();
    DatabaseFile::save(&schema, &path).unwrap();
    let mut data = std::fs::read(&path).unwrap();
    data.truncate(50);
    std::fs::write(&path, &data).unwrap();
    assert!(DatabaseFile::load(&path).is_err());
}

#[test]
fn test_file_format_verify_intact() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.kcm");
    let schema = Schema::new(100).unwrap();
    DatabaseFile::save(&schema, &path).unwrap();
    assert!(DatabaseFile::verify(&path).unwrap());
}

#[test]
fn test_file_format_verify_corrupted() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.kcm");
    let schema = Schema::new(100).unwrap();
    DatabaseFile::save(&schema, &path).unwrap();
    let mut data = std::fs::read(&path).unwrap();
    let last = data.len() - 1;
    data[last] = data[last].wrapping_add(1);
    std::fs::write(&path, &data).unwrap();
    assert!(!DatabaseFile::verify(&path).unwrap());
}

#[test]
fn test_recovery_fresh_database() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("new.kcm");
    let wal_path = dir.path().join("new.wal");
    let schema = RecoveryManager::recover(&db_path, &wal_path).unwrap();
    assert_eq!(schema.len(), 0);
}

#[test]
fn test_recovery_db_plus_wal() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.kcm");
    let wal_path = dir.path().join("test.wal");

    let mut schema = Schema::new(1000).unwrap();
    for i in 0..5u32 {
        schema
            .append_fact(&Fact::new(SubjectID(i), PredicateID(0), ObjectID(i), 0.9).unwrap())
            .unwrap();
    }
    DatabaseFile::save(&schema, &db_path).unwrap();

    {
        let wal = WriteAheadLog::new(&wal_path).unwrap();
        for i in 5..10u32 {
            let fact = Fact::new(SubjectID(i), PredicateID(0), ObjectID(i), 0.9).unwrap();
            wal.append_fact(&fact).unwrap();
        }
        wal.flush_buffer().unwrap();
    }

    let recovered = RecoveryManager::recover(&db_path, &wal_path).unwrap();
    assert_eq!(recovered.len(), 10);
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
fn test_wal_replay_preserves_all_fields() {
    let dir = tempfile::tempdir().unwrap();
    let wal_path = dir.path().join("fields.wal");
    let wal = WriteAheadLog::new(&wal_path).unwrap();
    let mut fact = Fact::new(SubjectID(1), PredicateID(2), ObjectID(3), 0.85).unwrap();
    fact.version = 42;
    fact.priority = 7;
    fact.owner = 99;
    wal.append_fact(&fact).unwrap();
    wal.flush_buffer().unwrap();

    let wal2 = WriteAheadLog::new(&wal_path).unwrap();
    let mut facts = Vec::new();
    wal2.replay(|entry| {
        if let Some(f) = entry.to_fact() {
            facts.push(f);
        }
        Ok(())
    })
    .unwrap();
    assert_eq!(facts.len(), 1);
    assert_eq!(facts[0].subject, SubjectID(1));
    assert_eq!(facts[0].predicate, PredicateID(2));
    assert_eq!(facts[0].object, ObjectID(3));
    assert!((facts[0].confidence - 0.85).abs() < f64::EPSILON);
    assert_eq!(facts[0].version, 42);
    assert_eq!(facts[0].priority, 7);
    assert_eq!(facts[0].owner, 99);
}

#[test]
fn test_wal_checksum_mismatch_on_replay() {
    let dir = tempfile::tempdir().unwrap();
    let wal_path = dir.path().join("checksum.wal");
    let wal = WriteAheadLog::new(&wal_path).unwrap();
    for i in 0..5u32 {
        let fact = Fact::new(SubjectID(i), PredicateID(0), ObjectID(i), 0.9).unwrap();
        wal.append_fact(&fact).unwrap();
    }
    wal.flush_buffer().unwrap();

    let mut data = std::fs::read(&wal_path).unwrap();
    if data.len() > 10 {
        data[10] = data[10].wrapping_add(1);
        std::fs::write(&wal_path, &data).unwrap();
    }
    let wal2 = WriteAheadLog::new(&wal_path).unwrap();
    let result = wal2.replay(|_| Ok(()));
    assert!(result.is_err());
}

#[test]
fn test_backup_full_save_load() {
    let dir = tempfile::tempdir().unwrap();
    let backup_dir = dir.path().join("backups");
    let mgr = BackupManager::new(&backup_dir).unwrap();
    let schema = Schema::new(50).unwrap();
    let path = mgr.create_full_backup(&schema).unwrap();
    assert!(path.exists());
    let restored = RestoreManager::restore(&path).unwrap();
    assert_eq!(restored.len(), schema.len());
}

#[test]
fn test_backup_manifest_created() {
    let dir = tempfile::tempdir().unwrap();
    let backup_dir = dir.path().join("backups");
    let mgr = BackupManager::new(&backup_dir).unwrap();
    let schema = Schema::new(10).unwrap();
    let path = mgr.create_full_backup(&schema).unwrap();
    let manifest_path = path.with_extension("manifest");
    assert!(manifest_path.exists());
    let content = std::fs::read_to_string(&manifest_path).unwrap();
    assert!(content.contains("backup_type: full"));
}

#[test]
fn test_backup_incremental() {
    let dir = tempfile::tempdir().unwrap();
    let backup_dir = dir.path().join("backups");
    let mgr = BackupManager::new(&backup_dir).unwrap();
    let mut schema = Schema::new(100).unwrap();
    let fact = Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap();
    schema.append_fact(&fact).unwrap();
    let base = mgr.create_full_backup(&schema).unwrap();
    let incr = mgr.create_incremental_backup(&schema, &base).unwrap();
    assert!(incr.exists());
}

#[test]
fn test_file_format_save_load_with_facts() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.kcm");
    let mut schema = Schema::new(100).unwrap();
    for i in 0..20u32 {
        let fact = Fact::new(SubjectID(i), PredicateID(0), ObjectID(i + 100), 0.9).unwrap();
        schema.append_fact(&fact).unwrap();
    }
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
fn test_file_format_empty_schema() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("empty.kcm");
    let schema = Schema::new(10).unwrap();
    DatabaseFile::save(&schema, &path).unwrap();
    assert!(DatabaseFile::verify(&path).unwrap());
    let loaded = DatabaseFile::load(&path).unwrap();
    assert_eq!(loaded.len(), 0);
}

#[test]
fn test_recovery_db_corrupted_wal_valid() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("corrupt.kcm");
    let wal_path = dir.path().join("valid.wal");

    let mut bad_data = vec![0u8; 64];
    bad_data[..19].copy_from_slice(b"NOT_A_VALID_DB_FILE");
    std::fs::write(&db_path, &bad_data).unwrap();

    {
        let wal = WriteAheadLog::new(&wal_path).unwrap();
        let fact = Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap();
        wal.append_fact(&fact).unwrap();
        wal.flush_buffer().unwrap();
    }

    let result = RecoveryManager::recover(&db_path, &wal_path);
    assert!(result.is_err());
}
