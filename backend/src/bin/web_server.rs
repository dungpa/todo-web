#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};

use backend::rest::*;

fn main() -> Result<(), Error> {
    let allowed_origins = AllowedOrigins::all();

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    rocket::ignite()
        .mount("/", routes![all_tasks::list, all_tasks::add, all_tasks::reset, 
                            single_task::show, single_task::edit, single_task::complete, single_task::remove,
                            stats::count,
                           ])
        .attach(cors)
        .launch();

    Ok(())
}