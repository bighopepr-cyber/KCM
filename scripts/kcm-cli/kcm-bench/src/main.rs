use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;
use std::time::Instant;

#[derive(Parser)]
#[command(name = "kcm-bench")]
#[command(about = "Benchmarking tool for KCM")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Insert {
        #[arg(short, long, default_value = "10000")]
        ops: usize,
    },
    Query {
        #[arg(short, long, default_value = "10000")]
        ops: usize,
    },
    Run,
    Batch {
        #[arg(short, long, default_value = "1000")]
        size: usize,
        #[arg(short, long, default_value = "100")]
        batches: usize,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Insert { ops } => {
            println!("{}", "Insert Benchmark".bold());
            let db = KnowledgeDatabase::new()?;
            let start = Instant::now();
            for i in 0..*ops {
                db.insert(&Fact::new(
                    SubjectID((i % 1000) as u32),
                    PredicateID((i % 10) as u8),
                    ObjectID((i % 500) as u32),
                    (i as f64 % 1000.0) / 1000.0,
                )?)?;
            }
            let elapsed = start.elapsed();
            println!("  {} ops in {:?}", ops, elapsed);
            println!(
                "  Throughput: {:.0} ops/sec",
                *ops as f64 / elapsed.as_secs_f64()
            );
            println!(
                "  Latency:    {:.2} us/op",
                elapsed.as_micros() as f64 / *ops as f64
            );
            Ok(())
        }
        Commands::Query { ops } => {
            println!("{}", "Query Benchmark".bold());
            let db = KnowledgeDatabase::new()?;
            for i in 0..10000 {
                db.insert(&Fact::new(
                    SubjectID((i % 1000) as u32),
                    PredicateID((i % 10) as u8),
                    ObjectID((i % 500) as u32),
                    0.95,
                )?)?;
            }
            let start = Instant::now();
            for _ in 0..*ops {
                let _ = db.query().execute()?;
            }
            let elapsed = start.elapsed();
            println!("  {} queries in {:?}", ops, elapsed);
            println!(
                "  Throughput: {:.0} queries/sec",
                *ops as f64 / elapsed.as_secs_f64()
            );
            println!(
                "  Latency:    {:.2} us/query",
                elapsed.as_micros() as f64 / *ops as f64
            );
            Ok(())
        }
        Commands::Run => {
            println!("{}", "Full Benchmark Suite".bold());
            println!();

            let db = KnowledgeDatabase::new()?;

            // Insert 10K
            let start = Instant::now();
            for i in 0..10_000 {
                db.insert(&Fact::new(
                    SubjectID((i % 1000) as u32),
                    PredicateID((i % 10) as u8),
                    ObjectID((i % 500) as u32),
                    (i as f64 % 1000.0) / 1000.0,
                )?)?;
            }
            let insert_time = start.elapsed();

            // Query 10K
            let start = Instant::now();
            for _ in 0..10_000 {
                let _ = db.query().execute()?;
            }
            let query_time = start.elapsed();

            // Filtered query 10K
            let start = Instant::now();
            for _ in 0..10_000 {
                let _ = db.query().with_subject(SubjectID(1)).execute()?;
            }
            let filtered_time = start.elapsed();

            // Batch insert
            let batch_size = 1000;
            let start = Instant::now();
            let facts: Vec<Fact> = (0..batch_size)
                .map(|i| {
                    Fact::new(
                        SubjectID((i % 1000) as u32),
                        PredicateID((i % 10) as u8),
                        ObjectID((i % 500) as u32),
                        0.95,
                    )
                    .expect("benchmark fact creation should always succeed")
                })
                .collect();
            db.insert_batch(&facts)?;
            let batch_time = start.elapsed();

            println!("  {:<25} {:>12} {:>15}", "Operation", "Time", "Throughput");
            println!(
                "  {:<25} {:>12} {:>15}",
                "-".repeat(25),
                "-".repeat(12),
                "-".repeat(15)
            );
            println!(
                "  {:<25} {:>12.2?} {:>14.0}/s",
                "Insert 10K",
                insert_time,
                10000.0 / insert_time.as_secs_f64()
            );
            println!(
                "  {:<25} {:>12.2?} {:>14.0}/s",
                "Query 10K",
                query_time,
                10000.0 / query_time.as_secs_f64()
            );
            println!(
                "  {:<25} {:>12.2?} {:>14.0}/s",
                "Filtered 10K",
                filtered_time,
                10000.0 / filtered_time.as_secs_f64()
            );
            println!(
                "  {:<25} {:>12.2?} {:>14.0}/s",
                "Batch insert 1K",
                batch_time,
                1000.0 / batch_time.as_secs_f64()
            );
            println!();
            println!("  Total facts: {}", db.fact_count());
            Ok(())
        }
        Commands::Batch { size, batches } => {
            println!("{}", "Batch Insert Benchmark".bold());
            let db = KnowledgeDatabase::new()?;
            let start = Instant::now();
            for b in 0..*batches {
                let facts: Vec<Fact> = (0..*size)
                    .map(|i| {
                        Fact::new(
                            SubjectID(((b * size + i) % 10000) as u32),
                            PredicateID((i % 10) as u8),
                            ObjectID((i % 500) as u32),
                            0.95,
                        )
                        .expect("benchmark fact creation should always succeed")
                    })
                    .collect();
                db.insert_batch(&facts)?;
            }
            let elapsed = start.elapsed();
            let total = *size * *batches;
            println!(
                "  Inserted {} facts in {} batches of {}",
                total, batches, size
            );
            println!("  Total:    {:?}", elapsed);
            println!(
                "  Throughput: {:.0} facts/sec",
                total as f64 / elapsed.as_secs_f64()
            );
            Ok(())
        }
    }
}
