use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::db::models::Task;
use crate::db::*;

#[derive(Serialize)]
pub struct TaskListResponse {
    pub data: Vec<Task>,
}

#[get("/tasks")]
pub fn list() -> Json<TaskListResponse> {
    let mut response = TaskListResponse { data: vec![], };

    let conn = establish_connection();
    for task in query_tasks(&conn) {
        response.data.push(task);
    }

    Json(response)
}

#[post("/tasks", data = "<title>")]
pub fn add(title: String) -> Status {
    let conn = establish_connection();
    create_task(&conn, &title[..]); 
    Status::Ok
}

#[delete("/tasks")]
pub fn reset() -> Status {
    let conn = establish_connection();
    delete_tasks(&conn); 
    Status::Ok
}
