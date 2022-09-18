use rocket::http::Status;
use rocket_contrib::json::Json;

use todo_web::{TaskRequest, TaskListResponse};
use crate::db::*;

fn create_api_task (db_task: models::Task) -> todo_web::Task {
    todo_web::Task {
        id: db_task.id,
        title: db_task.title,
        created_at: db_task.created_at,
        completed: db_task.completed,
    }
}

#[get("/tasks")]
pub fn list() -> Json<TaskListResponse> {
    let mut response = TaskListResponse { data: vec![], };

    let conn = establish_connection();
    for db_task in query_tasks(&conn) {
        response.data.push(create_api_task(db_task));
    }

    Json(response)
}

#[post("/tasks", format = "json", data = "<task>")]
pub fn add(task: Json<TaskRequest>) -> Json<TaskListResponse> {
    let mut response = TaskListResponse { data: vec![], };

    let title = task.into_inner().title;
    let conn = establish_connection();
    for db_task in create_task(&conn, &title[..]) {
        response.data.push(create_api_task(db_task));
    } 
    Json(response)
}

#[delete("/tasks")]
pub fn reset() -> Status {
    let conn = establish_connection();
    delete_tasks(&conn); 
    Status::Ok
}
