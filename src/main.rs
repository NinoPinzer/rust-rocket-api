mod config;
mod middleware;
mod routes;
mod repositories;
mod services;
mod models;
mod controllers;

pub mod schema;

use rocket::launch;
use middleware::cors::CORS;


#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(CORS)
    .attach(config::database::UsersDbConn::fairing())
    .mount("/", routes::get_routes())
    //.register("/", catchers![not_found])
}