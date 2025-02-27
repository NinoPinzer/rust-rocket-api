
mod config;
mod middelware;
mod routes;
mod repositories;
mod services;
mod models;
mod controller;

#[macro_use] extern crate rocket;


#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(config::logger::init())
    .mount("/", routes::get_routes())
    //.register("/", catchers![not_found])
}