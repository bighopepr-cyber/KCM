use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;
use kcm_runtime::executor::Executor;
use parking_lot::Mutex;
use std::sync::Arc;

#[test]
fn test_database_new() {
    let kb = KnowledgeDatabase::new().unwrap();
    assert_eq!(kb.fact_count(), 0);
}

#[test]
fn test_database_insert_and_query() {
    let kb = KnowledgeDatabase::new().unwrap();

    let fact1 = Fact::new(SubjectID(1), PredicateID(0), ObjectID(100), 0.9).unwrap();
    let fact2 = Fact::new(SubjectID(2), PredicateID(1), ObjectID(200), 0.8).unwrap();

    let row1 = kb.insert(&fact1).unwrap();
    let row2 = kb.insert(&fact2).unwrap();

    assert_eq!(row1, RowID(0));
    assert_eq!(row2, RowID(1));
    assert_eq!(kb.fact_count(), 2);

    let results = kb.query().execute().unwrap();
    assert_eq!(results.len(), 2);
}

#[test]
fn test_database_query_by_subject() {
    let kb = KnowledgeDatabase::new().unwrap();

    for i in 0..10u32 {
        let fact = Fact::new(SubjectID(i % 3), PredicateID(0), ObjectID(i), 0.9).unwrap();
        kb.insert(&fact).unwrap();
    }

    let results = kb.query().with_subject(SubjectID(1)).execute().unwrap();

    assert!(!results.is_empty());
    for fact in &results {
        assert_eq!(fact.subject, SubjectID(1));
    }
}

#[test]
fn test_database_query_by_predicate() {
    let kb = KnowledgeDatabase::new().unwrap();

    for i in 0..10u32 {
        let fact = Fact::new(SubjectID(i), PredicateID((i % 3) as u8), ObjectID(i), 0.9).unwrap();
        kb.insert(&fact).unwrap();
    }

    let results = kb.query().with_predicate(PredicateID(2)).execute().unwrap();

    for fact in &results {
        assert_eq!(fact.predicate, PredicateID(2));
    }
}

#[test]
fn test_database_query_by_object() {
    let kb = KnowledgeDatabase::new().unwrap();

    for i in 0..10u32 {
        let fact = Fact::new(SubjectID(i), PredicateID(0), ObjectID(i * 10), 0.9).unwrap();
        kb.insert(&fact).unwrap();
    }

    let results = kb.query().with_object(ObjectID(30)).execute().unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].object, ObjectID(30));
}

#[test]
fn test_database_query_by_confidence() {
    let kb = KnowledgeDatabase::new().unwrap();

    for i in 0..10u32 {
        let fact = Fact::new(
            SubjectID(i),
            PredicateID(0),
            ObjectID(i),
            0.1 + (i as f64 * 0.1),
        )
        .unwrap();
        kb.insert(&fact).unwrap();
    }

    let results = kb.query().with_confidence(0.5).execute().unwrap();

    for fact in &results {
        assert!(fact.confidence >= 0.5);
    }
}

#[test]
fn test_database_insert_batch() {
    let kb = KnowledgeDatabase::new().unwrap();

    let facts: Vec<Fact> = (0..100)
        .map(|i| {
            Fact::new(
                SubjectID(i % 10),
                PredicateID((i % 5) as u8),
                ObjectID(i),
                0.5,
            )
            .unwrap()
        })
        .collect();

    let row_ids = kb.insert_batch(&facts).unwrap();
    assert_eq!(row_ids.len(), 100);
    assert_eq!(kb.fact_count(), 100);
}

#[test]
fn test_database_get_fact() {
    let kb = KnowledgeDatabase::new().unwrap();

    let fact = Fact::new(SubjectID(42), PredicateID(7), ObjectID(99), 0.77).unwrap();
    let row_id = kb.insert(&fact).unwrap();

    let retrieved = kb.get_fact(row_id).unwrap().unwrap();
    assert_eq!(retrieved.subject, SubjectID(42));
    assert_eq!(retrieved.predicate, PredicateID(7));
    assert_eq!(retrieved.object, ObjectID(99));
    assert_eq!(retrieved.confidence, 0.77);
}

