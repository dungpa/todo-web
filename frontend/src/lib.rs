extern crate seed;

use std::mem;

use futures::Future;
use seed::{prelude::*, *};
use seed::{fetch, Request};

use todo_web::{Task, TaskListResponse};

struct Model {
    tasks: Vec<Task>,
    new_task_description: String,
}

#[derive(Clone, Debug)]
enum Msg {
    FetchedTasks(fetch::ResponseDataResult<TaskListResponse>),
    NewTaskDescriptionChanged(String),
    AddNewTask,
    NewTaskAdded(fetch::ResponseDataResult<String>),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FetchedTasks(Ok(mut result)) => {
            model.tasks.clear();
            model.tasks.append(&mut result.data);
        }
        Msg::FetchedTasks(Err(reason)) => {
            log!(format!("Error fetching: {:?}", reason));
        },
        Msg::NewTaskDescriptionChanged(description) => {
            model.new_task_description = description;
        },
        Msg::AddNewTask => {
            // TODO: id should not be exposed when it is unused.
            let fake_id = 0;
            model.tasks.push(Task {
                id: fake_id,
                title: mem::take(&mut model.new_task_description),
                created_at: chrono::Utc::now().naive_utc(),
                completed: false,
            });
            orders.perform_cmd(Request::new("http://localhost:8000/tasks/")
                .method(Method::Post)
                .send_json(&mut model.new_task_description)
                .fetch_json_data(Msg::NewTaskAdded)
            );
        },
        Msg::NewTaskAdded(_) => {
        }
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
                        At::Placeholder => "task description",
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
        .map(|t| {
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
            let cond_complete =
                if t.completed {
                    empty()
                } else {
                    button! [ 
                        class![ "button", "is-info" ], 
                        { "Complete" }, 
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
    Request::new("http://localhost:8000/tasks/").fetch_json_data(Msg::FetchedTasks)
}

fn init(_url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(fetch_drills());
    Model { tasks: vec![], new_task_description: "".to_owned() }
}

#[wasm_bindgen(start)]
pub fn render() {
    seed::App::build(init, update, view).finish().run();
}
