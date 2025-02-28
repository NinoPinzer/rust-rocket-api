use rocket::{routes, Route};
use crate::controllers::user_controller::{get_users, create_user, get_user_by_id, delete_user_by_id, update_user_by_id};

pub fn get_user_routes() -> Vec<Route> {
    routes![
        get_users,
        create_user,
        get_user_by_id,
        delete_user_by_id,
        update_user_by_id
    ]
}