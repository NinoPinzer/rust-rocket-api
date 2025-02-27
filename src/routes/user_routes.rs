use rocket::Route;
use crate::controller::user_controller;

pub fn get_user_routes() -> Vec<Route> {
    routes![user_controller::get_users, user_controller::create_user]
}