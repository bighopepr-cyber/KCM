use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use kcm_runtime::database::KnowledgeDatabase;
use kcm_storage::backup::BackupManager;
use kcm_storage::file_format::DatabaseFile;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "kcm-restore")]
#[command(about = "Restore tool for KCM Knowledge Columnar Model")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    From {
        #[arg(short, long)]
        backup: PathBuf,
    },
    List {
        #[arg(short, long, default_value = ".")]
        dir: PathBuf,
    },
    Verify {
        #[arg(short, long)]
        path: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::From { backup } => {
            if !backup.exists() {
                anyhow::bail!("Backup file does not exist: {}", backup.display());
            }

            println!("Restoring from backup: {}", backup.display());

            let valid = DatabaseFile::verify(backup)
                .with_context(|| format!("Failed to verify backup integrity: {}", backup.display()))?;

            if !valid {
                anyhow::bail!("Backup integrity check failed; file may be corrupted");
            }

            let schema = DatabaseFile::load(backup)
                .with_context(|| format!("Failed to load backup: {}", backup.display()))?;

            let total_facts = schema.len();

            let db = KnowledgeDatabase::new()
                .context("Failed to create KnowledgeDatabase for restore")?;

            for i in 0..total_facts {
                if let Some(fact) = schema.get_fact(i) {
                    db.insert(&fact)
                        .with_context(|| format!("Failed to insert restored fact {}", i))?;
                }
            }

            println!("Restore completed successfully:");
            println!("  Facts loaded:   {}", db.fact_count());
            println!("  Active facts:   {}", db.active_fact_count());
            Ok(())
        }
        Commands::List { dir } => {
            let manager =
                BackupManager::new(dir).context("Failed to initialize BackupManager")?;
            let backups = manager
                .list_backups()
                .context("Failed to list restore points")?;

            if backups.is_empty() {
                println!("No restore points found");
                return Ok(());
            }

            println!("Available restore points ({}):", backups.len());
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
        Commands::Verify { path } => {
            if !path.exists() {
                anyhow::bail!("Restore point does not exist: {}", path.display());
            }

            let valid = DatabaseFile::verify(path)
                .with_context(|| format!("Failed to verify restore point: {}", path.display()))?;

            let metadata = std::fs::metadata(path)?;

            println!("Restore point verification:");
            println!("  Path:           {}", path.display());
            println!("  File size:      {} bytes", metadata.len());
            println!(
                "  Integrity:      {}",
                if valid { "PASS" } else { "FAIL" }
            );

            if !valid {
                anyhow::bail!("Restore point integrity verification failed");
            }

            let schema = DatabaseFile::load(path)
                .with_context(|| format!("Failed to load restore point: {}", path.display()))?;

            println!("  Fact count:     {}", schema.len());
            println!("  Active facts:   {}", schema.active_count());
            Ok(())
        }
    }
}
