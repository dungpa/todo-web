use diesel::{prelude::*, sqlite::SqliteConnection};

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    let db = "./testdb.sqlite3";
    SqliteConnection::establish(db)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db))
}

pub fn create_task(connection: &SqliteConnection, title: &str) {
    let task = models::NewTask { title, completed: false };

    diesel::insert_into(schema::task::table)
        .values(&task)
        .execute(connection)
        .expect("Error inserting new task");
}

pub fn query_tasks(connection: &SqliteConnection) -> Vec<models::Task> {
    schema::task::table
        .load::<models::Task>(connection)
        .expect("Error loading tasks")
}

pub fn delete_tasks(connection: &SqliteConnection) {
    diesel::delete(schema::task::table)
        .execute(connection)
        .expect("Error deleting tasks");
}