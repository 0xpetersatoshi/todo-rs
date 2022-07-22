#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;

mod cli;
mod database;
mod models;
mod schema;
mod subcommands;

use crate::subcommands::Commands;
use crate::database::establish_connection;
use clap::Parser;

fn main() {
    let connection = establish_connection();
    let args = cli::Args::parse();

    match args.commands {
        Commands::Add(command) => {
            println!("running the Add subcommand");
            command.run(connection)
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
            println!("running the List subcommand");
            command.run(connection)
        }
    }
}
