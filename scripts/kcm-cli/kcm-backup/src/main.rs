use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;
use kcm_storage::backup::BackupManager;
use kcm_storage::file_format::DatabaseFile;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "kcm-backup")]
#[command(about = "Backup tool for KCM Knowledge Columnar Model")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create {
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(short, long, default_value = "1000")]
        count: usize,
        #[arg(short, long, default_value = ".")]
        backup_dir: PathBuf,
    },
    Verify {
        #[arg(short, long)]
        path: PathBuf,
    },
    List {
        #[arg(short, long, default_value = ".")]
        dir: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Create {
            output,
            count,
            backup_dir,
        } => {
            let db =
                KnowledgeDatabase::new().context("Failed to create KnowledgeDatabase instance")?;

            for i in 0..*count {
                let subject = SubjectID((i % 1000) as u32);
                let predicate = PredicateID((i % 10) as u8);
                let object = ObjectID((i % 500) as u32);
                let confidence = (i as f64 % 1000.0) / 1000.0;

                let fact = Fact::new(subject, predicate, object, confidence)
                    .context("Failed to create fact")?;
                db.insert(&fact)
                    .context("Failed to insert fact into database")?;
            }

            let fact_count = db.fact_count();
            let active_count = db.active_fact_count();

            let manager =
                BackupManager::new(backup_dir).context("Failed to initialize BackupManager")?;

            let backup_path = {
                let schema = db.get_schema();
                manager
                    .create_full_backup(&schema)
                    .context("Failed to create full backup")?
            };

            let final_path = if let Some(requested) = output {
                std::fs::copy(&backup_path, requested)
                    .context("Failed to copy backup to output path")?;
                requested.clone()
            } else {
                backup_path
            };

            let metadata =
                std::fs::metadata(&final_path).context("Failed to read backup file metadata")?;

            println!("Backup created successfully:");
            println!("  Source facts:   {}", fact_count);
            println!("  Active facts:   {}", active_count);
            println!("  Backup path:    {}", final_path.display());
            println!("  File size:      {} bytes", metadata.len());
            Ok(())
        }
        Commands::Verify { path } => {
            if !path.exists() {
                anyhow::bail!("Backup file does not exist: {}", path.display());
            }

            let valid = DatabaseFile::verify(path)
                .with_context(|| format!("Failed to verify backup: {}", path.display()))?;

            let metadata = std::fs::metadata(path)?;

            println!("Backup verification:");
            println!("  Path:           {}", path.display());
            println!("  File size:      {} bytes", metadata.len());
            println!("  Integrity:      {}", if valid { "PASS" } else { "FAIL" });

            if !valid {
                anyhow::bail!("Backup integrity verification failed");
            }

            let schema =
                DatabaseFile::load(path).context("Failed to load backup for inspection")?;

            println!("  Fact count:     {}", schema.len());
            println!("  Active facts:   {}", schema.active_count());
            Ok(())
        }
        Commands::List { dir } => {
            let manager = BackupManager::new(dir).context("Failed to initialize BackupManager")?;
            let backups = manager.list_backups().context("Failed to list backups")?;

            if backups.is_empty() {
                println!("No backups found");
                return Ok(());
            }

            println!("Available backups ({}):", backups.len());
            for path in &backups {
                let metadata = std::fs::metadata(path)
                    .with_context(|| format!("Failed to read metadata for {}", path.display()))?;
                let filename = path
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_else(|| path.display().to_string());
                println!("  {} - {} bytes", filename, metadata.len());
            }
            Ok(())
        }
    }
}
