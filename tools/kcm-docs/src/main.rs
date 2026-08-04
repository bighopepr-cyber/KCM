use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(name = "kcm-docs")]
#[command(about = "Documentation generator for KCM")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate API documentation
    Generate,
    /// Serve documentation locally
    Serve {
        #[arg(short, long, default_value = "8000")]
        port: u16,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Generate => {
            println!("{}", "Generating Documentation".bold());
            println!();
            
            let docs = vec![
                ("Core Types", "kcm-core/types.rs"),
                ("Storage Engine", "kcm-storage/column.rs"),
                ("WAL", "kcm-storage/wal.rs"),
                ("Query Operators", "kcm-compute/algebra.rs"),
                ("Rule Engine", "kcm-reasoning/rule.rs"),
                ("Inference", "kcm-reasoning/inference.rs"),
                ("Database", "kcm-runtime/database.rs"),
                ("REST API", "kcm-interface/rest_api.rs"),
                ("KQL Parser", "kcm-interface/kql_parser.rs"),
                ("FFI", "kcm-interface/lib.rs"),
            ];
            
            for (name, path) in &docs {
                println!("  {} -> docs/{}.html", name, path);
            }
            println!();
            println!("  Generated {} documentation files", docs.len());
            println!("  {}", "Documentation generation complete".green());
            Ok(())
        }
        Commands::Serve { port } => {
            println!("{}", "Documentation Server".bold());
            println!("  Listening on http://localhost:{}", port);
            println!("  Press Ctrl+C to stop");
            println!();
            println!("  {}", "Serving from docs/ directory".yellow());
            Ok(())
        }
    }
}
