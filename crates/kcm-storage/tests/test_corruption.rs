use kcm_core::types::*;
use kcm_storage::column::Schema;
use kcm_storage::file_format::DatabaseFile;

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
fn test_corrupt_magic_bytes() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.kcm");
    let schema = create_test_schema(5);
    DatabaseFile::save(&schema, &path).unwrap();

    let mut data = std::fs::read(&path).unwrap();
    data[0] = 0xFF;
    std::fs::write(&path, &data).unwrap();

    let result = DatabaseFile::load(&path);
    match result {
        Err(e) => assert!(e.to_string().contains("Invalid database magic")),
        Ok(_) => panic!("Expected error for corrupted magic bytes"),
    }
}

#[test]
fn test_corrupt_version_byte() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.kcm");
    let schema = create_test_schema(5);
    DatabaseFile::save(&schema, &path).unwrap();

    let mut data = std::fs::read(&path).unwrap();
    data[5] = 0xFF;
    std::fs::write(&path, &data).unwrap();

    let result = DatabaseFile::load(&path);
    assert!(result.is_err());
}

#[test]
fn test_truncated_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.kcm");
    let schema = create_test_schema(5);
    DatabaseFile::save(&schema, &path).unwrap();

    let mut data = std::fs::read(&path).unwrap();
    data.truncate(10);
    std::fs::write(&path, &data).unwrap();

    let result = DatabaseFile::load(&path);
    assert!(result.is_err());
}

#[test]
fn test_empty_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("empty.kcm");
    std::fs::write(&path, b"").unwrap();

    let result = DatabaseFile::load(&path);
    assert!(result.is_err());
}

#[test]
fn test_file_verify_after_save() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.kcm");
    let schema = create_test_schema(10);
    DatabaseFile::save(&schema, &path).unwrap();

    assert!(DatabaseFile::verify(&path).unwrap());
}

#[test]
fn test_file_verify_after_corrupt() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.kcm");
    let schema = create_test_schema(10);
    DatabaseFile::save(&schema, &path).unwrap();

    let mut data = std::fs::read(&path).unwrap();
    let last = data.len() - 1;
    data[last] = data[last].wrapping_add(1);
    std::fs::write(&path, &data).unwrap();

    assert!(!DatabaseFile::verify(&path).unwrap());
}

#[test]
fn test_save_load_roundtrip_integrity() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.kcm");

    let schema = create_test_schema(50);
    DatabaseFile::save(&schema, &path).unwrap();
    let loaded = DatabaseFile::load(&path).unwrap();

    assert_eq!(schema.len(), loaded.len());
    for i in 0..schema.len() {
        let orig = schema.get_fact(i).unwrap();
        let rest = loaded.get_fact(i).unwrap();
        assert_eq!(orig.subject, rest.subject);
        assert_eq!(orig.predicate, rest.predicate);
        assert_eq!(orig.object, rest.object);
        assert!((orig.confidence - rest.confidence).abs() < 1e-10);
        assert_eq!(orig.timestamp, rest.timestamp);
    }
}

#[test]
fn test_large_dataset_integrity() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.kcm");

    let schema = create_test_schema(1000);
    DatabaseFile::save(&schema, &path).unwrap();
    let loaded = DatabaseFile::load(&path).unwrap();

    assert_eq!(schema.len(), loaded.len());
    assert_eq!(schema.len(), 1000);
}
