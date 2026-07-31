use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;

#[test]
fn test_rollback_insert() {
    let kb = KnowledgeDatabase::new().unwrap();

    let fact = Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap();
    kb.insert(&fact).unwrap();
    assert_eq!(kb.active_fact_count(), 1);

    let mut txn = kb.begin_transaction();
    let new_fact = Fact::new(SubjectID(10), PredicateID(5), ObjectID(20), 0.8).unwrap();
    txn.insert(new_fact).unwrap();
    drop(txn);

    // After dropping (implicit rollback), the count should still be 1
    assert_eq!(kb.active_fact_count(), 1);
}

#[test]
fn test_rollback_delete() {
    let kb = KnowledgeDatabase::new().unwrap();

    for i in 0..5u32 {
        let fact = Fact::new(SubjectID(i), PredicateID(0), ObjectID(i), 0.9).unwrap();
        kb.insert(&fact).unwrap();
    }
    assert_eq!(kb.active_fact_count(), 5);

    kb.delete(RowID(2)).unwrap();
    assert_eq!(kb.active_fact_count(), 4);

    // The fact at row 2 should be deleted
    assert!(kb.get_fact(RowID(2)).unwrap().is_none());
}

#[test]
fn test_rollback_preserves_other_data() {
    let kb = KnowledgeDatabase::new().unwrap();

    let fact1 = Fact::new(SubjectID(1), PredicateID(0), ObjectID(10), 0.9).unwrap();
    let fact2 = Fact::new(SubjectID(2), PredicateID(0), ObjectID(20), 0.8).unwrap();
    kb.insert(&fact1).unwrap();
    kb.insert(&fact2).unwrap();

    kb.delete(RowID(0)).unwrap();

    // fact2 should still be accessible
    let retrieved = kb.get_fact(RowID(1)).unwrap().unwrap();
    assert_eq!(retrieved.subject, SubjectID(2));
    assert_eq!(retrieved.object, ObjectID(20));
}

#[test]
fn test_update_then_rollback() {
    let kb = KnowledgeDatabase::new().unwrap();

    let original = Fact::new(SubjectID(1), PredicateID(0), ObjectID(10), 0.9).unwrap();
    let row_id = kb.insert(&original).unwrap();

    let updated = Fact::new(SubjectID(1), PredicateID(0), ObjectID(99), 0.5).unwrap();
    kb.update(row_id, &updated).unwrap();

    let retrieved = kb.get_fact(row_id).unwrap().unwrap();
    assert_eq!(retrieved.object, ObjectID(99));
    assert!((retrieved.confidence - 0.5).abs() < 1e-10);
}

#[test]
fn test_multiple_deletes_and_queries() {
    let kb = KnowledgeDatabase::new().unwrap();

    for i in 0..10u32 {
        let fact = Fact::new(SubjectID(i % 3), PredicateID(0), ObjectID(i), 0.9).unwrap();
        kb.insert(&fact).unwrap();
    }

    // Delete all facts with subject 0
    for i in (0..10u64).step_by(3) {
        kb.delete(RowID(i)).unwrap();
    }

    let remaining = kb.query().with_subject(SubjectID(0)).execute().unwrap();
    assert_eq!(remaining.len(), 0);

    let subject1 = kb.query().with_subject(SubjectID(1)).execute().unwrap();
    assert!(!subject1.is_empty());
}

#[test]
fn test_transaction_commit_applies() {
    let kb = KnowledgeDatabase::new().unwrap();

    let mut txn = kb.begin_transaction();
    let fact = Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap();
    txn.insert(fact).unwrap();

    let mut schema = kb.get_schema_mut();
    txn.apply_to_schema(&mut schema).unwrap();
    drop(schema);

    let result = txn.commit();
    assert!(result.is_ok());
    assert_eq!(kb.fact_count(), 1);
}

#[test]
fn test_transaction_rollback_discards() {
    let kb = KnowledgeDatabase::new().unwrap();

    let mut txn = kb.begin_transaction();
    let fact = Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap();
    txn.insert(fact).unwrap();

    // Don't apply, just rollback
    let result = txn.rollback();
    assert!(result.is_ok());

    // Database should be empty
    assert_eq!(kb.fact_count(), 0);
}

#[test]
fn test_deterministic_insert_query() {
    let kb1 = KnowledgeDatabase::new().unwrap();
    let kb2 = KnowledgeDatabase::new().unwrap();

    for i in 0..50u32 {
        let fact = Fact::new(
            SubjectID(i),
            PredicateID(0),
            ObjectID(i * 2),
            0.5 + (i as f64 * 0.01),
        )
        .unwrap();
        kb1.insert(&fact).unwrap();
        kb2.insert(&fact).unwrap();
    }

    let r1 = kb1
        .query()
        .with_predicate(PredicateID(0))
        .execute()
        .unwrap();
    let r2 = kb2
        .query()
        .with_predicate(PredicateID(0))
        .execute()
        .unwrap();

    assert_eq!(r1.len(), r2.len());
    for (f1, f2) in r1.iter().zip(r2.iter()) {
        assert_eq!(f1.subject, f2.subject);
        assert_eq!(f1.object, f2.object);
        assert_eq!(f1.confidence, f2.confidence);
    }
}

