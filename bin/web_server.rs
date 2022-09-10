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
    for task in query_task(&conn) {
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

fn main() {
    rocket::ignite()
        .mount("/", routes![tasks_get, tasks_post])
        .launch();
}