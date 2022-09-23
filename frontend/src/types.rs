use indexmap::IndexMap;
use seed::{prelude::*};
use todo_web::{Task, TaskResponse, TaskListResponse};

pub struct Model {
    pub tasks: IndexMap<i32, Task>,
    pub new_task_description: String,
}

#[derive(Debug)]
pub enum Msg {
    TasksFetched(fetch::Result<TaskListResponse>),
    NewTaskDescriptionChanged(String),
    AddNewTask,
    NewTaskAdded(fetch::Result<TaskListResponse>),
    CompleteTask(i32),
    TaskCompleted(fetch::Result<TaskResponse>),
    DeleteTask(i32),
    TaskDeleted(fetch::Result<TaskResponse>),
}