#[test]
fn test_transaction_rollback_changes() {
    let kb = KnowledgeDatabase::new().unwrap();

    let fact1 = Fact::new(SubjectID(1), PredicateID(0), ObjectID(10), 0.9).unwrap();
    let fact2 = Fact::new(SubjectID(2), PredicateID(0), ObjectID(20), 0.8).unwrap();
    kb.insert(&fact1).unwrap();
    kb.insert(&fact2).unwrap();

    let mut txn = kb.begin_transaction();
    let fact3 = Fact::new(SubjectID(3), PredicateID(0), ObjectID(30), 0.7).unwrap();
    txn.insert(fact3).unwrap();

    let mut schema = kb.get_schema_mut();
    txn.apply_to_schema(&mut schema).unwrap();
    txn.rollback_changes(&mut schema).unwrap();
    drop(schema);

    assert_eq!(kb.active_fact_count(), 2);
    assert!(kb.get_fact(RowID(2)).unwrap().is_none());
}

#[test]
fn test_transaction_state_transitions() {
    use kcm_runtime::transaction::TransactionState;

    let mut txn = kcm_runtime::transaction::Transaction::new();
    assert_eq!(txn.state(), TransactionState::Active);

    let fact = Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap();
    txn.insert(fact).unwrap();
    assert_eq!(txn.state(), TransactionState::Active);

    txn.commit().unwrap();

    let txn2 = kcm_runtime::transaction::Transaction::new();
    assert_eq!(txn2.state(), TransactionState::Active);
    txn2.rollback().unwrap();
}

#[test]
fn test_transaction_changes_buffer() {
    let mut txn = kcm_runtime::transaction::Transaction::new();
    assert!(txn.changes().is_empty());

    let fact1 = Fact::new(SubjectID(1), PredicateID(0), ObjectID(10), 0.9).unwrap();
    let fact2 = Fact::new(SubjectID(2), PredicateID(0), ObjectID(20), 0.8).unwrap();
    txn.insert(fact1).unwrap();
    txn.insert(fact2).unwrap();

    let changes = txn.changes();
    assert_eq!(changes.len(), 2);
}

#[test]
fn test_transaction_insert_on_non_active() {
    let kb = KnowledgeDatabase::new().unwrap();

    let mut txn = kb.begin_transaction();
    let fact = Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap();
    txn.insert(fact).unwrap();

    let mut schema = kb.get_schema_mut();
    txn.apply_to_schema(&mut schema).unwrap();
    drop(schema);
    txn.commit().unwrap();

    assert_eq!(kb.active_fact_count(), 1);
    let retrieved = kb.get_fact(RowID(0)).unwrap().unwrap();
    assert_eq!(retrieved.subject, SubjectID(1));

    let txn2 = kb.begin_transaction();
    txn2.commit().unwrap();
}

#[test]
fn test_transaction_state_active() {
    let txn = kcm_runtime::transaction::Transaction::new();
    assert_eq!(
        txn.state(),
        kcm_runtime::transaction::TransactionState::Active
    );
}

#[test]
fn test_transaction_state_committed() {
    let mut txn = kcm_runtime::transaction::Transaction::new();
    assert_eq!(
        txn.state(),
        kcm_runtime::transaction::TransactionState::Active
    );

    let fact = Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap();
    txn.insert(fact).unwrap();

    let mut schema = kcm_storage::column::Schema::new(10).unwrap();
    txn.apply_to_schema(&mut schema).unwrap();
    txn.commit().unwrap();

    assert_eq!(schema.len(), 1);
    let retrieved = schema.get_fact(0).unwrap();
    assert_eq!(retrieved.subject, SubjectID(1));
}

#[test]
fn test_transaction_rollback_reverts_schema() {
    let kb = KnowledgeDatabase::new().unwrap();

    let fact1 = Fact::new(SubjectID(1), PredicateID(0), ObjectID(10), 0.9).unwrap();
    kb.insert(&fact1).unwrap();
    assert_eq!(kb.active_fact_count(), 1);

    let mut txn = kb.begin_transaction();
    let fact2 = Fact::new(SubjectID(2), PredicateID(0), ObjectID(20), 0.8).unwrap();
    txn.insert(fact2).unwrap();

    let mut schema = kb.get_schema_mut();
    txn.apply_to_schema(&mut schema).unwrap();
    assert_eq!(schema.len(), 2);
    txn.rollback_changes(&mut schema).unwrap();
    drop(schema);

    assert_eq!(kb.active_fact_count(), 1);
    let retrieved = kb.get_fact(RowID(0)).unwrap().unwrap();
    assert_eq!(retrieved.subject, SubjectID(1));
    assert_eq!(retrieved.object, ObjectID(10));
}
