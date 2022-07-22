use crate::models::Todos;
use crate::schema::todos::dsl::*;
use clap::{Parser, Subcommand};
use diesel::prelude::*;
use diesel::{pg::PgConnection, QueryDsl};

#[derive(Subcommand)]
pub enum Commands {
    Add(AddCommand),
    Delete(DeleteCommand),
    Update(UpdateCommand),
    Complete(CompleteCommand),
    List(ListCommand),
}

/// Add a new todo
#[derive(Parser, Debug)]
pub struct AddCommand {
    /// Name of the todo to add
    #[clap(short, long, value_parser)]
    pub name: String,
}

impl AddCommand {
    pub fn run(&self, connection: PgConnection) {
        println!("args: {}", self.name)
    }
}

/// Delete an existing todo
#[derive(Parser, Debug)]
pub struct DeleteCommand {
    /// Name of the todo to delete
    #[clap(short, long, value_parser)]
    pub name: String,
}

/// Update an existing todo
#[derive(Parser, Debug)]
pub struct UpdateCommand {
    /// Name of the todo to update
    #[clap(short, long, value_parser)]
    pub name: String,
}

/// Complete an existing todo
#[derive(Parser, Debug)]
pub struct CompleteCommand {
    /// Name of the todo to complete
    #[clap(short, long, value_parser)]
    pub name: String,
}

/// List existing todos
#[derive(Parser, Debug)]
pub struct ListCommand {}

impl ListCommand {
    pub fn run(&self, connection: PgConnection) {
        let results = todos
            .filter(is_complete.eq(true))
            .load::<Todos>(&connection)
            .expect("Error loading todos");

        if results.len() == 0 {
            println!("No todos to display!");
            return ;
        }

        println!("Displaying {} todos", results.len());
        for result in results {
            println!("todo: {}", result.todo_name)
        }
    }
}
