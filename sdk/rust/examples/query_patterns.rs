//! KCM Rust SDK — Query Patterns Example.
//!
//! Demonstrates: different query patterns and filtering options.

use kcm_sdk::{Database, Fact};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== KCM Rust SDK — Query Patterns Example ===\n");

    let db = Database::new()?;

    // Insert test data
    db.insert(&Fact::new(1, 0, 2, 0.95)?.with_evidence(1).with_context(1).with_owner(1)?)?;
    db.insert(&Fact::new(2, 1, 3, 0.90)?.with_evidence(2).with_context(1).with_owner(2)?)?;
    db.insert(&Fact::new(3, 2, 4, 0.85)?.with_evidence(3).with_context(2).with_owner(3)?)?;
    db.insert(&Fact::new(1, 3, 5, 0.80)?.with_evidence(1).with_context(2).with_owner(1)?)?;
    db.insert(&Fact::new(4, 0, 6, 0.75)?.with_evidence(2).with_context(1).with_owner(2)?)?;
    println!("Inserted 5 facts\n");

    // --- SELECT ALL (query "all") ---
    println!("--- Query: all ---");
    let results = db.query("all")?;
    println!("  Returned {} facts", results.count());

    // --- QUERY ALL CONVENIENCE ---
    println!("\n--- query_all() convenience method ---");
    let all_facts = db.query_all()?;
    println!("  Returned {} facts", all_facts.len());
    for f in &all_facts {
        println!(
            "  S={} P={} O={} conf={:.2}",
            f.subject, f.predicate, f.object, f.confidence
        );
    }
    assert_eq!(all_facts.len(), 5);

    // --- FILTER BY SUBJECT ---
    println!("\n--- Filter by Subject = 1 ---");
    let all = db.query_all()?;
    let filtered: Vec<_> = all.iter().filter(|f| f.subject == 1).collect();
    for f in &filtered {
        println!(
            "  S={} P={} O={}",
            f.subject, f.predicate, f.object
        );
    }
    println!("  Found {} facts with subject=1", filtered.len());
    assert_eq!(filtered.len(), 2);

    // --- FILTER BY PREDICATE ---
    println!("\n--- Filter by Predicate = 0 ---");
    let all = db.query_all()?;
    let filtered: Vec<_> = all.iter().filter(|f| f.predicate == 0).collect();
    for f in &filtered {
        println!(
            "  S={} P={} O={}",
            f.subject, f.predicate, f.object
        );
    }
    println!("  Found {} facts with predicate=0", filtered.len());
    assert_eq!(filtered.len(), 2);

    // --- MULTI-CONDITION FILTER ---
    println!("\n--- Filter: subject=1 AND predicate=3 ---");
    let all = db.query_all()?;
    let filtered: Vec<_> = all.iter()
        .filter(|f| f.subject == 1 && f.predicate == 3)
        .collect();
    for f in &filtered {
        println!(
            "  S={} P={} O={}",
            f.subject, f.predicate, f.object
        );
    }
    println!("  Found {} facts matching multi-condition", filtered.len());
    assert_eq!(filtered.len(), 1);

    // --- ITERATOR PATTERN ---
    println!("\n--- Iterator Pattern ---");
    let results = db.query("all")?;
    for fact in results {
        println!(
            "  S={} P={} O={} conf={:.2}",
            fact.subject, fact.predicate, fact.object, fact.confidence
        );
    }

    db.close();
    println!("\n=== All query patterns completed ===");
    Ok(())
}
