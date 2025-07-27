// src/main.rs
/*
 * Main executable for UltraCode
 */

use clap::Parser;
use ultracode::{Result, run};

#[derive(Parser)]
#[command(version, about = "UltraCode - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
