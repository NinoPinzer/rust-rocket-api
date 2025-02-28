use rocket::serde::json::Json;
use rocket::serde::json::serde_json;

use crate::models::message::Root;


pub fn post_message(message: Json<Root>) -> String {
    let message = message.into_inner();
    serde_json::to_string(&message).unwrap()
}
