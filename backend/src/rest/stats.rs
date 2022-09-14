use rocket_contrib::json::Json;
use crate::db::*;

#[derive(Serialize)]
pub struct StatsResponse {
    pub pending_count: usize,
    pub completed_count: usize,
}

#[get("/stats")]
pub fn count() -> Json<StatsResponse> {
    let conn = establish_connection();
    let tasks = query_tasks(&conn);

    let pending_count = tasks.iter().filter(|task| !task.completed).count();
    let completed_count = tasks.iter().filter(|task| task.completed).count();

    let response = StatsResponse { pending_count, completed_count };
    Json(response)
}