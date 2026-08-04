use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== KCM Transactions ===\n");

    let db = KnowledgeDatabase::new()?;

    // Successful transaction
    println!("--- Successful Transaction ---");
    let mut txn = db.begin_transaction();
    txn.insert(Fact::new(SubjectID(1), PredicateID(1), ObjectID(2), 0.95)?)?;
    txn.insert(Fact::new(SubjectID(2), PredicateID(1), ObjectID(3), 0.90)?)?;
    txn.commit()?;
    println!("Committed 2 facts (count={})", db.fact_count());

    // Rollback transaction
    println!("\n--- Rollback Transaction ---");
    let mut txn = db.begin_transaction();
    txn.insert(Fact::new(SubjectID(3), PredicateID(2), ObjectID(4), 0.85)?)?;
    txn.rollback()?;
    println!("Rolled back (count={})", db.fact_count());

    // Direct insert
    println!("\n--- Direct Insert ---");
    db.insert(&Fact::new(SubjectID(10), PredicateID(0), ObjectID(20), 0.99)?)?;
    println!("Direct insert (count={})", db.fact_count());

    println!("\nAll transaction operations completed!");
    Ok(())
}
