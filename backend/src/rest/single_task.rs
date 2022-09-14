use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::db::models::*;
use crate::db::*;

#[get("/tasks/<id>")]
pub fn show(id: i32) -> Result<Json<Task>, status::NotFound<String>> {
    let conn = establish_connection();
    for task in filter_tasks(&conn, id) {
        return Ok(Json(task)); 
    }
    
    Err(status::NotFound(format!("Error getting task {}.", id)))
}

#[post("/tasks/<id>", data = "<title>")]
pub fn edit(id: i32, title: String) -> Status {
    let conn = establish_connection();
    update_task(&conn, id, &title[..]); 
    Status::Ok
}

#[put("/tasks/<id>")]
pub fn complete(id: i32) -> Status {
    let conn = establish_connection();
    complete_task(&conn, id); 
    Status::Ok
}

#[delete("/tasks/<id>")]
pub fn remove(id: i32) -> Status {
    let conn = establish_connection();
    delete_task(&conn, id); 
    Status::Ok
}

