use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== KCM Basic Usage ===\n");

    let db = KnowledgeDatabase::new()?;
    let fact1 = Fact::new(SubjectID(1), PredicateID(1), ObjectID(2), 0.95)?;
    let fact2 = Fact::new(SubjectID(2), PredicateID(1), ObjectID(3), 0.90)?;
    let fact3 = Fact::new(SubjectID(3), PredicateID(2), ObjectID(4), 0.85)?;

    db.insert(&fact1)?;
    db.insert(&fact2)?;
    db.insert(&fact3)?;
    println!("Inserted 3 facts (count={})", db.fact_count());

    let results = db.query().execute()?;
    println!("\nQuery all ({} results):", results.len());
    for f in &results {
        println!("  subject={} predicate={} object={} confidence={:.2}",
            f.subject.0, f.predicate.0, f.object.0, f.confidence);
    }

    let filtered = db.query().with_subject(SubjectID(1)).execute()?;
    println!("\nFiltered by subject=1: {} results", filtered.len());

    let id = db.dict_insert_subject("planet")?;
    let name = db.dict_get_subject(id);
    println!("\nDictionary: planet={} -> {:?}", id, name);

    let mut txn = db.begin_transaction();
    txn.insert(Fact::new(SubjectID(4), PredicateID(3), ObjectID(5), 0.80)?)?;
    txn.commit()?;
    println!("Committed transaction (count={})", db.fact_count());

    db.delete(RowID(0))?;
    println!("Deleted row 0 (active={})", db.active_fact_count());

    println!("\nFact count: {}", db.fact_count());
    println!("Active:     {}", db.active_fact_count());
    println!("\nAll basic operations completed successfully!");
    Ok(())
}
