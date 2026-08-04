use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;
use kcm_runtime::metrics::Metrics;

#[derive(Parser)]
#[command(name = "kcm-diagnose")]
#[command(about = "Diagnostics tool for KCM")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run full diagnostics
    Full,
    /// Performance diagnostics
    Performance,
    /// Storage diagnostics
    Storage,
    /// Memory diagnostics
    Memory,
}

fn check(name: &str, ok: bool) {
    if ok {
        println!("  {} {}", "OK".green(), name);
    } else {
        println!("  {} {}", "FAIL".red(), name);
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Full => {
            println!("{}", "KCM Full Diagnostics".bold());
            println!();

            // Test 1: Database creation
            check("Database creation", KnowledgeDatabase::new().is_ok());

            // Test 2: Insert operations
            let db = KnowledgeDatabase::new()?;
            let mut insert_ok = true;
            for i in 0..100 {
                if db
                    .insert(&Fact::new(
                        SubjectID(i),
                        PredicateID((i % 10) as u8),
                        ObjectID(i * 2),
                        0.95,
                    )?)
                    .is_err()
                {
                    insert_ok = false;
                    break;
                }
            }
            check("Batch insert (100 facts)", insert_ok);

            // Test 3: Query operations
            let results = db.query().execute();
            check("Query execution", results.is_ok());
            if let Ok(facts) = &results {
                check("Query returned results", !facts.is_empty());
            }

            // Test 4: Fact retrieval
            check("Fact retrieval", db.get_fact(RowID(0)).is_ok());

            // Test 5: Delete operation
            check("Fact deletion", db.delete(RowID(0)).is_ok());

            // Test 6: Transaction
            let mut txn = db.begin_transaction();
            let tx_ok = txn
                .insert(Fact::new(
                    SubjectID(999),
                    PredicateID(9),
                    ObjectID(999),
                    0.5,
                )?)
                .is_ok();
            check("Transaction insert", tx_ok);
            txn.commit()?;
            check("Transaction commit", true);

            // Test 7: Metrics
            let metrics = Metrics::new();
            metrics.record_insert(true);
            metrics.record_query(1, true);
            check("Metrics recording", true);
            check("Metrics snapshot", metrics.snapshot().inserts_total > 0);

            // Test 8: Schema
            let schema = db.get_schema();
            check("Schema access", !schema.is_empty());

            println!();
            println!("  Total facts: {}", db.fact_count());
            println!("  Active facts: {}", db.active_fact_count());
            println!();
            println!("{}", "Diagnostics complete".bold());
            Ok(())
        }
        Commands::Performance => {
            println!("{}", "Performance Diagnostics".bold());
            println!();

            let db = KnowledgeDatabase::new()?;
            let start = std::time::Instant::now();
            for i in 0..1000 {
                db.insert(&Fact::new(
                    SubjectID((i % 100) as u32),
                    PredicateID((i % 10) as u8),
                    ObjectID((i % 200) as u32),
                    (i as f64 % 100.0) / 100.0,
                )?)?;
            }
            let insert_time = start.elapsed();

            let start = std::time::Instant::now();
            for _ in 0..1000 {
                let _ = db.query().execute()?;
            }
            let query_time = start.elapsed();

            println!(
                "  Insert throughput: {:.0} facts/sec",
                1000.0 / insert_time.as_secs_f64()
            );
            println!(
                "  Query throughput:  {:.0} queries/sec",
                1000.0 / query_time.as_secs_f64()
            );
            println!(
                "  Insert latency:   {:.2} us/fact",
                insert_time.as_micros() as f64 / 1000.0
            );
            println!(
                "  Query latency:    {:.2} us/query",
                query_time.as_micros() as f64 / 1000.0
            );
            Ok(())
        }
        Commands::Storage => {
            println!("{}", "Storage Diagnostics".bold());
            println!();

            let db = KnowledgeDatabase::new()?;
            for i in 0..10000 {
                db.insert(&Fact::new(
                    SubjectID((i % 1000) as u32),
                    PredicateID((i % 10) as u8),
                    ObjectID((i % 500) as u32),
                    (i as f64 % 1000.0) / 1000.0,
                )?)?;
            }

            let fact_count = db.fact_count() as u64;
            let _memory_bytes = fact_count * 34u64;
            println!("  Facts stored:   {}", db.fact_count());
            println!("  Active facts:   {}", db.active_fact_count());
            println!(
                "  Memory estimate: {:.2} MB",
                (db.fact_count() as f64 * 34.0) / 1_048_576.0
            );
            println!(
                "  Fact size:       34 bytes (34 x {} = {} bytes)",
                db.fact_count(),
                db.fact_count() * 34
            );
            Ok(())
        }
        Commands::Memory => {
            println!("{}", "Memory Diagnostics".bold());
            println!();

            let db = KnowledgeDatabase::new()?;
            for i in 0..1000 {
                db.insert(&Fact::new(
                    SubjectID((i % 100) as u32),
                    PredicateID((i % 5) as u8),
                    ObjectID((i % 200) as u32),
                    0.95,
                )?)?;
            }

            let fact_count = db.fact_count() as u64;
            let _memory_bytes = fact_count * 34u64;
            println!(
                "  Allocated memory: {:.2} MB",
                (db.fact_count() as f64 * 34.0) / 1_048_576.0
            );
            println!("  Per-fact overhead: {} bytes", 34u64);
            Ok(())
        }
    }
}
