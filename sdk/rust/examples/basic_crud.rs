//! KCM Rust SDK — Basic CRUD Example.
//!
//! Demonstrates: insert, query, update, delete operations on facts.

use kcm_sdk::{Database, Fact};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== KCM Rust SDK — Basic CRUD Example ===\n");

    let db = Database::new()?;

    // --- INSERT ---
    println!("--- Insert Facts ---");
    let fact0 = Fact::new(1, 0, 2, 0.95)?;
    let row0 = db.insert(&fact0)?;

    let fact1 = Fact::new(2, 1, 3, 0.90)?
        .with_evidence(1)
        .with_context(1)
        .with_owner(1);
    let row1 = db.insert(&fact1)?;

    let fact2 = Fact::new(3, 2, 4, 0.85)?
        .with_evidence(2)
        .with_context(2)
        .with_owner(2);
    let row2 = db.insert(&fact2)?;

    let fact3 = Fact::new(1, 3, 5, 0.80)?
        .with_evidence(3)
        .with_context(2)
        .with_priority(-1)
        .with_owner(7);
    let row3 = db.insert(&fact3)?;

    println!("  Inserted rows: {}, {}, {}, {}", row0, row1, row2, row3);
    println!(
        "  Total: {}, Active: {}",
        db.fact_count(),
        db.active_fact_count()
    );

    // --- QUERY ALL ---
    println!("\n--- Query All Facts ---");
    let all_facts = db.query_all()?;
    for f in &all_facts {
        println!(
            "  S={} P={} O={} conf={:.2}",
            f.subject, f.predicate, f.object, f.confidence
        );
    }

    // --- QUERY WITH KQL ---
    println!("\n--- KQL Query: all ---");
    let results: Vec<_> = db.query("all")?.collect();
    println!("  Query returned {} results:", results.len());
    for fact in &results {
        println!(
            "  S={} P={} O={}",
            fact.subject, fact.predicate, fact.object
        );
    }

    // --- UPDATE ---
    println!("\n--- Update Fact ---");
    let updated = Fact::new(10, 0, 20, 0.99)?
        .with_evidence(5)
        .with_context(3)
        .with_version(2)
        .with_priority(2)
        .with_owner(10);
    db.update(row0, &updated)?;
    println!("  Updated row {}: subject changed to 10", row0);

    // --- GET FACT ---
    println!("\n--- Get Fact ---");
    let retrieved = db.get_fact(row0)?.expect("fact not found");
    println!(
        "  Retrieved: S={} P={} O={} conf={:.2}",
        retrieved.subject, retrieved.predicate, retrieved.object, retrieved.confidence
    );

    // --- DELETE ---
    println!("\n--- Delete Fact ---");
    db.delete(row3)?;
    println!("  Deleted row {}", row3);
    println!(
        "  Total: {}, Active: {}",
        db.fact_count(),
        db.active_fact_count()
    );

    // --- VERIFY COUNTS ---
    println!("\n--- Verify Counts ---");
    assert_eq!(db.fact_count(), 4, "Expected 4 total");
    assert_eq!(db.active_fact_count(), 3, "Expected 3 active");
    println!("  Counts verified: 4 total, 3 active");

    db.close();
    println!("\n=== All operations completed ===");
    Ok(())
}
