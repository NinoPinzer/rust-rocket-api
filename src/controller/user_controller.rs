use rocket::{get, post, http::Status, serde::json::Json};

use crate::models::user::{User, NewUser};
use crate::services::user_service;


#[get("/users")]
pub fn get_users() -> Json<Vec<User>> {
    Json(user_service::get_all_users())
}

#[post("/users", data = "<new_user>")]
pub fn create_user(new_user: Json<NewUser>) -> Result<Json<User>, Status> {
    Ok(Json(user_service::create_user(new_user.into_inner())))
}
