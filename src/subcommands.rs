use crate::models::*;
use crate::schema::todos;
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
        self.create_todo(&connection, &self.name);
        println!("new todo added!")
    }

    fn create_todo<'a>(&self, connection: &PgConnection, name: &'a str) {
        let new_todo = NewTodo { todo_name: name };

        diesel::insert_into(todos::table)
            .values(&new_todo)
            .execute(connection)
            .expect("Error adding new todo");
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
    /// ID of the todo to complete
    #[clap(short = 'i', long = "id", value_parser)]
    pub todo_id: i32,
}

impl CompleteCommand {
    pub fn run(&self, connection: PgConnection) {
        self.update(&connection, self.todo_id);
        let name = get_todo(&connection, self.todo_id);
        println!("Completed todo: {}", name)
    }

    fn update<'a>(&self, connection: &PgConnection, todo_id: i32) {
        diesel::update(todos.find(todo_id))
            .set(is_complete.eq(true))
            .execute(connection)
            .expect(&format!("Unable to find todo with id={}", todo_id));
    }
}

/// List existing todos
#[derive(Parser, Debug)]
pub struct ListCommand {}

impl ListCommand {
    pub fn run(&self, connection: PgConnection) {
        let results = todos
            .filter(is_complete.eq(false))
            .load::<Todo>(&connection)
            .expect("Error loading todos");

        if results.len() == 0 {
            println!("No todos to display!");
            return;
        }

        println!("Displaying {} todos", results.len());
        for result in results {
            println!("(id={}): {}", result.id, result.todo_name)
        }
    }
}

fn get_todo(connection: &PgConnection, todo_id: i32) -> String {
    let result: Result<String, diesel::result::Error> = todos
        .filter(id.eq(todo_id))
        .select(todo_name)
        .first(connection);

    let name = match result {
        Ok(n) => n,
        Err(e) => format!("Error getting todo id={}: {:?}", todo_id, e),
    };

    name
}
