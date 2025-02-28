use rocket::{post, serde::json::Json};

use crate::models::message::Root;
use crate::services::message_service;

#[post("/message", format = "json", data = "<message>")]
pub fn post_message(message: Json<Root>) -> String {
    message_service::post_message(message)
}