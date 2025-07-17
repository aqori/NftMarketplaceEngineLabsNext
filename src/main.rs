// src/main.rs
/*
 * Main executable for NftMarketplaceEngineLabsNext
 */

use clap::Parser;
use nftmarketplaceenginelabsnext::{Result, run};

#[derive(Parser)]
#[command(version, about = "NftMarketplaceEngineLabsNext - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
