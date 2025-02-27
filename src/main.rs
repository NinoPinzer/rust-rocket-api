mod config;
mod middelware;
mod routes;
mod repositories;
mod services;
mod models;
mod controllers;
pub mod schema;

use rocket::launch;

#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(config::database::UsersDbConn::fairing())
    .mount("/", routes::get_routes())
    //.register("/", catchers![not_found])
}