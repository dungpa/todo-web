use diesel::{prelude::*, sqlite::SqliteConnection};
use crate::db::schema::task::*;

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    let db = "./testdb.sqlite3";
    SqliteConnection::establish(db)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db))
}

pub fn create_task(connection: &SqliteConnection, description: &str) {
    let task = models::NewTask { title: description, created_at: chrono::Utc::now().naive_utc(), completed: false };

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

pub fn filter_tasks(connection: &SqliteConnection, task_id: i32) -> Vec<models::Task> {
    schema::task::table.filter(id.eq(task_id))
        .load::<models::Task>(connection)
        .expect("Error filtering tasks")
}

pub fn update_task(connection: &SqliteConnection, task_id: i32, new_title: &str) {
    diesel::update(schema::task::table.filter(id.eq(task_id)))
        .set(title.eq(new_title))
        .execute(connection)
        .unwrap_or_else(|_| panic!("Error updating task {}", task_id));
}

pub fn complete_task(connection: &SqliteConnection, task_id: i32) {
    diesel::update(schema::task::table.filter(id.eq(task_id)))
        .set(completed.eq(true))
        .execute(connection)
        .unwrap_or_else(|_| panic!("Error updating task {}", task_id));
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