#[test]
fn test_database_get_fact_invalid() {
    let kb = KnowledgeDatabase::new().unwrap();

    let result = kb.get_fact(RowID(999));
    assert!(result.unwrap().is_none());
}

#[test]
fn test_database_dictionary() {
    let kb = KnowledgeDatabase::new().unwrap();

    let id1 = kb.dict_insert_subject("Alice").unwrap();
    let id2 = kb.dict_insert_subject("Bob").unwrap();
    let id1_again = kb.dict_insert_subject("Alice").unwrap();

    assert_eq!(id1, id1_again);
    assert_ne!(id1, id2);

    assert_eq!(kb.dict_get_subject(id1), Some("Alice".to_string()));
    assert_eq!(kb.dict_get_subject(id2), Some("Bob".to_string()));

    assert_eq!(kb.dict_lookup_subject("Alice"), Some(id1));
    assert_eq!(kb.dict_lookup_subject("Unknown"), None);
}

#[test]
fn test_database_determinism() {
    let kb = KnowledgeDatabase::new().unwrap();

    for i in 0..50 {
        let fact = Fact::new(
            SubjectID(i),
            PredicateID(0),
            ObjectID(i * 2),
            0.5 + (i as f64 * 0.01),
        )
        .unwrap();
        kb.insert(&fact).unwrap();
    }

    let mut results_history = Vec::new();
    for _ in 0..20 {
        let results = kb.query().execute().unwrap();
        results_history.push(results);
    }

    for i in 1..20 {
        assert_eq!(results_history[0], results_history[i]);
    }
}

