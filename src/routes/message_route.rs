use rocket::routes;

use crate::controllers::message_controller::post_message;

pub fn message_routes() -> Vec<rocket::Route> {
    routes![post_message]
}