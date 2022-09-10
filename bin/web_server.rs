#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate serde;

use rocket::http::Status;
use rocket_contrib::json::Json;

use todo_web::db::models::Task;
use todo_web::db::*;

#[derive(Serialize)]
struct JsonApiResponse {
    data: Vec<Task>,
}

#[get("/tasks")]
fn tasks_get() -> Json<JsonApiResponse> {
    let mut response = JsonApiResponse { data: vec![], };

    let conn = establish_connection();
    for task in query_tasks(&conn) {
        response.data.push(task);
    }

    Json(response)
}

#[post("/tasks", data = "<title>")]
fn tasks_post(title: String) -> Status {
    let conn = establish_connection();
    create_task(&conn, &title[..]); 
    Status::Ok
}

#[delete("/tasks")]
fn tasks_delete() -> Status {
    let conn = establish_connection();
    delete_tasks(&conn); 
    Status::Ok
}

#[get("/tasks/<id>")]
fn task_get(id: i32) -> Json<Task> {
    let conn = establish_connection();
    // TODO: build a dedicated query for the given id.
    for task in query_tasks(&conn) {
        if task.id == id {
            return Json(task);
        }
    }

    panic!("Task {} not found", id);
}

fn main() {
    rocket::ignite()
        .mount("/", routes![tasks_get, tasks_post, tasks_delete, task_get])
        .launch();
}