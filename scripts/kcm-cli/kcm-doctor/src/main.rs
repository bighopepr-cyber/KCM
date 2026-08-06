use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use std::path::PathBuf;

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
    Integrity {
        /// Path to the database file
        #[arg(long = "db")]
        db_path: PathBuf,
    },
    /// Check WAL consistency
    Wal {
        /// Path to the WAL file
        #[arg(long = "wal")]
        wal_path: PathBuf,
    },
    /// Attempt automatic repair
    Repair {
        /// Path to the database file
        #[arg(long = "db")]
        db_path: PathBuf,
    },
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
        Commands::Integrity { db_path } => {
            println!("{}", "Data Integrity Check".bold());
            println!();
            println!("  Checking: {}", db_path.display());

            if !db_path.exists() {
                println!("  {} File not found: {}", "FAILED".red(), db_path.display());
                return Ok(());
            }

            print!("  BLAKE3 checksum verification... ");
            match kcm_storage::file_format::DatabaseFile::verify(db_path) {
                Ok(true) => println!("{}", "OK".green()),
                Ok(false) => println!("{}", "CORRUPTED".red()),
                Err(e) => println!("{} ({})", "FAILED".red(), e),
            }

            print!("  Schema load... ");
            match kcm_storage::file_format::DatabaseFile::load(db_path) {
                Ok(schema) => {
                    println!(
                        "{} ({} rows, {} active)",
                        "OK".green(),
                        schema.len(),
                        schema.active_count()
                    );
                }
                Err(e) => println!("{} ({})", "FAILED".red(), e),
            }

            println!();
            println!("{}", "Integrity check complete".bold());
            Ok(())
        }
        Commands::Wal { wal_path } => {
            println!("{}", "WAL Consistency Check".bold());
            println!();
            println!("  Checking: {}", wal_path.display());

            if !wal_path.exists() {
                println!(
                    "  {} File not found: {}",
                    "FAILED".red(),
                    wal_path.display()
                );
                return Ok(());
            }

            print!("  WAL integrity verification... ");
            let wal = kcm_storage::wal::WriteAheadLog::new(wal_path)?;
            match wal.verify_integrity() {
                Ok(()) => println!("{}", "OK".green()),
                Err(e) => println!("{} ({})", "FAILED".red(), e),
            }

            println!();
            println!("{}", "WAL check complete".bold());
            Ok(())
        }
        Commands::Repair { db_path } => {
            println!("{}", "Automatic Repair".bold());
            println!();
            println!("  Database: {}", db_path.display());

            if !db_path.exists() {
                println!(
                    "  {} Database file not found: {}",
                    "FAILED".red(),
                    db_path.display()
                );
                return Ok(());
            }

            print!("  Loading schema... ");
            let schema = match kcm_storage::file_format::DatabaseFile::load(db_path) {
                Ok(s) => {
                    println!("{} ({} rows)", "OK".green(), s.len());
                    s
                }
                Err(e) => {
                    println!("{} ({})", "FAILED".red(), e);
                    println!();
                    println!("  {} Cannot repair: schema load failed", "FAILED".red());
                    return Ok(());
                }
            };

            print!("  Compacting schema... ");
            match schema.compact() {
                Ok(compacted) => {
                    println!(
                        "{} ({} → {} rows)",
                        "OK".green(),
                        schema.len(),
                        compacted.len()
                    );
                    print!("  Saving repaired schema... ");
                    match kcm_storage::file_format::DatabaseFile::save(&compacted, db_path) {
                        Ok(()) => println!("{}", "OK".green()),
                        Err(e) => println!("{} ({})", "FAILED".red(), e),
                    }
                }
                Err(e) => println!("{} ({})", "FAILED".red(), e),
            }

            println!();
            println!("{}", "Repair complete".bold());
            Ok(())
        }
    }
}
