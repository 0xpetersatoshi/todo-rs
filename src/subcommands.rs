use clap::{Parser, Subcommand};

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
