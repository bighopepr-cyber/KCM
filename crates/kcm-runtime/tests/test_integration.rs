use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;

#[test]
fn test_insert_query() {
    let kb = KnowledgeDatabase::new().unwrap();

    let fact = Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap();

    let _row_id = kb.insert(&fact).unwrap();

    assert_eq!(kb.fact_count(), 1);

    let results = kb.query().with_subject(SubjectID(1)).execute().unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].subject, SubjectID(1));
}

#[test]
fn test_transaction() {
    let kb = KnowledgeDatabase::new().unwrap();

    let mut txn = kb.begin_transaction();

    let fact = Fact::new(SubjectID(10), PredicateID(5), ObjectID(20), 0.8).unwrap();

    txn.insert(fact).unwrap();
    let result = txn.commit();
    assert!(result.is_ok());
}

#[test]
fn test_inference() {
    use kcm_reasoning::inference::InferenceEngine;
    use kcm_reasoning::rule::{Rule, RulePattern};
    use kcm_storage::column::Schema;

    let mut engine = InferenceEngine::new();

    let rule = Rule::new(
        1,
        "test_rule".to_string(),
        RulePattern::subject_predicate_object(None, PredicateID(0), None),
        PredicateID(1),
        Box::new(|confs| confs[0] * 0.9),
    );

    engine.register_rule(rule).unwrap();

    let mut schema = Schema::new(10_000).unwrap();
    let fact = Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap();
    schema.append_fact(&fact).unwrap();

    let derived = engine.infer_forward_chaining(&mut schema).unwrap();
    assert!(!derived.is_empty());
}

#[test]
fn test_insert_batch() {
    let kb = KnowledgeDatabase::new().unwrap();

    let facts: Vec<Fact> = (0..10)
        .map(|i| {
            Fact::new(
                SubjectID(i),
                PredicateID((i % 5) as u8),
                ObjectID(i * 10),
                0.5 + (i as f64 * 0.05),
            )
            .unwrap()
        })
        .collect();

    let row_ids = kb.insert_batch(&facts).unwrap();
    assert_eq!(row_ids.len(), 10);
    assert_eq!(kb.fact_count(), 10);
}

#[test]
fn test_query_with_confidence_filter() {
    let kb = KnowledgeDatabase::new().unwrap();

    for i in 0..5 {
        let fact = Fact::new(
            SubjectID(1),
            PredicateID(0),
            ObjectID(i),
            0.2 + (i as f64 * 0.2),
        )
        .unwrap();
        kb.insert(&fact).unwrap();
    }

    let results = kb
        .query()
        .with_subject(SubjectID(1))
        .with_confidence(0.5)
        .execute()
        .unwrap();

    assert!(results.iter().all(|f| f.confidence >= 0.5));
}

#[test]
fn test_get_fact() {
    let kb = KnowledgeDatabase::new().unwrap();

    let fact = Fact::new(SubjectID(42), PredicateID(3), ObjectID(99), 0.75).unwrap();

    let row_id = kb.insert(&fact).unwrap();
    let retrieved = kb.get_fact(row_id).unwrap().unwrap();

    assert_eq!(retrieved.subject, SubjectID(42));
    assert_eq!(retrieved.predicate, PredicateID(3));
    assert_eq!(retrieved.object, ObjectID(99));
    assert_eq!(retrieved.confidence, 0.75);
}

#[test]
fn test_dictionary_operations() {
    let kb = KnowledgeDatabase::new().unwrap();

    let id1 = kb.dict_insert_subject("Alice").unwrap();
    let id2 = kb.dict_insert_subject("Bob").unwrap();
    let id1_again = kb.dict_insert_subject("Alice").unwrap();

    assert_eq!(id1, id1_again);
    assert_ne!(id1, id2);

    assert_eq!(kb.dict_get_subject(id1), Some("Alice".to_string()));
    assert_eq!(kb.dict_get_subject(id2), Some("Bob".to_string()));

    assert_eq!(kb.dict_lookup_subject("Alice"), Some(id1));
    assert_eq!(kb.dict_lookup_subject("Charlie"), None);
}

#[test]
fn test_determinism() {
    let kb = KnowledgeDatabase::new().unwrap();

    for i in 0..100 {
        let fact = Fact::new(
            SubjectID(i),
            PredicateID(0),
            ObjectID(i * 2),
            0.5 + (i as f64 * 0.001),
        )
        .unwrap();
        kb.insert(&fact).unwrap();
    }

    let mut results_history = Vec::new();
    for _ in 0..10 {
        let results = kb.query().execute().unwrap();
        results_history.push(results);
    }

    for i in 1..10 {
        assert_eq!(results_history[0], results_history[i]);
    }
}
