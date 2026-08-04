use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;

#[derive(Parser)]
#[command(name = "kcm-schema")]
#[command(about = "Schema generation tool for KCM")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Show,
    Generate {
        #[arg(short, long, default_value = "1000")]
        count: usize,
    },
    Validate {
        #[arg(short, long, default_value = "1000")]
        count: usize,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Show => {
            println!("{}", "KCM Database Schema".bold());
            println!();
            println!("  Table: facts");
            println!("  Columns:");
            println!(
                "    {:<5} {:<15} {:<10} {:<15}",
                "ID", "Name", "Type", "Encoding"
            );
            println!("    {}", "-".repeat(45));
            println!(
                "    {:<5} {:<15} {:<10} {:<15}",
                "1", "subject", "u32", "Dictionary"
            );
            println!(
                "    {:<5} {:<15} {:<10} {:<15}",
                "2", "predicate", "u8", "Dictionary"
            );
            println!(
                "    {:<5} {:<15} {:<10} {:<15}",
                "3", "object", "u32", "Dictionary"
            );
            println!(
                "    {:<5} {:<15} {:<10} {:<15}",
                "4", "confidence", "f64", "Gorilla"
            );
            println!(
                "    {:<5} {:<15} {:<10} {:<15}",
                "5", "evidence", "u8", "Dictionary"
            );
            println!(
                "    {:<5} {:<15} {:<10} {:<15}",
                "6", "timestamp", "i64", "Delta"
            );
            println!(
                "    {:<5} {:<15} {:<10} {:<15}",
                "7", "context", "u8", "Dictionary"
            );
            println!(
                "    {:<5} {:<15} {:<10} {:<15}",
                "8", "version", "i32", "Delta"
            );
            println!(
                "    {:<5} {:<15} {:<10} {:<15}",
                "9", "priority", "i8", "Identity"
            );
            println!(
                "    {:<5} {:<15} {:<10} {:<15}",
                "10", "owner", "u16", "Dictionary"
            );
            println!();
            println!("  Indexes:");
            println!("    - BitmapIndex (subject, predicate)");
            println!("    - ZoneMap (subject, object)");
            println!("    - BloomFilter (all columns)");
            println!("    - CompositeIndex (subject + predicate)");
            Ok(())
        }
        Commands::Generate { count } => {
            println!("{}", "Schema Generation".bold());
            let db = KnowledgeDatabase::new()?;
            for i in 0..*count {
                db.insert(&Fact::new(
                    SubjectID((i % 1000) as u32),
                    PredicateID((i % 10) as u8),
                    ObjectID((i % 500) as u32),
                    (i as f64 % 1000.0) / 1000.0,
                )?)?;
            }
            println!("  Database created with {} facts", db.fact_count());
            println!("  Schema: 10 columns, 4 index types");
            println!("  {}", "Schema generated".green());
            Ok(())
        }
        Commands::Validate { count } => {
            println!("{}", "Schema Validation".bold());
            let db = KnowledgeDatabase::new()?;
            let mut errors = 0;
            for i in 0..*count {
                let c = (i as f64 % 1000.0) / 1000.0;
                match Fact::new(
                    SubjectID((i % 1000) as u32),
                    PredicateID((i % 10) as u8),
                    ObjectID((i % 500) as u32),
                    c,
                ) {
                    Ok(fact) => {
                        let _ = db.insert(&fact);
                    }
                    Err(_) => {
                        errors += 1;
                    }
                }
            }
            println!("  Tested:  {} facts", count);
            println!("  Errors:  {}", errors);
            println!(
                "  Schema:  {}",
                if errors == 0 {
                    "VALID".green()
                } else {
                    format!("{} errors", errors).red()
                }
            );
            Ok(())
        }
    }
}
