#[derive(Queryable)]
pub struct Todos {
    pub id: i32,
    pub todo_name: String,
    pub is_completed: bool,
}
