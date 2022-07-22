use clap::Parser;

use crate::subcommands::Commands;

/// Simple todo CLI
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Subcommands
    #[clap(subcommand)]
    pub commands: Commands,
}
