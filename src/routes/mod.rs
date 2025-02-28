use rocket::Route;

mod message_route;

pub fn get_routes() -> Vec<Route> {
    let mut routes = Vec::new();
    routes.extend(message_route::message_routes());
    
    routes
}