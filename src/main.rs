mod cli;
mod subcommands;

use crate::subcommands::Commands;
use clap::Parser;

fn main() {
    let args = cli::Args::parse();

    match args.commands {
        Commands::Add(command) => {
            println!("running the Add subcommand");
        }

        Commands::Delete(command) => {
            println!("running the Delete subcommand")
        }

        Commands::Update(command) => {
            println!("running the Update subcommand")
        }

        Commands::Complete(command) => {
            println!("running the Complete subcommand")
        }

        Commands::List(command) => {
            println!("running the List subcommand")
        }
    }
}
