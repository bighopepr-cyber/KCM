use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use kcm_distributed::sharding::{
    ConsistentHashSharding, HashSharding, RangeSharding, ShardingStrategy,
};

#[derive(Parser)]
#[command(name = "kcm-cluster")]
#[command(about = "Cluster management for KCM")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Status,
    Nodes,
    AddNode {
        addr: String,
        #[arg(long, default_value = "hash")]
        strategy: String,
    },
    RemoveNode {
        #[arg(short, long)]
        node_id: String,
    },
    Shard {
        #[arg(long, default_value = "1000")]
        facts: usize,
        #[arg(long, default_value = "3")]
        shards: usize,
        #[arg(long, default_value = "hash")]
        strategy: String,
    },
}

fn simulate_shard_distribution(
    strategy: &dyn ShardingStrategy,
    num_shards: usize,
    num_facts: usize,
) {
    let mut counts = vec![0u32; num_shards];
    for i in 0..num_facts {
        let shard = strategy.get_shard_id(i as u32, num_shards);
        counts[shard] += 1;
    }
    println!("  {:<8} {:<10} {:<10}", "Shard", "Facts", "Percent");
    println!("  {}", "-".repeat(28));
    for (i, count) in counts.iter().enumerate() {
        let pct = *count as f64 / num_facts as f64 * 100.0;
        println!("  {:<8} {:<10} {:<9.1}%", i, count, pct);
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Status => {
            println!("{}", "KCM Cluster Status".bold());
            println!();
            println!("  Mode:          {}", "single-node".yellow());
            println!("  Nodes:         1");
            println!("  Status:        {}", "healthy".green());
            println!("  Default shard: hash");
            println!();
            println!("  Sharding strategies: hash, range, consistent");
            Ok(())
        }
        Commands::Nodes => {
            println!("{}", "Cluster Nodes".bold());
            println!();
            println!("  {:<5} | {:<20} | {:<10}", "ID", "Address", "Status");
            println!("  {}", "-".repeat(42));
            println!("  {:<5} | {:<20} | {:<10}", "0", "0.0.0.0:8080", "active");
            Ok(())
        }
        Commands::AddNode { addr, strategy } => {
            println!("{}", "Adding Node".bold());
            println!("  Address:  {}", addr);
            println!("  Strategy: {}", strategy);

            let strategy: Box<dyn ShardingStrategy> = match strategy.as_str() {
                "hash" => Box::new(HashSharding),
                "range" => {
                    let boundaries = vec![0, 250, 500, 750, 1000];
                    Box::new(RangeSharding::new(boundaries))
                }
                "consistent" => Box::new(ConsistentHashSharding::new(4, 150)),
                _ => {
                    println!(
                        "  {}",
                        "Unknown strategy. Use: hash, range, consistent".red()
                    );
                    return Ok(());
                }
            };
            println!();
            simulate_shard_distribution(strategy.as_ref(), 4, 100);
            Ok(())
        }
        Commands::RemoveNode { node_id } => {
            println!("{}", "Removing Node".bold());
            println!("  Node:   {}", node_id);
            println!("  Status: {}", "Planned for distributed mode".yellow());
            Ok(())
        }
        Commands::Shard {
            facts,
            shards,
            strategy,
        } => {
            println!("{}", "Shard Distribution".bold());
            println!("  Facts:    {}", facts);
            println!("  Shards:   {}", shards);
            println!("  Strategy: {}", strategy);
            println!();

            match strategy.as_str() {
                "hash" => {
                    simulate_shard_distribution(&HashSharding, *shards, *facts);
                }
                "range" => {
                    let range_shard = *facts as u32;
                    let mut boundaries = Vec::new();
                    for i in 0..*shards {
                        boundaries.push(range_shard * i as u32 / *shards as u32);
                    }
                    let rs = RangeSharding::new(boundaries);
                    simulate_shard_distribution(&rs, *shards, *facts);
                }
                "consistent" => {
                    let cs = ConsistentHashSharding::new(*shards, 150);
                    simulate_shard_distribution(&cs, *shards, *facts);
                }
                _ => {
                    println!("  {}", "Unknown strategy".red());
                }
            }
            Ok(())
        }
    }
}
