use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;
use std::time::Instant;

#[derive(Parser)]
#[command(name = "kcm-profile")]
#[command(about = "Profiling tool for KCM")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Profile insert operations
    Insert {
        #[arg(short, long, default_value = "10000")]
        ops: usize,
    },
    /// Profile query operations
    Query {
        #[arg(short, long, default_value = "10000")]
        ops: usize,
    },
    /// Profile memory usage
    Memory,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Insert { ops } => {
            println!("{}", "Insert Profile".bold());
            println!("  Operations: {}", ops);
            println!();

            let db = KnowledgeDatabase::new()?;

            // Warm-up
            for i in 0..1000 {
                db.insert(&Fact::new(
                    SubjectID((i % 100) as u32),
                    PredicateID(0),
                    ObjectID(i as u32),
                    0.5,
                )?)?;
            }

            let mut latencies = Vec::new();
            let start = Instant::now();
            for i in 0..*ops {
                let op_start = Instant::now();
                db.insert(&Fact::new(
                    SubjectID((i % 1000) as u32),
                    PredicateID((i % 10) as u8),
                    ObjectID((i % 500) as u32),
                    (i as f64 % 1000.0) / 1000.0,
                )?)?;
                latencies.push(op_start.elapsed().as_nanos() as f64);
            }
            let total = start.elapsed();

            latencies.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let p50 = latencies[latencies.len() / 2];
            let p99 = latencies[latencies.len() * 99 / 100];

            println!("  Total:    {:?}", total);
            println!("  P50:      {:.2} us", p50 / 1000.0);
            println!("  P99:      {:.2} us", p99 / 1000.0);
            println!(
                "  Throughput: {:.0} ops/sec",
                *ops as f64 / total.as_secs_f64()
            );
            Ok(())
        }
        Commands::Query { ops } => {
            println!("{}", "Query Profile".bold());
            let db = KnowledgeDatabase::new()?;
            for i in 0..10000 {
                db.insert(&Fact::new(
                    SubjectID((i % 100) as u32),
                    PredicateID((i % 5) as u8),
                    ObjectID((i % 200) as u32),
                    (i as f64 % 100.0) / 100.0,
                )?)?;
            }

            let start = Instant::now();
            for _ in 0..*ops {
                let _ = db.query().execute()?;
            }
            let total = start.elapsed();

            println!("  Total:    {:?}", total);
            println!(
                "  Latency:  {:.2} us/query",
                total.as_micros() as f64 / *ops as f64
            );
            println!(
                "  Throughput: {:.0} queries/sec",
                *ops as f64 / total.as_secs_f64()
            );
            Ok(())
        }
        Commands::Memory => {
            println!("{}", "Memory Profile".bold());
            let db = KnowledgeDatabase::new()?;
            for i in 0..10000 {
                db.insert(&Fact::new(
                    SubjectID((i % 1000) as u32),
                    PredicateID((i % 10) as u8),
                    ObjectID((i % 500) as u32),
                    0.95,
                )?)?;
            }
            let fact_count = db.fact_count() as u64;
            let memory_bytes = fact_count * 34;
            println!("  Facts:     {}", db.fact_count());
            println!("  Memory:    {:.2} MB", memory_bytes as f64 / 1_048_576.0);
            println!(
                "  Per-fact:  {} bytes",
                memory_bytes / db.fact_count() as u64
            );
            Ok(())
        }
    }
}
