//! KCM Examples
//! 
//! Run individual examples with:
//! cargo run --example basic_usage
//! cargo run --example transactions
//! cargo run --example reasoning

mod basic_usage;
mod transactions;
mod reasoning;

fn main() {
    println!("KCM Examples");
    println!("============");
    println!();
    println!("Available examples:");
    println!("  basic_usage   - Create database, insert facts, query");
    println!("  transactions  - Transaction management");
    println!("  reasoning     - Define rules and run inference");
    println!();
    println!("Run with: cargo run --example <name>");
}
