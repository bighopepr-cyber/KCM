use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;

#[derive(Parser)]
#[command(name = "kcm-inspect")]
#[command(about = "Database inspection tool for KCM")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Schema,
    Columns,
    Stats {
        #[arg(short, long, default_value = "1000")]
        count: usize,
    },
    Dictionary,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Schema => {
            println!("{}", "Database Schema".bold());
            let db = KnowledgeDatabase::new()?;
            for i in 0..10 {
                let _ = db.insert(&Fact::new(SubjectID(i), PredicateID(0), ObjectID(i), 0.5)?);
            }
            println!("  Rows:      {}", db.fact_count());
            println!(
                "  Columns:   10 (Subject, Predicate, Object, Confidence, Evidence, Timestamp, Context, Version, Priority, Owner)"
            );
            println!("  Fact size: 34 bytes");
            Ok(())
        }
        Commands::Columns => {
            println!("{}", "Column Metadata".bold());
            println!("  {:<15} {:<10} {:<15}", "Column", "Type", "Encoding");
            println!("  {}", "-".repeat(42));
            for (col, ty, enc) in [
                ("Subject", "u32", "Dictionary"),
                ("Predicate", "u8", "Dictionary"),
                ("Object", "u32", "Dictionary"),
                ("Confidence", "f64", "Gorilla"),
                ("Evidence", "u8", "Dictionary"),
                ("Timestamp", "i64", "Delta"),
                ("Context", "u8", "Dictionary"),
                ("Version", "i32", "Delta"),
                ("Priority", "i8", "Identity"),
                ("Owner", "u16", "Dictionary"),
            ] {
                println!("  {:<15} {:<10} {:<15}", col, ty, enc);
            }
            Ok(())
        }
        Commands::Stats { count } => {
            println!("{}", "Database Statistics".bold());
            let db = KnowledgeDatabase::new()?;
            for i in 0..*count {
                db.insert(&Fact::new(
                    SubjectID((i % 1000) as u32),
                    PredicateID((i % 10) as u8),
                    ObjectID((i % 500) as u32),
                    (i as f64 % 1000.0) / 1000.0,
                )?)?;
            }
            println!("  Total facts:   {}", db.fact_count());
            println!("  Active facts:  {}", db.active_fact_count());
            println!(
                "  Memory est:    {:.2} MB",
                db.fact_count() as f64 * 34.0 / 1_048_576.0
            );
            Ok(())
        }
        Commands::Dictionary => {
            println!("{}", "Dictionary Contents".bold());
            let db = KnowledgeDatabase::new()?;
            let id1 = db.dict_insert_subject("planet")?;
            let id2 = db.dict_insert_subject("star")?;
            println!("  Subject 0 -> {:?}", db.dict_get_subject(id1));
            println!("  Subject 1 -> {:?}", db.dict_get_subject(id2));
            Ok(())
        }
    }
}
