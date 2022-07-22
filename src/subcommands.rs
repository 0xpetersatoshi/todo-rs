use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum Commands {
    Add(AddCommand),
    Delete(DeleteCommand),
    Update(UpdateCommand),
    Complete(CompleteCommand),
    List(ListCommand),
}

/// Add a new task
#[derive(Parser, Debug)]
pub struct AddCommand {
    /// Name of the task to add
    #[clap(short, long, value_parser)]
    pub name: String,
}

/// Delete an existing task
#[derive(Parser, Debug)]
pub struct DeleteCommand {
    /// Name of the task to delete
    #[clap(short, long, value_parser)]
    pub name: String,
}

/// Update an existing task
#[derive(Parser, Debug)]
pub struct UpdateCommand {
    /// Name of the task to update
    #[clap(short, long, value_parser)]
    pub name: String,
}

/// Complete an existing task
#[derive(Parser, Debug)]
pub struct CompleteCommand {
    /// Name of the task to complete
    #[clap(short, long, value_parser)]
    pub name: String,
}

/// List existing tasks
#[derive(Parser, Debug)]
pub struct ListCommand {}
