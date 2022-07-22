use clap::Parser;

use crate::subcommands::Commands;

/// Simple program to greet a person
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[clap(subcommand)]
    pub commands: Commands,
}
