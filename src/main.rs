mod middleware;
mod routes;
mod services;
mod models;
mod controllers;


use rocket::launch;
use middleware::cors::CORS;


#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(CORS)
    .mount("/", routes::get_routes())
    //.register("/", catchers![not_found])
}