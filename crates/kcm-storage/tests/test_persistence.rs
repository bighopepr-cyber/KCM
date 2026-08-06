#![allow(clippy::unwrap_used, clippy::panic)]
use kcm_core::types::*;
use kcm_storage::backup::*;
use kcm_storage::column::Schema;
use kcm_storage::file_format::*;
use kcm_storage::recovery::*;
use kcm_storage::wal::*;

fn test_schema(n: usize) -> Schema {
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
fn test_wal_append_and_replay() {
    let dir = tempfile::tempdir().unwrap();
    let wal_path = dir.path().join("test.wal");
    let wal = WriteAheadLog::new(&wal_path).unwrap();

    for i in 0..50u32 {
        let fact = Fact::new(SubjectID(i), PredicateID(0), ObjectID(i * 10), 0.9).unwrap();
        wal.append_fact(&fact).unwrap();
    }
    wal.flush_buffer().unwrap();

    let mut count = 0;
    wal.replay(|entry| {
        match entry {
            WALEntry::Insert { subject, .. } => {
                assert_eq!(subject.0, count as u32);
                count += 1;
            }
            _ => panic!("Expected Insert"),
        }
        Ok(())
    })
    .unwrap();
    assert_eq!(count, 50);
}

#[test]
fn test_wal_delete_entry() {
    let dir = tempfile::tempdir().unwrap();
    let wal_path = dir.path().join("test.wal");
    let wal = WriteAheadLog::new(&wal_path).unwrap();

    let fact = Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap();
    wal.append_fact(&fact).unwrap();
    wal.append_delete(42).unwrap();
    wal.flush_buffer().unwrap();

    let mut ops = Vec::new();
    wal.replay(|entry| {
        match entry {
            WALEntry::Insert { .. } => ops.push("insert"),
            WALEntry::Delete { row_id } => {
                assert_eq!(row_id, 42);
                ops.push("delete");
            }
        }
        Ok(())
    })
    .unwrap();
    assert_eq!(ops, vec!["insert", "delete"]);
}

#[test]
fn test_file_format_save_load() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.kcm");

    let schema = test_schema(10);
    DatabaseFile::save(&schema, &path).unwrap();

    let loaded = DatabaseFile::load(&path).unwrap();
    assert_eq!(loaded.len(), 10);

    for i in 0..10 {
        let original = schema.get_fact(i).unwrap();
        let restored = loaded.get_fact(i).unwrap();
        assert_eq!(original.subject, restored.subject);
        assert_eq!(original.predicate, restored.predicate);
        assert_eq!(original.object, restored.object);
    }
}

#[test]
fn test_file_format_verify() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.kcm");
    let schema = test_schema(5);
    DatabaseFile::save(&schema, &path).unwrap();
    assert!(DatabaseFile::verify(&path).unwrap());
}

#[test]
fn test_backup_full() {
    let dir = tempfile::tempdir().unwrap();
    let backup_dir = dir.path().join("backups");
    let manager = BackupManager::new(&backup_dir).unwrap();

    let schema = test_schema(10);
    let backup_path = manager.create_full_backup(&schema).unwrap();
    assert!(backup_path.exists());

    let manifest_path = backup_path.with_extension("manifest");
    assert!(manifest_path.exists());

    let loaded = RestoreManager::restore(&backup_path).unwrap();
    assert_eq!(loaded.len(), 10);
}

#[test]
fn test_backup_list() {
    let dir = tempfile::tempdir().unwrap();
    let backup_dir = dir.path().join("backups");
    let manager = BackupManager::new(&backup_dir).unwrap();

    let schema = test_schema(5);
    manager.create_full_backup(&schema).unwrap();
    manager.create_full_backup(&schema).unwrap();

    let backups = manager.list_backups().unwrap();
    assert_eq!(backups.len(), 2);
}

#[test]
fn test_recovery_from_empty() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("nonexistent.kcm");
    let wal_path = dir.path().join("nonexistent.wal");
    let schema = RecoveryManager::recover(&db_path, &wal_path).unwrap();
    assert_eq!(schema.len(), 0);
}

