use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser)]
#[command(name = "kcm-perf")]
#[command(about = "Performance analyzer for KCM")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run performance analysis
    Analyze,
    /// Save baseline measurements
    Baseline {
        /// Output file for baseline data
        #[arg(long, default_value = "kcm_baseline.json")]
        output: PathBuf,
    },
    /// Compare against baseline
    Compare {
        /// Baseline file to compare against
        #[arg(long, default_value = "kcm_baseline.json")]
        baseline: PathBuf,
    },
}

struct BenchmarkResult {
    insert_ops_per_sec: f64,
    insert_latency_us: f64,
    query_ops_per_sec: f64,
    query_latency_us: f64,
    filtered_ops_per_sec: f64,
    filtered_latency_us: f64,
}

fn run_benchmark() -> Result<BenchmarkResult> {
    let db = KnowledgeDatabase::new()?;

    let start = Instant::now();
    for i in 0..10000 {
        db.insert(&Fact::new(
            SubjectID((i % 1000) as u32),
            PredicateID((i % 10) as u8),
            ObjectID((i % 500) as u32),
            (i as f64 % 1000.0) / 1000.0,
        )?)?;
    }
    let insert_elapsed = start.elapsed();

    let start = Instant::now();
    for _ in 0..10000 {
        let _ = db.query().execute()?;
    }
    let query_elapsed = start.elapsed();

    let start = Instant::now();
    for _ in 0..10000 {
        let _ = db.query().with_subject(SubjectID(1)).execute()?;
    }
    let filtered_elapsed = start.elapsed();

    Ok(BenchmarkResult {
        insert_ops_per_sec: 10000.0 / insert_elapsed.as_secs_f64(),
        insert_latency_us: insert_elapsed.as_micros() as f64 / 10000.0,
        query_ops_per_sec: 10000.0 / query_elapsed.as_secs_f64(),
        query_latency_us: query_elapsed.as_micros() as f64 / 10000.0,
        filtered_ops_per_sec: 10000.0 / filtered_elapsed.as_secs_f64(),
        filtered_latency_us: filtered_elapsed.as_micros() as f64 / 10000.0,
    })
}

fn save_baseline(result: &BenchmarkResult, path: &PathBuf) -> Result<()> {
    let data = serde_json::json!({
        "timestamp": std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
        "insert_ops_per_sec": result.insert_ops_per_sec,
        "insert_latency_us": result.insert_latency_us,
        "query_ops_per_sec": result.query_ops_per_sec,
        "query_latency_us": result.query_latency_us,
        "filtered_ops_per_sec": result.filtered_ops_per_sec,
        "filtered_latency_us": result.filtered_latency_us,
    });
    std::fs::write(path, serde_json::to_string_pretty(&data)?)?;
    Ok(())
}

fn load_baseline(path: &PathBuf) -> Result<BenchmarkResult> {
    let content = std::fs::read_to_string(path)?;
    let data: serde_json::Value = serde_json::from_str(&content)?;
    Ok(BenchmarkResult {
        insert_ops_per_sec: data["insert_ops_per_sec"].as_f64().unwrap_or(0.0),
        insert_latency_us: data["insert_latency_us"].as_f64().unwrap_or(0.0),
        query_ops_per_sec: data["query_ops_per_sec"].as_f64().unwrap_or(0.0),
        query_latency_us: data["query_latency_us"].as_f64().unwrap_or(0.0),
        filtered_ops_per_sec: data["filtered_ops_per_sec"].as_f64().unwrap_or(0.0),
        filtered_latency_us: data["filtered_latency_us"].as_f64().unwrap_or(0.0),
    })
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Analyze => {
            println!("{}", "Performance Analysis".bold());
            println!();

            let result = run_benchmark()?;

            println!("  Operation           | Throughput      | Latency");
            println!("  --------------------|-----------------|----------");
            println!(
                "  Insert (10K)        | {:.0} ops/sec    | {:.2} us",
                result.insert_ops_per_sec, result.insert_latency_us
            );
            println!(
                "  Query (10K)         | {:.0} ops/sec    | {:.2} us",
                result.query_ops_per_sec, result.query_latency_us
            );
            println!(
                "  Filtered (10K)      | {:.0} ops/sec    | {:.2} us",
                result.filtered_ops_per_sec, result.filtered_latency_us
            );
            Ok(())
        }
        Commands::Baseline { output } => {
            println!("{}", "Saving Baseline".bold());
            let result = run_benchmark()?;
            save_baseline(&result, output)?;
            println!(
                "  Status: {} to {}",
                "Baseline measurement saved".green(),
                output.display()
            );
            Ok(())
        }
        Commands::Compare { baseline } => {
            println!("{}", "Comparing Against Baseline".bold());
            if !baseline.exists() {
                println!(
                    "  Status: {}",
                    "No baseline found - run 'kcm-perf baseline' first".yellow()
                );
                return Ok(());
            }

            let base = load_baseline(baseline)?;
            let current = run_benchmark()?;

            println!("  Baseline: {:?}", baseline);
            println!();
            println!("  Operation           | Baseline      | Current       | Delta");
            println!("  --------------------|---------------|---------------|--------");

            let insert_delta = (current.insert_ops_per_sec - base.insert_ops_per_sec)
                / base.insert_ops_per_sec
                * 100.0;
            let query_delta = (current.query_ops_per_sec - base.query_ops_per_sec)
                / base.query_ops_per_sec
                * 100.0;
            let filtered_delta = (current.filtered_ops_per_sec - base.filtered_ops_per_sec)
                / base.filtered_ops_per_sec
                * 100.0;

            println!(
                "  Insert              | {:>10.0}/s | {:>10.0}/s | {:+.1}%",
                base.insert_ops_per_sec, current.insert_ops_per_sec, insert_delta
            );
            println!(
                "  Query               | {:>10.0}/s | {:>10.0}/s | {:+.1}%",
                base.query_ops_per_sec, current.query_ops_per_sec, query_delta
            );
            println!(
                "  Filtered            | {:>10.0}/s | {:>10.0}/s | {:+.1}%",
                base.filtered_ops_per_sec, current.filtered_ops_per_sec, filtered_delta
            );

            if insert_delta < -5.0 || query_delta < -5.0 || filtered_delta < -5.0 {
                println!();
                println!(
                    "  {}",
                    "WARNING: Performance regression detected (>5% decrease)".red()
                );
            } else {
                println!();
                println!("  {}", "Performance within acceptable range".green());
            }

            Ok(())
        }
    }
}
