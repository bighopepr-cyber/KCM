//! KCM Rust SDK — Persistence Example.
//!
//! Demonstrates: save, load, and verify database persistence.

use kcm_sdk::{Database, Fact};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== KCM Rust SDK — Persistence Example ===\n");

    let dir = tempfile::tempdir()?;
    let path = dir.path().join("example.kcm");
    let path_str = path.to_str().expect("invalid path");

    // --- SAVE DATABASE ---
    println!("--- Save Database ---");
    let db = Database::new()?;
    db.insert(
        &Fact::new(1, 0, 2, 0.95)?
            .with_evidence(1)
            .with_context(1)
            .with_owner(1),
    )?;
    db.insert(
        &Fact::new(2, 1, 3, 0.90)?
            .with_evidence(2)
            .with_context(1)
            .with_owner(2),
    )?;
    db.insert(
        &Fact::new(3, 2, 4, 0.85)?
            .with_evidence(3)
            .with_context(2)
            .with_owner(3),
    )?;
    println!(
        "  Facts before save: {} total, {} active",
        db.fact_count(),
        db.active_fact_count()
    );
    db.save(path_str)?;
    println!("  Saved to {}", path_str);

    // --- VERIFY FILE ---
    println!("\n--- Verify Database File ---");
    Database::verify(path_str)?;
    println!("  Verification passed");

    // --- LOAD INTO NEW DATABASE ---
    println!("\n--- Load Into New Database ---");
    let db2 = Database::load(path_str)?;
    println!(
        "  Loaded: {} total, {} active",
        db2.fact_count(),
        db2.active_fact_count()
    );
    assert_eq!(db2.fact_count(), 3);
    assert_eq!(db2.active_fact_count(), 3);

    // --- VERIFY DATA INTEGRITY ---
    println!("\n--- Verify Data Integrity ---");
    let all_facts = db2.query_all()?;
    for f in &all_facts {
        println!(
            "  S={} P={} O={} conf={:.2}",
            f.subject, f.predicate, f.object, f.confidence
        );
    }
    assert_eq!(all_facts.len(), 3);

    // --- SAVE-LOAD ROUND TRIP ---
    println!("\n--- Save-Load Round Trip ---");
    db2.insert(&Fact::new(10, 0, 20, 0.99)?)?;
    db2.save(path_str)?;
    let db3 = Database::load(path_str)?;
    println!(
        "  Round-trip: {} total, {} active",
        db3.fact_count(),
        db3.active_fact_count()
    );
    assert_eq!(db3.fact_count(), 4);
    assert_eq!(db3.active_fact_count(), 4);

    db.close();
    println!("\n=== All persistence operations completed ===");
    Ok(())
}
