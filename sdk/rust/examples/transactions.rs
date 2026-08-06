//! KCM Rust SDK — Transaction Example.
//!
//! Demonstrates: begin, commit, and rollback scenarios with transactions.

use kcm_sdk::{Database, Fact};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== KCM Rust SDK — Transaction Example ===\n");

    let db = Database::new()?;

    // Insert baseline facts
    db.insert(&Fact::new(1, 0, 2, 0.95)?)?;
    db.insert(&Fact::new(2, 1, 3, 0.90)?)?;
    println!("Initial: {} active facts\n", db.active_fact_count());

    // --- COMMITTED TRANSACTION ---
    println!("--- Committed Transaction ---");
    let mut txn = db.begin_transaction()?;
    let txn_fact = Fact::new(3, 2, 4, 0.85)?
        .with_evidence(2)
        .with_context(2)
        .with_owner(2);
    txn.insert(&txn_fact)?;
    println!("  Inserted fact in transaction, changes: {}", txn.change_count());
    txn.commit()?;
    println!("  Committed transaction");
    println!("  After commit: {} active facts", db.active_fact_count());
    assert_eq!(db.active_fact_count(), 3);

    // --- ROLLED BACK TRANSACTION ---
    println!("\n--- Rolled Back Transaction ---");
    let mut txn2 = db.begin_transaction()?;
    let txn_fact2 = Fact::new(4, 3, 5, 0.80)?
        .with_evidence(3)
        .with_context(2)
        .with_owner(3);
    txn2.insert(&txn_fact2)?;
    println!("  Inserted fact in transaction");
    txn2.rollback()?;
    println!("  Rolled back transaction");
    println!("  After rollback: {} active facts", db.active_fact_count());
    assert_eq!(db.active_fact_count(), 3);

    // --- MULTIPLE OPERATIONS ---
    println!("\n--- Multiple Operations in Transaction ---");
    let mut txn3 = db.begin_transaction()?;
    txn3.insert(&Fact::new(10, 0, 20, 0.99)?)?;
    txn3.insert(&Fact::new(30, 1, 40, 0.88)?)?;
    txn3.insert(&Fact::new(50, 2, 60, 0.77)?)?;
    println!("  3 pending operations, changes: {}", txn3.change_count());
    txn3.commit()?;
    println!("  After commit: {} active facts", db.active_fact_count());

    db.close();
    println!("\n=== All transaction operations completed ===");
    Ok(())
}