#[test]
fn test_database_concurrent_inserts() {
    let kb = Arc::new(Mutex::new(KnowledgeDatabase::new().unwrap()));
    let mut handles = vec![];

    for t in 0..4 {
        let kb_clone = kb.clone();
        let handle = std::thread::spawn(move || {
            for i in 0..25 {
                let fact =
                    Fact::new(SubjectID(t * 100 + i), PredicateID(0), ObjectID(i), 0.9).unwrap();
                kb_clone.lock().insert(&fact).unwrap();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(kb.lock().fact_count(), 100);
}

#[test]
fn test_transaction_begin_commit() {
    let kb = KnowledgeDatabase::new().unwrap();
    let mut txn = kb.begin_transaction();

    let fact = Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap();
    txn.insert(fact).unwrap();

    let result = txn.commit();
    assert!(result.is_ok());
}

#[test]
fn test_transaction_rollback() {
    let kb = KnowledgeDatabase::new().unwrap();
    let mut txn = kb.begin_transaction();

    let fact = Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap();
    txn.insert(fact).unwrap();

    let result = txn.rollback();
    assert!(result.is_ok());
}

#[test]
fn test_executor_new() {
    let executor = Executor::new(4).unwrap();
    assert_eq!(executor.num_threads(), 4);
}

#[test]
fn test_executor_parallel_map() {
    let executor = Executor::new(4).unwrap();
    let items: Vec<u32> = (0..100).collect();

    let results = executor.parallel_map(items, |x| x * 2);

    assert_eq!(results.len(), 100);
    for (i, &result) in results.iter().enumerate() {
        assert_eq!(result, (i as u32) * 2);
    }
}

#[test]
fn test_executor_parallel_filter() {
    let executor = Executor::new(4).unwrap();
    let items: Vec<u32> = (0..100).collect();

    let results = executor.parallel_filter(items, |&x| x % 2 == 0);

    assert_eq!(results.len(), 50);
    for &result in &results {
        assert!(result % 2 == 0);
    }
}

#[test]
fn test_database_delete() {
    let kb = KnowledgeDatabase::new().unwrap();

    let fact1 = Fact::new(SubjectID(1), PredicateID(0), ObjectID(100), 0.9).unwrap();
    let fact2 = Fact::new(SubjectID(2), PredicateID(1), ObjectID(200), 0.8).unwrap();

    let row1 = kb.insert(&fact1).unwrap();
    let _row2 = kb.insert(&fact2).unwrap();

    assert_eq!(kb.fact_count(), 2);
    assert_eq!(kb.active_fact_count(), 2);

    kb.delete(row1).unwrap();

    assert_eq!(kb.fact_count(), 2);
    assert_eq!(kb.active_fact_count(), 1);

    let results = kb.query().execute().unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].subject, SubjectID(2));
}

#[test]
fn test_database_delete_invalid() {
    let kb = KnowledgeDatabase::new().unwrap();
    let result = kb.delete(RowID(999));
    assert!(result.is_err());
}

#[test]
fn test_database_update() {
    let kb = KnowledgeDatabase::new().unwrap();

    let fact = Fact::new(SubjectID(1), PredicateID(0), ObjectID(100), 0.9).unwrap();
    let row_id = kb.insert(&fact).unwrap();

    let updated_fact = Fact::new(SubjectID(5), PredicateID(2), ObjectID(500), 0.7).unwrap();
    kb.update(row_id, &updated_fact).unwrap();

    let retrieved = kb.get_fact(row_id).unwrap().unwrap();
    assert_eq!(retrieved.subject, SubjectID(5));
    assert_eq!(retrieved.predicate, PredicateID(2));
    assert_eq!(retrieved.object, ObjectID(500));
    assert_eq!(retrieved.confidence, 0.7);
}

#[test]
fn test_database_update_invalid() {
    let kb = KnowledgeDatabase::new().unwrap();
    let fact = Fact::new(SubjectID(1), PredicateID(0), ObjectID(100), 0.9).unwrap();
    let result = kb.update(RowID(999), &fact);
    assert!(result.is_err());
}

#[test]
fn test_database_update_then_query() {
    let kb = KnowledgeDatabase::new().unwrap();

    let fact1 = Fact::new(SubjectID(1), PredicateID(0), ObjectID(100), 0.9).unwrap();
    let fact2 = Fact::new(SubjectID(2), PredicateID(0), ObjectID(200), 0.8).unwrap();

    let row1 = kb.insert(&fact1).unwrap();
    let _row2 = kb.insert(&fact2).unwrap();

    let updated = Fact::new(SubjectID(3), PredicateID(0), ObjectID(300), 0.95).unwrap();
    kb.update(row1, &updated).unwrap();

    let results = kb.query().with_subject(SubjectID(3)).execute().unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].subject, SubjectID(3));
    assert_eq!(results[0].object, ObjectID(300));
}

#[test]
fn test_database_delete_then_insert() {
    let kb = KnowledgeDatabase::new().unwrap();

    let fact1 = Fact::new(SubjectID(1), PredicateID(0), ObjectID(100), 0.9).unwrap();
    let row1 = kb.insert(&fact1).unwrap();

    assert_eq!(kb.active_fact_count(), 1);

    kb.delete(row1).unwrap();
    assert_eq!(kb.active_fact_count(), 0);

    let fact2 = Fact::new(SubjectID(2), PredicateID(1), ObjectID(200), 0.8).unwrap();
    let _row2 = kb.insert(&fact2).unwrap();

    assert_eq!(kb.active_fact_count(), 1);

    let results = kb.query().execute().unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].subject, SubjectID(2));
}

