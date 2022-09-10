use rocket::http::Status;
use rocket_contrib::json::Json;

use todo_web::db::models::Task;
use todo_web::db::*;

#[derive(Serialize)]
pub struct TaskListResponse {
    pub data: Vec<Task>,
}

#[get("/tasks")]
pub fn get() -> Json<TaskListResponse> {
    let mut response = TaskListResponse { data: vec![], };

    let conn = establish_connection();
    for task in query_tasks(&conn) {
        response.data.push(task);
    }

    Json(response)
}

#[post("/tasks", data = "<title>")]
pub fn post(title: String) -> Status {
    let conn = establish_connection();
    create_task(&conn, &title[..]); 
    Status::Ok
}

#[delete("/tasks")]
pub fn delete() -> Status {
    let conn = establish_connection();
    delete_tasks(&conn); 
    Status::Ok
}
