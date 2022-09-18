#[macro_use]
extern crate serde;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub created_at: chrono::NaiveDateTime,
    pub completed: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaskListResponse {
    pub data: Vec<Task>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaskRequest {
    pub title: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaskResponse {
    pub id: i32,
}