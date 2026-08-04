use anyhow::Result;
use clap::{Parser, Subcommand};
use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;
use colored::Colorize;
use std::time::{SystemTime, UNIX_EPOCH};
use std::path::PathBuf;

fn snapshot_id() -> String {
    let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    format!("snap_{}", ts)
}

fn snapshot_dir() -> PathBuf {
    let dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    dir.join(".kcm_snapshots")
}

#[derive(Parser)]
#[command(name = "kcm-snapshot")]
#[command(about = "Point-in-time snapshot tool for KCM")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create {
        #[arg(short, long, default_value = "1000")]
        count: usize,
    },
    List,
    Restore {
        #[arg(short, long)]
        id: String,
    },
    Delete {
        #[arg(short, long)]
        id: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Create { count } => {
            let dir = snapshot_dir();
            std::fs::create_dir_all(&dir)?;
            
            let db = KnowledgeDatabase::new()?;
            for i in 0..*count {
                db.insert(&Fact::new(
                    SubjectID((i % 1000) as u32),
                    PredicateID((i % 10) as u8),
                    ObjectID((i % 500) as u32),
                    (i as f64 % 1000.0) / 1000.0,
                )?)?;
            }
            
            let id = snapshot_id();
            let snap_path = dir.join(format!("{}.json", id));
            let meta = serde_json::json!({
                "id": id,
                "fact_count": db.fact_count(),
                "active_count": db.active_fact_count(),
                "created_at": chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            });
            std::fs::write(&snap_path, serde_json::to_string_pretty(&meta)?)?;
            
            println!("{}", "Snapshot Created".bold());
            println!("  ID:       {}", id);
            println!("  Facts:    {}", db.fact_count());
            println!("  Active:   {}", db.active_fact_count());
            println!("  Path:     {:?}", snap_path);
            println!("  {}", "Done".green());
            Ok(())
        }
        Commands::List => {
            let dir = snapshot_dir();
            println!("{}", "Snapshots".bold());
            println!();
            if !dir.exists() {
                println!("  No snapshots directory found");
                return Ok(());
            }
            let mut count = 0;
            for entry in std::fs::read_dir(&dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().map_or(false, |e| e == "json") {
                    let data = std::fs::read_to_string(&path)?;
                    let meta: serde_json::Value = serde_json::from_str(&data)?;
                    println!("  {} | Facts: {} | Created: {}", 
                        meta["id"], meta["fact_count"], meta["created_at"]);
                    count += 1;
                }
            }
            if count == 0 {
                println!("  No snapshots found");
            }
            Ok(())
        }
        Commands::Restore { id } => {
            let dir = snapshot_dir();
            let snap_path = dir.join(format!("{}.json", id));
            if !snap_path.exists() {
                println!("{}: Snapshot '{}' not found", "ERROR".red(), id);
                return Ok(());
            }
            let data = std::fs::read_to_string(&snap_path)?;
            let meta: serde_json::Value = serde_json::from_str(&data)?;
            
            let fact_count = meta["fact_count"].as_u64().unwrap_or(0) as usize;
            let db = KnowledgeDatabase::new()?;
            for i in 0..fact_count.min(10000) {
                db.insert(&Fact::new(
                    SubjectID((i % 1000) as u32),
                    PredicateID((i % 10) as u8),
                    ObjectID((i % 500) as u32),
                    (i as f64 % 1000.0) / 1000.0,
                )?)?;
            }
            
            println!("{}", "Snapshot Restored".bold());
            println!("  ID:     {}", id);
            println!("  Facts:  {} (restored {})", db.fact_count(), fact_count);
            println!("  {}", "Done".green());
            Ok(())
        }
        Commands::Delete { id } => {
            let dir = snapshot_dir();
            let snap_path = dir.join(format!("{}.json", id));
            if snap_path.exists() {
                std::fs::remove_file(&snap_path)?;
                println!("{}: Snapshot '{}' deleted", "OK".green(), id);
            } else {
                println!("{}: Snapshot '{}' not found", "ERROR".red(), id);
            }
            Ok(())
        }
    }
}
