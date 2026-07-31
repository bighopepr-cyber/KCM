use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;

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

    let results1 = kb1
        .query()
        .with_predicate(PredicateID(0))
        .execute()
        .unwrap();
    let results2 = kb2
        .query()
        .with_predicate(PredicateID(0))
        .execute()
        .unwrap();

    assert_eq!(results1.len(), results2.len());
    for (f1, f2) in results1.iter().zip(results2.iter()) {
        assert_eq!(f1.subject, f2.subject);
        assert_eq!(f1.object, f2.object);
        assert_eq!(f1.confidence, f2.confidence);
    }
}

#[test]
fn test_batch_insert_consistency() {
    let kb = KnowledgeDatabase::new().unwrap();
    let facts: Vec<Fact> = (0..500)
        .map(|i| {
            Fact::new(
                SubjectID(i % 20),
                PredicateID((i % 5) as u8),
                ObjectID(i),
                0.5 + (i as f64 * 0.001),
            )
            .unwrap()
        })
        .collect();

    let ids = kb.insert_batch(&facts).unwrap();
    assert_eq!(ids.len(), 500);
    assert_eq!(kb.fact_count(), 500);
}

#[test]
fn test_update_preserves_other_facts() {
    let kb = KnowledgeDatabase::new().unwrap();

    for i in 0..10u32 {
        let fact = Fact::new(SubjectID(i), PredicateID(0), ObjectID(i * 10), 0.9).unwrap();
        kb.insert(&fact).unwrap();
    }

    let updated = Fact::new(SubjectID(0), PredicateID(9), ObjectID(999), 0.1).unwrap();
    kb.update(RowID(0), &updated).unwrap();

    assert_eq!(kb.fact_count(), 10);

    let fact5 = kb.get_fact(RowID(5)).unwrap().unwrap();
    assert_eq!(fact5.subject, SubjectID(5));
    assert_eq!(fact5.predicate, PredicateID(0));

    let fact0 = kb.get_fact(RowID(0)).unwrap().unwrap();
    assert_eq!(fact0.predicate, PredicateID(9));
    assert_eq!(fact0.object, ObjectID(999));
}

#[test]
fn test_delete_query_consistency() {
    let kb = KnowledgeDatabase::new().unwrap();

    for i in 0..20u32 {
        let fact = Fact::new(SubjectID(i % 5), PredicateID(0), ObjectID(i), 0.8).unwrap();
        kb.insert(&fact).unwrap();
    }

    for i in 0..5u64 {
        kb.delete(RowID(i * 4)).unwrap();
    }

    assert_eq!(kb.active_fact_count(), 15);

    let results = kb.query().with_subject(SubjectID(0)).execute().unwrap();
    for fact in &results {
        assert_eq!(fact.subject, SubjectID(0));
        assert!(kb.get_fact(RowID(0)).unwrap().is_none());
    }
}

#[test]
fn test_concurrent_read_write() {
    use std::sync::Arc;
    let kb = Arc::new(KnowledgeDatabase::new().unwrap());

    for i in 0..100 {
        let fact = Fact::new(SubjectID(i % 10), PredicateID(0), ObjectID(i), 0.9).unwrap();
        kb.insert(&fact).unwrap();
    }

    let mut handles = Vec::new();
    let kb_read = kb.clone();
    handles.push(std::thread::spawn(move || {
        for _ in 0..50 {
            let results = kb_read.query().execute().unwrap();
            assert!(!results.is_empty());
        }
    }));

    let kb_write = kb.clone();
    handles.push(std::thread::spawn(move || {
        for i in 100..150 {
            let fact = Fact::new(SubjectID(i % 10), PredicateID(1), ObjectID(i), 0.7).unwrap();
            kb_write.insert(&fact).unwrap();
        }
    }));

    for h in handles {
        h.join().unwrap();
    }

    assert_eq!(kb.fact_count(), 150);
}

#[test]
fn test_query_builder_chaining() {
    let kb = KnowledgeDatabase::new().unwrap();

    for i in 0..100u32 {
        let fact = Fact::new(
            SubjectID(i % 10),
            PredicateID((i % 5) as u8),
            ObjectID(i),
            0.1 + (i as f64 * 0.009),
        )
        .unwrap();
        kb.insert(&fact).unwrap();
    }

    let results = kb
        .query()
        .with_subject(SubjectID(5))
        .with_predicate(PredicateID(2))
        .with_confidence(0.5)
        .execute()
        .unwrap();

    for fact in &results {
        assert_eq!(fact.subject, SubjectID(5));
        assert_eq!(fact.predicate, PredicateID(2));
        assert!(fact.confidence >= 0.5);
    }
}

#[test]
fn test_large_dataset_query_performance() {
    let kb = KnowledgeDatabase::new().unwrap();

    for i in 0..10_000u32 {
        let fact = Fact::new(
            SubjectID(i % 100),
            PredicateID((i % 10) as u8),
            ObjectID(i),
            0.5,
        )
        .unwrap();
        kb.insert(&fact).unwrap();
    }

    let start = std::time::Instant::now();
    let results = kb.query().with_predicate(PredicateID(5)).execute().unwrap();
    let elapsed = start.elapsed();

    assert_eq!(results.len(), 1000);
    assert!(
        elapsed.as_millis() < 1000,
        "Query took {}ms",
        elapsed.as_millis()
    );
}

#[test]
fn test_fact_getter_boundary() {
    let kb = KnowledgeDatabase::new().unwrap();
    let fact = Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap();
    let id = kb.insert(&fact).unwrap();

    assert!(kb.get_fact(RowID(id.0)).unwrap().is_some());
    assert!(kb.get_fact(RowID(id.0 + 1)).unwrap().is_none());
    assert!(kb.get_fact(RowID(u64::MAX)).unwrap().is_none());
}

#[test]
fn test_multiple_delete_query() {
    let kb = KnowledgeDatabase::new().unwrap();

    for i in 0..10u32 {
        let fact = Fact::new(SubjectID(1), PredicateID(0), ObjectID(i), 0.9).unwrap();
        kb.insert(&fact).unwrap();
    }

    for i in 0..10u64 {
        kb.delete(RowID(i)).unwrap();
    }

    assert_eq!(kb.active_fact_count(), 0);

    let results = kb.query().execute().unwrap();
    assert!(results.is_empty());
}
