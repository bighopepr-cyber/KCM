use anyhow::Result;
use clap::{Parser, Subcommand};
use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;

#[derive(Parser)]
#[command(name = "kcm")]
#[command(about = "KCM Knowledge Columnar Model CLI")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new database and insert sample data
    Create {
        /// Number of facts to insert
        #[arg(short, long, default_value = "100")]
        count: usize,
    },
    /// Show database statistics
    Stats {
        /// Number of facts in database
        #[arg(short, long, default_value = "100")]
        count: usize,
    },
    /// Run a benchmark
    Benchmark {
        /// Number of operations
        #[arg(short, long, default_value = "1000")]
        ops: usize,
    },
    /// Show version information
    Version,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Create { count } => {
            let db = KnowledgeDatabase::new()?;
            println!("Created new database");
            
            for i in 0..*count {
                let fact = Fact::new(
                    SubjectID((i % 100) as u32),
                    PredicateID((i % 10) as u8),
                    ObjectID((i % 200) as u32),
                    (i as f64 % 100.0) / 100.0,
                )?;
                db.insert(&fact)?;
            }
            
            println!("Inserted {} facts", count);
            println!("Total facts: {}", db.fact_count());
            Ok(())
        }
        Commands::Stats { count } => {
            let db = KnowledgeDatabase::new()?;
            for i in 0..*count {
                let fact = Fact::new(
                    SubjectID((i % 100) as u32),
                    PredicateID((i % 10) as u8),
                    ObjectID((i % 200) as u32),
                    (i as f64 % 100.0) / 100.0,
                )?;
                db.insert(&fact)?;
            }
            println!("Database Statistics:");
            println!("  Total facts: {}", db.fact_count());
            println!("  Active facts: {}", db.active_fact_count());
            Ok(())
        }
        Commands::Benchmark { ops } => {
            let db = KnowledgeDatabase::new()?;
            let start = std::time::Instant::now();
            
            for i in 0..*ops {
                let fact = Fact::new(
                    SubjectID((i % 100) as u32),
                    PredicateID((i % 10) as u8),
                    ObjectID((i % 200) as u32),
                    (i as f64 % 100.0) / 100.0,
                )?;
                db.insert(&fact)?;
            }
            
            let elapsed = start.elapsed();
            println!("Benchmark Results:");
            println!("  Operations: {}", ops);
            println!("  Elapsed: {:?}", elapsed);
            println!("  Throughput: {:.2} ops/sec", *ops as f64 / elapsed.as_secs_f64());
            Ok(())
        }
        Commands::Version => {
            println!("KCM CLI v{}", env!("CARGO_PKG_VERSION"));
            println!("KCM Knowledge Columnar Model");
            Ok(())
        }
    }
}
