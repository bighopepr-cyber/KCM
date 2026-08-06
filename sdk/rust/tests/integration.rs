use kcm_sdk::{Database, Fact, SdkError};

#[test]
fn test_database_new_and_fact_count() {
    let db = Database::new().expect("failed to create database");
    assert_eq!(db.fact_count(), 0);
    assert_eq!(db.active_fact_count(), 0);
}

#[test]
fn test_insert_and_query() {
    let db = Database::new().expect("failed to create database");
    let fact = Fact::new(1, 2, 3, 0.95).expect("failed to create fact");
    let row_id = db.insert(&fact).expect("failed to insert fact");
    assert_eq!(row_id, 0);
    assert_eq!(db.fact_count(), 1);
    assert_eq!(db.active_fact_count(), 1);
}

#[test]
fn test_insert_multiple_facts() {
    let db = Database::new().expect("failed to create database");
    for i in 0..10 {
        let fact = Fact::new(i, (i % 5) as u8, i + 100, 0.5 + (i as f64) * 0.05)
            .expect("failed to create fact");
        db.insert(&fact).expect("failed to insert fact");
    }
    assert_eq!(db.fact_count(), 10);
    assert_eq!(db.active_fact_count(), 10);
}

#[test]
fn test_update_fact() {
    let db = Database::new().expect("failed to create database");
    let fact = Fact::new(1, 2, 3, 0.95).expect("failed to create fact");
    let row_id = db.insert(&fact).expect("failed to insert fact");

    let updated_fact = Fact::new(10, 20, 30, 0.8).expect("failed to create updated fact");
    db.update(row_id, &updated_fact)
        .expect("failed to update fact");

    let retrieved = db.get_fact(row_id).expect("failed to get fact").expect("fact not found");
    assert_eq!(retrieved.subject, 10);
    assert_eq!(retrieved.predicate, 20);
    assert_eq!(retrieved.object, 30);
}

#[test]
fn test_delete_fact() {
    let db = Database::new().expect("failed to create database");
    let fact = Fact::new(1, 2, 3, 0.95).expect("failed to create fact");
    let row_id = db.insert(&fact).expect("failed to insert fact");
    assert_eq!(db.fact_count(), 1);

    db.delete(row_id).expect("failed to delete fact");
    assert_eq!(db.fact_count(), 1);
    assert_eq!(db.active_fact_count(), 0);
}

#[test]
fn test_query_all() {
    let db = Database::new().expect("failed to create database");
    let fact1 = Fact::new(1, 2, 3, 0.95).expect("failed to create fact");
    let fact2 = Fact::new(4, 5, 6, 0.85).expect("failed to create fact");
    db.insert(&fact1).expect("failed to insert");
    db.insert(&fact2).expect("failed to insert");

    let results = db.query_all().expect("failed to query all");
    assert_eq!(results.len(), 2);
}

#[test]
fn test_query_iterator() {
    let db = Database::new().expect("failed to create database");
    for i in 0..5 {
        let fact = Fact::new(i, 0, i, 0.5).expect("failed to create fact");
        db.insert(&fact).expect("failed to insert");
    }

    let result = db.query("all").expect("failed to query");
    assert_eq!(result.count(), 5);

    let mut count = 0;
    for fact in result {
        let _ = fact;
        count += 1;
    }
    assert_eq!(count, 5);
}

#[test]
fn test_save_and_load() {
    let dir = tempfile::tempdir().expect("failed to create temp dir");
    let path = dir.path().join("test.kcm");

    {
        let db = Database::new().expect("failed to create database");
        let fact = Fact::new(1, 2, 3, 0.95).expect("failed to create fact");
        db.insert(&fact).expect("failed to insert");
        db.save(path.to_str().expect("invalid path"))
            .expect("failed to save");
    }

    {
        let db = Database::load(path.to_str().expect("invalid path"))
            .expect("failed to load");
        assert_eq!(db.fact_count(), 1);
        let facts = db.query_all().expect("failed to query all");
        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].subject, 1);
        assert_eq!(facts[0].predicate, 2);
        assert_eq!(facts[0].object, 3);
    }
}