#[test]
fn test_database_active_count_after_multiple_operations() {
    let kb = KnowledgeDatabase::new().unwrap();

    let mut row_ids = Vec::new();
    for i in 0..10u32 {
        let fact = Fact::new(SubjectID(i), PredicateID(0), ObjectID(i), 0.9).unwrap();
        row_ids.push(kb.insert(&fact).unwrap());
    }

    assert_eq!(kb.active_fact_count(), 10);

    for row_id in &row_ids[..5] {
        kb.delete(*row_id).unwrap();
    }

    assert_eq!(kb.active_fact_count(), 5);

    let updated = Fact::new(SubjectID(99), PredicateID(9), ObjectID(999), 0.5).unwrap();
    kb.update(row_ids[5], &updated).unwrap();

    assert_eq!(kb.active_fact_count(), 5);

    let results = kb.query().with_subject(SubjectID(99)).execute().unwrap();
    assert_eq!(results.len(), 1);
}

#[test]
fn test_async_executor_basic() {
    let executor = kcm_runtime::async_executor::AsyncExecutor::new().unwrap();
    let result = executor.block_on(async { 42 });
    assert_eq!(result, 42);
}

#[test]
fn test_async_insert_and_query() {
    let executor = kcm_runtime::async_executor::AsyncExecutor::new().unwrap();
    let db = Arc::new(Mutex::new(KnowledgeDatabase::new().unwrap()));

    let fact = Fact::new(SubjectID(1), PredicateID(0), ObjectID(100), 0.9).unwrap();
    let row_id = executor.block_on(kcm_runtime::async_executor::async_insert(db.clone(), fact));
    assert_eq!(row_id.unwrap(), RowID(0));

    let results = executor.block_on(kcm_runtime::async_executor::async_query_all(db.clone()));
    let facts = results.unwrap();
    assert_eq!(facts.len(), 1);
    assert_eq!(facts[0].subject, SubjectID(1));
    assert_eq!(facts[0].object, ObjectID(100));
}

#[test]
fn test_async_fact_count() {
    let executor = kcm_runtime::async_executor::AsyncExecutor::new().unwrap();
    let db = Arc::new(Mutex::new(KnowledgeDatabase::new().unwrap()));

    for i in 0..5 {
        let fact = Fact::new(SubjectID(i), PredicateID(0), ObjectID(i * 10), 0.9).unwrap();
        executor
            .block_on(kcm_runtime::async_executor::async_insert(db.clone(), fact))
            .unwrap();
    }

    let count = executor
        .block_on(kcm_runtime::async_executor::async_fact_count(db.clone()))
        .unwrap();
    assert_eq!(count, 5);
}

#[test]
fn test_compaction_reclaims_space() {
    let kb = KnowledgeDatabase::new().unwrap();

    // Insert 100 facts
    for i in 0..100u32 {
        let fact = Fact::new(SubjectID(i), PredicateID(0), ObjectID(i), 0.9).unwrap();
        kb.insert(&fact).unwrap();
    }
    assert_eq!(kb.active_fact_count(), 100);

    // Delete 50 facts
    for i in 0..50u64 {
        kb.delete(RowID(i)).unwrap();
    }
    assert_eq!(kb.active_fact_count(), 50);

    // Compact
    let compacted = kb.compact().unwrap();
    assert_eq!(compacted.active_fact_count(), 50);
    assert_eq!(compacted.fact_count(), 50);

    // Verify all remaining facts are accessible
    let results = compacted.query().execute().unwrap();
    assert_eq!(results.len(), 50);
}

#[test]
fn test_compaction_preserves_data() {
    let kb = KnowledgeDatabase::new().unwrap();

    for i in 0..20u32 {
        let fact = Fact::new(
            SubjectID(i),
            PredicateID((i % 5) as u8),
            ObjectID(i * 3),
            0.5 + (i as f64 * 0.02),
        )
        .unwrap();
        kb.insert(&fact).unwrap();
    }

    // Delete even-indexed facts
    for i in (0..20u64).step_by(2) {
        kb.delete(RowID(i)).unwrap();
    }

    let compacted = kb.compact().unwrap();
    let results = compacted.query().execute().unwrap();
    assert_eq!(results.len(), 10);

    // Verify data integrity
    for fact in &results {
        assert!(fact.subject.0 < 20);
        assert!(fact.confidence >= 0.0 && fact.confidence < 1.0);
    }
}
