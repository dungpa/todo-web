use diesel::{prelude::*, sqlite::SqliteConnection};
use crate::db::schema::task::id;

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

pub fn delete_task(connection: &SqliteConnection, task_id: i32) {
    diesel::delete(schema::task::table.filter(id.eq(task_id)))
        .execute(connection)
        .unwrap_or_else(|_| panic!("Error deleting task {}", task_id));
}