#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde;

use rocket_contrib::json::Json;

use todo_web::db::models::Task;
use todo_web::db::{query_task, establish_connection};

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

fn main() {
    rocket::ignite()
        .mount("/", routes![tasks_get])
        .launch();
}