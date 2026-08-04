//! Transaction example
//! 
//! This example demonstrates:
//! - Beginning a transaction
//! - Inserting facts within a transaction
//! - Committing a transaction
//! - Rolling back a transaction

use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;
use anyhow::Result;

fn main() -> Result<()> {
    let db = KnowledgeDatabase::new()?;
    
    // Successful transaction
    println!("=== Successful Transaction ===");
    let mut txn = db.begin_transaction();
    txn.insert(Fact::new(SubjectID(1), PredicateID(1), ObjectID(2), 0.95)?)?;
    txn.insert(Fact::new(SubjectID(2), PredicateID(1), ObjectID(3), 0.90)?)?;
    txn.commit()?;
    println!("Committed 2 facts");
    println!("Total facts: {}", db.fact_count());
    
    // Failed transaction (rollback)
    println!("\n=== Failed Transaction (Rollback) ===");
    let mut txn = db.begin_transaction();
    txn.insert(Fact::new(SubjectID(3), PredicateID(2), ObjectID(4), 0.85)?)?;
    txn.rollback()?;
    println!("Rolled back transaction");
    println!("Total facts: {}", db.fact_count());
    
    Ok(())
}
