#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde;

use todo_web::rest::*;

pub mod all_tasks;

fn main() {
    rocket::ignite()
        .mount("/", routes![all_tasks::list, all_tasks::add, all_tasks::reset, 
                            single_task::show, single_task::edit, single_task::complete, single_task::remove,
                            stats::count,
                           ])
        .launch();
}