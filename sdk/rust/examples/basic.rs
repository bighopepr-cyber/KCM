use kcm_sdk::{Database, Fact};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::new()?;
    println!("Created database");

    let fact1 = Fact::new(1, 2, 3, 0.95)?
        .with_evidence(1)
        .with_context(1)
        .with_priority(0)
        .with_owner(1);
    let row1 = db.insert(&fact1)?;
    println!("Inserted fact at row {}", row1);

    let fact2 = Fact::new(4, 5, 6, 0.85)?
        .with_evidence(2)
        .with_context(1)
        .with_priority(1)
        .with_owner(2);
    let row2 = db.insert(&fact2)?;
    println!("Inserted fact at row {}", row2);

    let fact3 = Fact::new(1, 5, 9, 0.75)?
        .with_evidence(3)
        .with_context(2)
        .with_priority(0)
        .with_owner(3);
    db.insert(&fact3)?;

    println!("Total facts: {}", db.fact_count());
    println!("Active facts: {}", db.active_fact_count());

    let results = db.query("all")?;
    println!("Query returned {} results:", results.count());
    for fact in results {
        println!(
            "  S={} P={} O={} conf={:.2}",
            fact.subject, fact.predicate, fact.object, fact.confidence
        );
    }

    let updated = Fact::new(10, 20, 30, 0.99)?;
    db.update(row1, &updated)?;
    println!("Updated fact at row {}", row1);

    let retrieved = db.get_fact(row1)?.expect("fact not found");
    println!(
        "Retrieved: S={} P={} O={} conf={:.2}",
        retrieved.subject, retrieved.predicate, retrieved.object, retrieved.confidence
    );

    db.delete(row2)?;
    println!("Deleted fact at row {}", row2);
    println!("Active facts after delete: {}", db.active_fact_count());

    let mut txn = db.begin_transaction()?;
    let txn_fact = Fact::new(100, 200, 300, 0.6)?;
    txn.insert(&txn_fact)?;
    println!("Transaction changes: {}", txn.change_count());
    txn.commit()?;
    println!("Transaction committed");
    println!("Final fact count: {}", db.fact_count());

    let dir = tempfile::tempdir()?;
    let path = dir.path().join("example.kcm");
    let path_str = path.to_str().expect("invalid path");
    db.save(path_str)?;
    println!("Saved database to {}", path_str);

    Database::verify(path_str)?;
    println!("Database verification passed");

    let loaded = Database::load(path_str)?;
    println!("Loaded database with {} facts", loaded.fact_count());

    db.close();
    println!("Done!");

    Ok(())
}
