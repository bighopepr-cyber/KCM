use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;
use kcm_runtime::metrics::Metrics;
use kcm_runtime::health::HealthCheck;
use std::sync::Arc;

#[test]
fn test_database_full_lifecycle() {
    let db = KnowledgeDatabase::new().unwrap();
    assert_eq!(db.fact_count(), 0);

    for i in 0..1000 {
        db.insert(&Fact::new(
            SubjectID((i % 100) as u32),
            PredicateID((i % 10) as u8),
            ObjectID((i % 200) as u32),
            (i as f64 % 100.0) / 100.0,
        )
        .unwrap())
        .unwrap();
    }
    assert_eq!(db.fact_count(), 1000);
    assert_eq!(db.active_fact_count(), 1000);

    let results = db.query().execute().unwrap();
    assert_eq!(results.len(), 1000);

    let filtered = db.query().with_subject(SubjectID(1)).execute().unwrap();
    assert!(!filtered.is_empty());

    let fact = db.get_fact(RowID(0)).unwrap();
    assert!(fact.is_some());

    db.delete(RowID(0)).unwrap();
    assert_eq!(db.active_fact_count(), 999);

    let mut txn = db.begin_transaction();
    txn.insert(Fact::new(SubjectID(999), PredicateID(9), ObjectID(999), 0.5).unwrap())
        .unwrap();
    txn.commit().unwrap();

    let id = db.dict_insert_subject("test_subject").unwrap();
    let name = db.dict_get_subject(id);
    assert_eq!(name.as_deref(), Some("test_subject"));

    let schema = db.get_schema();
    assert!(!schema.is_empty());

    let compacted = db.compact().unwrap();
    assert!(compacted.fact_count() > 0);

    println!("Full lifecycle: {} facts", db.fact_count());
}

#[test]
fn test_metrics_full_coverage() {
    let metrics = Metrics::new();

    metrics.record_insert(true);
    metrics.record_insert(false);
    metrics.record_query(10, true);
    metrics.record_query(20, false);
    metrics.record_cache_hit();
    metrics.record_cache_miss();
    metrics.record_inference(5);
    metrics.update_memory_estimate(1024);
    metrics.update_schema_stats(100, 90, 10);

    let snapshot = metrics.snapshot();
    assert!(snapshot.inserts_total > 0);
    assert!(snapshot.queries_total > 0);
    assert!(snapshot.estimated_memory_bytes > 0);

    let json = metrics.snapshot().to_json();
    assert!(json.contains("inserts"));
    assert!(json.contains("queries"));

    let avg = metrics.get_avg_query_latency_ms();
    assert!(avg >= 0.0);

    let ratio = metrics.get_cache_hit_ratio();
    assert!(ratio >= 0.0 && ratio <= 1.0);
}

#[test]
fn test_health_check() {
    let metrics = Arc::new(Metrics::new());
    let health = HealthCheck::new(metrics);

    let result = health.check();
    drop(result);
}

#[test]
fn test_optimized_query_pipeline() {
    let db = KnowledgeDatabase::new().unwrap();

    for i in 0..500 {
        db.insert(&Fact::new(
            SubjectID((i % 10) as u32),
            PredicateID((i % 5) as u8),
            ObjectID((i % 50) as u32),
            (i as f64 % 100.0) / 100.0,
        )
        .unwrap())
        .unwrap();
    }

    let by_subject = db.query().with_subject(SubjectID(3)).execute().unwrap();
    assert!(by_subject.iter().all(|f| f.subject.0 == 3));

    let by_predicate = db.query().with_predicate(PredicateID(2)).execute().unwrap();
    assert!(by_predicate.iter().all(|f| f.predicate.0 == 2));

    let by_object = db.query().with_object(ObjectID(7)).execute().unwrap();
    assert!(by_object.iter().all(|f| f.object.0 == 7));

    let by_confidence = db.query().with_confidence(0.9).execute().unwrap();
    assert!(by_confidence.iter().all(|f| f.confidence >= 0.9));
}
