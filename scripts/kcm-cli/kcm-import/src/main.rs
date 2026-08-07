use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "kcm-import")]
#[command(about = "Data import tool for KCM")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Csv {
        #[arg(short, long)]
        input: PathBuf,
        /// Subject column index (default: 0)
        #[arg(long, default_value = "0")]
        subject_col: usize,
        /// Predicate column index (default: 1)
        #[arg(long, default_value = "1")]
        predicate_col: usize,
        /// Object column index (default: 2)
        #[arg(long, default_value = "2")]
        object_col: usize,
        /// Confidence column index (default: 3)
        #[arg(long, default_value = "3")]
        confidence_col: usize,
    },
    Json {
        #[arg(short, long)]
        input: PathBuf,
    },
    Schema {
        #[arg(short, long)]
        input: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Csv {
            input,
            subject_col,
            predicate_col,
            object_col,
            confidence_col,
        } => {
            println!("{}", "CSV Import".bold());
            println!("  File: {:?}", input);

            let data = std::fs::read_to_string(input)?;
            let lines: Vec<&str> = data.lines().collect();
            if lines.is_empty() {
                println!("  {}", "Empty file".red());
                return Ok(());
            }

            // Detect header
            let (header_line, data_lines) = if lines[0].contains(',')
                && !lines[0]
                    .chars()
                    .all(|c| c.is_ascii_digit() || c == '.' || c == '-' || c == ',')
            {
                (Some(lines[0]), &lines[1..])
            } else {
                (None, &lines[..])
            };

            if let Some(h) = header_line {
                println!("  Header: {}", h);
            }

            let db = KnowledgeDatabase::new()?;
            let mut count = 0;
            let mut errors = 0;

            for line in data_lines.iter() {
                let parts: Vec<&str> = line.split(',').collect();
                let max_col = *subject_col
                    .max(predicate_col)
                    .max(object_col)
                    .max(confidence_col);
                if parts.len() <= max_col + 1 {
                    errors += 1;
                    continue;
                }

                match (
                    parts[*subject_col].trim().parse::<u32>(),
                    parts[*predicate_col].trim().parse::<u8>(),
                    parts[*object_col].trim().parse::<u32>(),
                    parts[*confidence_col].trim().parse::<f64>(),
                ) {
                    (Ok(s), Ok(p), Ok(o), Ok(c)) => {
                        if let Ok(fact) = Fact::new(SubjectID(s), PredicateID(p), ObjectID(o), c) {
                            let _ = db.insert(&fact);
                            count += 1;
                        } else {
                            errors += 1;
                        }
                    }
                    _ => {
                        errors += 1;
                    }
                }
            }

            println!("  Imported: {} facts", count);
            if errors > 0 {
                println!("  Errors:   {} rows skipped", errors);
            }
            println!(
                "  Database: {} total, {} active",
                db.fact_count(),
                db.active_fact_count()
            );
            println!("  {}", "Import complete".green());
            Ok(())
        }
        Commands::Json { input } => {
            println!("{}", "JSON Import".bold());
            println!("  File: {:?}", input);

            let data = std::fs::read_to_string(input)?;
            let json: serde_json::Value = serde_json::from_str(&data)?;

            let db = KnowledgeDatabase::new()?;
            let mut count = 0;

            if let Some(facts) = json.as_array() {
                for fact in facts {
                    let get_u64 = |key: &str| {
                        fact.get(key)
                            .and_then(|v| v.as_u64())
                            .or_else(|| match key {
                                "subject" => fact.get("s").and_then(|v| v.as_u64()),
                                "predicate" => fact.get("p").and_then(|v| v.as_u64()),
                                "object" => fact.get("o").and_then(|v| v.as_u64()),
                                _ => None,
                            })
                    };

                    let get_f64 = |key: &str, fallback: f64| {
                        fact.get(key)
                            .and_then(|v| v.as_f64())
                            .or_else(|| {
                                if key == "confidence" {
                                    fact.get("c").and_then(|v| v.as_f64())
                                } else {
                                    None
                                }
                            })
                            .unwrap_or(fallback)
                    };

                    let s = get_u64("subject").unwrap_or(0) as u32;
                    let p = get_u64("predicate").unwrap_or(0) as u8;
                    let o = get_u64("object").unwrap_or(0) as u32;
                    let c = get_f64("confidence", 0.5);

                    if s == 0 && p == 0 && o == 0 {
                        log::warn!(
                            "Skipping JSON fact with missing subject/predicate/object: {}",
                            fact
                        );
                        continue;
                    }

                    if let Ok(f) = Fact::new(SubjectID(s), PredicateID(p), ObjectID(o), c) {
                        let _ = db.insert(&f);
                        count += 1;
                    } else {
                        log::warn!("Invalid fact skipped: {}", fact);
                    }
                }
            }

            println!("  Imported: {} facts", count);
            println!("  Database: {} total", db.fact_count());
            println!("  {}", "Import complete".green());
            Ok(())
        }
        Commands::Schema { input } => {
            println!("{}", "Schema Inference".bold());
            println!("  File: {:?}", input);
            let data = std::fs::read_to_string(input)?;
            let lines: Vec<&str> = data.lines().collect();
            println!("  Lines: {}", lines.len());
            if let Some(header) = lines.first() {
                let cols: Vec<&str> = header.split(',').collect();
                println!("  Columns: {}", cols.len());
                for (i, col) in cols.iter().enumerate() {
                    println!("    {}: {}", i, col.trim());
                }
            }
            Ok(())
        }
    }
}
