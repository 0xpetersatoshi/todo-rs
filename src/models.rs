use super::schema::todos;

#[derive(Queryable)]
pub struct Todos {
    pub id: i32,
    pub todo_name: String,
    pub is_completed: bool,
}

#[derive(Insertable)]
#[table_name = "todos"]
pub struct NewTodo<'a> {
    pub todo_name: &'a str,
}
