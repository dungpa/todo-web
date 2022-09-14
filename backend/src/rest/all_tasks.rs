use rocket::http::Status;
use rocket_contrib::json::Json;

use todo_web::TaskListResponse;
use crate::db::*;

#[get("/tasks")]
pub fn list() -> Json<TaskListResponse> {
    let mut response = TaskListResponse { data: vec![], };

    let conn = establish_connection();
    for db_task in query_tasks(&conn) {
        let api_task = todo_web::Task {
            id: db_task.id,
            title: db_task.title,
            created_at: db_task.created_at,
            completed: db_task.completed,
        };
        response.data.push(api_task);
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
