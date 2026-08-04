use anyhow::Result;
use clap::{Parser, Subcommand};
use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;
use colored::Colorize;
use std::time::Instant;

#[derive(Parser)]
#[command(name = "kcm-compact")]
#[command(about = "Storage compaction tool for KCM")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run compaction on a populated database
    Run {
        /// Number of facts to populate before compaction
        #[arg(short, long, default_value = "10000")]
        count: usize,
    },
    /// Analyze storage fragmentation
    Analyze {
        /// Number of facts to analyze
        #[arg(short, long, default_value = "10000")]
        count: usize,
    },
    /// Show compaction statistics
    Stats {
        #[arg(short, long, default_value = "10000")]
        count: usize,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run { count } => {
            println!("{}", "Storage Compaction".bold());
            
            let db = KnowledgeDatabase::new()?;
            for i in 0..*count {
                db.insert(&Fact::new(
                    SubjectID((i % 1000) as u32),
                    PredicateID((i % 10) as u8),
                    ObjectID((i % 500) as u32),
                    (i as f64 % 1000.0) / 1000.0,
                )?)?;
            }
            println!("  Populated {} facts", count);
            
            let start = Instant::now();
            let compacted = db.compact()?;
            let elapsed = start.elapsed();
            
            println!("  Compacted in {:?}", elapsed);
            println!("  Facts after: {}", compacted.fact_count());
            println!("  {}", "Compaction complete".green());
            Ok(())
        }
        Commands::Analyze { count } => {
            println!("{}", "Storage Analysis".bold());
            
            let db = KnowledgeDatabase::new()?;
            for i in 0..*count {
                db.insert(&Fact::new(
                    SubjectID((i % 1000) as u32),
                    PredicateID((i % 10) as u8),
                    ObjectID((i % 500) as u32),
                    (i as f64 % 1000.0) / 1000.0,
                )?)?;
            }
            
            println!("  Total facts:      {}", db.fact_count());
            println!("  Active facts:     {}", db.active_fact_count());
            let fact_count = db.fact_count() as u64;
            let _memory_bytes = fact_count * 34;
            println!("  Estimated memory: {:.2} MB", (db.fact_count() as f64 * 34.0) / 1_048_576.0);
            println!("  {}", "Analysis complete".green());
            Ok(())
        }
        Commands::Stats { count } => {
            println!("{}", "Compaction Statistics".bold());
            let db = KnowledgeDatabase::new()?;
            for i in 0..*count {
                db.insert(&Fact::new(
                    SubjectID((i % 1000) as u32),
                    PredicateID((i % 10) as u8),
                    ObjectID((i % 500) as u32),
                    (i as f64 % 1000.0) / 1000.0,
                )?)?;
            }
            
            let start = Instant::now();
            let _compacted = db.compact()?;
            let elapsed = start.elapsed();
            
            println!("  Facts compacted: {}", count);
            println!("  Compaction time:  {:?}", elapsed);
            println!("  Throughput:       {:.0} facts/sec", *count as f64 / elapsed.as_secs_f64());
            Ok(())
        }
    }
}
