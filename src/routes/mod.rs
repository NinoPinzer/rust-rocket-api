use rocket::Route;

mod user_routes;

pub fn get_routes() -> Vec<Route> {
    let mut routes = Vec::new();
    routes.extend(user_routes::get_user_routes());

    routes
}