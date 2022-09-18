extern crate seed;

use std::mem;
use indexmap::IndexMap;
use itertools::Itertools;

use futures::Future;
use seed::{prelude::*, *};
use seed::{fetch, Request};

use todo_web::{Task, TaskRequest, TaskResponse, TaskListResponse};

struct Model {
    tasks: IndexMap<i32, Task>,
    new_task_description: String,
}

#[derive(Clone, Debug)]
enum Msg {
    TasksFetched(fetch::ResponseDataResult<TaskListResponse>),
    NewTaskDescriptionChanged(String),
    AddNewTask,
    NewTaskAdded(fetch::ResponseDataResult<TaskListResponse>),
    CompleteTask(i32),
    TaskCompleted(fetch::ResponseDataResult<TaskResponse>),
    DeleteTask(i32),
    TaskDeleted(fetch::ResponseDataResult<TaskResponse>),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::TasksFetched(Ok(result)) => {
            model.tasks.clear();
            for task in result.data {
                model.tasks.insert(task.id, task);
            }
        }
        Msg::TasksFetched(Err(reason)) => {
            log!(format!("Error fetching: {:?}", reason));
        },
        Msg::NewTaskDescriptionChanged(description) => {
            model.new_task_description = description;
        },
        Msg::AddNewTask => {
            let task_request = TaskRequest { title: mem::take(&mut model.new_task_description) };
            orders.perform_cmd(Request::new("http://localhost:8000/tasks/")
                .method(Method::Post)
                .send_json(&task_request)
                .fetch_json_data(Msg::NewTaskAdded)
            );
        },
        Msg::NewTaskAdded(Ok(result)) => {
            for task in result.data {
                model.tasks.insert(task.id, task);
            }
        },
        Msg::NewTaskAdded(Err(reason)) => {
            log!(format!("Error adding a new task: {:?}", reason));
        },
        Msg::CompleteTask(task_id) => {
            orders.perform_cmd(Request::new(format!("http://localhost:8000/tasks/{}/", task_id))
                .method(Method::Put)
                .fetch_json_data(Msg::TaskCompleted)
            );
        },
        Msg::TaskCompleted(Ok(result)) => {
            if let Some(value) = model.tasks.get(&result.id) {
                let new_value = Task {
                    id: value.id,
                    title: value.title.clone(),
                    created_at: value.created_at,
                    completed: true,
                };
                model.tasks.insert(result.id, new_value);
            };
        },
        Msg::TaskCompleted(Err(reason)) => {
            log!(format!("Error completing a task: {:?}", reason));
        },
        Msg::DeleteTask(task_id) => {
            orders.perform_cmd(Request::new(format!("http://localhost:8000/tasks/{}/", task_id))
                .method(Method::Delete)
                .fetch_json_data(Msg::TaskDeleted)
            );
        },
        Msg::TaskDeleted(Ok(result)) => {
            model.tasks.remove(&result.id);
        },
        Msg::TaskDeleted(Err(reason)) => {
            log!(format!("Error deleting a task: {:?}", reason));
        },
    }
}

fn view(model: &Model) -> impl View<Msg> {
    let add_new_task =
        div! [
            class! [ "field", "has-addons"],
            style! [
                St::Padding => px(5),
                St::Width => px(400),
            ],
            div! [
                class! ["control", "is-large"],
                input! [
                    class! ["input", "is-large"],
                    attrs! {
                        At::Placeholder => "Todo description",
                        At::Value => model.new_task_description;
                    },
                    input_ev(Ev::Input, Msg::NewTaskDescriptionChanged),
                ],
            ],
            div! [
                class! ["control", "is-large"],
                button! [ 
                    class![ "button", "is-primary", "is-large" ], 
                    { "Add Todo" }, 
                    raw_ev(Ev::Click, |_| Msg::AddNewTask),
                ],
            ]
        ];
    let current_tasks: Vec<Node<Msg>> = model
        .tasks
        .iter()
        .sorted_by(|(k1, _), (k2, _)| k1.cmp(k2))
        .map(|(_, t)| {
            let task_style = 
                if t.completed {
                    style! {
                        St::Color => "red",
                        St::FontSize => px(19),
                        St::Padding => px(5),
                        St::TextDecoration => "line-through",
                    }
                } else {
                    style! {
                        St::Color => "green",
                        St::FontSize => px(19),
                        St::Padding => px(5),
                    }
                };
            let title =
                p! [
                    task_style,
                    { t.title.clone() },
                ];
            let id = t.id;
            let cond_complete =
                if t.completed {
                    empty()
                } else {
                    button! [ 
                        class![ "button", "is-info" ], 
                        { "Complete" },
                        raw_ev(Ev::Click, move |_| Msg::CompleteTask(id)), 
                    ]
                };
            let divider = span! [ 
                    style! [ 
                        St::MarginLeft => px(5),
                        St::MarginRight => px(5), 
                    ] 
                ];
            let delete =
                button! [ 
                    class![ "button", "is-danger" ], 
                    { "Delete" }, 
                    raw_ev(Ev::Click, move |_| Msg::DeleteTask(id)),
                ];
            div! [
                title,
                cond_complete,
                divider,
                delete,
            ]
        })
        .collect();

    div! [
        style! {
            St::Padding => px(20),
        },
        h1! [
            { "Todo list" },
            style! {
                St::FontSize => px(44),
            },
        ],
        add_new_task,
        current_tasks,
    ]
}

fn fetch_drills() -> impl Future<Item = Msg, Error = Msg> {
    Request::new("http://localhost:8000/tasks/").fetch_json_data(Msg::TasksFetched)
}

fn init(_url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(fetch_drills());
    Model { tasks: IndexMap::new(), new_task_description: "".to_owned() }
}

#[wasm_bindgen(start)]
pub fn render() {
    seed::App::build(init, update, view).finish().run();
}
