use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;
use std::time::Instant;

#[test]
fn test_soak_insert_query_cycle() {
    let db = KnowledgeDatabase::new().unwrap();
    let start = Instant::now();
    let duration_secs = 5;

    let mut total_inserts = 0u64;
    let mut total_queries = 0u64;

    while start.elapsed().as_secs() < duration_secs {
        for i in 0..100 {
            db.insert(
                &Fact::new(
                    SubjectID(((total_inserts as usize + i) % 10000) as u32),
                    PredicateID((i % 10) as u8),
                    ObjectID((i as u32) % 5000),
                    0.95,
                )
                .unwrap(),
            )
            .unwrap();
        }
        total_inserts += 100;

        for _ in 0..10 {
            let results = db.query().execute().unwrap();
            assert!(!results.is_empty());
            total_queries += 1;
        }

        if total_inserts.is_multiple_of(500) {
            for i in 0..50 {
                let _ = db.delete(RowID(i));
            }
        }
    }

    let elapsed = start.elapsed();
    println!("Soak test results:");
    println!("  Duration:   {:?}", elapsed);
    println!(
        "  Inserts:    {} ({:.0}/sec)",
        total_inserts,
        total_inserts as f64 / elapsed.as_secs_f64()
    );
    println!(
        "  Queries:    {} ({:.0}/sec)",
        total_queries,
        total_queries as f64 / elapsed.as_secs_f64()
    );
    println!("  Final facts: {}", db.fact_count());
    println!("  Active:     {}", db.active_fact_count());
}

#[test]
fn test_soak_memory_stability() {
    let db = KnowledgeDatabase::new().unwrap();

    for cycle in 0..3 {
        let start = Instant::now();

        for i in 0..50_000 {
            db.insert(
                &Fact::new(
                    SubjectID((i % 10000) as u32),
                    PredicateID(0),
                    ObjectID(i as u32 % 5000),
                    0.5,
                )
                .unwrap(),
            )
            .unwrap();
        }

        for i in 0..50_000 {
            let _ = db.delete(RowID(i));
        }

        println!(
            "Cycle {}: {:?} (facts={}, active={})",
            cycle + 1,
            start.elapsed(),
            db.fact_count(),
            db.active_fact_count()
        );
    }

    assert_eq!(db.fact_count(), 150_000);

    db.insert(&Fact::new(SubjectID(1), PredicateID(0), ObjectID(1), 0.95).unwrap())
        .unwrap();
    assert!(db.active_fact_count() >= 1);
}
