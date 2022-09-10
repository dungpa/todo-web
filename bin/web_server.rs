#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde;

use rocket_contrib::json::Json;

use todo_web::db::models::Task;
use todo_web::db::*;

pub mod all_tasks;

#[get("/tasks/<id>")]
fn task_get(id: i32) -> Json<Task> {
    let conn = establish_connection();
    // TODO: build a dedicated query for the given id.
    for task in query_tasks(&conn) {
        if task.id == id {
            return Json(task);
        }
    }
    // TODO: return a Result value instead.
    panic!("Task {} not found", id);
}

fn main() {
    rocket::ignite()
        .mount("/", routes![all_tasks::get, all_tasks::post, all_tasks::delete, task_get])
        .launch();
}