#[macro_use]
extern crate seed;
use seed::prelude::*;

#[derive(Clone, Debug)]
enum Direction {
    Coming,
    Going,
}

struct Model {
    direction: Direction,
}

#[derive(Clone, Debug)]
enum Msg {
    Click,
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Click => {
            model.direction = match model.direction {
                Direction::Coming => Direction::Going,
                Direction::Going => Direction::Coming,
            }
        }
    }
}

fn view(model: &Model) -> impl View<Msg> {
    let greeting = match model.direction {
        Direction::Coming => "Hello, World!",
        Direction::Going => "¡Hasta la vista!",
    };
    h1![
        class! {"heading"},
        style!["height" => "100vh",
               "width" => "100vw",
        ],
        { greeting },
        simple_ev(Ev::Click, Msg::Click),
    ]
}

fn init(_url: Url, _orders: &mut impl Orders<Msg>) -> Model {
    Model {
        direction: Direction::Coming,
    }
}

#[wasm_bindgen(start)]
pub fn render() {
    seed::App::build(init, update, view).finish().run();
}

