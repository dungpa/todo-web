use super::schema::task;

#[derive(Insertable)]
#[table_name = "task"]
pub struct NewTask<'a> {
    pub title: &'a str,
    pub created_at: chrono::NaiveDateTime,
    pub completed: bool,
}

#[derive(Clone, Debug, Queryable, Deserialize, Serialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub created_at: chrono::NaiveDateTime,
    pub completed: bool,
}

