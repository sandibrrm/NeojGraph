// src/main.rs
/*
 * Main executable for Neo4jGraph
 */

use clap::Parser;
use neo4jgraph::{Result, run};

#[derive(Parser)]
#[command(version, about = "Neo4jGraph - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
    
    /// Input file path
    #[arg(short, long)]
    input: Option<String>,
    
    /// Output file path
    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose, args.input, args.output)
}
