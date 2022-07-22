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
    /// ID of the todo to delete
    #[clap(short = 'i', long = "id", value_parser)]
    pub todo_id: i32,
}

impl DeleteCommand {
    pub fn run(&self, connection: PgConnection) {
        let name = match get_todo(&connection, self.todo_id) {
            Ok(n) => n,
            Err(e) => {
                println!("Error deleting todo id={}: {}", self.todo_id, e);
                return
            },
        };
        self.delete(&connection);
        println!("Deleted todo: {}", name)
    }

    fn delete(&self, connection: &PgConnection) {
        diesel::delete(todos.find(self.todo_id))
            .execute(connection)
            .expect("Error deleting todo");
    }
}

/// Update an existing todo
#[derive(Parser, Debug)]
pub struct UpdateCommand {
    /// ID of the todo to update
    #[clap(short = 'i', long = "id", value_parser)]
    pub todo_id: i32,
    /// New name for todo item
    #[clap(short, long, value_parser)]
    pub name: String,
}

impl UpdateCommand {
    pub fn run(&self, connection: PgConnection) {
        let old_name = match get_todo(&connection, self.todo_id) {
            Ok(n) => n,
            Err(e) => {
                println!("Error updating todo id={}: {}", self.todo_id, e);
                return ;
            },
        };
        self.update(&connection);
        println!("Updated todo from '{}' to '{}'", old_name, self.name)
    }

    fn update(&self, connection: &PgConnection) {
        diesel::update(todos.find(self.todo_id))
            .set(todo_name.eq(&self.name))
            .execute(connection)
            .expect(&format!("Unable to find todo with id={}", self.todo_id));
    }
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
        self.update(&connection);
        let name = match get_todo(&connection, self.todo_id) {
            Ok(n) => n,
            Err(e) => {
                println!("Error trying to complete todo id={}: {}", self.todo_id, e);
                return ;
            },
        };
        println!("Completed todo: {}", name)
    }

    fn update(&self, connection: &PgConnection) {
        diesel::update(todos.find(self.todo_id))
            .set(is_complete.eq(true))
            .execute(connection)
            .expect(&format!("Unable to find todo with id={}", self.todo_id));
    }
}

/// List existing todos
#[derive(Parser, Debug)]
pub struct ListCommand {
    /// Show completed todos
    #[clap(short, long, takes_value = false)]
    pub completed: bool,
}

impl ListCommand {
    pub fn run(&self, connection: PgConnection) {
        let results = todos
            .filter(is_complete.eq(self.completed))
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

fn get_todo(connection: &PgConnection, todo_id: i32) -> Result<String, diesel::result::Error> {
    todos
        .filter(id.eq(todo_id))
        .select(todo_name)
        .first(connection)
}
