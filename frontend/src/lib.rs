extern crate seed;

use seed::{prelude::*, *};

use futures::Future;
use seed::{fetch, Request};

use todo_web::{Task, TaskListResponse};

struct Model {
    tasks: Vec<Task>,
}

#[derive(Clone, Debug)]
enum Msg {
    FetchedTasks(fetch::ResponseDataResult<TaskListResponse>),
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FetchedTasks(Ok(mut result)) => {
            model.tasks.clear();
            model.tasks.append(&mut result.data);
        }
        Msg::FetchedTasks(Err(reason)) => {
            log!(format!("Error fetching: {:?}", reason));
        }
    }
}

fn view(model: &Model) -> impl View<Msg> {
    let tasks: Vec<Node<Msg>> = model
        .tasks
        .iter()
        .map(|t| {
            let todo_style = 
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
            let cond_complete =
                if t.completed {
                    empty()
                } else {
                    button! [ 
                        class![ "button", "is-info" ], 
                        {"Complete"}, 
                    ]
                };
            let divider = span! [ 
                    style! [ 
                        St::MarginLeft => px(5),
                        St::MarginRight => px(5), 
                    ] 
                ];
            div! [
                p! [
                    todo_style,
                    { t.title.clone() }
                ],
                cond_complete,
                divider,
                button! [ 
                    class![ "button", "is-danger" ], 
                    {"Delete"}, 
                ]
            ]
        })
        .collect();

    div! [
        style! {
            St::Padding => px(20),
        },
        h1! [
            { "TODO list" },
            style! {
                St::FontSize => px(44),
            },
        ],
        tasks
    ]
}

fn fetch_drills() -> impl Future<Item = Msg, Error = Msg> {
    Request::new("http://localhost:8000/tasks/").fetch_json_data(Msg::FetchedTasks)
}

fn init(_url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(fetch_drills());
    Model { tasks: vec![] }
}

#[wasm_bindgen(start)]
pub fn render() {
    seed::App::build(init, update, view).finish().run();
}