#[test]
fn test_verify_valid_file() {
    let dir = tempfile::tempdir().expect("failed to create temp dir");
    let path = dir.path().join("test.kcm");

    let db = Database::new().expect("failed to create database");
    let fact = Fact::new(1, 2, 3, 0.95).expect("failed to create fact");
    db.insert(&fact).expect("failed to insert");
    db.save(path.to_str().expect("invalid path"))
        .expect("failed to save");

    Database::verify(path.to_str().expect("invalid path")).expect("verification failed");
}

#[test]
fn test_verify_invalid_file() {
    let dir = tempfile::tempdir().expect("failed to create temp dir");
    let path = dir.path().join("bad.kcm");
    std::fs::write(&path, b"not a valid kcm file").expect("failed to write");

    let result = Database::verify(path.to_str().expect("invalid path"));
    assert!(result.is_err());
}

#[test]
fn test_begin_transaction() {
    let db = Database::new().expect("failed to create database");
    let mut txn = db.begin_transaction().expect("failed to begin txn");
    assert!(txn.is_active());
    assert_eq!(txn.change_count(), 0);

    let fact = Fact::new(1, 2, 3, 0.95).expect("failed to create fact");
    txn.insert(&fact).expect("failed to insert in txn");
    assert_eq!(txn.change_count(), 1);

    txn.commit().expect("failed to commit");
}

#[test]
fn test_rollback_transaction() {
    let db = Database::new().expect("failed to create database");
    let mut txn = db.begin_transaction().expect("failed to begin txn");

    let fact = Fact::new(1, 2, 3, 0.95).expect("failed to create fact");
    txn.insert(&fact).expect("failed to insert in txn");
    txn.rollback().expect("failed to rollback");

    assert_eq!(db.fact_count(), 0);
}

#[test]
fn test_fact_builder_methods() {
    let fact = Fact::new(1, 2, 3, 0.95)
        .expect("failed to create fact")
        .with_evidence(5)
        .with_context(10)
        .with_version(2)
        .with_priority(3)
        .with_owner(42);

    assert_eq!(fact.evidence, 5);
    assert_eq!(fact.context, 10);
    assert_eq!(fact.version, 2);
    assert_eq!(fact.priority, 3);
    assert_eq!(fact.owner, 42);
}

#[test]
fn test_fact_invalid_confidence() {
    let result = Fact::new(1, 2, 3, -0.1);
    assert!(result.is_err());
    match result {
        Err(SdkError::InvalidArgument(_)) => {}
        _ => panic!("Expected InvalidArgument error"),
    }

    let result = Fact::new(1, 2, 3, 1.5);
    assert!(result.is_err());

    let result = Fact::new(1, 2, 3, f64::NAN);
    assert!(result.is_err());
}

#[test]
fn test_error_codes() {
    let err = SdkError::NotFound("test".to_string());
    assert_eq!(err.code(), 1001);
    assert_eq!(err.name(), "NOT_FOUND");

    let err = SdkError::OutOfMemory;
    assert_eq!(err.code(), 1002);

    let err = SdkError::TransactionAborted;
    assert_eq!(err.code(), 1007);
}

#[test]
fn test_error_display() {
    let err = SdkError::NotFound("missing".to_string());
    assert!(err.to_string().contains("NotFound"));
    assert!(err.to_string().contains("missing"));
}

#[test]
fn test_error_json() {
    let err = SdkError::InvalidArgument("bad value".to_string());
    let json = err.to_json();
    assert!(json.contains("1003"));
    assert!(json.contains("INVALID_ARGUMENT"));
    assert!(json.contains("bad value"));
}

#[test]
fn test_close() {
    let db = Database::new().expect("failed to create database");
    let fact = Fact::new(1, 2, 3, 0.95).expect("failed to create fact");
    db.insert(&fact).expect("failed to insert");
    db.close();
}
