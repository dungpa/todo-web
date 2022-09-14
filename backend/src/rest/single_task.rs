use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use todo_web::Task;

use crate::db::*;

#[get("/tasks/<id>")]
pub fn show(id: i32) -> Result<Json<Task>, status::NotFound<String>> {
    let conn = establish_connection();
    for db_task in filter_tasks(&conn, id) {
        let api_task = todo_web::Task {
            id: db_task.id,
            title: db_task.title,
            created_at: db_task.created_at,
            completed: db_task.completed,
        };
        return Ok(Json(api_task)); 
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