#[test]
fn test_recovery_with_wal() {
    let dir = tempfile::tempdir().unwrap();
    let wal_path = dir.path().join("test.wal");

    {
        let wal = WriteAheadLog::new(&wal_path).unwrap();
        for i in 0..5u32 {
            let fact = Fact::new(SubjectID(i), PredicateID(0), ObjectID(i), 0.9).unwrap();
            wal.append_fact(&fact).unwrap();
        }
        wal.flush_buffer().unwrap();
    }

    let db_path = dir.path().join("nonexistent.kcm");
    let schema = RecoveryManager::recover(&db_path, &wal_path).unwrap();
    assert_eq!(schema.len(), 5);
}

#[test]
fn test_schema_compaction() {
    let mut schema = Schema::new(100).unwrap();
    for i in 0..10u32 {
        schema
            .append_fact(&Fact::new(SubjectID(i), PredicateID(0), ObjectID(i), 0.9).unwrap())
            .unwrap();
    }
    schema.delete_fact(0).unwrap();
    schema.delete_fact(3).unwrap();
    schema.delete_fact(7).unwrap();
    assert_eq!(schema.len(), 10);
    assert_eq!(schema.active_count(), 7);

    let compacted = schema.compact().unwrap();
    assert_eq!(compacted.len(), 7);
    assert_eq!(compacted.active_count(), 7);
    assert!(!compacted.is_deleted(0));
}

#[test]
fn test_schema_compaction_preserves_data() {
    let mut schema = Schema::new(100).unwrap();
    for i in 0..10u32 {
        let fact = Fact::new(
            SubjectID(i),
            PredicateID((i % 5) as u8),
            ObjectID(i * 10),
            0.5 + (i as f64 * 0.05),
        )
        .unwrap();
        schema.append_fact(&fact).unwrap();
    }
    schema.delete_fact(0).unwrap();
    schema.delete_fact(5).unwrap();

    let compacted = schema.compact().unwrap();
    assert_eq!(compacted.len(), 8);
    for i in 0..8 {
        let fact = compacted.get_fact(i).unwrap();
        assert!(fact.subject.0 < 10);
        assert!(fact.confidence >= 0.0 && fact.confidence < 1.0);
    }
}

#[test]
fn test_wal_roundtrip_with_all_fields() {
    let dir = tempfile::tempdir().unwrap();
    let wal_path = dir.path().join("full_fields.wal");
    let wal = WriteAheadLog::new(&wal_path).unwrap();
    let mut fact = Fact::new(SubjectID(42), PredicateID(7), ObjectID(99), 0.77).unwrap();
    fact.version = 42;
    fact.priority = 5;
    fact.owner = 99;
    wal.append_fact(&fact).unwrap();
    wal.flush_buffer().unwrap();

    let mut recovered_facts = Vec::new();
    let wal2 = WriteAheadLog::new(&wal_path).unwrap();
    wal2.replay(|entry| {
        if let Some(f) = entry.to_fact() {
            recovered_facts.push(f);
        }
        Ok(())
    })
    .unwrap();

    assert_eq!(recovered_facts.len(), 1);
    let recovered = &recovered_facts[0];
    assert_eq!(recovered.subject.0, 42);
    assert_eq!(recovered.predicate.0, 7);
    assert_eq!(recovered.object.0, 99);
    assert!((recovered.confidence - 0.77).abs() < 1e-10);
    assert_eq!(recovered.version, 42);
    assert_eq!(recovered.priority, 5);
    assert_eq!(recovered.owner, 99);
}

#[test]
fn test_file_format_with_all_columns() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("all_cols.kcm");
    let mut schema = Schema::new(100).unwrap();
    for i in 0..50 {
        let fact = Fact::new(
            SubjectID((i % 100) as u32),
            PredicateID((i % 10) as u8),
            ObjectID((i * 3) as u32),
            0.1 + (i as f64 * 0.01),
        )
        .unwrap();
        schema.append_fact(&fact).unwrap();
    }
    DatabaseFile::save(&schema, &path).unwrap();
    let loaded = DatabaseFile::load(&path).unwrap();
    assert_eq!(loaded.len(), 50);
    assert!(DatabaseFile::verify(&path).unwrap());
}
