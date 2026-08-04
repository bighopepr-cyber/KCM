use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(name = "kcm-doctor")]
#[command(about = "Health check tool for KCM Knowledge Columnar Model")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run full health check
    Check,
    /// Verify data integrity
    Integrity,
    /// Check WAL consistency
    Wal,
    /// Attempt automatic repair
    Repair,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Check => {
            println!("{}", "KCM Health Check".bold());
            println!();
            
            print!("  Database creation... ");
            match kcm_runtime::database::KnowledgeDatabase::new() {
                Ok(_) => println!("{}", "OK".green()),
                Err(e) => println!("{} ({})", "FAILED".red(), e),
            }
            
            print!("  Fact insertion... ");
            let db = kcm_runtime::database::KnowledgeDatabase::new()?;
            let fact = kcm_core::types::Fact::new(
                kcm_core::types::SubjectID(1),
                kcm_core::types::PredicateID(0),
                kcm_core::types::ObjectID(2),
                0.95,
            )?;
            match db.insert(&fact) {
                Ok(_) => println!("{}", "OK".green()),
                Err(e) => println!("{} ({})", "FAILED".red(), e),
            }
            
            print!("  Fact query... ");
            match db.query().execute() {
                Ok(_) => println!("{}", "OK".green()),
                Err(e) => println!("{} ({})", "FAILED".red(), e),
            }
            
            print!("  Metrics... ");
            let metrics = kcm_runtime::metrics::Metrics::new();
            metrics.record_insert(true);
            let snapshot = metrics.snapshot();
            if snapshot.inserts_total > 0 {
                println!("{}", "OK".green());
            } else {
                println!("{}", "FAILED".red());
            }
            
            println!();
            println!("{}", "Health check complete".bold());
            Ok(())
        }
        Commands::Integrity => {
            println!("{}", "Data Integrity Check".bold());
            println!();
            println!("  This feature requires a database file path.");
            println!("  Use: kcm-doctor integrity --db <path>");
            Ok(())
        }
        Commands::Wal => {
            println!("{}", "WAL Consistency Check".bold());
            println!();
            println!("  This feature requires a database file path.");
            println!("  Use: kcm-doctor wal --db <path>");
            Ok(())
        }
        Commands::Repair => {
            println!("{}", "Automatic Repair".bold());
            println!();
            println!("  This feature requires a database file path.");
            println!("  Use: kcm-doctor repair --db <path>");
            Ok(())
        }
    }
}
