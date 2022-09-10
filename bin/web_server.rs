#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde;

pub mod all_tasks;
pub mod single_task;
pub mod stats;

fn main() {
    rocket::ignite()
        .mount("/", routes![all_tasks::get, all_tasks::post, all_tasks::delete, 
                            single_task::get, single_task::post, single_task::delete,
                            stats::get,
                           ])
        .launch();
}