//! Basic KCM usage example
//! 
//! This example demonstrates:
//! - Creating a database
//! - Inserting facts
//! - Querying facts
//! - Checking statistics

use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;
use anyhow::Result;

fn main() -> Result<()> {
    // Create a new in-memory database
    let db = KnowledgeDatabase::new()?;
    
    println!("Created database");
    
    // Insert some facts
    let facts = vec![
        Fact::new(SubjectID(1), PredicateID(1), ObjectID(2), 0.95)?,
        Fact::new(SubjectID(2), PredicateID(1), ObjectID(3), 0.90)?,
        Fact::new(SubjectID(3), PredicateID(2), ObjectID(4), 0.85)?,
    ];
    
    for fact in &facts {
        db.insert(fact)?;
        println!("Inserted: subject={}, predicate={}, object={}, confidence={}",
            fact.subject.0, fact.predicate.0, fact.object.0, fact.confidence);
    }
    
    // Check statistics
    println!("\nStatistics:");
    println!("  Total facts: {}", db.fact_count());
    println!("  Active facts: {}", db.active_fact_count());
    
    Ok(())
}
