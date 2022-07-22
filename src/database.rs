use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_migrations::embed_migrations;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    // Run migrations
    embed_migrations!();
    match embedded_migrations::run(&connection) {
        Err(e) => println!("Error running migrations: {:?}", e),
        _ => ()
    }

    connection
}
