use rocket::{routes, Route};
use crate::controllers::user_controller::{get_users, create_user};

pub fn get_user_routes() -> Vec<Route> {
    routes![
        get_users,
        create_user
    ]
}