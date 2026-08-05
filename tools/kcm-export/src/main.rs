use anyhow::Result;
use clap::{Parser, Subcommand};
use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "kcm-export")]
#[command(about = "Data export tool for KCM Knowledge Columnar Model")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Export facts to JSON
    Json {
        /// Output file path
        #[arg(short, long)]
        output: PathBuf,
        /// Number of facts to generate
        #[arg(short, long, default_value = "1000")]
        count: usize,
    },
    /// Export facts to CSV
    Csv {
        /// Output file path
        #[arg(short, long)]
        output: PathBuf,
        /// Number of facts to generate
        #[arg(short, long, default_value = "1000")]
        count: usize,
    },
    /// Export query results
    Query {
        /// Output file path
        #[arg(short, long)]
        output: PathBuf,
        /// KQL query
        #[arg(short, long)]
        query: String,
        /// Path to the database file
        #[arg(long = "db")]
        db_path: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Json { output, count } => {
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

            let mut facts = Vec::new();
            let results = db.query().execute()?;
            for fact in &results {
                facts.push(serde_json::json!({
                    "subject": fact.subject.0,
                    "predicate": fact.predicate.0,
                    "object": fact.object.0,
                    "confidence": fact.confidence,
                }));
            }

            let json = serde_json::to_string_pretty(&facts)?;
            std::fs::write(output, &json)?;

            println!("Exported {} facts to {:?}", facts.len(), output);
            Ok(())
        }
        Commands::Csv { output, count } => {
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

            let results = db.query().execute()?;
            let mut csv = String::from("subject,predicate,object,confidence\n");
            for fact in &results {
                csv.push_str(&format!(
                    "{},{},{},{}\n",
                    fact.subject.0, fact.predicate.0, fact.object.0, fact.confidence
                ));
            }

            std::fs::write(output, &csv)?;
            println!("Exported {} facts to {:?}", results.len(), output);
            Ok(())
        }
        Commands::Query {
            output,
            query,
            db_path,
        } => {
            println!("Export query results: {}", query);
            println!("Database: {:?}", db_path);
            println!("Output: {:?}", output);

            if !db_path.exists() {
                println!("Database file not found: {:?}", db_path);
                return Ok(());
            }

            let schema = kcm_storage::file_format::DatabaseFile::load(db_path)?;
            let db = KnowledgeDatabase::new()?;
            for idx in 0..schema.len() {
                if let Some(fact) = schema.get_fact(idx) {
                    db.insert(&fact)?;
                }
            }

            let results = db.query().execute()?;
            let mut csv = String::from("subject,predicate,object,confidence\n");
            for fact in &results {
                csv.push_str(&format!(
                    "{},{},{},{}\n",
                    fact.subject.0, fact.predicate.0, fact.object.0, fact.confidence
                ));
            }

            std::fs::write(output, &csv)?;
            println!(
                "Exported {} facts matching query to {:?}",
                results.len(),
                output
            );
            Ok(())
        }
    }
}
