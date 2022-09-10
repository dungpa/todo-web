use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use todo_web::db::models::*;
use todo_web::db::*;

#[get("/tasks/<id>")]
pub fn get(id: i32) -> Result<Json<Task>, status::NotFound<String>> {
    let conn = establish_connection();
    for task in filter_tasks(&conn, id) {
        return Ok(Json(task)); 
    }
    
    Err(status::NotFound(format!("Error getting task {}.", id)))
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

