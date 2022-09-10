use rocket::http::Status;
use rocket_contrib::json::Json;

use todo_web::db::models::*;
use todo_web::db::*;

#[get("/tasks/<id>")]
pub fn get(id: i32) -> Json<Task> {
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

#[post("/tasks/<id>")]
pub fn post(id: i32) -> Status {
    // TODO: we should allow updating both completed status and title.
    let conn = establish_connection();
    let completed = true;
    update_task(&conn, id, completed); 
    Status::Ok
}

#[delete("/tasks/<id>")]
pub fn delete(id: i32) -> Status {
    let conn = establish_connection();
    delete_task(&conn, id); 
    Status::Ok
}

