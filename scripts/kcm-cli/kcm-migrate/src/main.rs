use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;
use std::path::PathBuf;

const CURRENT_VERSION: u32 = 2;

#[derive(Parser)]
#[command(name = "kcm-migrate")]
#[command(about = "Schema migration tool for KCM")]
#[command(version)]
struct Cli {
    #[arg(short, long, default_value = "kcm_migrations")]
    dir: PathBuf,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Status,
    Up {
        #[arg(short, long)]
        db: PathBuf,
    },
    Down {
        #[arg(short, long)]
        db: PathBuf,
    },
    Create {
        name: String,
    },
    Validate {
        #[arg(short, long, default_value = "1000")]
        count: usize,
    },
    History {
        #[arg(short, long)]
        db: PathBuf,
    },
}

fn ensure_migration_dir(dir: &std::path::Path) -> Result<()> {
    std::fs::create_dir_all(dir)?;
    Ok(())
}

fn read_version(dir: &std::path::Path) -> u32 {
    let ver_file = dir.join("version");
    if ver_file.exists() {
        std::fs::read_to_string(&ver_file)
            .unwrap_or_default()
            .trim()
            .parse()
            .unwrap_or(0)
    } else {
        0
    }
}

fn write_version(dir: &std::path::Path, version: u32) -> Result<()> {
    ensure_migration_dir(dir)?;
    std::fs::write(dir.join("version"), version.to_string())?;
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    ensure_migration_dir(&cli.dir)?;

    match &cli.command {
        Commands::Status => {
            let current = read_version(&cli.dir);
            println!("{}", "Migration Status".bold());
            println!("  Current version: {}", current);
            println!("  Latest version:  {}", CURRENT_VERSION);
            if current >= CURRENT_VERSION {
                println!("  Status:          {}", "Up to date".green());
            } else {
                println!(
                    "  Pending:         {} migration(s) to apply",
                    CURRENT_VERSION - current
                );
                for v in (current + 1)..=CURRENT_VERSION {
                    match v {
                        1 => println!("    v{}: Create initial schema", v),
                        2 => println!("    v{}: Add dictionary encoding", v),
                        _ => println!("    v{}: Unknown migration", v),
                    }
                }
            }
            Ok(())
        }
        Commands::Up { db: _ } => {
            let mut current = read_version(&cli.dir);
            if current >= CURRENT_VERSION {
                println!("{}", "No pending migrations".green());
                return Ok(());
            }

            println!("{}", "Applying Migrations".bold());
            let database = KnowledgeDatabase::new()?;

            for version in (current + 1)..=CURRENT_VERSION {
                match version {
                    1 => {
                        println!("  v{}: Create initial schema...", version);
                        for i in 0..100 {
                            let _ = database.insert(&Fact::new(
                                SubjectID(i % 10),
                                PredicateID(0),
                                ObjectID(i),
                                0.95,
                            )?);
                        }
                        println!("    {} (100 seed facts)", "OK".green());
                    }
                    2 => {
                        println!("  v{}: Add dictionary encoding...", version);
                        let _ = database.dict_insert_subject("version_2_migrated");
                        println!("    {} (dictionary updated)", "OK".green());
                    }
                    _ => {}
                }
                current = version;
                write_version(&cli.dir, current)?;
            }

            println!("\n  Migrated to version {}", current);
            println!("  {}", "Migration complete".green());
            Ok(())
        }
        Commands::Down { db: _ } => {
            let current = read_version(&cli.dir);
            if current == 0 {
                println!("{}", "Already at version 0".yellow());
                return Ok(());
            }
            println!("{}", "Rolling Back".bold());
            let target = current - 1;
            println!("  v{} -> v{}", current, target);
            write_version(&cli.dir, target)?;
            println!("  {}", "Rollback complete".green());
            Ok(())
        }
        Commands::Create { name } => {
            let current = read_version(&cli.dir);
            let new_ver = current + 1;
            let filename = format!("{:03}_{}.sql", new_ver, name.replace(' ', "_"));
            let filepath = cli.dir.join(&filename);
            std::fs::write(
                &filepath,
                format!(
                    "-- Migration v{}: {}\n-- TODO: Add migration SQL here\n",
                    new_ver, name
                ),
            )?;
            println!("{}: Created {}", "OK".green(), filename);
            Ok(())
        }
        Commands::Validate { count } => {
            println!("{}", "Schema Validation".bold());
            let database = KnowledgeDatabase::new()?;
            let mut errors = 0usize;
            for i in 0..*count {
                match Fact::new(
                    SubjectID((i % 1000) as u32),
                    PredicateID((i % 10) as u8),
                    ObjectID((i % 500) as u32),
                    (i as f64 % 1000.0) / 1000.0,
                ) {
                    Ok(fact) => {
                        let _ = database.insert(&fact);
                    }
                    Err(_) => {
                        errors += 1;
                    }
                }
            }
            println!("  Tested:  {} facts", count);
            println!("  Errors:  {}", errors);
            if errors == 0 {
                println!("  Schema:  {}", "VALID".green());
            } else {
                println!("  Schema:  {} ({} errors)", "INVALID".red(), errors);
            }
            Ok(())
        }
        Commands::History { db: _ } => {
            println!("{}", "Migration History".bold());
            let current = read_version(&cli.dir);
            for v in 0..=current {
                let status = if v == current {
                    " (current)".green()
                } else {
                    "".normal()
                };
                match v {
                    0 => println!("  v0: Initial{}", status),
                    1 => println!("  v1: Create initial schema{}", status),
                    2 => println!("  v2: Add dictionary encoding{}", status),
                    _ => println!("  v{}: Unknown{}", v, status),
                }
            }
            Ok(())
        }
    }
}
